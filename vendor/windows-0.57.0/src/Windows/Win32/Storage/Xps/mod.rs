#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AbortDoc<P0>(hdc: P0) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn AbortDoc(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    AbortDoc(hdc.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DeviceCapabilitiesA<P0, P1>(pdevice: P0, pport: P1, fwcapability: PRINTER_DEVICE_CAPABILITIES, poutput: windows_core::PSTR, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEA>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("winspool.drv" "system" fn DeviceCapabilitiesA(pdevice : windows_core::PCSTR, pport : windows_core::PCSTR, fwcapability : PRINTER_DEVICE_CAPABILITIES, poutput : windows_core::PSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA) -> i32);
    DeviceCapabilitiesA(pdevice.param().abi(), pport.param().abi(), fwcapability, core::mem::transmute(poutput), core::mem::transmute(pdevmode.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DeviceCapabilitiesW<P0, P1>(pdevice: P0, pport: P1, fwcapability: PRINTER_DEVICE_CAPABILITIES, poutput: windows_core::PWSTR, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEW>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("winspool.drv" "system" fn DeviceCapabilitiesW(pdevice : windows_core::PCWSTR, pport : windows_core::PCWSTR, fwcapability : PRINTER_DEVICE_CAPABILITIES, poutput : windows_core::PWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW) -> i32);
    DeviceCapabilitiesW(pdevice.param().abi(), pport.param().abi(), fwcapability, core::mem::transmute(poutput), core::mem::transmute(pdevmode.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndDoc<P0>(hdc: P0) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn EndDoc(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    EndDoc(hdc.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndPage<P0>(hdc: P0) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn EndPage(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    EndPage(hdc.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn Escape<P0>(hdc: P0, iescape: i32, pvin: Option<&[u8]>, pvout: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn Escape(hdc : super::super::Graphics::Gdi:: HDC, iescape : i32, cjin : i32, pvin : windows_core::PCSTR, pvout : *mut core::ffi::c_void) -> i32);
    Escape(hdc.param().abi(), iescape, pvin.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pvin.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(pvout.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ExtEscape<P0>(hdc: P0, iescape: i32, lpindata: Option<&[u8]>, lpoutdata: Option<&mut [u8]>) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn ExtEscape(hdc : super::super::Graphics::Gdi:: HDC, iescape : i32, cjinput : i32, lpindata : windows_core::PCSTR, cjoutput : i32, lpoutdata : windows_core::PSTR) -> i32);
    ExtEscape(hdc.param().abi(), iescape, lpindata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpindata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpoutdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpoutdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn PrintWindow<P0, P1>(hwnd: P0, hdcblt: P1, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("user32.dll" "system" fn PrintWindow(hwnd : super::super::Foundation:: HWND, hdcblt : super::super::Graphics::Gdi:: HDC, nflags : PRINT_WINDOW_FLAGS) -> super::super::Foundation:: BOOL);
    PrintWindow(hwnd.param().abi(), hdcblt.param().abi(), nflags)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetAbortProc<P0>(hdc: P0, proc: ABORTPROC) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn SetAbortProc(hdc : super::super::Graphics::Gdi:: HDC, proc : ABORTPROC) -> i32);
    SetAbortProc(hdc.param().abi(), proc)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocA<P0>(hdc: P0, lpdi: *const DOCINFOA) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn StartDocA(hdc : super::super::Graphics::Gdi:: HDC, lpdi : *const DOCINFOA) -> i32);
    StartDocA(hdc.param().abi(), lpdi)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocW<P0>(hdc: P0, lpdi: *const DOCINFOW) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn StartDocW(hdc : super::super::Graphics::Gdi:: HDC, lpdi : *const DOCINFOW) -> i32);
    StartDocW(hdc.param().abi(), lpdi)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartPage<P0>(hdc: P0) -> i32
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("gdi32.dll" "system" fn StartPage(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    StartPage(hdc.param().abi())
}
windows_core::imp::define_interface!(IXpsDocumentPackageTarget, IXpsDocumentPackageTarget_Vtbl, 0x3b0b6d38_53ad_41da_b212_d37637a6714e);
impl core::ops::Deref for IXpsDocumentPackageTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsDocumentPackageTarget, windows_core::IUnknown);
impl IXpsDocumentPackageTarget {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter<P0, P1>(&self, documentsequencepartname: P0, discardcontrolpartname: P1) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXpsOMPackageWriter)(windows_core::Interface::as_raw(self), documentsequencepartname.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXpsOMFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetXpsType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXpsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXpsDocumentPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetXpsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsDocumentPackageTarget3D, IXpsDocumentPackageTarget3D_Vtbl, 0x60ba71b8_3101_4984_9199_f4ea775ff01d);
impl core::ops::Deref for IXpsDocumentPackageTarget3D {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsDocumentPackageTarget3D, windows_core::IUnknown);
impl IXpsDocumentPackageTarget3D {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter3D<P0, P1, P2, P3>(&self, documentsequencepartname: P0, discardcontrolpartname: P1, modelpartname: P2, modeldata: P3) -> windows_core::Result<IXpsOMPackageWriter3D>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXpsOMPackageWriter3D)(windows_core::Interface::as_raw(self), documentsequencepartname.param().abi(), discardcontrolpartname.param().abi(), modelpartname.param().abi(), modeldata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXpsOMFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsDocumentPackageTarget3D_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter3D: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMBrush, IXpsOMBrush_Vtbl, 0x56a3f80c_ea4c_4187_a57b_a2a473b2b42b);
impl core::ops::Deref for IXpsOMBrush {
    type Target = IXpsOMShareable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMBrush, windows_core::IUnknown, IXpsOMShareable);
impl IXpsOMBrush {
    pub unsafe fn GetOpacity(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity).ok()
    }
}
#[repr(C)]
pub struct IXpsOMBrush_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMCanvas, IXpsOMCanvas_Vtbl, 0x221d1452_331e_47c6_87e9_6ccefb9b5ba3);
impl core::ops::Deref for IXpsOMCanvas {
    type Target = IXpsOMVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMCanvas, windows_core::IUnknown, IXpsOMShareable, IXpsOMVisual);
impl IXpsOMCanvas {
    pub unsafe fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisuals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUseAliasedEdgeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUseAliasedEdgeMode<P0>(&self, usealiasededgemode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseAliasedEdgeMode)(windows_core::Interface::as_raw(self), usealiasededgemode.param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), shortdescription.param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), longdescription.param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionaryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
    {
        (windows_core::Interface::vtable(self).SetDictionaryLocal)(windows_core::Interface::as_raw(self), resourcedictionary.param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionaryResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        (windows_core::Interface::vtable(self).SetDictionaryResource)(windows_core::Interface::as_raw(self), remotedictionaryresource.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMCanvas> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMCanvas_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUseAliasedEdgeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUseAliasedEdgeMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMColorProfileResource, IXpsOMColorProfileResource_Vtbl, 0x67bd7d69_1eef_4bb1_b5e7_6f4f87be8abe);
impl core::ops::Deref for IXpsOMColorProfileResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMColorProfileResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMColorProfileResource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMColorProfileResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
windows_core::imp::define_interface!(IXpsOMColorProfileResourceCollection, IXpsOMColorProfileResourceCollection_Vtbl, 0x12759630_5fba_4283_8f7d_cca849809edb);
impl core::ops::Deref for IXpsOMColorProfileResourceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMColorProfileResourceCollection, windows_core::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMColorProfileResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMColorProfileResourceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
windows_core::imp::define_interface!(IXpsOMCoreProperties, IXpsOMCoreProperties_Vtbl, 0x3340fe8f_4027_4aa1_8f5f_d35ae45fe597);
impl core::ops::Deref for IXpsOMCoreProperties {
    type Target = IXpsOMPart;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMCoreProperties, windows_core::IUnknown, IXpsOMPart);
impl IXpsOMCoreProperties {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMPackage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCategory<P0>(&self, category: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCategory)(windows_core::Interface::as_raw(self), category.param().abi()).ok()
    }
    pub unsafe fn GetContentStatus(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetContentStatus<P0>(&self, contentstatus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetContentStatus)(windows_core::Interface::as_raw(self), contentstatus.param().abi()).ok()
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetContentType<P0>(&self, contenttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetContentType)(windows_core::Interface::as_raw(self), contenttype.param().abi()).ok()
    }
    pub unsafe fn GetCreated(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCreated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCreated)(windows_core::Interface::as_raw(self), created).ok()
    }
    pub unsafe fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCreator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreator<P0>(&self, creator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCreator)(windows_core::Interface::as_raw(self), creator.param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn GetIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIdentifier<P0>(&self, identifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetIdentifier)(windows_core::Interface::as_raw(self), identifier.param().abi()).ok()
    }
    pub unsafe fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKeywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeywords<P0>(&self, keywords: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetKeywords)(windows_core::Interface::as_raw(self), keywords.param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), language.param().abi()).ok()
    }
    pub unsafe fn GetLastModifiedBy(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastModifiedBy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLastModifiedBy<P0>(&self, lastmodifiedby: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLastModifiedBy)(windows_core::Interface::as_raw(self), lastmodifiedby.param().abi()).ok()
    }
    pub unsafe fn GetLastPrinted(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastPrinted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLastPrinted)(windows_core::Interface::as_raw(self), lastprinted).ok()
    }
    pub unsafe fn GetModified(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetModified)(windows_core::Interface::as_raw(self), modified).ok()
    }
    pub unsafe fn GetRevision(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRevision<P0>(&self, revision: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRevision)(windows_core::Interface::as_raw(self), revision.param().abi()).ok()
    }
    pub unsafe fn GetSubject(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubject)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSubject<P0>(&self, subject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), subject.param().abi()).ok()
    }
    pub unsafe fn GetTitle(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTitle<P0>(&self, title: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), title.param().abi()).ok()
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVersion<P0>(&self, version: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), version.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMCoreProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMCoreProperties_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetContentStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetContentStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub SetCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub GetCreator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLastModifiedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLastModifiedBy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLastPrinted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub SetLastPrinted: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub GetModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub SetModified: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub GetRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMDashCollection, IXpsOMDashCollection_Vtbl, 0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
impl core::ops::Deref for IXpsOMDashCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMDashCollection, windows_core::IUnknown);
impl IXpsOMDashCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<XPS_DASH> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, dash).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, dash).ok()
    }
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), dash).ok()
    }
}
#[repr(C)]
pub struct IXpsOMDashCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut XPS_DASH) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XPS_DASH) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XPS_DASH) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_DASH) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMDictionary, IXpsOMDictionary_Vtbl, 0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
impl core::ops::Deref for IXpsOMDictionary {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMDictionary, windows_core::IUnknown);
impl IXpsOMDictionary {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32, key: *mut windows_core::PWSTR) -> windows_core::Result<IXpsOMShareable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetByKey<P0, P1>(&self, key: P0, beforeentry: P1) -> windows_core::Result<IXpsOMShareable>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IXpsOMShareable>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByKey)(windows_core::Interface::as_raw(self), key.param().abi(), beforeentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIndex<P0>(&self, entry: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IXpsOMShareable>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), entry.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Append<P0, P1>(&self, key: P0, entry: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IXpsOMShareable>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), key.param().abi(), entry.param().abi()).ok()
    }
    pub unsafe fn InsertAt<P0, P1>(&self, index: u32, key: P0, entry: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IXpsOMShareable>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, key.param().abi(), entry.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0, P1>(&self, index: u32, key: P0, entry: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IXpsOMShareable>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, key.param().abi(), entry.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMDictionary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetByKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMDocument, IXpsOMDocument_Vtbl, 0x2c2c94cb_ac5f_4254_8ee9_23948309d9f0);
impl core::ops::Deref for IXpsOMDocument {
    type Target = IXpsOMPart;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMDocument, windows_core::IUnknown, IXpsOMPart);
impl IXpsOMDocument {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMDocumentSequence> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPageReferences(&self) -> windows_core::Result<IXpsOMPageReferenceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPageReferences)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrintTicketResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPrintTicketResource>,
    {
        (windows_core::Interface::vtable(self).SetPrintTicketResource)(windows_core::Interface::as_raw(self), printticketresource.param().abi()).ok()
    }
    pub unsafe fn GetDocumentStructureResource(&self) -> windows_core::Result<IXpsOMDocumentStructureResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentStructureResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDocumentStructureResource<P0>(&self, documentstructureresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocumentStructureResource>,
    {
        (windows_core::Interface::vtable(self).SetDocumentStructureResource)(windows_core::Interface::as_raw(self), documentstructureresource.param().abi()).ok()
    }
    pub unsafe fn GetSignatureBlockResources(&self) -> windows_core::Result<IXpsOMSignatureBlockResourceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureBlockResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMDocument_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPageReferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentStructureResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentStructureResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSignatureBlockResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMDocumentCollection, IXpsOMDocumentCollection_Vtbl, 0xd1c87f0d_e947_4754_8a25_971478f7e83e);
impl core::ops::Deref for IXpsOMDocumentCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMDocumentCollection, windows_core::IUnknown);
impl IXpsOMDocumentCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, document: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocument>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, document.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, document: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocument>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, document.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, document: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocument>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), document.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMDocumentCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMDocumentSequence, IXpsOMDocumentSequence_Vtbl, 0x56492eb4_d8d5_425e_8256_4c2b64ad0264);
impl core::ops::Deref for IXpsOMDocumentSequence {
    type Target = IXpsOMPart;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMDocumentSequence, windows_core::IUnknown, IXpsOMPart);
impl IXpsOMDocumentSequence {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMPackage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocuments(&self) -> windows_core::Result<IXpsOMDocumentCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocuments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrintTicketResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPrintTicketResource>,
    {
        (windows_core::Interface::vtable(self).SetPrintTicketResource)(windows_core::Interface::as_raw(self), printticketresource.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMDocumentSequence_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMDocumentStructureResource, IXpsOMDocumentStructureResource_Vtbl, 0x85febc8a_6b63_48a9_af07_7064e4ecff30);
impl core::ops::Deref for IXpsOMDocumentStructureResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMDocumentStructureResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMDocumentStructureResource {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMDocumentStructureResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
windows_core::imp::define_interface!(IXpsOMFontResource, IXpsOMFontResource_Vtbl, 0xa8c45708_47d9_4af4_8d20_33b48c9b8485);
impl core::ops::Deref for IXpsOMFontResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMFontResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMFontResource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, embeddingoption: XPS_FONT_EMBEDDING, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), embeddingoption, partname.param().abi()).ok()
    }
    pub unsafe fn GetEmbeddingOption(&self) -> windows_core::Result<XPS_FONT_EMBEDDING> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEmbeddingOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXpsOMFontResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_FONT_EMBEDDING, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetEmbeddingOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_FONT_EMBEDDING) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMFontResourceCollection, IXpsOMFontResourceCollection_Vtbl, 0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
impl core::ops::Deref for IXpsOMFontResourceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMFontResourceCollection, windows_core::IUnknown);
impl IXpsOMFontResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMFontResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok()
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMFontResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMFontResourceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
windows_core::imp::define_interface!(IXpsOMGeometry, IXpsOMGeometry_Vtbl, 0x64fcf3d7_4d58_44ba_ad73_a13af6492072);
impl core::ops::Deref for IXpsOMGeometry {
    type Target = IXpsOMShareable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGeometry, windows_core::IUnknown, IXpsOMShareable);
impl IXpsOMGeometry {
    pub unsafe fn GetFigures(&self) -> windows_core::Result<IXpsOMGeometryFigureCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFigures)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFillRule(&self) -> windows_core::Result<XPS_FILL_RULE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFillRule)(windows_core::Interface::as_raw(self), fillrule).ok()
    }
    pub unsafe fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), transform.param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMGeometry_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetFigures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFillRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_FILL_RULE) -> windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_FILL_RULE) -> windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGeometryFigure, IXpsOMGeometryFigure_Vtbl, 0xd410dc83_908c_443e_8947_b1795d3c165a);
impl core::ops::Deref for IXpsOMGeometryFigure {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGeometryFigure, windows_core::IUnknown);
impl IXpsOMGeometryFigure {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSegmentData)(windows_core::Interface::as_raw(self), datacount, segmentdata).ok()
    }
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSegmentTypes)(windows_core::Interface::as_raw(self), segmentcount, segmenttypes).ok()
    }
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSegmentStrokes)(windows_core::Interface::as_raw(self), segmentcount, segmentstrokes).ok()
    }
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSegments)(windows_core::Interface::as_raw(self), segmentcount, segmentdatacount, segmenttypes, segmentdata, segmentstrokes).ok()
    }
    pub unsafe fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), startpoint).ok()
    }
    pub unsafe fn GetIsClosed(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsClosed<P0>(&self, isclosed: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsClosed)(windows_core::Interface::as_raw(self), isclosed.param().abi()).ok()
    }
    pub unsafe fn GetIsFilled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsFilled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsFilled<P0>(&self, isfilled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsFilled)(windows_core::Interface::as_raw(self), isfilled.param().abi()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSegmentDataCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSegmentDataCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSegmentStrokePattern(&self) -> windows_core::Result<XPS_SEGMENT_STROKE_PATTERN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSegmentStrokePattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMGeometryFigure_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut f32) -> windows_core::HRESULT,
    pub GetSegmentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut XPS_SEGMENT_TYPE) -> windows_core::HRESULT,
    pub GetSegmentStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const XPS_SEGMENT_TYPE, *const f32, *const super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub GetIsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetIsFilled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSegmentStrokePattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SEGMENT_STROKE_PATTERN) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGeometryFigureCollection, IXpsOMGeometryFigureCollection_Vtbl, 0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
impl core::ops::Deref for IXpsOMGeometryFigureCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGeometryFigureCollection, windows_core::IUnknown);
impl IXpsOMGeometryFigureCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, geometryfigure: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometryFigure>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, geometryfigure.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, geometryfigure: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometryFigure>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, geometryfigure.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, geometryfigure: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometryFigure>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), geometryfigure.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMGeometryFigureCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGlyphs, IXpsOMGlyphs_Vtbl, 0x819b3199_0a5a_4b64_bec7_a9e17e780de2);
impl core::ops::Deref for IXpsOMGlyphs {
    type Target = IXpsOMVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGlyphs, windows_core::IUnknown, IXpsOMShareable, IXpsOMVisual);
impl IXpsOMGlyphs {
    pub unsafe fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUnicodeString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphIndexCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGlyphIndices)(windows_core::Interface::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphMappingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGlyphMappings)(windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProhibitedCaretStopCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProhibitedCaretStops)(windows_core::Interface::as_raw(self), prohibitedcaretstopcount, prohibitedcaretstops).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBidiLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIsSideways(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsSideways)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceFontName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStyleSimulations(&self) -> windows_core::Result<XPS_STYLE_SIMULATION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStyleSimulations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStyleSimulations)(windows_core::Interface::as_raw(self), stylesimulations).ok()
    }
    pub unsafe fn GetOrigin(&self) -> windows_core::Result<XPS_POINT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOrigin)(windows_core::Interface::as_raw(self), origin).ok()
    }
    pub unsafe fn GetFontRenderingEmSize(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontRenderingEmSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontRenderingEmSize)(windows_core::Interface::as_raw(self), fontrenderingemsize).ok()
    }
    pub unsafe fn GetFontResource(&self) -> windows_core::Result<IXpsOMFontResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFontResource<P0>(&self, fontresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        (windows_core::Interface::vtable(self).SetFontResource)(windows_core::Interface::as_raw(self), fontresource.param().abi()).ok()
    }
    pub unsafe fn GetFontFaceIndex(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontFaceIndex)(windows_core::Interface::as_raw(self), fontfaceindex).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, fillbrush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        (windows_core::Interface::vtable(self).SetFillBrushLocal)(windows_core::Interface::as_raw(self), fillbrush.param().abi()).ok()
    }
    pub unsafe fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFillBrushLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn GetGlyphsEditor(&self) -> windows_core::Result<IXpsOMGlyphsEditor> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphsEditor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGlyphs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMGlyphs_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetUnicodeString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetIsSideways: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDeviceFontName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetStyleSimulations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_STYLE_SIMULATION) -> windows_core::HRESULT,
    pub SetStyleSimulations: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_STYLE_SIMULATION) -> windows_core::HRESULT,
    pub GetOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub GetFontRenderingEmSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetFontFaceIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetGlyphsEditor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGlyphsEditor, IXpsOMGlyphsEditor_Vtbl, 0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
impl core::ops::Deref for IXpsOMGlyphsEditor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGlyphsEditor, windows_core::IUnknown);
impl IXpsOMGlyphsEditor {
    pub unsafe fn ApplyEdits(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyEdits)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUnicodeString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUnicodeString<P0>(&self, unicodestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetUnicodeString)(windows_core::Interface::as_raw(self), unicodestring.param().abi()).ok()
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphIndexCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGlyphIndices)(windows_core::Interface::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGlyphIndices)(windows_core::Interface::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphMappingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGlyphMappings)(windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGlyphMappings)(windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProhibitedCaretStopCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProhibitedCaretStops)(windows_core::Interface::as_raw(self), count, prohibitedcaretstops).ok()
    }
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProhibitedCaretStops)(windows_core::Interface::as_raw(self), count, prohibitedcaretstops).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBidiLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBidiLevel)(windows_core::Interface::as_raw(self), bidilevel).ok()
    }
    pub unsafe fn GetIsSideways(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsSideways)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsSideways<P0>(&self, issideways: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsSideways)(windows_core::Interface::as_raw(self), issideways.param().abi()).ok()
    }
    pub unsafe fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceFontName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDeviceFontName<P0>(&self, devicefontname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDeviceFontName)(windows_core::Interface::as_raw(self), devicefontname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMGlyphsEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ApplyEdits: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUnicodeString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT,
    pub SetGlyphIndices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XPS_GLYPH_INDEX) -> windows_core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT,
    pub SetGlyphMappings: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XPS_GLYPH_MAPPING) -> windows_core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetProhibitedCaretStops: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetIsSideways: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIsSideways: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDeviceFontName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDeviceFontName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGradientBrush, IXpsOMGradientBrush_Vtbl, 0xedb59622_61a2_42c3_bace_acf2286c06bf);
impl core::ops::Deref for IXpsOMGradientBrush {
    type Target = IXpsOMBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGradientBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush);
impl IXpsOMGradientBrush {
    pub unsafe fn GetGradientStops(&self) -> windows_core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGradientStops)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), transform.param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> windows_core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpreadMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSpreadMethod)(windows_core::Interface::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> windows_core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColorInterpolationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorInterpolationMode)(windows_core::Interface::as_raw(self), colorinterpolationmode).ok()
    }
}
#[repr(C)]
pub struct IXpsOMGradientBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetGradientStops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSpreadMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SPREAD_METHOD) -> windows_core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_SPREAD_METHOD) -> windows_core::HRESULT,
    pub GetColorInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGradientStop, IXpsOMGradientStop_Vtbl, 0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
impl core::ops::Deref for IXpsOMGradientStop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGradientStop, windows_core::IUnknown);
impl IXpsOMGradientStop {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMGradientBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOffset(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOffset(&self, offset: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOffset)(windows_core::Interface::as_raw(self), offset).ok()
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), color, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetColor<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGradientStop> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMGradientStop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_COLOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_COLOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMGradientStopCollection, IXpsOMGradientStopCollection_Vtbl, 0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
impl core::ops::Deref for IXpsOMGradientStopCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMGradientStopCollection, windows_core::IUnknown);
impl IXpsOMGradientStopCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGradientStop> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, stop.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, stop.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), stop.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMGradientStopCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMImageBrush, IXpsOMImageBrush_Vtbl, 0x3df0b466_d382_49ef_8550_dd94c80242e4);
impl core::ops::Deref for IXpsOMImageBrush {
    type Target = IXpsOMTileBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMImageBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMTileBrush);
impl IXpsOMImageBrush {
    pub unsafe fn GetImageResource(&self) -> windows_core::Result<IXpsOMImageResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetImageResource<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).SetImageResource)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok()
    }
    pub unsafe fn GetColorProfileResource(&self) -> windows_core::Result<IXpsOMColorProfileResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColorProfileResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetColorProfileResource<P0>(&self, colorprofileresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        (windows_core::Interface::vtable(self).SetColorProfileResource)(windows_core::Interface::as_raw(self), colorprofileresource.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMImageBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMImageBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetImageResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetImageResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorProfileResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorProfileResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMImageResource, IXpsOMImageResource_Vtbl, 0x3db8417d_ae50_485e_9a44_d7758f78a23f);
impl core::ops::Deref for IXpsOMImageResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMImageResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMImageResource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, imagetype: XPS_IMAGE_TYPE, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), imagetype, partname.param().abi()).ok()
    }
    pub unsafe fn GetImageType(&self) -> windows_core::Result<XPS_IMAGE_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXpsOMImageResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_IMAGE_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetImageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_IMAGE_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMImageResourceCollection, IXpsOMImageResourceCollection_Vtbl, 0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
impl core::ops::Deref for IXpsOMImageResourceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMImageResourceCollection, windows_core::IUnknown);
impl IXpsOMImageResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMImageResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMImageResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMImageResourceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
windows_core::imp::define_interface!(IXpsOMLinearGradientBrush, IXpsOMLinearGradientBrush_Vtbl, 0x005e279f_c30d_40ff_93ec_1950d3c528db);
impl core::ops::Deref for IXpsOMLinearGradientBrush {
    type Target = IXpsOMGradientBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMLinearGradientBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMGradientBrush);
impl IXpsOMLinearGradientBrush {
    pub unsafe fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), startpoint).ok()
    }
    pub unsafe fn GetEndPoint(&self) -> windows_core::Result<XPS_POINT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEndPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEndPoint)(windows_core::Interface::as_raw(self), endpoint).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMLinearGradientBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMLinearGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub GetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMMatrixTransform, IXpsOMMatrixTransform_Vtbl, 0xb77330ff_bb37_4501_a93e_f1b1e50bfc46);
impl core::ops::Deref for IXpsOMMatrixTransform {
    type Target = IXpsOMShareable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMMatrixTransform, windows_core::IUnknown, IXpsOMShareable);
impl IXpsOMMatrixTransform {
    pub unsafe fn GetMatrix(&self) -> windows_core::Result<XPS_MATRIX> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatrix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMMatrixTransform_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_MATRIX) -> windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_MATRIX) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMNameCollection, IXpsOMNameCollection_Vtbl, 0x4bddf8ec_c915_421b_a166_d173d25653d2);
impl core::ops::Deref for IXpsOMNameCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMNameCollection, windows_core::IUnknown);
impl IXpsOMNameCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXpsOMNameCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMObjectFactory, IXpsOMObjectFactory_Vtbl, 0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
impl core::ops::Deref for IXpsOMObjectFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMObjectFactory, windows_core::IUnknown);
impl IXpsOMObjectFactory {
    pub unsafe fn CreatePackage(&self) -> windows_core::Result<IXpsOMPackage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePackageFromFile<P0, P1>(&self, filename: P0, reuseobjects: P1) -> windows_core::Result<IXpsOMPackage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), reuseobjects.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageFromStream<P0, P1>(&self, stream: P0, reuseobjects: P1) -> windows_core::Result<IXpsOMPackage>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageFromStream)(windows_core::Interface::as_raw(self), stream.param().abi(), reuseobjects.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMStoryFragmentsResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStoryFragmentsResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMDocumentStructureResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDocumentStructureResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSignatureBlockResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<P0, P1>(&self, dictionary: P0, parturi: P1) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRemoteDictionaryResource)(windows_core::Interface::as_raw(self), dictionary.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<P0, P1, P2>(&self, dictionarymarkupstream: P0, dictionaryparturi: P1, resources: P2) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream)(windows_core::Interface::as_raw(self), dictionarymarkupstream.param().abi(), dictionaryparturi.param().abi(), resources.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePartResources(&self) -> windows_core::Result<IXpsOMPartResources> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePartResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<P0>(&self, parturi: P0) -> windows_core::Result<IXpsOMDocumentSequence>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDocumentSequence)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<P0>(&self, parturi: P0) -> windows_core::Result<IXpsOMDocument>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDocument)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> windows_core::Result<IXpsOMPageReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePageReference)(windows_core::Interface::as_raw(self), advisorypagedimensions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> windows_core::Result<IXpsOMPage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePage)(windows_core::Interface::as_raw(self), pagedimensions, language.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> windows_core::Result<IXpsOMPage>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePageFromStream)(windows_core::Interface::as_raw(self), pagemarkupstream.param().abi(), parturi.param().abi(), resources.param().abi(), reuseobjects.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCanvas(&self) -> windows_core::Result<IXpsOMCanvas> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCanvas)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGlyphs<P0>(&self, fontresource: P0) -> windows_core::Result<IXpsOMGlyphs>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGlyphs)(windows_core::Interface::as_raw(self), fontresource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePath(&self) -> windows_core::Result<IXpsOMPath> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGeometry(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGeometryFigure)(windows_core::Interface::as_raw(self), startpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMatrixTransform)(windows_core::Interface::as_raw(self), matrix, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSolidColorBrush<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> windows_core::Result<IXpsOMSolidColorBrush>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSolidColorBrush)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorProfileResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMImageBrush>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageBrush)(windows_core::Interface::as_raw(self), image.param().abi(), viewbox, viewport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMVisualBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVisualBrush)(windows_core::Interface::as_raw(self), viewbox, viewport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<P0, P1>(&self, acquiredstream: P0, contenttype: XPS_IMAGE_TYPE, parturi: P1) -> windows_core::Result<IXpsOMImageResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), contenttype, parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMPrintTicketResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePrintTicketResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<P0, P1, P2>(&self, acquiredstream: P0, fontembedding: XPS_FONT_EMBEDDING, parturi: P1, isobfsourcestream: P2) -> windows_core::Result<IXpsOMFontResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), fontembedding, parturi.param().abi(), isobfsourcestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGradientStop<P0>(&self, color: *const XPS_COLOR, colorprofile: P0, offset: f32) -> windows_core::Result<IXpsOMGradientStop>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGradientStop)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi(), offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMLinearGradientBrush>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
        P1: windows_core::Param<IXpsOMGradientStop>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(windows_core::Interface::as_raw(self), gradstop1.param().abi(), gradstop2.param().abi(), startpoint, endpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRadialGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> windows_core::Result<IXpsOMRadialGradientBrush>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
        P1: windows_core::Param<IXpsOMGradientStop>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRadialGradientBrush)(windows_core::Interface::as_raw(self), gradstop1.param().abi(), gradstop2.param().abi(), centerpoint, gradientorigin, radiisizes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<P0>(&self, parturi: P0) -> windows_core::Result<IXpsOMCoreProperties>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCoreProperties)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePartUriCollection(&self) -> windows_core::Result<IXpsOMPartUriCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePartUriCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P3: windows_core::Param<IXpsOMCoreProperties>,
        P4: windows_core::Param<IXpsOMImageResource>,
        P5: windows_core::Param<IXpsOMPrintTicketResource>,
        P6: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageWriterOnFile)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.param().abi(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P3: windows_core::Param<IXpsOMCoreProperties>,
        P4: windows_core::Param<IXpsOMImageResource>,
        P5: windows_core::Param<IXpsOMPrintTicketResource>,
        P6: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageWriterOnStream)(windows_core::Interface::as_raw(self), outputstream.param().abi(), optimizemarkupsize.param().abi(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<P0>(&self, uri: P0) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePartUri)(windows_core::Interface::as_raw(self), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<P0>(&self, filename: P0) -> windows_core::Result<super::super::System::Com::IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateReadOnlyStreamOnFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMObjectFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePackageFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageFromStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateStoryFragmentsResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateStoryFragmentsResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentStructureResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentStructureResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateSignatureBlockResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateSignatureBlockResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream: usize,
    pub CreatePartResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentSequence: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocument: usize,
    pub CreatePageReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream: usize,
    pub CreateCanvas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeometryFigure: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_MATRIX, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_COLOR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateColorProfileResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateColorProfileResource: usize,
    pub CreateImageBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const XPS_RECT, *const XPS_RECT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVisualBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_RECT, *const XPS_RECT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateImageResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_IMAGE_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateImageResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePrintTicketResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_FONT_EMBEDDING, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateFontResource: usize,
    pub CreateGradientStop: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_COLOR, *mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const XPS_POINT, *const XPS_POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const XPS_POINT, *const XPS_POINT, *const XPS_SIZE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateCoreProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateCoreProperties: usize,
    pub CreateDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePartUriCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, super::super::Foundation::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePartUri: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamOnFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamOnFile: usize,
}
windows_core::imp::define_interface!(IXpsOMObjectFactory1, IXpsOMObjectFactory1_Vtbl, 0x0a91b617_d612_4181_bf7c_be5824e9cc8f);
impl core::ops::Deref for IXpsOMObjectFactory1 {
    type Target = IXpsOMObjectFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMObjectFactory1, windows_core::IUnknown, IXpsOMObjectFactory);
impl IXpsOMObjectFactory1 {
    pub unsafe fn GetDocumentTypeFromFile<P0>(&self, filename: P0) -> windows_core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentTypeFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentTypeFromStream<P0>(&self, xpsdocumentstream: P0) -> windows_core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentTypeFromStream)(windows_core::Interface::as_raw(self), xpsdocumentstream.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ConvertHDPhotoToJpegXR<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).ConvertHDPhotoToJpegXR)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok()
    }
    pub unsafe fn ConvertJpegXRToHDPhoto<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).ConvertJpegXRToHDPhoto)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile1<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P3: windows_core::Param<IXpsOMCoreProperties>,
        P4: windows_core::Param<IXpsOMImageResource>,
        P5: windows_core::Param<IXpsOMPrintTicketResource>,
        P6: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageWriterOnFile1)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.param().abi(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), documenttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream1<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P3: windows_core::Param<IXpsOMCoreProperties>,
        P4: windows_core::Param<IXpsOMImageResource>,
        P5: windows_core::Param<IXpsOMPrintTicketResource>,
        P6: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageWriterOnStream1)(windows_core::Interface::as_raw(self), outputstream.param().abi(), optimizemarkupsize.param().abi(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), documenttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePackage1(&self) -> windows_core::Result<IXpsOMPackage1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackage1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageFromStream1<P0, P1>(&self, stream: P0, reuseobjects: P1) -> windows_core::Result<IXpsOMPackage1>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageFromStream1)(windows_core::Interface::as_raw(self), stream.param().abi(), reuseobjects.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePackageFromFile1<P0, P1>(&self, filename: P0, reuseobjects: P1) -> windows_core::Result<IXpsOMPackage1>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageFromFile1)(windows_core::Interface::as_raw(self), filename.param().abi(), reuseobjects.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage1<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> windows_core::Result<IXpsOMPage1>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePage1)(windows_core::Interface::as_raw(self), pagedimensions, language.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream1<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> windows_core::Result<IXpsOMPage1>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePageFromStream1)(windows_core::Interface::as_raw(self), pagemarkupstream.param().abi(), parturi.param().abi(), resources.param().abi(), reuseobjects.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<P0, P1, P2>(&self, dictionarymarkupstream: P0, parturi: P1, resources: P2) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream1)(windows_core::Interface::as_raw(self), dictionarymarkupstream.param().abi(), parturi.param().abi(), resources.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMObjectFactory1_Vtbl {
    pub base__: IXpsOMObjectFactory_Vtbl,
    pub GetDocumentTypeFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentTypeFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentTypeFromStream: usize,
    pub ConvertHDPhotoToJpegXR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertJpegXRToHDPhoto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile1: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, super::super::Foundation::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, XPS_DOCUMENT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, XPS_DOCUMENT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream1: usize,
    pub CreatePackage1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageFromStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageFromStream1: usize,
    pub CreatePackageFromFile1: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage1: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream1: usize,
}
windows_core::imp::define_interface!(IXpsOMPackage, IXpsOMPackage_Vtbl, 0x18c3df65_81e1_4674_91dc_fc452f5a416f);
impl core::ops::Deref for IXpsOMPackage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPackage, windows_core::IUnknown);
impl IXpsOMPackage {
    pub unsafe fn GetDocumentSequence(&self) -> windows_core::Result<IXpsOMDocumentSequence> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentSequence)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDocumentSequence<P0>(&self, documentsequence: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocumentSequence>,
    {
        (windows_core::Interface::vtable(self).SetDocumentSequence)(windows_core::Interface::as_raw(self), documentsequence.param().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> windows_core::Result<IXpsOMCoreProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCoreProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCoreProperties<P0>(&self, coreproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMCoreProperties>,
    {
        (windows_core::Interface::vtable(self).SetCoreProperties)(windows_core::Interface::as_raw(self), coreproperties.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDiscardControlPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<P0>(&self, discardcontrolparturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetDiscardControlPartName)(windows_core::Interface::as_raw(self), discardcontrolparturi.param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThumbnailResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).SetThumbnailResource)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn WriteToFile<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).WriteToFile)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteToStream<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).WriteToStream)(windows_core::Interface::as_raw(self), stream.param().abi(), optimizemarkupsize.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPackage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCoreProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCoreProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDiscardControlPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDiscardControlPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetDiscardControlPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetDiscardControlPartName: usize,
    pub GetThumbnailResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub WriteToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    WriteToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteToStream: usize,
}
windows_core::imp::define_interface!(IXpsOMPackage1, IXpsOMPackage1_Vtbl, 0x95a9435e_12bb_461b_8e7f_c6adb04cd96a);
impl core::ops::Deref for IXpsOMPackage1 {
    type Target = IXpsOMPackage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPackage1, windows_core::IUnknown, IXpsOMPackage);
impl IXpsOMPackage1 {
    pub unsafe fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn WriteToFile1<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).WriteToFile1)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.param().abi(), documenttype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteToStream1<P0, P1>(&self, outputstream: P0, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).WriteToStream1)(windows_core::Interface::as_raw(self), outputstream.param().abi(), optimizemarkupsize.param().abi(), documenttype).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPackage1_Vtbl {
    pub base__: IXpsOMPackage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub WriteToFile1: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, super::super::Foundation::BOOL, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    WriteToFile1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteToStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteToStream1: usize,
}
windows_core::imp::define_interface!(IXpsOMPackageTarget, IXpsOMPackageTarget_Vtbl, 0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
impl core::ops::Deref for IXpsOMPackageTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPackageTarget, windows_core::IUnknown);
impl IXpsOMPackageTarget {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateXpsOMPackageWriter<P0, P1, P2>(&self, documentsequencepartname: P0, documentsequenceprintticket: P1, discardcontrolpartname: P2) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<IXpsOMPrintTicketResource>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateXpsOMPackageWriter)(windows_core::Interface::as_raw(self), documentsequencepartname.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateXpsOMPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateXpsOMPackageWriter: usize,
}
windows_core::imp::define_interface!(IXpsOMPackageWriter, IXpsOMPackageWriter_Vtbl, 0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
impl core::ops::Deref for IXpsOMPackageWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPackageWriter, windows_core::IUnknown);
impl IXpsOMPackageWriter {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<P0, P1, P2, P3, P4>(&self, documentpartname: P0, documentprintticket: P1, documentstructure: P2, signatureblockresources: P3, restrictedfonts: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<IXpsOMPrintTicketResource>,
        P2: windows_core::Param<IXpsOMDocumentStructureResource>,
        P3: windows_core::Param<IXpsOMSignatureBlockResourceCollection>,
        P4: windows_core::Param<IXpsOMPartUriCollection>,
    {
        (windows_core::Interface::vtable(self).StartNewDocument)(windows_core::Interface::as_raw(self), documentpartname.param().abi(), documentprintticket.param().abi(), documentstructure.param().abi(), signatureblockresources.param().abi(), restrictedfonts.param().abi()).ok()
    }
    pub unsafe fn AddPage<P0, P1, P2, P3, P4>(&self, page: P0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: P1, storyfragments: P2, pageprintticket: P3, pagethumbnail: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPage>,
        P1: windows_core::Param<IXpsOMPartUriCollection>,
        P2: windows_core::Param<IXpsOMStoryFragmentsResource>,
        P3: windows_core::Param<IXpsOMPrintTicketResource>,
        P4: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), page.param().abi(), advisorypagedimensions, discardableresourceparts.param().abi(), storyfragments.param().abi(), pageprintticket.param().abi(), pagethumbnail.param().abi()).ok()
    }
    pub unsafe fn AddResource<P0>(&self, resource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMResource>,
    {
        (windows_core::Interface::vtable(self).AddResource)(windows_core::Interface::as_raw(self), resource.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsClosed(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXpsOMPackageWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub StartNewDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    StartNewDocument: usize,
    pub AddPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const XPS_SIZE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMPackageWriter3D, IXpsOMPackageWriter3D_Vtbl, 0xe8a45033_640e_43fa_9bdf_fddeaa31c6a0);
impl core::ops::Deref for IXpsOMPackageWriter3D {
    type Target = IXpsOMPackageWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPackageWriter3D, windows_core::IUnknown, IXpsOMPackageWriter);
impl IXpsOMPackageWriter3D {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddModelTexture<P0, P1>(&self, texturepartname: P0, texturedata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddModelTexture)(windows_core::Interface::as_raw(self), texturepartname.param().abi(), texturedata.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetModelPrintTicket<P0, P1>(&self, printticketpartname: P0, printticketdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SetModelPrintTicket)(windows_core::Interface::as_raw(self), printticketpartname.param().abi(), printticketdata.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPackageWriter3D_Vtbl {
    pub base__: IXpsOMPackageWriter_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddModelTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddModelTexture: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetModelPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetModelPrintTicket: usize,
}
windows_core::imp::define_interface!(IXpsOMPage, IXpsOMPage_Vtbl, 0xd3e18888_f120_4fee_8c68_35296eae91d4);
impl core::ops::Deref for IXpsOMPage {
    type Target = IXpsOMPart;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPage, windows_core::IUnknown, IXpsOMPart);
impl IXpsOMPage {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMPageReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisuals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPageDimensions(&self) -> windows_core::Result<XPS_SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPageDimensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPageDimensions)(windows_core::Interface::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetContentBox(&self) -> windows_core::Result<XPS_RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentBox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContentBox)(windows_core::Interface::as_raw(self), contentbox).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> windows_core::Result<XPS_RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBleedBox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBleedBox)(windows_core::Interface::as_raw(self), bleedbox).ok()
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), language.param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn GetIsHyperlinkTarget(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlinktarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), ishyperlinktarget.param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionaryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
    {
        (windows_core::Interface::vtable(self).SetDictionaryLocal)(windows_core::Interface::as_raw(self), resourcedictionary.param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionaryResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        (windows_core::Interface::vtable(self).SetDictionaryResource)(windows_core::Interface::as_raw(self), remotedictionaryresource.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), stream.param().abi(), optimizemarkupsize.param().abi()).ok()
    }
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateUnusedLookupKey)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMPage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMPage_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPageDimensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SIZE) -> windows_core::HRESULT,
    pub SetPageDimensions: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE) -> windows_core::HRESULT,
    pub GetContentBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_RECT) -> windows_core::HRESULT,
    pub SetContentBox: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_RECT) -> windows_core::HRESULT,
    pub GetBleedBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_RECT) -> windows_core::HRESULT,
    pub SetBleedBox: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_RECT) -> windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
    pub GenerateUnusedLookupKey: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_OBJECT_TYPE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMPage1, IXpsOMPage1_Vtbl, 0x305b60ef_6892_4dda_9cbb_3aa65974508a);
impl core::ops::Deref for IXpsOMPage1 {
    type Target = IXpsOMPage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPage1, windows_core::IUnknown, IXpsOMPart, IXpsOMPage);
impl IXpsOMPage1 {
    pub unsafe fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<P0, P1>(&self, stream: P0, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Write1)(windows_core::Interface::as_raw(self), stream.param().abi(), optimizemarkupsize.param().abi(), documenttype).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPage1_Vtbl {
    pub base__: IXpsOMPage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
windows_core::imp::define_interface!(IXpsOMPageReference, IXpsOMPageReference_Vtbl, 0xed360180_6f92_4998_890d_2f208531a0a0);
impl core::ops::Deref for IXpsOMPageReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPageReference, windows_core::IUnknown);
impl IXpsOMPageReference {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPage(&self) -> windows_core::Result<IXpsOMPage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPage<P0>(&self, page: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPage>,
    {
        (windows_core::Interface::vtable(self).SetPage)(windows_core::Interface::as_raw(self), page.param().abi()).ok()
    }
    pub unsafe fn DiscardPage(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DiscardPage)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsPageLoaded(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPageLoaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> windows_core::Result<XPS_SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAdvisoryPageDimensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAdvisoryPageDimensions)(windows_core::Interface::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetStoryFragmentsResource(&self) -> windows_core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStoryFragmentsResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStoryFragmentsResource<P0>(&self, storyfragmentsresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMStoryFragmentsResource>,
    {
        (windows_core::Interface::vtable(self).SetStoryFragmentsResource)(windows_core::Interface::as_raw(self), storyfragmentsresource.param().abi()).ok()
    }
    pub unsafe fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrintTicketResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPrintTicketResource>,
    {
        (windows_core::Interface::vtable(self).SetPrintTicketResource)(windows_core::Interface::as_raw(self), printticketresource.param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThumbnailResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        (windows_core::Interface::vtable(self).SetThumbnailResource)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok()
    }
    pub unsafe fn CollectLinkTargets(&self) -> windows_core::Result<IXpsOMNameCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CollectLinkTargets)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CollectPartResources(&self) -> windows_core::Result<IXpsOMPartResources> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CollectPartResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HasRestrictedFonts(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasRestrictedFonts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMPageReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMPageReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscardPage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPageLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAdvisoryPageDimensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SIZE) -> windows_core::HRESULT,
    pub SetAdvisoryPageDimensions: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE) -> windows_core::HRESULT,
    pub GetStoryFragmentsResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStoryFragmentsResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThumbnailResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CollectLinkTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CollectPartResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasRestrictedFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMPageReferenceCollection, IXpsOMPageReferenceCollection_Vtbl, 0xca16ba4d_e7b9_45c5_958b_f98022473745);
impl core::ops::Deref for IXpsOMPageReferenceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPageReferenceCollection, windows_core::IUnknown);
impl IXpsOMPageReferenceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMPageReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, pagereference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPageReference>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, pagereference.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, pagereference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPageReference>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, pagereference.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, pagereference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPageReference>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), pagereference.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPageReferenceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMPart, IXpsOMPart_Vtbl, 0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
impl core::ops::Deref for IXpsOMPart {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPart, windows_core::IUnknown);
impl IXpsOMPart {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetPartName)(windows_core::Interface::as_raw(self), parturi.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPart_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetPartName: usize,
}
windows_core::imp::define_interface!(IXpsOMPartResources, IXpsOMPartResources_Vtbl, 0xf4cf7729_4864_4275_99b3_a8717163ecaf);
impl core::ops::Deref for IXpsOMPartResources {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPartResources, windows_core::IUnknown);
impl IXpsOMPartResources {
    pub unsafe fn GetFontResources(&self) -> windows_core::Result<IXpsOMFontResourceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetImageResources(&self) -> windows_core::Result<IXpsOMImageResourceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetColorProfileResources(&self) -> windows_core::Result<IXpsOMColorProfileResourceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColorProfileResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRemoteDictionaryResources(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResourceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRemoteDictionaryResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMPartResources_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImageResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorProfileResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteDictionaryResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMPartUriCollection, IXpsOMPartUriCollection_Vtbl, 0x57c650d4_067c_4893_8c33_f62a0633730f);
impl core::ops::Deref for IXpsOMPartUriCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPartUriCollection, windows_core::IUnknown);
impl IXpsOMPartUriCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn InsertAt<P0>(&self, index: u32, parturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, parturi.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetAt<P0>(&self, index: u32, parturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, parturi.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn Append<P0>(&self, parturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), parturi.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPartUriCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    InsertAt: usize,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    Append: usize,
}
windows_core::imp::define_interface!(IXpsOMPath, IXpsOMPath_Vtbl, 0x37d38bb6_3ee9_4110_9312_14b194163337);
impl core::ops::Deref for IXpsOMPath {
    type Target = IXpsOMVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPath, windows_core::IUnknown, IXpsOMShareable, IXpsOMVisual);
impl IXpsOMPath {
    pub unsafe fn GetGeometry(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeometryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetGeometryLocal<P0>(&self, geometry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometry>,
    {
        (windows_core::Interface::vtable(self).SetGeometryLocal)(windows_core::Interface::as_raw(self), geometry.param().abi()).ok()
    }
    pub unsafe fn GetGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeometryLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGeometryLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetGeometryLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), shortdescription.param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), longdescription.param().abi()).ok()
    }
    pub unsafe fn GetSnapsToPixels(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSnapsToPixels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSnapsToPixels<P0>(&self, snapstopixels: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSnapsToPixels)(windows_core::Interface::as_raw(self), snapstopixels.param().abi()).ok()
    }
    pub unsafe fn GetStrokeBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStrokeBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStrokeBrushLocal<P0>(&self, brush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        (windows_core::Interface::vtable(self).SetStrokeBrushLocal)(windows_core::Interface::as_raw(self), brush.param().abi()).ok()
    }
    pub unsafe fn GetStrokeBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeBrushLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStrokeBrushLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok()
    }
    pub unsafe fn GetStrokeDashes(&self) -> windows_core::Result<IXpsOMDashCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeDashes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStrokeDashCap(&self) -> windows_core::Result<XPS_DASH_CAP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeDashCap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeDashCap)(windows_core::Interface::as_raw(self), strokedashcap).ok()
    }
    pub unsafe fn GetStrokeDashOffset(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeDashOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeDashOffset)(windows_core::Interface::as_raw(self), strokedashoffset).ok()
    }
    pub unsafe fn GetStrokeStartLineCap(&self) -> windows_core::Result<XPS_LINE_CAP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeStartLineCap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeStartLineCap)(windows_core::Interface::as_raw(self), strokestartlinecap).ok()
    }
    pub unsafe fn GetStrokeEndLineCap(&self) -> windows_core::Result<XPS_LINE_CAP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeEndLineCap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeEndLineCap)(windows_core::Interface::as_raw(self), strokeendlinecap).ok()
    }
    pub unsafe fn GetStrokeLineJoin(&self) -> windows_core::Result<XPS_LINE_JOIN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeLineJoin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeLineJoin)(windows_core::Interface::as_raw(self), strokelinejoin).ok()
    }
    pub unsafe fn GetStrokeMiterLimit(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeMiterLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeMiterLimit)(windows_core::Interface::as_raw(self), strokemiterlimit).ok()
    }
    pub unsafe fn GetStrokeThickness(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStrokeThickness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStrokeThickness)(windows_core::Interface::as_raw(self), strokethickness).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, brush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        (windows_core::Interface::vtable(self).SetFillBrushLocal)(windows_core::Interface::as_raw(self), brush.param().abi()).ok()
    }
    pub unsafe fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFillBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFillBrushLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMPath> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMPath_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGeometryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGeometryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGeometryLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetGeometryLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSnapsToPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetSnapsToPixels: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetStrokeBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStrokeBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStrokeBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStrokeBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetStrokeBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetStrokeDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStrokeDashCap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DASH_CAP) -> windows_core::HRESULT,
    pub SetStrokeDashCap: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_DASH_CAP) -> windows_core::HRESULT,
    pub GetStrokeDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetStrokeStartLineCap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_LINE_CAP) -> windows_core::HRESULT,
    pub SetStrokeStartLineCap: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_LINE_CAP) -> windows_core::HRESULT,
    pub GetStrokeEndLineCap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_LINE_CAP) -> windows_core::HRESULT,
    pub SetStrokeEndLineCap: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_LINE_CAP) -> windows_core::HRESULT,
    pub GetStrokeLineJoin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_LINE_JOIN) -> windows_core::HRESULT,
    pub SetStrokeLineJoin: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_LINE_JOIN) -> windows_core::HRESULT,
    pub GetStrokeMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetStrokeThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMPrintTicketResource, IXpsOMPrintTicketResource_Vtbl, 0xe7ff32d2_34aa_499b_bbe9_9cd4ee6c59f7);
impl core::ops::Deref for IXpsOMPrintTicketResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMPrintTicketResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMPrintTicketResource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMPrintTicketResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
windows_core::imp::define_interface!(IXpsOMRadialGradientBrush, IXpsOMRadialGradientBrush_Vtbl, 0x75f207e5_08bf_413c_96b1_b82b4064176b);
impl core::ops::Deref for IXpsOMRadialGradientBrush {
    type Target = IXpsOMGradientBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMRadialGradientBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMGradientBrush);
impl IXpsOMRadialGradientBrush {
    pub unsafe fn GetCenter(&self) -> windows_core::Result<XPS_POINT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCenter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCenter)(windows_core::Interface::as_raw(self), center).ok()
    }
    pub unsafe fn GetRadiiSizes(&self) -> windows_core::Result<XPS_SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRadiiSizes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRadiiSizes)(windows_core::Interface::as_raw(self), radiisizes).ok()
    }
    pub unsafe fn GetGradientOrigin(&self) -> windows_core::Result<XPS_POINT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGradientOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGradientOrigin)(windows_core::Interface::as_raw(self), origin).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMRadialGradientBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMRadialGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub GetRadiiSizes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SIZE) -> windows_core::HRESULT,
    pub SetRadiiSizes: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE) -> windows_core::HRESULT,
    pub GetGradientOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetGradientOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMRemoteDictionaryResource, IXpsOMRemoteDictionaryResource_Vtbl, 0xc9bd7cd4_e16a_4bf8_8c84_c950af7a3061);
impl core::ops::Deref for IXpsOMRemoteDictionaryResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMRemoteDictionaryResource {
    pub unsafe fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDictionary<P0>(&self, dictionary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
    {
        (windows_core::Interface::vtable(self).SetDictionary)(windows_core::Interface::as_raw(self), dictionary.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMRemoteDictionaryResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMRemoteDictionaryResource1, IXpsOMRemoteDictionaryResource1_Vtbl, 0xbf8fc1d4_9d46_4141_ba5f_94bb9250d041);
impl core::ops::Deref for IXpsOMRemoteDictionaryResource1 {
    type Target = IXpsOMRemoteDictionaryResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResource1, windows_core::IUnknown, IXpsOMPart, IXpsOMResource, IXpsOMRemoteDictionaryResource);
impl IXpsOMRemoteDictionaryResource1 {
    pub unsafe fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<P0>(&self, stream: P0, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
    {
        (windows_core::Interface::vtable(self).Write1)(windows_core::Interface::as_raw(self), stream.param().abi(), documenttype).ok()
    }
}
#[repr(C)]
pub struct IXpsOMRemoteDictionaryResource1_Vtbl {
    pub base__: IXpsOMRemoteDictionaryResource_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
windows_core::imp::define_interface!(IXpsOMRemoteDictionaryResourceCollection, IXpsOMRemoteDictionaryResourceCollection_Vtbl, 0x5c38db61_7fec_464a_87bd_41e3bef018be);
impl core::ops::Deref for IXpsOMRemoteDictionaryResourceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResourceCollection, windows_core::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
windows_core::imp::define_interface!(IXpsOMResource, IXpsOMResource_Vtbl, 0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
impl core::ops::Deref for IXpsOMResource {
    type Target = IXpsOMPart;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMResource, windows_core::IUnknown, IXpsOMPart);
impl IXpsOMResource {}
#[repr(C)]
pub struct IXpsOMResource_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
}
windows_core::imp::define_interface!(IXpsOMShareable, IXpsOMShareable_Vtbl, 0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
impl core::ops::Deref for IXpsOMShareable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMShareable, windows_core::IUnknown);
impl IXpsOMShareable {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXpsOMShareable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_OBJECT_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMSignatureBlockResource, IXpsOMSignatureBlockResource_Vtbl, 0x4776ad35_2e04_4357_8743_ebf6c171a905);
impl core::ops::Deref for IXpsOMSignatureBlockResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMSignatureBlockResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMSignatureBlockResource {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMSignatureBlockResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
windows_core::imp::define_interface!(IXpsOMSignatureBlockResourceCollection, IXpsOMSignatureBlockResourceCollection_Vtbl, 0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
impl core::ops::Deref for IXpsOMSignatureBlockResourceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMSignatureBlockResourceCollection, windows_core::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMSignatureBlockResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, signatureblockresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMSignatureBlockResource>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, signatureblockresource.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, signatureblockresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMSignatureBlockResource>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, signatureblockresource.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, signatureblockresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMSignatureBlockResource>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), signatureblockresource.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
windows_core::imp::define_interface!(IXpsOMSolidColorBrush, IXpsOMSolidColorBrush_Vtbl, 0xa06f9f05_3be9_4763_98a8_094fc672e488);
impl core::ops::Deref for IXpsOMSolidColorBrush {
    type Target = IXpsOMBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMSolidColorBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush);
impl IXpsOMSolidColorBrush {
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), color, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetColor<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMSolidColorBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMSolidColorBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_COLOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_COLOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMStoryFragmentsResource, IXpsOMStoryFragmentsResource_Vtbl, 0xc2b3ca09_0473_4282_87ae_1780863223f0);
impl core::ops::Deref for IXpsOMStoryFragmentsResource {
    type Target = IXpsOMResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMStoryFragmentsResource, windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
impl IXpsOMStoryFragmentsResource {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMPageReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMStoryFragmentsResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
windows_core::imp::define_interface!(IXpsOMThumbnailGenerator, IXpsOMThumbnailGenerator_Vtbl, 0x15b873d5_1971_41e8_83a3_6578403064c7);
impl core::ops::Deref for IXpsOMThumbnailGenerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMThumbnailGenerator, windows_core::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateThumbnail<P0, P1>(&self, page: P0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: P1) -> windows_core::Result<IXpsOMImageResource>
    where
        P0: windows_core::Param<IXpsOMPage>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerateThumbnail)(windows_core::Interface::as_raw(self), page.param().abi(), thumbnailtype, thumbnailsize, imageresourcepartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMThumbnailGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GenerateThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_IMAGE_TYPE, XPS_THUMBNAIL_SIZE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GenerateThumbnail: usize,
}
windows_core::imp::define_interface!(IXpsOMTileBrush, IXpsOMTileBrush_Vtbl, 0x0fc2328d_d722_4a54_b2ec_be90218a789e);
impl core::ops::Deref for IXpsOMTileBrush {
    type Target = IXpsOMBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMTileBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush);
impl IXpsOMTileBrush {
    pub unsafe fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), transform.param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> windows_core::Result<XPS_RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewbox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewbox)(windows_core::Interface::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> windows_core::Result<XPS_RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewport)(windows_core::Interface::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> windows_core::Result<XPS_TILE_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTileMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTileMode)(windows_core::Interface::as_raw(self), tilemode).ok()
    }
}
#[repr(C)]
pub struct IXpsOMTileBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetViewbox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_RECT) -> windows_core::HRESULT,
    pub SetViewbox: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_RECT) -> windows_core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_RECT) -> windows_core::HRESULT,
    pub SetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_RECT) -> windows_core::HRESULT,
    pub GetTileMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_TILE_MODE) -> windows_core::HRESULT,
    pub SetTileMode: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_TILE_MODE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMVisual, IXpsOMVisual_Vtbl, 0xbc3e7333_fb0b_4af3_a819_0b4eaad0d2fd);
impl core::ops::Deref for IXpsOMVisual {
    type Target = IXpsOMShareable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMVisual, windows_core::IUnknown, IXpsOMShareable);
impl IXpsOMVisual {
    pub unsafe fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), matrixtransform.param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClipGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClipGeometryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometry>,
    {
        (windows_core::Interface::vtable(self).SetClipGeometryLocal)(windows_core::Interface::as_raw(self), clipgeometry.param().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClipGeometryLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetClipGeometryLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOpacityMaskBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOpacityMaskBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        (windows_core::Interface::vtable(self).SetOpacityMaskBrushLocal)(windows_core::Interface::as_raw(self), opacitymaskbrush.param().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOpacityMaskBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetOpacityMaskBrushLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn GetIsHyperlinkTarget(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), ishyperlink.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> windows_core::Result<super::super::System::Com::IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHyperlinkNavigateUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IUri>,
    {
        (windows_core::Interface::vtable(self).SetHyperlinkNavigateUri)(windows_core::Interface::as_raw(self), hyperlinkuri.param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), language.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMVisual_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetClipGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClipGeometryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipGeometryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClipGeometryLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetClipGeometryLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetOpacityMaskBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOpacityMaskBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpacityMaskBrushLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOpacityMaskBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetOpacityMaskBrushLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHyperlinkNavigateUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHyperlinkNavigateUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHyperlinkNavigateUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHyperlinkNavigateUri: usize,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMVisualBrush, IXpsOMVisualBrush_Vtbl, 0x97e294af_5b37_46b4_8057_874d2f64119b);
impl core::ops::Deref for IXpsOMVisualBrush {
    type Target = IXpsOMTileBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMVisualBrush, windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMTileBrush);
impl IXpsOMVisualBrush {
    pub unsafe fn GetVisual(&self) -> windows_core::Result<IXpsOMVisual> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisual)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVisualLocal(&self) -> windows_core::Result<IXpsOMVisual> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisualLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVisualLocal<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMVisual>,
    {
        (windows_core::Interface::vtable(self).SetVisualLocal)(windows_core::Interface::as_raw(self), visual.param().abi()).ok()
    }
    pub unsafe fn GetVisualLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisualLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVisualLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetVisualLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMVisualBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsOMVisualBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisualLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVisualLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisualLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetVisualLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsOMVisualCollection, IXpsOMVisualCollection_Vtbl, 0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
impl core::ops::Deref for IXpsOMVisualCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMVisualCollection, windows_core::IUnknown);
impl IXpsOMVisualCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMVisual> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMVisual>,
    {
        (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMVisual>,
    {
        (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMVisual>,
    {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsOMVisualCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSignature, IXpsSignature_Vtbl, 0x6ae4c93e_1ade_42fb_898b_3a5658284857);
impl core::ops::Deref for IXpsSignature {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignature, windows_core::IUnknown);
impl IXpsSignature {
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSignatureValue)(windows_core::Interface::as_raw(self), signaturehashvalue, count).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<XPS_SIGNATURE_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Verify)(windows_core::Interface::as_raw(self), x509certificate, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomObjectEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSignatureXml)(windows_core::Interface::as_raw(self), signaturexml, count).ok()
    }
    pub unsafe fn SetSignatureXml(&self, signaturexml: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSignatureXml)(windows_core::Interface::as_raw(self), core::mem::transmute(signaturexml.as_ptr()), signaturexml.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IXpsSignature_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateEnumerator: usize,
    pub GetSigningTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub Verify: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::Cryptography::CERT_CONTEXT, *mut XPS_SIGNATURE_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    Verify: usize,
    pub GetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SIGN_POLICY) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjectEnumerator: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferenceEnumerator: usize,
    pub GetSignatureXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetSignatureXml: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSignatureBlock, IXpsSignatureBlock_Vtbl, 0x151fac09_0b97_4ac6_a323_5e4297d4322b);
impl core::ops::Deref for IXpsSignatureBlock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignatureBlock, windows_core::IUnknown);
impl IXpsSignatureBlock {
    pub unsafe fn GetRequests(&self) -> windows_core::Result<IXpsSignatureRequestCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequests)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocumentIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocumentName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocumentName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRequest<P0>(&self, requestid: P0) -> windows_core::Result<IXpsSignatureRequest>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRequest)(windows_core::Interface::as_raw(self), requestid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsSignatureBlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    pub GetDocumentIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDocumentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDocumentName: usize,
    pub CreateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSignatureBlockCollection, IXpsSignatureBlockCollection_Vtbl, 0x23397050_fe99_467a_8dce_9237f074ffe4);
impl core::ops::Deref for IXpsSignatureBlockCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignatureBlockCollection, windows_core::IUnknown);
impl IXpsSignatureBlockCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureBlock> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
}
#[repr(C)]
pub struct IXpsSignatureBlockCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSignatureCollection, IXpsSignatureCollection_Vtbl, 0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
impl core::ops::Deref for IXpsSignatureCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignatureCollection, windows_core::IUnknown);
impl IXpsSignatureCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignature> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
}
#[repr(C)]
pub struct IXpsSignatureCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSignatureManager, IXpsSignatureManager_Vtbl, 0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
impl core::ops::Deref for IXpsSignatureManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignatureManager, windows_core::IUnknown);
impl IXpsSignatureManager {
    pub unsafe fn LoadPackageFile<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadPackageFile)(windows_core::Interface::as_raw(self), filename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadPackageStream<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).LoadPackageStream)(windows_core::Interface::as_raw(self), stream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Sign<P0>(&self, signoptions: P0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<IXpsSignature>
    where
        P0: windows_core::Param<IXpsSigningOptions>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Sign)(windows_core::Interface::as_raw(self), signoptions.param().abi(), x509certificate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignatureOriginPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureOriginPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetSignatureOriginPartName)(windows_core::Interface::as_raw(self), signatureoriginpartname.param().abi()).ok()
    }
    pub unsafe fn GetSignatures(&self) -> windows_core::Result<IXpsSignatureCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatures)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddSignatureBlock<P0>(&self, partname: P0, fixeddocumentindex: u32) -> windows_core::Result<IXpsSignatureBlock>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddSignatureBlock)(windows_core::Interface::as_raw(self), partname.param().abi(), fixeddocumentindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSignatureBlocks(&self) -> windows_core::Result<IXpsSignatureBlockCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSigningOptions(&self) -> windows_core::Result<IXpsSigningOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSigningOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn SavePackageToFile<P0>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SavePackageToFile)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SavePackageToStream<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SavePackageToStream)(windows_core::Interface::as_raw(self), stream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IXpsSignatureManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadPackageFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadPackageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadPackageStream: usize,
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub Sign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Security::Cryptography::CERT_CONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security_Cryptography"))]
    Sign: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignatureOriginPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignatureOriginPartName: usize,
    pub GetSignatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddSignatureBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddSignatureBlock: usize,
    pub GetSignatureBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSigningOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub SavePackageToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    SavePackageToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SavePackageToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SavePackageToStream: usize,
}
windows_core::imp::define_interface!(IXpsSignatureRequest, IXpsSignatureRequest_Vtbl, 0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
impl core::ops::Deref for IXpsSignatureRequest {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignatureRequest, windows_core::IUnknown);
impl IXpsSignatureRequest {
    pub unsafe fn GetIntent(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIntent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIntent<P0>(&self, intent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetIntent)(windows_core::Interface::as_raw(self), intent.param().abi()).ok()
    }
    pub unsafe fn GetRequestedSigner(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestedSigner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRequestedSigner<P0>(&self, signername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestedSigner)(windows_core::Interface::as_raw(self), signername.param().abi()).ok()
    }
    pub unsafe fn GetRequestSignByDate(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestSignByDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRequestSignByDate<P0>(&self, datestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRequestSignByDate)(windows_core::Interface::as_raw(self), datestring.param().abi()).ok()
    }
    pub unsafe fn GetSigningLocale(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSigningLocale<P0>(&self, place: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSigningLocale)(windows_core::Interface::as_raw(self), place.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSpotLocation)(windows_core::Interface::as_raw(self), pageindex, core::mem::transmute(pagepartname), x, y).ok()
    }
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSpotLocation)(windows_core::Interface::as_raw(self), pageindex, x, y).ok()
    }
    pub unsafe fn GetRequestId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequestId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSignature(&self) -> windows_core::Result<IXpsSignature> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXpsSignatureRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIntent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIntent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRequestedSigner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRequestedSigner: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRequestSignByDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRequestSignByDate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSigningLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSigningLocale: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSpotLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSpotLocation: usize,
    pub SetSpotLocation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f32, f32) -> windows_core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSignatureRequestCollection, IXpsSignatureRequestCollection_Vtbl, 0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
impl core::ops::Deref for IXpsSignatureRequestCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSignatureRequestCollection, windows_core::IUnknown);
impl IXpsSignatureRequestCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureRequest> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
}
#[repr(C)]
pub struct IXpsSignatureRequestCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXpsSigningOptions, IXpsSigningOptions_Vtbl, 0x7718eae4_3215_49be_af5b_594fef7fcfa6);
impl core::ops::Deref for IXpsSigningOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsSigningOptions, windows_core::IUnknown);
impl IXpsSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSignatureId)(windows_core::Interface::as_raw(self), signatureid.param().abi()).ok()
    }
    pub unsafe fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignatureMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSignatureMethod)(windows_core::Interface::as_raw(self), signaturemethod.param().abi()).ok()
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDigestMethod<P0>(&self, digestmethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDigestMethod)(windows_core::Interface::as_raw(self), digestmethod.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        (windows_core::Interface::vtable(self).SetSignaturePartName)(windows_core::Interface::as_raw(self), signaturepartname.param().abi()).ok()
    }
    pub unsafe fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPolicy)(windows_core::Interface::as_raw(self), policy).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSigningTimeFormat)(windows_core::Interface::as_raw(self), timeformat).ok()
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjects(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferences(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustomReferences)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateSet(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCertificateSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<XPS_SIGN_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
}
#[repr(C)]
pub struct IXpsSigningOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDigestMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignaturePartName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignaturePartName: usize,
    pub GetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SIGN_POLICY) -> windows_core::HRESULT,
    pub SetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_SIGN_POLICY) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub SetSigningTimeFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    SetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjects: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferences: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateSet: usize,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SIGN_FLAGS) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_SIGN_FLAGS) -> windows_core::HRESULT,
}
pub const DC_BINNAMES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(12u16);
pub const DC_BINS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(6u16);
pub const DC_COLLATE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(22u16);
pub const DC_COLORDEVICE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(32u16);
pub const DC_COPIES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(18u16);
pub const DC_DRIVER: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(11u16);
pub const DC_DUPLEX: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(7u16);
pub const DC_ENUMRESOLUTIONS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(13u16);
pub const DC_EXTRA: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(9u16);
pub const DC_FIELDS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(1u16);
pub const DC_FILEDEPENDENCIES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(14u16);
pub const DC_MAXEXTENT: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(5u16);
pub const DC_MEDIAREADY: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(29u16);
pub const DC_MEDIATYPENAMES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(34u16);
pub const DC_MEDIATYPES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(35u16);
pub const DC_MINEXTENT: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(4u16);
pub const DC_NUP: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(33u16);
pub const DC_ORIENTATION: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(17u16);
pub const DC_PAPERNAMES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(16u16);
pub const DC_PAPERS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(2u16);
pub const DC_PAPERSIZE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(3u16);
pub const DC_PERSONALITY: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(25u16);
pub const DC_PRINTERMEM: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(28u16);
pub const DC_PRINTRATE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(26u16);
pub const DC_PRINTRATEPPM: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(31u16);
pub const DC_PRINTRATEUNIT: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(27u16);
pub const DC_SIZE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(8u16);
pub const DC_STAPLE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(30u16);
pub const DC_TRUETYPE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(15u16);
pub const DC_VERSION: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(10u16);
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(12u16);
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(101u16);
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = PSINJECT_POINT(14u16);
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = PSINJECT_POINT(16u16);
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = PSINJECT_POINT(1u16);
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = PSINJECT_POINT(9u16);
pub const PSINJECT_COMMENTS: PSINJECT_POINT = PSINJECT_POINT(11u16);
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = PSINJECT_POINT(5u16);
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = PSINJECT_POINT(6u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = PSINJECT_POINT(10u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = PSINJECT_POINT(21u16);
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(13u16);
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = PSINJECT_POINT(107u16);
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(102u16);
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = PSINJECT_POINT(15u16);
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = PSINJECT_POINT(17u16);
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = PSINJECT_POINT(20u16);
pub const PSINJECT_EOF: PSINJECT_POINT = PSINJECT_POINT(19u16);
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = PSINJECT_POINT(8u16);
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = PSINJECT_POINT(106u16);
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = PSINJECT_POINT(100u16);
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = PSINJECT_POINT(7u16);
pub const PSINJECT_PAGES: PSINJECT_POINT = PSINJECT_POINT(4u16);
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = PSINJECT_POINT(3u16);
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = PSINJECT_POINT(103u16);
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = PSINJECT_POINT(104u16);
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(3i32);
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(1i32);
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(2i32);
pub const XPS_E_ABSOLUTE_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x80520601_u32 as _);
pub const XPS_E_ALREADY_OWNED: windows_core::HRESULT = windows_core::HRESULT(0x80520503_u32 as _);
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: windows_core::HRESULT = windows_core::HRESULT(0x80520509_u32 as _);
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80520507_u32 as _);
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80520508_u32 as _);
pub const XPS_E_CARET_OUTSIDE_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80520305_u32 as _);
pub const XPS_E_CARET_OUT_OF_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x80520306_u32 as _);
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80520506_u32 as _);
pub const XPS_E_DICTIONARY_ITEM_NAMED: windows_core::HRESULT = windows_core::HRESULT(0x80520401_u32 as _);
pub const XPS_E_DUPLICATE_NAMES: windows_core::HRESULT = windows_core::HRESULT(0x80520209_u32 as _);
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: windows_core::HRESULT = windows_core::HRESULT(0x80520200_u32 as _);
pub const XPS_E_INDEX_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80520500_u32 as _);
pub const XPS_E_INVALID_BLEED_BOX: windows_core::HRESULT = windows_core::HRESULT(0x80520004_u32 as _);
pub const XPS_E_INVALID_CONTENT_BOX: windows_core::HRESULT = windows_core::HRESULT(0x8052000B_u32 as _);
pub const XPS_E_INVALID_CONTENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8052000E_u32 as _);
pub const XPS_E_INVALID_FLOAT: windows_core::HRESULT = windows_core::HRESULT(0x80520007_u32 as _);
pub const XPS_E_INVALID_FONT_URI: windows_core::HRESULT = windows_core::HRESULT(0x8052000A_u32 as _);
pub const XPS_E_INVALID_LANGUAGE: windows_core::HRESULT = windows_core::HRESULT(0x80520000_u32 as _);
pub const XPS_E_INVALID_LOOKUP_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80520006_u32 as _);
pub const XPS_E_INVALID_MARKUP: windows_core::HRESULT = windows_core::HRESULT(0x8052000C_u32 as _);
pub const XPS_E_INVALID_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80520001_u32 as _);
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: windows_core::HRESULT = windows_core::HRESULT(0x80520602_u32 as _);
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: windows_core::HRESULT = windows_core::HRESULT(0x80520600_u32 as _);
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: windows_core::HRESULT = windows_core::HRESULT(0x8052000F_u32 as _);
pub const XPS_E_INVALID_PAGE_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80520003_u32 as _);
pub const XPS_E_INVALID_RESOURCE_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80520002_u32 as _);
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: windows_core::HRESULT = windows_core::HRESULT(0x8052038B_u32 as _);
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80520005_u32 as _);
pub const XPS_E_INVALID_XML_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x8052000D_u32 as _);
pub const XPS_E_MAPPING_OUTSIDE_INDICES: windows_core::HRESULT = windows_core::HRESULT(0x80520304_u32 as _);
pub const XPS_E_MAPPING_OUTSIDE_STRING: windows_core::HRESULT = windows_core::HRESULT(0x80520303_u32 as _);
pub const XPS_E_MAPPING_OUT_OF_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x80520302_u32 as _);
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: windows_core::HRESULT = windows_core::HRESULT(0x80520389_u32 as _);
pub const XPS_E_MISSING_COLORPROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80520104_u32 as _);
pub const XPS_E_MISSING_DISCARDCONTROL: windows_core::HRESULT = windows_core::HRESULT(0x80520112_u32 as _);
pub const XPS_E_MISSING_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80520109_u32 as _);
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80520108_u32 as _);
pub const XPS_E_MISSING_FONTURI: windows_core::HRESULT = windows_core::HRESULT(0x80520107_u32 as _);
pub const XPS_E_MISSING_GLYPHS: windows_core::HRESULT = windows_core::HRESULT(0x80520102_u32 as _);
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: windows_core::HRESULT = windows_core::HRESULT(0x8052010E_u32 as _);
pub const XPS_E_MISSING_LOOKUP: windows_core::HRESULT = windows_core::HRESULT(0x80520101_u32 as _);
pub const XPS_E_MISSING_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80520100_u32 as _);
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x8052010C_u32 as _);
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x8052010D_u32 as _);
pub const XPS_E_MISSING_PART_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x80520110_u32 as _);
pub const XPS_E_MISSING_PART_STREAM: windows_core::HRESULT = windows_core::HRESULT(0x80520113_u32 as _);
pub const XPS_E_MISSING_REFERRED_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x8052010A_u32 as _);
pub const XPS_E_MISSING_REFERRED_PAGE: windows_core::HRESULT = windows_core::HRESULT(0x8052010B_u32 as _);
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: windows_core::HRESULT = windows_core::HRESULT(0x80520105_u32 as _);
pub const XPS_E_MISSING_RESOURCE_KEY: windows_core::HRESULT = windows_core::HRESULT(0x8052010F_u32 as _);
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80520106_u32 as _);
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80520111_u32 as _);
pub const XPS_E_MISSING_SEGMENT_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80520103_u32 as _);
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: windows_core::HRESULT = windows_core::HRESULT(0x80520202_u32 as _);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80520206_u32 as _);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: windows_core::HRESULT = windows_core::HRESULT(0x80520207_u32 as _);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: windows_core::HRESULT = windows_core::HRESULT(0x80520205_u32 as _);
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: windows_core::HRESULT = windows_core::HRESULT(0x80520208_u32 as _);
pub const XPS_E_MULTIPLE_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x80520201_u32 as _);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: windows_core::HRESULT = windows_core::HRESULT(0x80520204_u32 as _);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: windows_core::HRESULT = windows_core::HRESULT(0x80520203_u32 as _);
pub const XPS_E_NEGATIVE_FLOAT: windows_core::HRESULT = windows_core::HRESULT(0x8052030A_u32 as _);
pub const XPS_E_NESTED_REMOTE_DICTIONARY: windows_core::HRESULT = windows_core::HRESULT(0x80520402_u32 as _);
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: windows_core::HRESULT = windows_core::HRESULT(0x8052050B_u32 as _);
pub const XPS_E_NO_CUSTOM_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x80520502_u32 as _);
pub const XPS_E_OBJECT_DETACHED: windows_core::HRESULT = windows_core::HRESULT(0x8052038A_u32 as _);
pub const XPS_E_ODD_BIDILEVEL: windows_core::HRESULT = windows_core::HRESULT(0x80520307_u32 as _);
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80520308_u32 as _);
pub const XPS_E_PACKAGE_ALREADY_OPENED: windows_core::HRESULT = windows_core::HRESULT(0x80520387_u32 as _);
pub const XPS_E_PACKAGE_NOT_OPENED: windows_core::HRESULT = windows_core::HRESULT(0x80520386_u32 as _);
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: windows_core::HRESULT = windows_core::HRESULT(0x8052050C_u32 as _);
pub const XPS_E_RELATIONSHIP_EXTERNAL: windows_core::HRESULT = windows_core::HRESULT(0x8052050A_u32 as _);
pub const XPS_E_RESOURCE_NOT_OWNED: windows_core::HRESULT = windows_core::HRESULT(0x80520504_u32 as _);
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: windows_core::HRESULT = windows_core::HRESULT(0x80520309_u32 as _);
pub const XPS_E_SIGNATUREID_DUP: windows_core::HRESULT = windows_core::HRESULT(0x80520388_u32 as _);
pub const XPS_E_SIGREQUESTID_DUP: windows_core::HRESULT = windows_core::HRESULT(0x80520385_u32 as _);
pub const XPS_E_STRING_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80520300_u32 as _);
pub const XPS_E_TOO_MANY_INDICES: windows_core::HRESULT = windows_core::HRESULT(0x80520301_u32 as _);
pub const XPS_E_UNAVAILABLE_PACKAGE: windows_core::HRESULT = windows_core::HRESULT(0x80520114_u32 as _);
pub const XPS_E_UNEXPECTED_COLORPROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80520505_u32 as _);
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80520008_u32 as _);
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80520010_u32 as _);
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80520011_u32 as _);
pub const XPS_E_VISUAL_CIRCULAR_REF: windows_core::HRESULT = windows_core::HRESULT(0x80520501_u32 as _);
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: windows_core::HRESULT = windows_core::HRESULT(0x80520400_u32 as _);
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(1i32);
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(5i32);
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(2i32);
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(7i32);
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(8i32);
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(4i32);
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(3i32);
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(9i32);
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(6i32);
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(10i32);
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRINTER_DEVICE_CAPABILITIES(pub u16);
impl windows_core::TypeKind for PRINTER_DEVICE_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRINTER_DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRINTER_DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for PRINT_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRINT_WINDOW_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRINT_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSINJECT_POINT(pub u16);
impl windows_core::TypeKind for PSINJECT_POINT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSINJECT_POINT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSINJECT_POINT").field(&self.0).finish()
    }
}
impl PSINJECT_POINT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSINJECT_POINT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSINJECT_POINT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSINJECT_POINT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSINJECT_POINT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSINJECT_POINT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
impl windows_core::TypeKind for XPS_COLOR_INTERPOLATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_COLOR_INTERPOLATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_COLOR_INTERPOLATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_COLOR_TYPE(pub i32);
impl windows_core::TypeKind for XPS_COLOR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_COLOR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_COLOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_DASH_CAP(pub i32);
impl windows_core::TypeKind for XPS_DASH_CAP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_DASH_CAP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_DASH_CAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
impl windows_core::TypeKind for XPS_DOCUMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_DOCUMENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_DOCUMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_FILL_RULE(pub i32);
impl windows_core::TypeKind for XPS_FILL_RULE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_FILL_RULE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_FILL_RULE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_FONT_EMBEDDING(pub i32);
impl windows_core::TypeKind for XPS_FONT_EMBEDDING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_FONT_EMBEDDING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_FONT_EMBEDDING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_IMAGE_TYPE(pub i32);
impl windows_core::TypeKind for XPS_IMAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_IMAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_INTERLEAVING(pub i32);
impl windows_core::TypeKind for XPS_INTERLEAVING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_INTERLEAVING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_INTERLEAVING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_LINE_CAP(pub i32);
impl windows_core::TypeKind for XPS_LINE_CAP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_LINE_CAP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_LINE_CAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_LINE_JOIN(pub i32);
impl windows_core::TypeKind for XPS_LINE_JOIN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_LINE_JOIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_LINE_JOIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for XPS_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
impl windows_core::TypeKind for XPS_SEGMENT_STROKE_PATTERN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_SEGMENT_STROKE_PATTERN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_STROKE_PATTERN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_SEGMENT_TYPE(pub i32);
impl windows_core::TypeKind for XPS_SEGMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_SEGMENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
impl windows_core::TypeKind for XPS_SIGNATURE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_SIGN_FLAGS(pub i32);
impl windows_core::TypeKind for XPS_SIGN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_SIGN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_SIGN_FLAGS").field(&self.0).finish()
    }
}
impl XPS_SIGN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XPS_SIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XPS_SIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XPS_SIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XPS_SIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XPS_SIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_SIGN_POLICY(pub i32);
impl windows_core::TypeKind for XPS_SIGN_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_SIGN_POLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_SIGN_POLICY").field(&self.0).finish()
    }
}
impl XPS_SIGN_POLICY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XPS_SIGN_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XPS_SIGN_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XPS_SIGN_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XPS_SIGN_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XPS_SIGN_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_SPREAD_METHOD(pub i32);
impl windows_core::TypeKind for XPS_SPREAD_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_SPREAD_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_SPREAD_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_STYLE_SIMULATION(pub i32);
impl windows_core::TypeKind for XPS_STYLE_SIMULATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_STYLE_SIMULATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_STYLE_SIMULATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
impl windows_core::TypeKind for XPS_THUMBNAIL_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_THUMBNAIL_SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_THUMBNAIL_SIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XPS_TILE_MODE(pub i32);
impl windows_core::TypeKind for XPS_TILE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XPS_TILE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XPS_TILE_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: windows_core::PCSTR,
    pub lpszOutput: windows_core::PCSTR,
    pub lpszDatatype: windows_core::PCSTR,
    pub fwType: u32,
}
impl windows_core::TypeKind for DOCINFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DOCINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: windows_core::PCWSTR,
    pub lpszOutput: windows_core::PCWSTR,
    pub lpszDatatype: windows_core::PCWSTR,
    pub fwType: u32,
}
impl windows_core::TypeKind for DOCINFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DOCINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
impl windows_core::TypeKind for DRAWPATRECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRAWPATRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPTPROVIDER(pub isize);
impl HPTPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HPTPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HPTPROVIDER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl windows_core::TypeKind for PSFEATURE_CUSTPAPER {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSFEATURE_CUSTPAPER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for PSFEATURE_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSFEATURE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl windows_core::TypeKind for PSINJECTDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSINJECTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl windows_core::TypeKind for XPS_COLOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_1,
    pub scRGB: XPS_COLOR_0_2,
    pub context: XPS_COLOR_0_0,
}
impl windows_core::TypeKind for XPS_COLOR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR_0_0 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl windows_core::TypeKind for XPS_COLOR_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_COLOR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XPS_COLOR_0_1 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl windows_core::TypeKind for XPS_COLOR_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_COLOR_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR_0_2 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl windows_core::TypeKind for XPS_COLOR_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl windows_core::TypeKind for XPS_DASH {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_DASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl windows_core::TypeKind for XPS_GLYPH_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_GLYPH_INDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl windows_core::TypeKind for XPS_GLYPH_MAPPING {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_GLYPH_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl windows_core::TypeKind for XPS_MATRIX {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_MATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl windows_core::TypeKind for XPS_POINT {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl windows_core::TypeKind for XPS_RECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl windows_core::TypeKind for XPS_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for XPS_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XpsOMObjectFactory: windows_core::GUID = windows_core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: windows_core::GUID = windows_core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type ABORTPROC = Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

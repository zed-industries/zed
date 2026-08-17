#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AbortDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn AbortDoc(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    unsafe { AbortDoc(hdc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DeviceCapabilitiesA<P0, P1>(pdevice: P0, pport: P1, fwcapability: PRINTER_DEVICE_CAPABILITIES, poutput: Option<windows_core::PSTR>, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEA>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeviceCapabilitiesA(pdevice : windows_core::PCSTR, pport : windows_core::PCSTR, fwcapability : PRINTER_DEVICE_CAPABILITIES, poutput : windows_core::PSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA) -> i32);
    unsafe { DeviceCapabilitiesA(pdevice.param().abi(), pport.param().abi(), fwcapability, poutput.unwrap_or(core::mem::zeroed()) as _, pdevmode.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DeviceCapabilitiesW<P0, P1>(pdevice: P0, pport: P1, fwcapability: PRINTER_DEVICE_CAPABILITIES, poutput: Option<windows_core::PWSTR>, pdevmode: Option<*const super::super::Graphics::Gdi::DEVMODEW>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winspool.drv" "system" fn DeviceCapabilitiesW(pdevice : windows_core::PCWSTR, pport : windows_core::PCWSTR, fwcapability : PRINTER_DEVICE_CAPABILITIES, poutput : windows_core::PWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW) -> i32);
    unsafe { DeviceCapabilitiesW(pdevice.param().abi(), pport.param().abi(), fwcapability, poutput.unwrap_or(core::mem::zeroed()) as _, pdevmode.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndDoc(hdc: super::super::Graphics::Gdi::HDC) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn EndDoc(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    unsafe { EndDoc(hdc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndPage(hdc: super::super::Graphics::Gdi::HDC) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn EndPage(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    unsafe { EndPage(hdc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn Escape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, pvin: Option<&[u8]>, pvout: Option<*mut core::ffi::c_void>) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn Escape(hdc : super::super::Graphics::Gdi:: HDC, iescape : i32, cjin : i32, pvin : windows_core::PCSTR, pvout : *mut core::ffi::c_void) -> i32);
    unsafe { Escape(hdc, iescape, pvin.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pvin.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pvout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ExtEscape(hdc: super::super::Graphics::Gdi::HDC, iescape: i32, lpindata: Option<&[u8]>, lpoutdata: Option<&mut [u8]>) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn ExtEscape(hdc : super::super::Graphics::Gdi:: HDC, iescape : i32, cjinput : i32, lpindata : windows_core::PCSTR, cjoutput : i32, lpoutdata : windows_core::PSTR) -> i32);
    unsafe { ExtEscape(hdc, iescape, lpindata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpindata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpoutdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpoutdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn PrintWindow(hwnd: super::super::Foundation::HWND, hdcblt: super::super::Graphics::Gdi::HDC, nflags: PRINT_WINDOW_FLAGS) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PrintWindow(hwnd : super::super::Foundation:: HWND, hdcblt : super::super::Graphics::Gdi:: HDC, nflags : PRINT_WINDOW_FLAGS) -> windows_core::BOOL);
    unsafe { PrintWindow(hwnd, hdcblt, nflags) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetAbortProc(hdc: super::super::Graphics::Gdi::HDC, proc: ABORTPROC) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn SetAbortProc(hdc : super::super::Graphics::Gdi:: HDC, proc : ABORTPROC) -> i32);
    unsafe { SetAbortProc(hdc, proc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocA(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOA) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn StartDocA(hdc : super::super::Graphics::Gdi:: HDC, lpdi : *const DOCINFOA) -> i32);
    unsafe { StartDocA(hdc, lpdi) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocW(hdc: super::super::Graphics::Gdi::HDC, lpdi: *const DOCINFOW) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn StartDocW(hdc : super::super::Graphics::Gdi:: HDC, lpdi : *const DOCINFOW) -> i32);
    unsafe { StartDocW(hdc, lpdi) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartPage(hdc: super::super::Graphics::Gdi::HDC) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn StartPage(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    unsafe { StartPage(hdc) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type ABORTPROC = Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> windows_core::BOOL>;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: windows_core::PCSTR,
    pub lpszOutput: windows_core::PCSTR,
    pub lpszDatatype: windows_core::PCSTR,
    pub fwType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: windows_core::PCWSTR,
    pub lpszOutput: windows_core::PCWSTR,
    pub lpszDatatype: windows_core::PCWSTR,
    pub fwType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
windows_core::imp::define_interface!(IXpsDocumentPackageTarget, IXpsDocumentPackageTarget_Vtbl, 0x3b0b6d38_53ad_41da_b212_d37637a6714e);
windows_core::imp::interface_hierarchy!(IXpsDocumentPackageTarget, windows_core::IUnknown);
impl IXpsDocumentPackageTarget {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter<P0, P1>(&self, documentsequencepartname: P0, discardcontrolpartname: P1) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsOMPackageWriter)(windows_core::Interface::as_raw(self), documentsequencepartname.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsOMFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetXpsType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetXpsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget_Impl: windows_core::IUnknownImpl {
    fn GetXpsOMPackageWriter(&self, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
    fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory>;
    fn GetXpsType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget_Vtbl {
    pub const fn new<Identity: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXpsOMPackageWriter<Identity: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocumentPackageTarget_Impl::GetXpsOMPackageWriter(this, core::mem::transmute_copy(&documentsequencepartname), core::mem::transmute_copy(&discardcontrolpartname)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocumentPackageTarget_Impl::GetXpsOMFactory(this) {
                    Ok(ok__) => {
                        xpsfactory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXpsType<Identity: IXpsDocumentPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocumentPackageTarget_Impl::GetXpsType(this) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter: GetXpsOMPackageWriter::<Identity, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, OFFSET>,
            GetXpsType: GetXpsType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsDocumentPackageTarget {}
windows_core::imp::define_interface!(IXpsDocumentPackageTarget3D, IXpsDocumentPackageTarget3D_Vtbl, 0x60ba71b8_3101_4984_9199_f4ea775ff01d);
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsOMPackageWriter3D)(windows_core::Interface::as_raw(self), documentsequencepartname.param().abi(), discardcontrolpartname.param().abi(), modelpartname.param().abi(), modeldata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXpsOMFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget3D_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter3D: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget3D_Impl: windows_core::IUnknownImpl {
    fn GetXpsOMPackageWriter3D(&self, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, modelpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, modeldata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<IXpsOMPackageWriter3D>;
    fn GetXpsOMFactory(&self) -> windows_core::Result<IXpsOMObjectFactory>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsDocumentPackageTarget3D_Vtbl {
    pub const fn new<Identity: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Identity: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, modelpartname: *mut core::ffi::c_void, modeldata: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocumentPackageTarget3D_Impl::GetXpsOMPackageWriter3D(this, core::mem::transmute_copy(&documentsequencepartname), core::mem::transmute_copy(&discardcontrolpartname), core::mem::transmute_copy(&modelpartname), core::mem::transmute_copy(&modeldata)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: IXpsDocumentPackageTarget3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsDocumentPackageTarget3D_Impl::GetXpsOMFactory(this) {
                    Ok(ok__) => {
                        xpsfactory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetXpsOMPackageWriter3D: GetXpsOMPackageWriter3D::<Identity, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsDocumentPackageTarget3D as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsDocumentPackageTarget3D {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMBrush_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IXpsOMBrush_Impl: IXpsOMShareable_Impl {
    fn GetOpacity(&self) -> windows_core::Result<f32>;
    fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()>;
}
impl IXpsOMBrush_Vtbl {
    pub const fn new<Identity: IXpsOMBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOpacity<Identity: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMBrush_Impl::GetOpacity(this) {
                    Ok(ok__) => {
                        opacity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: IXpsOMBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMBrush_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity)).into()
            }
        }
        Self { base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(), GetOpacity: GetOpacity::<Identity, OFFSET>, SetOpacity: SetOpacity::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMBrush {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisuals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUseAliasedEdgeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseAliasedEdgeMode(&self, usealiasededgemode: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseAliasedEdgeMode)(windows_core::Interface::as_raw(self), usealiasededgemode.into()).ok() }
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), shortdescription.param().abi()).ok() }
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), longdescription.param().abi()).ok() }
    }
    pub unsafe fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionaryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDictionaryLocal)(windows_core::Interface::as_raw(self), resourcedictionary.param().abi()).ok() }
    }
    pub unsafe fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionaryResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDictionaryResource)(windows_core::Interface::as_raw(self), remotedictionaryresource.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMCanvas> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCanvas_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUseAliasedEdgeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetUseAliasedEdgeMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMCanvas_Impl: IXpsOMVisual_Impl {
    fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection>;
    fn GetUseAliasedEdgeMode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetUseAliasedEdgeMode(&self, usealiasededgemode: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(&self, shortdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(&self, longdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&self, resourcedictionary: windows_core::Ref<IXpsOMDictionary>) -> windows_core::Result<()>;
    fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&self, remotedictionaryresource: windows_core::Ref<IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMCanvas>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMCanvas_Vtbl {
    pub const fn new<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVisuals<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visuals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetVisuals(this) {
                    Ok(ok__) => {
                        visuals.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usealiasededgemode: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetUseAliasedEdgeMode(this) {
                    Ok(ok__) => {
                        usealiasededgemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usealiasededgemode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCanvas_Impl::SetUseAliasedEdgeMode(this, core::mem::transmute_copy(&usealiasededgemode)).into()
            }
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetAccessibilityShortDescription(this) {
                    Ok(ok__) => {
                        shortdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCanvas_Impl::SetAccessibilityShortDescription(this, core::mem::transmute(&shortdescription)).into()
            }
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetAccessibilityLongDescription(this) {
                    Ok(ok__) => {
                        longdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCanvas_Impl::SetAccessibilityLongDescription(this, core::mem::transmute(&longdescription)).into()
            }
        }
        unsafe extern "system" fn GetDictionary<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetDictionary(this) {
                    Ok(ok__) => {
                        resourcedictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetDictionaryLocal(this) {
                    Ok(ok__) => {
                        resourcedictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCanvas_Impl::SetDictionaryLocal(this, core::mem::transmute_copy(&resourcedictionary)).into()
            }
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::GetDictionaryResource(this) {
                    Ok(ok__) => {
                        remotedictionaryresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCanvas_Impl::SetDictionaryResource(this, core::mem::transmute_copy(&remotedictionaryresource)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMCanvas_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canvas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCanvas_Impl::Clone(this) {
                    Ok(ok__) => {
                        canvas.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, OFFSET>(),
            GetVisuals: GetVisuals::<Identity, OFFSET>,
            GetUseAliasedEdgeMode: GetUseAliasedEdgeMode::<Identity, OFFSET>,
            SetUseAliasedEdgeMode: SetUseAliasedEdgeMode::<Identity, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, OFFSET>,
            GetDictionary: GetDictionary::<Identity, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMCanvas as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMVisual as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMCanvas {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResource_Impl: IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResource_Vtbl {
    pub const fn new<Identity: IXpsOMColorProfileResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStream<Identity: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMColorProfileResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMColorProfileResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMColorProfileResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&partname)).into()
            }
        }
        Self { base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, OFFSET>, SetContent: SetContent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMColorProfileResource {}
windows_core::imp::define_interface!(IXpsOMColorProfileResourceCollection, IXpsOMColorProfileResourceCollection_Vtbl, 0x12759630_5fba_4283_8f7d_cca849809edb);
windows_core::imp::interface_hierarchy!(IXpsOMColorProfileResourceCollection, windows_core::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMColorProfileResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn InsertAt(&self, index: u32, object: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Append(&self, object: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMColorProfileResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMColorProfileResourceCollection_Vtbl {
    pub const fn new<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMColorProfileResourceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMColorProfileResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        object.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMColorProfileResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMColorProfileResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMColorProfileResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMColorProfileResourceCollection_Impl::Append(this, core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn GetByPartName<Identity: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMColorProfileResourceCollection_Impl::GetByPartName(this, core::mem::transmute_copy(&partname)) {
                    Ok(ok__) => {
                        part.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMColorProfileResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMColorProfileResourceCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCategory<P0>(&self, category: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCategory)(windows_core::Interface::as_raw(self), category.param().abi()).ok() }
    }
    pub unsafe fn GetContentStatus(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetContentStatus<P0>(&self, contentstatus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContentStatus)(windows_core::Interface::as_raw(self), contentstatus.param().abi()).ok() }
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetContentType<P0>(&self, contenttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContentType)(windows_core::Interface::as_raw(self), contenttype.param().abi()).ok() }
    }
    pub unsafe fn GetCreated(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCreated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCreated)(windows_core::Interface::as_raw(self), created).ok() }
    }
    pub unsafe fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCreator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreator<P0>(&self, creator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCreator)(windows_core::Interface::as_raw(self), creator.param().abi()).ok() }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok() }
    }
    pub unsafe fn GetIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIdentifier<P0>(&self, identifier: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIdentifier)(windows_core::Interface::as_raw(self), identifier.param().abi()).ok() }
    }
    pub unsafe fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeywords<P0>(&self, keywords: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKeywords)(windows_core::Interface::as_raw(self), keywords.param().abi()).ok() }
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), language.param().abi()).ok() }
    }
    pub unsafe fn GetLastModifiedBy(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastModifiedBy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastModifiedBy<P0>(&self, lastmodifiedby: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLastModifiedBy)(windows_core::Interface::as_raw(self), lastmodifiedby.param().abi()).ok() }
    }
    pub unsafe fn GetLastPrinted(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPrinted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLastPrinted)(windows_core::Interface::as_raw(self), lastprinted).ok() }
    }
    pub unsafe fn GetModified(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModified)(windows_core::Interface::as_raw(self), modified).ok() }
    }
    pub unsafe fn GetRevision(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRevision<P0>(&self, revision: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRevision)(windows_core::Interface::as_raw(self), revision.param().abi()).ok() }
    }
    pub unsafe fn GetSubject(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubject)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSubject<P0>(&self, subject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), subject.param().abi()).ok() }
    }
    pub unsafe fn GetTitle(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTitle<P0>(&self, title: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), title.param().abi()).ok() }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVersion<P0>(&self, version: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), version.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMCoreProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMCoreProperties_Impl: IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPackage>;
    fn GetCategory(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCategory(&self, category: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetContentStatus(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetContentStatus(&self, contentstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetContentType(&self, contenttype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCreated(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCreator(&self, creator: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDescription(&self, description: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIdentifier(&self, identifier: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetKeywords(&self, keywords: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLanguage(&self, language: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLastModifiedBy(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLastModifiedBy(&self, lastmodifiedby: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLastPrinted(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetModified(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetRevision(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRevision(&self, revision: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSubject(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSubject(&self, subject: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTitle(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTitle(&self, title: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetVersion(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetVersion(&self, version: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMCoreProperties>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMCoreProperties_Vtbl {
    pub const fn new<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategory<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetCategory(this) {
                    Ok(ok__) => {
                        category.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCategory<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetCategory(this, core::mem::transmute(&category)).into()
            }
        }
        unsafe extern "system" fn GetContentStatus<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentstatus: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetContentStatus(this) {
                    Ok(ok__) => {
                        contentstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentStatus<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentstatus: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetContentStatus(this, core::mem::transmute(&contentstatus)).into()
            }
        }
        unsafe extern "system" fn GetContentType<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetContentType(this) {
                    Ok(ok__) => {
                        contenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentType<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetContentType(this, core::mem::transmute(&contenttype)).into()
            }
        }
        unsafe extern "system" fn GetCreated<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetCreated(this) {
                    Ok(ok__) => {
                        created.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreated<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetCreated(this, core::mem::transmute_copy(&created)).into()
            }
        }
        unsafe extern "system" fn GetCreator<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creator: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetCreator(this) {
                    Ok(ok__) => {
                        creator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreator<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creator: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetCreator(this, core::mem::transmute(&creator)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn GetIdentifier<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetIdentifier(this) {
                    Ok(ok__) => {
                        identifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIdentifier<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetIdentifier(this, core::mem::transmute(&identifier)).into()
            }
        }
        unsafe extern "system" fn GetKeywords<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetKeywords(this) {
                    Ok(ok__) => {
                        keywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetKeywords(this, core::mem::transmute(&keywords)).into()
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        language.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
            }
        }
        unsafe extern "system" fn GetLastModifiedBy<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodifiedby: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetLastModifiedBy(this) {
                    Ok(ok__) => {
                        lastmodifiedby.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastModifiedBy<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodifiedby: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetLastModifiedBy(this, core::mem::transmute(&lastmodifiedby)).into()
            }
        }
        unsafe extern "system" fn GetLastPrinted<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetLastPrinted(this) {
                    Ok(ok__) => {
                        lastprinted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastPrinted<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetLastPrinted(this, core::mem::transmute_copy(&lastprinted)).into()
            }
        }
        unsafe extern "system" fn GetModified<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetModified(this) {
                    Ok(ok__) => {
                        modified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModified<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetModified(this, core::mem::transmute_copy(&modified)).into()
            }
        }
        unsafe extern "system" fn GetRevision<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, revision: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetRevision(this) {
                    Ok(ok__) => {
                        revision.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRevision<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, revision: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetRevision(this, core::mem::transmute(&revision)).into()
            }
        }
        unsafe extern "system" fn GetSubject<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetSubject(this) {
                    Ok(ok__) => {
                        subject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubject<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetSubject(this, core::mem::transmute(&subject)).into()
            }
        }
        unsafe extern "system" fn GetTitle<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetTitle(this) {
                    Ok(ok__) => {
                        title.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetTitle(this, core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVersion<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMCoreProperties_Impl::SetVersion(this, core::mem::transmute(&version)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMCoreProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMCoreProperties_Impl::Clone(this) {
                    Ok(ok__) => {
                        coreproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
            SetCategory: SetCategory::<Identity, OFFSET>,
            GetContentStatus: GetContentStatus::<Identity, OFFSET>,
            SetContentStatus: SetContentStatus::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            SetContentType: SetContentType::<Identity, OFFSET>,
            GetCreated: GetCreated::<Identity, OFFSET>,
            SetCreated: SetCreated::<Identity, OFFSET>,
            GetCreator: GetCreator::<Identity, OFFSET>,
            SetCreator: SetCreator::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, OFFSET>,
            SetIdentifier: SetIdentifier::<Identity, OFFSET>,
            GetKeywords: GetKeywords::<Identity, OFFSET>,
            SetKeywords: SetKeywords::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            GetLastModifiedBy: GetLastModifiedBy::<Identity, OFFSET>,
            SetLastModifiedBy: SetLastModifiedBy::<Identity, OFFSET>,
            GetLastPrinted: GetLastPrinted::<Identity, OFFSET>,
            SetLastPrinted: SetLastPrinted::<Identity, OFFSET>,
            GetModified: GetModified::<Identity, OFFSET>,
            SetModified: SetModified::<Identity, OFFSET>,
            GetRevision: GetRevision::<Identity, OFFSET>,
            SetRevision: SetRevision::<Identity, OFFSET>,
            GetSubject: GetSubject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            GetTitle: GetTitle::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMCoreProperties as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMCoreProperties {}
windows_core::imp::define_interface!(IXpsOMDashCollection, IXpsOMDashCollection_Vtbl, 0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
windows_core::imp::interface_hierarchy!(IXpsOMDashCollection, windows_core::IUnknown);
impl IXpsOMDashCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<XPS_DASH> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, dash).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, dash).ok() }
    }
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), dash).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDashCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut XPS_DASH) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XPS_DASH) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XPS_DASH) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_DASH) -> windows_core::HRESULT,
}
pub trait IXpsOMDashCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<XPS_DASH>;
    fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> windows_core::Result<()>;
    fn Append(&self, dash: *const XPS_DASH) -> windows_core::Result<()>;
}
impl IXpsOMDashCollection_Vtbl {
    pub const fn new<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDashCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDashCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        dash.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDashCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dash)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDashCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDashCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dash)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMDashCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dash: *const XPS_DASH) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDashCollection_Impl::Append(this, core::mem::transmute_copy(&dash)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDashCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMDashCollection {}
windows_core::imp::define_interface!(IXpsOMDictionary, IXpsOMDictionary_Vtbl, 0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
windows_core::imp::interface_hierarchy!(IXpsOMDictionary, windows_core::IUnknown);
impl IXpsOMDictionary {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32, key: *mut windows_core::PWSTR) -> windows_core::Result<IXpsOMShareable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, key as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetByKey<P0, P1>(&self, key: P0, beforeentry: P1) -> windows_core::Result<IXpsOMShareable>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IXpsOMShareable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByKey)(windows_core::Interface::as_raw(self), key.param().abi(), beforeentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIndex<P0>(&self, entry: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IXpsOMShareable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), entry.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Append<P0, P1>(&self, key: P0, entry: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IXpsOMShareable>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), key.param().abi(), entry.param().abi()).ok() }
    }
    pub unsafe fn InsertAt<P1, P2>(&self, index: u32, key: P1, entry: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IXpsOMShareable>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, key.param().abi(), entry.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1, P2>(&self, index: u32, key: P1, entry: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IXpsOMShareable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, key.param().abi(), entry.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IXpsOMDictionary_Impl: windows_core::IUnknownImpl {
    fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32, key: *mut windows_core::PWSTR) -> windows_core::Result<IXpsOMShareable>;
    fn GetByKey(&self, key: &windows_core::PCWSTR, beforeentry: windows_core::Ref<IXpsOMShareable>) -> windows_core::Result<IXpsOMShareable>;
    fn GetIndex(&self, entry: windows_core::Ref<IXpsOMShareable>) -> windows_core::Result<u32>;
    fn Append(&self, key: &windows_core::PCWSTR, entry: windows_core::Ref<IXpsOMShareable>) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, key: &windows_core::PCWSTR, entry: windows_core::Ref<IXpsOMShareable>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, key: &windows_core::PCWSTR, entry: windows_core::Ref<IXpsOMShareable>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMDictionary>;
}
impl IXpsOMDictionary_Vtbl {
    pub const fn new<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDictionary_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDictionary_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: *mut windows_core::PWSTR, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDictionary_Impl::GetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        entry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetByKey<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR, beforeentry: *mut core::ffi::c_void, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDictionary_Impl::GetByKey(this, core::mem::transmute(&key), core::mem::transmute_copy(&beforeentry)) {
                    Ok(ok__) => {
                        entry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndex<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entry: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDictionary_Impl::GetIndex(this, core::mem::transmute_copy(&entry)) {
                    Ok(ok__) => {
                        index.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDictionary_Impl::Append(this, core::mem::transmute(&key), core::mem::transmute_copy(&entry)).into()
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDictionary_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute(&key), core::mem::transmute_copy(&entry)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDictionary_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, key: windows_core::PCWSTR, entry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDictionary_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute(&key), core::mem::transmute_copy(&entry)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDictionary_Impl::Clone(this) {
                    Ok(ok__) => {
                        dictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetByKey: GetByKey::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDictionary as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMDictionary {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPageReferences(&self) -> windows_core::Result<IXpsOMPageReferenceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageReferences)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintTicketResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPrintTicketResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrintTicketResource)(windows_core::Interface::as_raw(self), printticketresource.param().abi()).ok() }
    }
    pub unsafe fn GetDocumentStructureResource(&self) -> windows_core::Result<IXpsOMDocumentStructureResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentStructureResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentStructureResource<P0>(&self, documentstructureresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocumentStructureResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentStructureResource)(windows_core::Interface::as_raw(self), documentstructureresource.param().abi()).ok() }
    }
    pub unsafe fn GetSignatureBlockResources(&self) -> windows_core::Result<IXpsOMSignatureBlockResourceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureBlockResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocument_Impl: IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocumentSequence>;
    fn GetPageReferences(&self) -> windows_core::Result<IXpsOMPageReferenceCollection>;
    fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: windows_core::Ref<IXpsOMPrintTicketResource>) -> windows_core::Result<()>;
    fn GetDocumentStructureResource(&self) -> windows_core::Result<IXpsOMDocumentStructureResource>;
    fn SetDocumentStructureResource(&self, documentstructureresource: windows_core::Ref<IXpsOMDocumentStructureResource>) -> windows_core::Result<()>;
    fn GetSignatureBlockResources(&self) -> windows_core::Result<IXpsOMSignatureBlockResourceCollection>;
    fn Clone(&self) -> windows_core::Result<IXpsOMDocument>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocument_Vtbl {
    pub const fn new<Identity: IXpsOMDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocument_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        documentsequence.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPageReferences<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereferences: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocument_Impl::GetPageReferences(this) {
                    Ok(ok__) => {
                        pagereferences.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocument_Impl::GetPrintTicketResource(this) {
                    Ok(ok__) => {
                        printticketresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocument_Impl::SetPrintTicketResource(this, core::mem::transmute_copy(&printticketresource)).into()
            }
        }
        unsafe extern "system" fn GetDocumentStructureResource<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentstructureresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocument_Impl::GetDocumentStructureResource(this) {
                    Ok(ok__) => {
                        documentstructureresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentStructureResource<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentstructureresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocument_Impl::SetDocumentStructureResource(this, core::mem::transmute_copy(&documentstructureresource)).into()
            }
        }
        unsafe extern "system" fn GetSignatureBlockResources<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblockresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocument_Impl::GetSignatureBlockResources(this) {
                    Ok(ok__) => {
                        signatureblockresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocument_Impl::Clone(this) {
                    Ok(ok__) => {
                        document.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetPageReferences: GetPageReferences::<Identity, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, OFFSET>,
            GetDocumentStructureResource: GetDocumentStructureResource::<Identity, OFFSET>,
            SetDocumentStructureResource: SetDocumentStructureResource::<Identity, OFFSET>,
            GetSignatureBlockResources: GetSignatureBlockResources::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocument as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMDocument {}
windows_core::imp::define_interface!(IXpsOMDocumentCollection, IXpsOMDocumentCollection_Vtbl, 0xd1c87f0d_e947_4754_8a25_971478f7e83e);
windows_core::imp::interface_hierarchy!(IXpsOMDocumentCollection, windows_core::IUnknown);
impl IXpsOMDocumentCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, document: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, document.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, document: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, document.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, document: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), document.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMDocumentCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMDocument>;
    fn InsertAt(&self, index: u32, document: windows_core::Ref<IXpsOMDocument>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, document: windows_core::Ref<IXpsOMDocument>) -> windows_core::Result<()>;
    fn Append(&self, document: windows_core::Ref<IXpsOMDocument>) -> windows_core::Result<()>;
}
impl IXpsOMDocumentCollection_Vtbl {
    pub const fn new<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        document.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocumentCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&document)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocumentCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocumentCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&document)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMDocumentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocumentCollection_Impl::Append(this, core::mem::transmute_copy(&document)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMDocumentCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocuments(&self) -> windows_core::Result<IXpsOMDocumentCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocuments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintTicketResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPrintTicketResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrintTicketResource)(windows_core::Interface::as_raw(self), printticketresource.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentSequence_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentSequence_Impl: IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPackage>;
    fn GetDocuments(&self) -> windows_core::Result<IXpsOMDocumentCollection>;
    fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: windows_core::Ref<IXpsOMPrintTicketResource>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentSequence_Vtbl {
    pub const fn new<Identity: IXpsOMDocumentSequence_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentSequence_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocuments<Identity: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentSequence_Impl::GetDocuments(this) {
                    Ok(ok__) => {
                        documents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentSequence_Impl::GetPrintTicketResource(this) {
                    Ok(ok__) => {
                        printticketresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: IXpsOMDocumentSequence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocumentSequence_Impl::SetPrintTicketResource(this, core::mem::transmute_copy(&printticketresource)).into()
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetDocuments: GetDocuments::<Identity, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentSequence as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMDocumentSequence {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentStructureResource_Impl: IXpsOMResource_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMDocumentStructureResource_Vtbl {
    pub const fn new<Identity: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentStructureResource_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMDocumentStructureResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMDocumentStructureResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMDocumentStructureResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&partname)).into()
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMDocumentStructureResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMDocumentStructureResource {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P2>(&self, sourcestream: P0, embeddingoption: XPS_FONT_EMBEDDING, partname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), embeddingoption, partname.param().abi()).ok() }
    }
    pub unsafe fn GetEmbeddingOption(&self) -> windows_core::Result<XPS_FONT_EMBEDDING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddingOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResource_Impl: IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, embeddingoption: XPS_FONT_EMBEDDING, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetEmbeddingOption(&self) -> windows_core::Result<XPS_FONT_EMBEDDING>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResource_Vtbl {
    pub const fn new<Identity: IXpsOMFontResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStream<Identity: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readerstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMFontResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        readerstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMFontResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&embeddingoption), core::mem::transmute_copy(&partname)).into()
            }
        }
        unsafe extern "system" fn GetEmbeddingOption<Identity: IXpsOMFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMFontResource_Impl::GetEmbeddingOption(this) {
                    Ok(ok__) => {
                        embeddingoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            GetEmbeddingOption: GetEmbeddingOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMFontResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMFontResource {}
windows_core::imp::define_interface!(IXpsOMFontResourceCollection, IXpsOMFontResourceCollection_Vtbl, 0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
windows_core::imp::interface_hierarchy!(IXpsOMFontResourceCollection, windows_core::IUnknown);
impl IXpsOMFontResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMFontResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMFontResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok() }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMFontResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMFontResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMFontResource>;
    fn SetAt(&self, index: u32, value: windows_core::Ref<IXpsOMFontResource>) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: windows_core::Ref<IXpsOMFontResource>) -> windows_core::Result<()>;
    fn Append(&self, value: windows_core::Ref<IXpsOMFontResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMFontResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMFontResourceCollection_Vtbl {
    pub const fn new<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMFontResourceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMFontResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMFontResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMFontResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMFontResourceCollection_Impl::Append(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMFontResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn GetByPartName<Identity: IXpsOMFontResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMFontResourceCollection_Impl::GetByPartName(this, core::mem::transmute_copy(&partname)) {
                    Ok(ok__) => {
                        part.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMFontResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMFontResourceCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFigures)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFillRule(&self) -> windows_core::Result<XPS_FILL_RULE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFillRule)(windows_core::Interface::as_raw(self), fillrule).ok() }
    }
    pub unsafe fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), transform.param().abi()).ok() }
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransformLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IXpsOMGeometry_Impl: IXpsOMShareable_Impl {
    fn GetFigures(&self) -> windows_core::Result<IXpsOMGeometryFigureCollection>;
    fn GetFillRule(&self) -> windows_core::Result<XPS_FILL_RULE>;
    fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> windows_core::Result<()>;
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: windows_core::Ref<IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGeometry>;
}
impl IXpsOMGeometry_Vtbl {
    pub const fn new<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFigures<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, figures: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometry_Impl::GetFigures(this) {
                    Ok(ok__) => {
                        figures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFillRule<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometry_Impl::GetFillRule(this) {
                    Ok(ok__) => {
                        fillrule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillRule<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillrule: XPS_FILL_RULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometry_Impl::SetFillRule(this, core::mem::transmute_copy(&fillrule)).into()
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometry_Impl::GetTransform(this) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometry_Impl::GetTransformLocal(this) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometry_Impl::SetTransformLocal(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetTransformLookup<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometry_Impl::GetTransformLookup(this) {
                    Ok(ok__) => {
                        lookup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometry_Impl::SetTransformLookup(this, core::mem::transmute(&lookup)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometry_Impl::Clone(this) {
                    Ok(ok__) => {
                        geometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(),
            GetFigures: GetFigures::<Identity, OFFSET>,
            GetFillRule: GetFillRule::<Identity, OFFSET>,
            SetFillRule: SetFillRule::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometry as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGeometry {}
windows_core::imp::define_interface!(IXpsOMGeometryFigure, IXpsOMGeometryFigure_Vtbl, 0xd410dc83_908c_443e_8947_b1795d3c165a);
windows_core::imp::interface_hierarchy!(IXpsOMGeometryFigure, windows_core::IUnknown);
impl IXpsOMGeometryFigure {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSegmentData)(windows_core::Interface::as_raw(self), datacount as _, segmentdata as _).ok() }
    }
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSegmentTypes)(windows_core::Interface::as_raw(self), segmentcount as _, segmenttypes as _).ok() }
    }
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSegmentStrokes)(windows_core::Interface::as_raw(self), segmentcount as _, segmentstrokes as _).ok() }
    }
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSegments)(windows_core::Interface::as_raw(self), segmentcount, segmentdatacount, segmenttypes, segmentdata, segmentstrokes).ok() }
    }
    pub unsafe fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), startpoint).ok() }
    }
    pub unsafe fn GetIsClosed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsClosed(&self, isclosed: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsClosed)(windows_core::Interface::as_raw(self), isclosed.into()).ok() }
    }
    pub unsafe fn GetIsFilled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsFilled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsFilled(&self, isfilled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsFilled)(windows_core::Interface::as_raw(self), isfilled.into()).ok() }
    }
    pub unsafe fn GetSegmentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSegmentDataCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSegmentDataCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSegmentStrokePattern(&self) -> windows_core::Result<XPS_SEGMENT_STROKE_PATTERN> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSegmentStrokePattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGeometryFigure> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigure_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut f32) -> windows_core::HRESULT,
    pub GetSegmentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut XPS_SEGMENT_TYPE) -> windows_core::HRESULT,
    pub GetSegmentStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const XPS_SEGMENT_TYPE, *const f32, *const windows_core::BOOL) -> windows_core::HRESULT,
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub GetIsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetIsFilled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSegmentStrokePattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_SEGMENT_STROKE_PATTERN) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMGeometryFigure_Impl: windows_core::IUnknownImpl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::Result<()>;
    fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::Result<()>;
    fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const windows_core::BOOL) -> windows_core::Result<()>;
    fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT>;
    fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetIsClosed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsClosed(&self, isclosed: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetIsFilled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsFilled(&self, isfilled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSegmentCount(&self) -> windows_core::Result<u32>;
    fn GetSegmentDataCount(&self) -> windows_core::Result<u32>;
    fn GetSegmentStrokePattern(&self) -> windows_core::Result<XPS_SEGMENT_STROKE_PATTERN>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGeometryFigure>;
}
impl IXpsOMGeometryFigure_Vtbl {
    pub const fn new<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSegmentData<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::GetSegmentData(this, core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&segmentdata)).into()
            }
        }
        unsafe extern "system" fn GetSegmentTypes<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::GetSegmentTypes(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmenttypes)).into()
            }
        }
        unsafe extern "system" fn GetSegmentStrokes<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::GetSegmentStrokes(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmentstrokes)).into()
            }
        }
        unsafe extern "system" fn SetSegments<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::SetSegments(this, core::mem::transmute_copy(&segmentcount), core::mem::transmute_copy(&segmentdatacount), core::mem::transmute_copy(&segmenttypes), core::mem::transmute_copy(&segmentdata), core::mem::transmute_copy(&segmentstrokes)).into()
            }
        }
        unsafe extern "system" fn GetStartPoint<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *mut XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetStartPoint(this) {
                    Ok(ok__) => {
                        startpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint)).into()
            }
        }
        unsafe extern "system" fn GetIsClosed<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetIsClosed(this) {
                    Ok(ok__) => {
                        isclosed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsClosed<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::SetIsClosed(this, core::mem::transmute_copy(&isclosed)).into()
            }
        }
        unsafe extern "system" fn GetIsFilled<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isfilled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetIsFilled(this) {
                    Ok(ok__) => {
                        isfilled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsFilled<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isfilled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigure_Impl::SetIsFilled(this, core::mem::transmute_copy(&isfilled)).into()
            }
        }
        unsafe extern "system" fn GetSegmentCount<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetSegmentCount(this) {
                    Ok(ok__) => {
                        segmentcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentdatacount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetSegmentDataCount(this) {
                    Ok(ok__) => {
                        segmentdatacount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::GetSegmentStrokePattern(this) {
                    Ok(ok__) => {
                        segmentstrokepattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMGeometryFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryfigure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigure_Impl::Clone(this) {
                    Ok(ok__) => {
                        geometryfigure.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, OFFSET>,
            GetSegmentTypes: GetSegmentTypes::<Identity, OFFSET>,
            GetSegmentStrokes: GetSegmentStrokes::<Identity, OFFSET>,
            SetSegments: SetSegments::<Identity, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            GetIsClosed: GetIsClosed::<Identity, OFFSET>,
            SetIsClosed: SetIsClosed::<Identity, OFFSET>,
            GetIsFilled: GetIsFilled::<Identity, OFFSET>,
            SetIsFilled: SetIsFilled::<Identity, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, OFFSET>,
            GetSegmentStrokePattern: GetSegmentStrokePattern::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigure as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGeometryFigure {}
windows_core::imp::define_interface!(IXpsOMGeometryFigureCollection, IXpsOMGeometryFigureCollection_Vtbl, 0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
windows_core::imp::interface_hierarchy!(IXpsOMGeometryFigureCollection, windows_core::IUnknown);
impl IXpsOMGeometryFigureCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGeometryFigure> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, geometryfigure: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMGeometryFigure>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, geometryfigure.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, geometryfigure: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMGeometryFigure>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, geometryfigure.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, geometryfigure: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometryFigure>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), geometryfigure.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMGeometryFigureCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGeometryFigure>;
    fn InsertAt(&self, index: u32, geometryfigure: windows_core::Ref<IXpsOMGeometryFigure>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, geometryfigure: windows_core::Ref<IXpsOMGeometryFigure>) -> windows_core::Result<()>;
    fn Append(&self, geometryfigure: windows_core::Ref<IXpsOMGeometryFigure>) -> windows_core::Result<()>;
}
impl IXpsOMGeometryFigureCollection_Vtbl {
    pub const fn new<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigureCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGeometryFigureCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        geometryfigure.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigureCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&geometryfigure)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigureCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigureCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&geometryfigure)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMGeometryFigureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryfigure: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGeometryFigureCollection_Impl::Append(this, core::mem::transmute_copy(&geometryfigure)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGeometryFigureCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGeometryFigureCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnicodeString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphIndexCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphIndices)(windows_core::Interface::as_raw(self), indexcount as _, glyphindices as _).ok() }
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphMappingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphMappings)(windows_core::Interface::as_raw(self), glyphmappingcount as _, glyphmappings as _).ok() }
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProhibitedCaretStopCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProhibitedCaretStops)(windows_core::Interface::as_raw(self), prohibitedcaretstopcount as _, prohibitedcaretstops as _).ok() }
    }
    pub unsafe fn GetBidiLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBidiLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIsSideways(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsSideways)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceFontName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStyleSimulations(&self) -> windows_core::Result<XPS_STYLE_SIMULATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyleSimulations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStyleSimulations)(windows_core::Interface::as_raw(self), stylesimulations).ok() }
    }
    pub unsafe fn GetOrigin(&self) -> windows_core::Result<XPS_POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOrigin)(windows_core::Interface::as_raw(self), origin).ok() }
    }
    pub unsafe fn GetFontRenderingEmSize(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontRenderingEmSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFontRenderingEmSize)(windows_core::Interface::as_raw(self), fontrenderingemsize).ok() }
    }
    pub unsafe fn GetFontResource(&self) -> windows_core::Result<IXpsOMFontResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFontResource<P0>(&self, fontresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontResource)(windows_core::Interface::as_raw(self), fontresource.param().abi()).ok() }
    }
    pub unsafe fn GetFontFaceIndex(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFontFaceIndex)(windows_core::Interface::as_raw(self), fontfaceindex).ok() }
    }
    pub unsafe fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, fillbrush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFillBrushLocal)(windows_core::Interface::as_raw(self), fillbrush.param().abi()).ok() }
    }
    pub unsafe fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFillBrushLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok() }
    }
    pub unsafe fn GetGlyphsEditor(&self) -> windows_core::Result<IXpsOMGlyphsEditor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphsEditor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGlyphs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetIsSideways: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMGlyphs_Impl: IXpsOMVisual_Impl {
    fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetGlyphIndexCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()>;
    fn GetGlyphMappingCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32>;
    fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()>;
    fn GetBidiLevel(&self) -> windows_core::Result<u32>;
    fn GetIsSideways(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetStyleSimulations(&self) -> windows_core::Result<XPS_STYLE_SIMULATION>;
    fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::Result<()>;
    fn GetOrigin(&self) -> windows_core::Result<XPS_POINT>;
    fn SetOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetFontRenderingEmSize(&self) -> windows_core::Result<f32>;
    fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> windows_core::Result<()>;
    fn GetFontResource(&self) -> windows_core::Result<IXpsOMFontResource>;
    fn SetFontResource(&self, fontresource: windows_core::Ref<IXpsOMFontResource>) -> windows_core::Result<()>;
    fn GetFontFaceIndex(&self) -> windows_core::Result<i16>;
    fn SetFontFaceIndex(&self, fontfaceindex: i16) -> windows_core::Result<()>;
    fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&self, fillbrush: windows_core::Ref<IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetFillBrushLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetGlyphsEditor(&self) -> windows_core::Result<IXpsOMGlyphsEditor>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGlyphs>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMGlyphs_Vtbl {
    pub const fn new<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUnicodeString<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetUnicodeString(this) {
                    Ok(ok__) => {
                        unicodestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetGlyphIndexCount(this) {
                    Ok(ok__) => {
                        indexcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
            }
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetGlyphMappingCount(this) {
                    Ok(ok__) => {
                        glyphmappingcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::GetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetProhibitedCaretStopCount(this) {
                    Ok(ok__) => {
                        prohibitedcaretstopcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::GetProhibitedCaretStops(this, core::mem::transmute_copy(&prohibitedcaretstopcount), core::mem::transmute_copy(&prohibitedcaretstops)).into()
            }
        }
        unsafe extern "system" fn GetBidiLevel<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetBidiLevel(this) {
                    Ok(ok__) => {
                        bidilevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIsSideways<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetIsSideways(this) {
                    Ok(ok__) => {
                        issideways.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetDeviceFontName(this) {
                    Ok(ok__) => {
                        devicefontname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStyleSimulations<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetStyleSimulations(this) {
                    Ok(ok__) => {
                        stylesimulations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyleSimulations<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetStyleSimulations(this, core::mem::transmute_copy(&stylesimulations)).into()
            }
        }
        unsafe extern "system" fn GetOrigin<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *mut XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetOrigin(this) {
                    Ok(ok__) => {
                        origin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOrigin<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *const XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetOrigin(this, core::mem::transmute_copy(&origin)).into()
            }
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontrenderingemsize: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetFontRenderingEmSize(this) {
                    Ok(ok__) => {
                        fontrenderingemsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontrenderingemsize: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetFontRenderingEmSize(this, core::mem::transmute_copy(&fontrenderingemsize)).into()
            }
        }
        unsafe extern "system" fn GetFontResource<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetFontResource(this) {
                    Ok(ok__) => {
                        fontresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontResource<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetFontResource(this, core::mem::transmute_copy(&fontresource)).into()
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfaceindex: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetFontFaceIndex(this) {
                    Ok(ok__) => {
                        fontfaceindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontFaceIndex<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfaceindex: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetFontFaceIndex(this, core::mem::transmute_copy(&fontfaceindex)).into()
            }
        }
        unsafe extern "system" fn GetFillBrush<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetFillBrush(this) {
                    Ok(ok__) => {
                        fillbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetFillBrushLocal(this) {
                    Ok(ok__) => {
                        fillbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillbrush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetFillBrushLocal(this, core::mem::transmute_copy(&fillbrush)).into()
            }
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetFillBrushLookup(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphs_Impl::SetFillBrushLookup(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn GetGlyphsEditor<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, editor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::GetGlyphsEditor(this) {
                    Ok(ok__) => {
                        editor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMGlyphs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphs_Impl::Clone(this) {
                    Ok(ok__) => {
                        glyphs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, OFFSET>(),
            GetUnicodeString: GetUnicodeString::<Identity, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, OFFSET>,
            GetStyleSimulations: GetStyleSimulations::<Identity, OFFSET>,
            SetStyleSimulations: SetStyleSimulations::<Identity, OFFSET>,
            GetOrigin: GetOrigin::<Identity, OFFSET>,
            SetOrigin: SetOrigin::<Identity, OFFSET>,
            GetFontRenderingEmSize: GetFontRenderingEmSize::<Identity, OFFSET>,
            SetFontRenderingEmSize: SetFontRenderingEmSize::<Identity, OFFSET>,
            GetFontResource: GetFontResource::<Identity, OFFSET>,
            SetFontResource: SetFontResource::<Identity, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, OFFSET>,
            SetFontFaceIndex: SetFontFaceIndex::<Identity, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, OFFSET>,
            GetGlyphsEditor: GetGlyphsEditor::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGlyphs as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMVisual as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMGlyphs {}
windows_core::imp::define_interface!(IXpsOMGlyphsEditor, IXpsOMGlyphsEditor_Vtbl, 0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
windows_core::imp::interface_hierarchy!(IXpsOMGlyphsEditor, windows_core::IUnknown);
impl IXpsOMGlyphsEditor {
    pub unsafe fn ApplyEdits(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplyEdits)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnicodeString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnicodeString<P0>(&self, unicodestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUnicodeString)(windows_core::Interface::as_raw(self), unicodestring.param().abi()).ok() }
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphIndexCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphIndices)(windows_core::Interface::as_raw(self), indexcount as _, glyphindices as _).ok() }
    }
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGlyphIndices)(windows_core::Interface::as_raw(self), indexcount, glyphindices).ok() }
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphMappingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphMappings)(windows_core::Interface::as_raw(self), glyphmappingcount as _, glyphmappings as _).ok() }
    }
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGlyphMappings)(windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok() }
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProhibitedCaretStopCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProhibitedCaretStops)(windows_core::Interface::as_raw(self), count as _, prohibitedcaretstops as _).ok() }
    }
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProhibitedCaretStops)(windows_core::Interface::as_raw(self), count, prohibitedcaretstops).ok() }
    }
    pub unsafe fn GetBidiLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBidiLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBidiLevel)(windows_core::Interface::as_raw(self), bidilevel).ok() }
    }
    pub unsafe fn GetIsSideways(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsSideways)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsSideways(&self, issideways: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsSideways)(windows_core::Interface::as_raw(self), issideways.into()).ok() }
    }
    pub unsafe fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceFontName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDeviceFontName<P0>(&self, devicefontname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDeviceFontName)(windows_core::Interface::as_raw(self), devicefontname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetIsSideways: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsSideways: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetDeviceFontName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDeviceFontName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IXpsOMGlyphsEditor_Impl: windows_core::IUnknownImpl {
    fn ApplyEdits(&self) -> windows_core::Result<()>;
    fn GetUnicodeString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUnicodeString(&self, unicodestring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetGlyphIndexCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::Result<()>;
    fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::Result<()>;
    fn GetGlyphMappingCount(&self) -> windows_core::Result<u32>;
    fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::Result<()>;
    fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(&self) -> windows_core::Result<u32>;
    fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::Result<()>;
    fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> windows_core::Result<()>;
    fn GetBidiLevel(&self) -> windows_core::Result<u32>;
    fn SetBidiLevel(&self, bidilevel: u32) -> windows_core::Result<()>;
    fn GetIsSideways(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsSideways(&self, issideways: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDeviceFontName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDeviceFontName(&self, devicefontname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IXpsOMGlyphsEditor_Vtbl {
    pub const fn new<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplyEdits<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::ApplyEdits(this).into()
            }
        }
        unsafe extern "system" fn GetUnicodeString<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetUnicodeString(this) {
                    Ok(ok__) => {
                        unicodestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnicodeString<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodestring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetUnicodeString(this, core::mem::transmute(&unicodestring)).into()
            }
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetGlyphIndexCount(this) {
                    Ok(ok__) => {
                        indexcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
            }
        }
        unsafe extern "system" fn SetGlyphIndices<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetGlyphIndices(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&glyphindices)).into()
            }
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetGlyphMappingCount(this) {
                    Ok(ok__) => {
                        glyphmappingcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::GetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
            }
        }
        unsafe extern "system" fn SetGlyphMappings<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetGlyphMappings(this, core::mem::transmute_copy(&glyphmappingcount), core::mem::transmute_copy(&glyphmappings)).into()
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetProhibitedCaretStopCount(this) {
                    Ok(ok__) => {
                        prohibitedcaretstopcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::GetProhibitedCaretStops(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prohibitedcaretstops)).into()
            }
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetProhibitedCaretStops(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prohibitedcaretstops)).into()
            }
        }
        unsafe extern "system" fn GetBidiLevel<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetBidiLevel(this) {
                    Ok(ok__) => {
                        bidilevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBidiLevel<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bidilevel: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetBidiLevel(this, core::mem::transmute_copy(&bidilevel)).into()
            }
        }
        unsafe extern "system" fn GetIsSideways<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetIsSideways(this) {
                    Ok(ok__) => {
                        issideways.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsSideways<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issideways: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetIsSideways(this, core::mem::transmute_copy(&issideways)).into()
            }
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGlyphsEditor_Impl::GetDeviceFontName(this) {
                    Ok(ok__) => {
                        devicefontname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDeviceFontName<Identity: IXpsOMGlyphsEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicefontname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGlyphsEditor_Impl::SetDeviceFontName(this, core::mem::transmute(&devicefontname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplyEdits: ApplyEdits::<Identity, OFFSET>,
            GetUnicodeString: GetUnicodeString::<Identity, OFFSET>,
            SetUnicodeString: SetUnicodeString::<Identity, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, OFFSET>,
            SetGlyphIndices: SetGlyphIndices::<Identity, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, OFFSET>,
            SetGlyphMappings: SetGlyphMappings::<Identity, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, OFFSET>,
            SetProhibitedCaretStops: SetProhibitedCaretStops::<Identity, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, OFFSET>,
            SetIsSideways: SetIsSideways::<Identity, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, OFFSET>,
            SetDeviceFontName: SetDeviceFontName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGlyphsEditor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGlyphsEditor {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGradientStops)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), transform.param().abi()).ok() }
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok() }
    }
    pub unsafe fn GetSpreadMethod(&self) -> windows_core::Result<XPS_SPREAD_METHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpreadMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpreadMethod)(windows_core::Interface::as_raw(self), spreadmethod).ok() }
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> windows_core::Result<XPS_COLOR_INTERPOLATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorInterpolationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColorInterpolationMode)(windows_core::Interface::as_raw(self), colorinterpolationmode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IXpsOMGradientBrush_Impl: IXpsOMBrush_Impl {
    fn GetGradientStops(&self) -> windows_core::Result<IXpsOMGradientStopCollection>;
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: windows_core::Ref<IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSpreadMethod(&self) -> windows_core::Result<XPS_SPREAD_METHOD>;
    fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::Result<()>;
    fn GetColorInterpolationMode(&self) -> windows_core::Result<XPS_COLOR_INTERPOLATION>;
    fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::Result<()>;
}
impl IXpsOMGradientBrush_Vtbl {
    pub const fn new<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGradientStops<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientBrush_Impl::GetGradientStops(this) {
                    Ok(ok__) => {
                        gradientstops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientBrush_Impl::GetTransform(this) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientBrush_Impl::GetTransformLocal(this) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientBrush_Impl::SetTransformLocal(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetTransformLookup<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientBrush_Impl::GetTransformLookup(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientBrush_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn GetSpreadMethod<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientBrush_Impl::GetSpreadMethod(this) {
                    Ok(ok__) => {
                        spreadmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpreadMethod<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientBrush_Impl::SetSpreadMethod(this, core::mem::transmute_copy(&spreadmethod)).into()
            }
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientBrush_Impl::GetColorInterpolationMode(this) {
                    Ok(ok__) => {
                        colorinterpolationmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorInterpolationMode<Identity: IXpsOMGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientBrush_Impl::SetColorInterpolationMode(this, core::mem::transmute_copy(&colorinterpolationmode)).into()
            }
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, OFFSET>(),
            GetGradientStops: GetGradientStops::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            GetSpreadMethod: GetSpreadMethod::<Identity, OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Identity, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGradientBrush {}
windows_core::imp::define_interface!(IXpsOMGradientStop, IXpsOMGradientStop_Vtbl, 0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
windows_core::imp::interface_hierarchy!(IXpsOMGradientStop, windows_core::IUnknown);
impl IXpsOMGradientStop {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMGradientBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOffset(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOffset(&self, offset: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOffset)(windows_core::Interface::as_raw(self), offset).ok() }
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), color as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetColor<P1>(&self, color: *const XPS_COLOR, colorprofile: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMGradientStop> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_COLOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_COLOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMGradientStop_Impl: windows_core::IUnknownImpl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMGradientBrush>;
    fn GetOffset(&self) -> windows_core::Result<f32>;
    fn SetOffset(&self, offset: f32) -> windows_core::Result<()>;
    fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColor(&self, color: *const XPS_COLOR, colorprofile: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMGradientStop>;
}
impl IXpsOMGradientStop_Vtbl {
    pub const fn new<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientStop_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOffset<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientStop_Impl::GetOffset(this) {
                    Ok(ok__) => {
                        offset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOffset<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientStop_Impl::SetOffset(this, core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn GetColor<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientStop_Impl::GetColor(this, core::mem::transmute_copy(&color)) {
                    Ok(ok__) => {
                        colorprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColor<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientStop_Impl::SetColor(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&colorprofile)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMGradientStop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientStop_Impl::Clone(this) {
                    Ok(ok__) => {
                        gradientstop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetOffset: GetOffset::<Identity, OFFSET>,
            SetOffset: SetOffset::<Identity, OFFSET>,
            GetColor: GetColor::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientStop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGradientStop {}
windows_core::imp::define_interface!(IXpsOMGradientStopCollection, IXpsOMGradientStopCollection_Vtbl, 0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
windows_core::imp::interface_hierarchy!(IXpsOMGradientStopCollection, windows_core::IUnknown);
impl IXpsOMGradientStopCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGradientStop> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, stop: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMGradientStop>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, stop.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, stop: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMGradientStop>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, stop.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), stop.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMGradientStopCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMGradientStop>;
    fn InsertAt(&self, index: u32, stop: windows_core::Ref<IXpsOMGradientStop>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, stop: windows_core::Ref<IXpsOMGradientStop>) -> windows_core::Result<()>;
    fn Append(&self, stop: windows_core::Ref<IXpsOMGradientStop>) -> windows_core::Result<()>;
}
impl IXpsOMGradientStopCollection_Vtbl {
    pub const fn new<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientStopCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMGradientStopCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        stop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientStopCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientStopCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientStopCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMGradientStopCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMGradientStopCollection_Impl::Append(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMGradientStopCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMGradientStopCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetImageResource<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetImageResource)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok() }
    }
    pub unsafe fn GetColorProfileResource(&self) -> windows_core::Result<IXpsOMColorProfileResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorProfileResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetColorProfileResource<P0>(&self, colorprofileresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetColorProfileResource)(windows_core::Interface::as_raw(self), colorprofileresource.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMImageBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetImageResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetImageResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorProfileResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorProfileResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMImageBrush_Impl: IXpsOMTileBrush_Impl {
    fn GetImageResource(&self) -> windows_core::Result<IXpsOMImageResource>;
    fn SetImageResource(&self, imageresource: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn GetColorProfileResource(&self) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColorProfileResource(&self, colorprofileresource: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMImageBrush>;
}
impl IXpsOMImageBrush_Vtbl {
    pub const fn new<Identity: IXpsOMImageBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageResource<Identity: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageBrush_Impl::GetImageResource(this) {
                    Ok(ok__) => {
                        imageresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImageResource<Identity: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageBrush_Impl::SetImageResource(this, core::mem::transmute_copy(&imageresource)).into()
            }
        }
        unsafe extern "system" fn GetColorProfileResource<Identity: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageBrush_Impl::GetColorProfileResource(this) {
                    Ok(ok__) => {
                        colorprofileresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorProfileResource<Identity: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageBrush_Impl::SetColorProfileResource(this, core::mem::transmute_copy(&colorprofileresource)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMImageBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageBrush_Impl::Clone(this) {
                    Ok(ok__) => {
                        imagebrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, OFFSET>(),
            GetImageResource: GetImageResource::<Identity, OFFSET>,
            SetImageResource: SetImageResource::<Identity, OFFSET>,
            GetColorProfileResource: GetColorProfileResource::<Identity, OFFSET>,
            SetColorProfileResource: SetColorProfileResource::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMImageBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMTileBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMImageBrush {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P2>(&self, sourcestream: P0, imagetype: XPS_IMAGE_TYPE, partname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), imagetype, partname.param().abi()).ok() }
    }
    pub unsafe fn GetImageType(&self) -> windows_core::Result<XPS_IMAGE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResource_Impl: IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, imagetype: XPS_IMAGE_TYPE, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetImageType(&self) -> windows_core::Result<XPS_IMAGE_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResource_Vtbl {
    pub const fn new<Identity: IXpsOMImageResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStream<Identity: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readerstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        readerstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&imagetype), core::mem::transmute_copy(&partname)).into()
            }
        }
        unsafe extern "system" fn GetImageType<Identity: IXpsOMImageResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageResource_Impl::GetImageType(this) {
                    Ok(ok__) => {
                        imagetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            GetImageType: GetImageType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMImageResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMImageResource {}
windows_core::imp::define_interface!(IXpsOMImageResourceCollection, IXpsOMImageResourceCollection_Vtbl, 0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
windows_core::imp::interface_hierarchy!(IXpsOMImageResourceCollection, windows_core::IUnknown);
impl IXpsOMImageResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMImageResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMImageResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMImageResource>;
    fn InsertAt(&self, index: u32, object: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn Append(&self, object: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMImageResourceCollection_Vtbl {
    pub const fn new<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageResourceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        object.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMImageResourceCollection_Impl::Append(this, core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn GetByPartName<Identity: IXpsOMImageResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, part: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMImageResourceCollection_Impl::GetByPartName(this, core::mem::transmute_copy(&partname)) {
                    Ok(ok__) => {
                        part.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMImageResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMImageResourceCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), startpoint).ok() }
    }
    pub unsafe fn GetEndPoint(&self) -> windows_core::Result<XPS_POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEndPoint)(windows_core::Interface::as_raw(self), endpoint).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMLinearGradientBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMLinearGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub GetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_POINT) -> windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_POINT) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMLinearGradientBrush_Impl: IXpsOMGradientBrush_Impl {
    fn GetStartPoint(&self) -> windows_core::Result<XPS_POINT>;
    fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetEndPoint(&self) -> windows_core::Result<XPS_POINT>;
    fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMLinearGradientBrush>;
}
impl IXpsOMLinearGradientBrush_Vtbl {
    pub const fn new<Identity: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStartPoint<Identity: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *mut XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMLinearGradientBrush_Impl::GetStartPoint(this) {
                    Ok(ok__) => {
                        startpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMLinearGradientBrush_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint)).into()
            }
        }
        unsafe extern "system" fn GetEndPoint<Identity: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: *mut XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMLinearGradientBrush_Impl::GetEndPoint(this) {
                    Ok(ok__) => {
                        endpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndPoint<Identity: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: *const XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMLinearGradientBrush_Impl::SetEndPoint(this, core::mem::transmute_copy(&endpoint)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMLinearGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMLinearGradientBrush_Impl::Clone(this) {
                    Ok(ok__) => {
                        lineargradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, OFFSET>(),
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMLinearGradientBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMGradientBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMLinearGradientBrush {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatrix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrix)(windows_core::Interface::as_raw(self), matrix).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMMatrixTransform_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_MATRIX) -> windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_MATRIX) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMMatrixTransform_Impl: IXpsOMShareable_Impl {
    fn GetMatrix(&self) -> windows_core::Result<XPS_MATRIX>;
    fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
}
impl IXpsOMMatrixTransform_Vtbl {
    pub const fn new<Identity: IXpsOMMatrixTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMatrix<Identity: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut XPS_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMMatrixTransform_Impl::GetMatrix(this) {
                    Ok(ok__) => {
                        matrix.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMatrix<Identity: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const XPS_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMMatrixTransform_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMMatrixTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMMatrixTransform_Impl::Clone(this) {
                    Ok(ok__) => {
                        matrixtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(),
            GetMatrix: GetMatrix::<Identity, OFFSET>,
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMMatrixTransform as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMMatrixTransform {}
windows_core::imp::define_interface!(IXpsOMNameCollection, IXpsOMNameCollection_Vtbl, 0x4bddf8ec_c915_421b_a166_d173d25653d2);
windows_core::imp::interface_hierarchy!(IXpsOMNameCollection, windows_core::IUnknown);
impl IXpsOMNameCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMNameCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IXpsOMNameCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IXpsOMNameCollection_Vtbl {
    pub const fn new<Identity: IXpsOMNameCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMNameCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMNameCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMNameCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMNameCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMNameCollection {}
windows_core::imp::define_interface!(IXpsOMObjectFactory, IXpsOMObjectFactory_Vtbl, 0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
windows_core::imp::interface_hierarchy!(IXpsOMObjectFactory, windows_core::IUnknown);
impl IXpsOMObjectFactory {
    pub unsafe fn CreatePackage(&self) -> windows_core::Result<IXpsOMPackage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePackageFromFile<P0>(&self, filename: P0, reuseobjects: bool) -> windows_core::Result<IXpsOMPackage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), reuseobjects.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageFromStream<P0>(&self, stream: P0, reuseobjects: bool) -> windows_core::Result<IXpsOMPackage>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageFromStream)(windows_core::Interface::as_raw(self), stream.param().abi(), reuseobjects.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMStoryFragmentsResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStoryFragmentsResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMDocumentStructureResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDocumentStructureResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSignatureBlockResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<P0, P1>(&self, dictionary: P0, parturi: P1) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRemoteDictionaryResource)(windows_core::Interface::as_raw(self), dictionary.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<P0, P1, P2>(&self, dictionarymarkupstream: P0, dictionaryparturi: P1, resources: P2) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream)(windows_core::Interface::as_raw(self), dictionarymarkupstream.param().abi(), dictionaryparturi.param().abi(), resources.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePartResources(&self) -> windows_core::Result<IXpsOMPartResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePartResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<P0>(&self, parturi: P0) -> windows_core::Result<IXpsOMDocumentSequence>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDocumentSequence)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<P0>(&self, parturi: P0) -> windows_core::Result<IXpsOMDocument>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDocument)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> windows_core::Result<IXpsOMPageReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePageReference)(windows_core::Interface::as_raw(self), advisorypagedimensions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<P1, P2>(&self, pagedimensions: *const XPS_SIZE, language: P1, parturi: P2) -> windows_core::Result<IXpsOMPage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePage)(windows_core::Interface::as_raw(self), pagedimensions, language.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<P0, P1, P2>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: bool) -> windows_core::Result<IXpsOMPage>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePageFromStream)(windows_core::Interface::as_raw(self), pagemarkupstream.param().abi(), parturi.param().abi(), resources.param().abi(), reuseobjects.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCanvas(&self) -> windows_core::Result<IXpsOMCanvas> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCanvas)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGlyphs<P0>(&self, fontresource: P0) -> windows_core::Result<IXpsOMGlyphs>
    where
        P0: windows_core::Param<IXpsOMFontResource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphs)(windows_core::Interface::as_raw(self), fontresource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePath(&self) -> windows_core::Result<IXpsOMPath> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGeometry(&self) -> windows_core::Result<IXpsOMGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMGeometryFigure> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGeometryFigure)(windows_core::Interface::as_raw(self), startpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform)(windows_core::Interface::as_raw(self), matrix, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSolidColorBrush<P1>(&self, color: *const XPS_COLOR, colorprofile: P1) -> windows_core::Result<IXpsOMSolidColorBrush>
    where
        P1: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSolidColorBrush)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorProfileResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMImageBrush>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageBrush)(windows_core::Interface::as_raw(self), image.param().abi(), viewbox, viewport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMVisualBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVisualBrush)(windows_core::Interface::as_raw(self), viewbox, viewport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<P0, P2>(&self, acquiredstream: P0, contenttype: XPS_IMAGE_TYPE, parturi: P2) -> windows_core::Result<IXpsOMImageResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), contenttype, parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> windows_core::Result<IXpsOMPrintTicketResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePrintTicketResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<P0, P2>(&self, acquiredstream: P0, fontembedding: XPS_FONT_EMBEDDING, parturi: P2, isobfsourcestream: bool) -> windows_core::Result<IXpsOMFontResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontResource)(windows_core::Interface::as_raw(self), acquiredstream.param().abi(), fontembedding, parturi.param().abi(), isobfsourcestream.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGradientStop<P1>(&self, color: *const XPS_COLOR, colorprofile: P1, offset: f32) -> windows_core::Result<IXpsOMGradientStop>
    where
        P1: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientStop)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi(), offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLinearGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMLinearGradientBrush>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
        P1: windows_core::Param<IXpsOMGradientStop>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(windows_core::Interface::as_raw(self), gradstop1.param().abi(), gradstop2.param().abi(), startpoint, endpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRadialGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> windows_core::Result<IXpsOMRadialGradientBrush>
    where
        P0: windows_core::Param<IXpsOMGradientStop>,
        P1: windows_core::Param<IXpsOMGradientStop>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRadialGradientBrush)(windows_core::Interface::as_raw(self), gradstop1.param().abi(), gradstop2.param().abi(), centerpoint, gradientorigin, radiisizes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<P0>(&self, parturi: P0) -> windows_core::Result<IXpsOMCoreProperties>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCoreProperties)(windows_core::Interface::as_raw(self), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePartUriCollection(&self) -> windows_core::Result<IXpsOMPartUriCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePartUriCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<P0, P5, P6, P7, P8, P9>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: bool, interleaving: XPS_INTERLEAVING, documentsequencepartname: P5, coreproperties: P6, packagethumbnail: P7, documentsequenceprintticket: P8, discardcontrolpartname: P9) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P6: windows_core::Param<IXpsOMCoreProperties>,
        P7: windows_core::Param<IXpsOMImageResource>,
        P8: windows_core::Param<IXpsOMPrintTicketResource>,
        P9: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageWriterOnFile)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<P0, P3, P4, P5, P6, P7>(&self, outputstream: P0, optimizemarkupsize: bool, interleaving: XPS_INTERLEAVING, documentsequencepartname: P3, coreproperties: P4, packagethumbnail: P5, documentsequenceprintticket: P6, discardcontrolpartname: P7) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P3: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P4: windows_core::Param<IXpsOMCoreProperties>,
        P5: windows_core::Param<IXpsOMImageResource>,
        P6: windows_core::Param<IXpsOMPrintTicketResource>,
        P7: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageWriterOnStream)(windows_core::Interface::as_raw(self), outputstream.param().abi(), optimizemarkupsize.into(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<P0>(&self, uri: P0) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePartUri)(windows_core::Interface::as_raw(self), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<P0>(&self, filename: P0) -> windows_core::Result<super::super::System::Com::IStream>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateReadOnlyStreamOnFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePackageFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub CreatePageFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub CreateFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_FONT_EMBEDDING, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub CreatePackageWriterOnFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory_Impl: windows_core::IUnknownImpl {
    fn CreatePackage(&self) -> windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromFile(&self, filename: &windows_core::PCWSTR, reuseobjects: windows_core::BOOL) -> windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromStream(&self, stream: windows_core::Ref<super::super::System::Com::IStream>, reuseobjects: windows_core::BOOL) -> windows_core::Result<IXpsOMPackage>;
    fn CreateStoryFragmentsResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn CreateDocumentStructureResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMDocumentStructureResource>;
    fn CreateSignatureBlockResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMSignatureBlockResource>;
    fn CreateRemoteDictionaryResource(&self, dictionary: windows_core::Ref<IXpsOMDictionary>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreateRemoteDictionaryResourceFromStream(&self, dictionarymarkupstream: windows_core::Ref<super::super::System::Com::IStream>, dictionaryparturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, resources: windows_core::Ref<IXpsOMPartResources>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreatePartResources(&self) -> windows_core::Result<IXpsOMPartResources>;
    fn CreateDocumentSequence(&self, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMDocumentSequence>;
    fn CreateDocument(&self, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMDocument>;
    fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> windows_core::Result<IXpsOMPageReference>;
    fn CreatePage(&self, pagedimensions: *const XPS_SIZE, language: &windows_core::PCWSTR, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPage>;
    fn CreatePageFromStream(&self, pagemarkupstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, resources: windows_core::Ref<IXpsOMPartResources>, reuseobjects: windows_core::BOOL) -> windows_core::Result<IXpsOMPage>;
    fn CreateCanvas(&self) -> windows_core::Result<IXpsOMCanvas>;
    fn CreateGlyphs(&self, fontresource: windows_core::Ref<IXpsOMFontResource>) -> windows_core::Result<IXpsOMGlyphs>;
    fn CreatePath(&self) -> windows_core::Result<IXpsOMPath>;
    fn CreateGeometry(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMGeometryFigure>;
    fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn CreateSolidColorBrush(&self, color: *const XPS_COLOR, colorprofile: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<IXpsOMSolidColorBrush>;
    fn CreateColorProfileResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn CreateImageBrush(&self, image: windows_core::Ref<IXpsOMImageResource>, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMImageBrush>;
    fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> windows_core::Result<IXpsOMVisualBrush>;
    fn CreateImageResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, contenttype: XPS_IMAGE_TYPE, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMImageResource>;
    fn CreatePrintTicketResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn CreateFontResource(&self, acquiredstream: windows_core::Ref<super::super::System::Com::IStream>, fontembedding: XPS_FONT_EMBEDDING, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, isobfsourcestream: windows_core::BOOL) -> windows_core::Result<IXpsOMFontResource>;
    fn CreateGradientStop(&self, color: *const XPS_COLOR, colorprofile: windows_core::Ref<IXpsOMColorProfileResource>, offset: f32) -> windows_core::Result<IXpsOMGradientStop>;
    fn CreateLinearGradientBrush(&self, gradstop1: windows_core::Ref<IXpsOMGradientStop>, gradstop2: windows_core::Ref<IXpsOMGradientStop>, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> windows_core::Result<IXpsOMLinearGradientBrush>;
    fn CreateRadialGradientBrush(&self, gradstop1: windows_core::Ref<IXpsOMGradientStop>, gradstop2: windows_core::Ref<IXpsOMGradientStop>, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> windows_core::Result<IXpsOMRadialGradientBrush>;
    fn CreateCoreProperties(&self, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMCoreProperties>;
    fn CreateDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn CreatePartUriCollection(&self) -> windows_core::Result<IXpsOMPartUriCollection>;
    fn CreatePackageWriterOnFile(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, coreproperties: windows_core::Ref<IXpsOMCoreProperties>, packagethumbnail: windows_core::Ref<IXpsOMImageResource>, documentsequenceprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream(&self, outputstream: windows_core::Ref<super::super::System::Com::ISequentialStream>, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, coreproperties: windows_core::Ref<IXpsOMCoreProperties>, packagethumbnail: windows_core::Ref<IXpsOMImageResource>, documentsequenceprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePartUri(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateReadOnlyStreamOnFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory_Vtbl {
    pub const fn new<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePackage<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePackage(this) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageFromFile<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, reuseobjects: windows_core::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePackageFromFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&reuseobjects)) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageFromStream<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, reuseobjects: windows_core::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePackageFromStream(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&reuseobjects)) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, storyfragmentsresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateStoryFragmentsResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        storyfragmentsresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, documentstructureresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateDocumentStructureResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        documentstructureresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateSignatureBlockResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        signatureblockresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateRemoteDictionaryResource(this, core::mem::transmute_copy(&dictionary), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        remotedictionaryresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarymarkupstream: *mut core::ffi::c_void, dictionaryparturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, dictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateRemoteDictionaryResourceFromStream(this, core::mem::transmute_copy(&dictionarymarkupstream), core::mem::transmute_copy(&dictionaryparturi), core::mem::transmute_copy(&resources)) {
                    Ok(ok__) => {
                        dictionaryresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePartResources<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePartResources(this) {
                    Ok(ok__) => {
                        partresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDocumentSequence<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateDocumentSequence(this, core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        documentsequence.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDocument<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateDocument(this, core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        document.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePageReference<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePageReference(this, core::mem::transmute_copy(&advisorypagedimensions)) {
                    Ok(ok__) => {
                        pagereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePage<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: windows_core::PCWSTR, parturi: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePage(this, core::mem::transmute_copy(&pagedimensions), core::mem::transmute(&language), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        page.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePageFromStream<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagemarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, reuseobjects: windows_core::BOOL, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePageFromStream(this, core::mem::transmute_copy(&pagemarkupstream), core::mem::transmute_copy(&parturi), core::mem::transmute_copy(&resources), core::mem::transmute_copy(&reuseobjects)) {
                    Ok(ok__) => {
                        page.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCanvas<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canvas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateCanvas(this) {
                    Ok(ok__) => {
                        canvas.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGlyphs<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut core::ffi::c_void, glyphs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateGlyphs(this, core::mem::transmute_copy(&fontresource)) {
                    Ok(ok__) => {
                        glyphs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePath<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePath(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGeometry<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateGeometry(this) {
                    Ok(ok__) => {
                        geometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGeometryFigure<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateGeometryFigure(this, core::mem::transmute_copy(&startpoint)) {
                    Ok(ok__) => {
                        figure.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateMatrixTransform(this, core::mem::transmute_copy(&matrix)) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateSolidColorBrush(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&colorprofile)) {
                    Ok(ok__) => {
                        solidcolorbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorProfileResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, colorprofileresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateColorProfileResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        colorprofileresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateImageBrush<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateImageBrush(this, core::mem::transmute_copy(&image), core::mem::transmute_copy(&viewbox), core::mem::transmute_copy(&viewport)) {
                    Ok(ok__) => {
                        imagebrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVisualBrush<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateVisualBrush(this, core::mem::transmute_copy(&viewbox), core::mem::transmute_copy(&viewport)) {
                    Ok(ok__) => {
                        visualbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateImageResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateImageResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&contenttype), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        imageresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePrintTicketResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePrintTicketResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        printticketresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acquiredstream: *mut core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut core::ffi::c_void, isobfsourcestream: windows_core::BOOL, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateFontResource(this, core::mem::transmute_copy(&acquiredstream), core::mem::transmute_copy(&fontembedding), core::mem::transmute_copy(&parturi), core::mem::transmute_copy(&isobfsourcestream)) {
                    Ok(ok__) => {
                        fontresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGradientStop<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void, offset: f32, gradientstop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateGradientStop(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&colorprofile), core::mem::transmute_copy(&offset)) {
                    Ok(ok__) => {
                        gradientstop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradstop1: *mut core::ffi::c_void, gradstop2: *mut core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateLinearGradientBrush(this, core::mem::transmute_copy(&gradstop1), core::mem::transmute_copy(&gradstop2), core::mem::transmute_copy(&startpoint), core::mem::transmute_copy(&endpoint)) {
                    Ok(ok__) => {
                        lineargradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradstop1: *mut core::ffi::c_void, gradstop2: *mut core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateRadialGradientBrush(this, core::mem::transmute_copy(&gradstop1), core::mem::transmute_copy(&gradstop2), core::mem::transmute_copy(&centerpoint), core::mem::transmute_copy(&gradientorigin), core::mem::transmute_copy(&radiisizes)) {
                    Ok(ok__) => {
                        radialgradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCoreProperties<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateCoreProperties(this, core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        coreproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDictionary<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateDictionary(this) {
                    Ok(ok__) => {
                        dictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePartUriCollection<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturicollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePartUriCollection(this) {
                    Ok(ok__) => {
                        parturicollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePackageWriterOnFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), core::mem::transmute_copy(&documentsequencepartname), core::mem::transmute_copy(&coreproperties), core::mem::transmute_copy(&packagethumbnail), core::mem::transmute_copy(&documentsequenceprintticket), core::mem::transmute_copy(&discardcontrolpartname)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePackageWriterOnStream(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), core::mem::transmute_copy(&documentsequencepartname), core::mem::transmute_copy(&coreproperties), core::mem::transmute_copy(&packagethumbnail), core::mem::transmute_copy(&documentsequenceprintticket), core::mem::transmute_copy(&discardcontrolpartname)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePartUri<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreatePartUri(this, core::mem::transmute(&uri)) {
                    Ok(ok__) => {
                        parturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Identity: IXpsOMObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory_Impl::CreateReadOnlyStreamOnFile(this, core::mem::transmute(&filename)) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackage: CreatePackage::<Identity, OFFSET>,
            CreatePackageFromFile: CreatePackageFromFile::<Identity, OFFSET>,
            CreatePackageFromStream: CreatePackageFromStream::<Identity, OFFSET>,
            CreateStoryFragmentsResource: CreateStoryFragmentsResource::<Identity, OFFSET>,
            CreateDocumentStructureResource: CreateDocumentStructureResource::<Identity, OFFSET>,
            CreateSignatureBlockResource: CreateSignatureBlockResource::<Identity, OFFSET>,
            CreateRemoteDictionaryResource: CreateRemoteDictionaryResource::<Identity, OFFSET>,
            CreateRemoteDictionaryResourceFromStream: CreateRemoteDictionaryResourceFromStream::<Identity, OFFSET>,
            CreatePartResources: CreatePartResources::<Identity, OFFSET>,
            CreateDocumentSequence: CreateDocumentSequence::<Identity, OFFSET>,
            CreateDocument: CreateDocument::<Identity, OFFSET>,
            CreatePageReference: CreatePageReference::<Identity, OFFSET>,
            CreatePage: CreatePage::<Identity, OFFSET>,
            CreatePageFromStream: CreatePageFromStream::<Identity, OFFSET>,
            CreateCanvas: CreateCanvas::<Identity, OFFSET>,
            CreateGlyphs: CreateGlyphs::<Identity, OFFSET>,
            CreatePath: CreatePath::<Identity, OFFSET>,
            CreateGeometry: CreateGeometry::<Identity, OFFSET>,
            CreateGeometryFigure: CreateGeometryFigure::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, OFFSET>,
            CreateColorProfileResource: CreateColorProfileResource::<Identity, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, OFFSET>,
            CreateVisualBrush: CreateVisualBrush::<Identity, OFFSET>,
            CreateImageResource: CreateImageResource::<Identity, OFFSET>,
            CreatePrintTicketResource: CreatePrintTicketResource::<Identity, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, OFFSET>,
            CreateGradientStop: CreateGradientStop::<Identity, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, OFFSET>,
            CreateCoreProperties: CreateCoreProperties::<Identity, OFFSET>,
            CreateDictionary: CreateDictionary::<Identity, OFFSET>,
            CreatePartUriCollection: CreatePartUriCollection::<Identity, OFFSET>,
            CreatePackageWriterOnFile: CreatePackageWriterOnFile::<Identity, OFFSET>,
            CreatePackageWriterOnStream: CreatePackageWriterOnStream::<Identity, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, OFFSET>,
            CreateReadOnlyStreamOnFile: CreateReadOnlyStreamOnFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMObjectFactory {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentTypeFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentTypeFromStream<P0>(&self, xpsdocumentstream: P0) -> windows_core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentTypeFromStream)(windows_core::Interface::as_raw(self), xpsdocumentstream.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ConvertHDPhotoToJpegXR<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertHDPhotoToJpegXR)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok() }
    }
    pub unsafe fn ConvertJpegXRToHDPhoto<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertJpegXRToHDPhoto)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile1<P0, P5, P6, P7, P8, P9>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: bool, interleaving: XPS_INTERLEAVING, documentsequencepartname: P5, coreproperties: P6, packagethumbnail: P7, documentsequenceprintticket: P8, discardcontrolpartname: P9, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P6: windows_core::Param<IXpsOMCoreProperties>,
        P7: windows_core::Param<IXpsOMImageResource>,
        P8: windows_core::Param<IXpsOMPrintTicketResource>,
        P9: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageWriterOnFile1)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), documenttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream1<P0, P3, P4, P5, P6, P7>(&self, outputstream: P0, optimizemarkupsize: bool, interleaving: XPS_INTERLEAVING, documentsequencepartname: P3, coreproperties: P4, packagethumbnail: P5, documentsequenceprintticket: P6, discardcontrolpartname: P7, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
        P3: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P4: windows_core::Param<IXpsOMCoreProperties>,
        P5: windows_core::Param<IXpsOMImageResource>,
        P6: windows_core::Param<IXpsOMPrintTicketResource>,
        P7: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageWriterOnStream1)(windows_core::Interface::as_raw(self), outputstream.param().abi(), optimizemarkupsize.into(), interleaving, documentsequencepartname.param().abi(), coreproperties.param().abi(), packagethumbnail.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), documenttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePackage1(&self) -> windows_core::Result<IXpsOMPackage1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackage1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageFromStream1<P0>(&self, stream: P0, reuseobjects: bool) -> windows_core::Result<IXpsOMPackage1>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageFromStream1)(windows_core::Interface::as_raw(self), stream.param().abi(), reuseobjects.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePackageFromFile1<P0>(&self, filename: P0, reuseobjects: bool) -> windows_core::Result<IXpsOMPackage1>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageFromFile1)(windows_core::Interface::as_raw(self), filename.param().abi(), reuseobjects.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage1<P1, P2>(&self, pagedimensions: *const XPS_SIZE, language: P1, parturi: P2) -> windows_core::Result<IXpsOMPage1>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePage1)(windows_core::Interface::as_raw(self), pagedimensions, language.param().abi(), parturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream1<P0, P1, P2>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: bool) -> windows_core::Result<IXpsOMPage1>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePageFromStream1)(windows_core::Interface::as_raw(self), pagemarkupstream.param().abi(), parturi.param().abi(), resources.param().abi(), reuseobjects.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<P0, P1, P2>(&self, dictionarymarkupstream: P0, parturi: P1, resources: P2) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P2: windows_core::Param<IXpsOMPartResources>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream1)(windows_core::Interface::as_raw(self), dictionarymarkupstream.param().abi(), parturi.param().abi(), resources.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub CreatePackageWriterOnFile1: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, XPS_DOCUMENT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, XPS_INTERLEAVING, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, XPS_DOCUMENT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream1: usize,
    pub CreatePackage1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageFromStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageFromStream1: usize,
    pub CreatePackageFromFile1: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage1: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_SIZE, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream1: usize,
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory1_Impl: IXpsOMObjectFactory_Impl {
    fn GetDocumentTypeFromFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn GetDocumentTypeFromStream(&self, xpsdocumentstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn ConvertHDPhotoToJpegXR(&self, imageresource: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn ConvertJpegXRToHDPhoto(&self, imageresource: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn CreatePackageWriterOnFile1(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, coreproperties: windows_core::Ref<IXpsOMCoreProperties>, packagethumbnail: windows_core::Ref<IXpsOMImageResource>, documentsequenceprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream1(&self, outputstream: windows_core::Ref<super::super::System::Com::ISequentialStream>, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, coreproperties: windows_core::Ref<IXpsOMCoreProperties>, packagethumbnail: windows_core::Ref<IXpsOMImageResource>, documentsequenceprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackage1(&self) -> windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromStream1(&self, stream: windows_core::Ref<super::super::System::Com::IStream>, reuseobjects: windows_core::BOOL) -> windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromFile1(&self, filename: &windows_core::PCWSTR, reuseobjects: windows_core::BOOL) -> windows_core::Result<IXpsOMPackage1>;
    fn CreatePage1(&self, pagedimensions: *const XPS_SIZE, language: &windows_core::PCWSTR, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPage1>;
    fn CreatePageFromStream1(&self, pagemarkupstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, resources: windows_core::Ref<IXpsOMPartResources>, reuseobjects: windows_core::BOOL) -> windows_core::Result<IXpsOMPage1>;
    fn CreateRemoteDictionaryResourceFromStream1(&self, dictionarymarkupstream: windows_core::Ref<super::super::System::Com::IStream>, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, resources: windows_core::Ref<IXpsOMPartResources>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMObjectFactory1_Vtbl {
    pub const fn new<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentTypeFromFile<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::GetDocumentTypeFromFile(this, core::mem::transmute(&filename)) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpsdocumentstream: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::GetDocumentTypeFromStream(this, core::mem::transmute_copy(&xpsdocumentstream)) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMObjectFactory1_Impl::ConvertHDPhotoToJpegXR(this, core::mem::transmute_copy(&imageresource)).into()
            }
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMObjectFactory1_Impl::ConvertJpegXRToHDPhoto(this, core::mem::transmute_copy(&imageresource)).into()
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePackageWriterOnFile1(
                    this,
                    core::mem::transmute(&filename),
                    core::mem::transmute_copy(&securityattributes),
                    core::mem::transmute_copy(&flagsandattributes),
                    core::mem::transmute_copy(&optimizemarkupsize),
                    core::mem::transmute_copy(&interleaving),
                    core::mem::transmute_copy(&documentsequencepartname),
                    core::mem::transmute_copy(&coreproperties),
                    core::mem::transmute_copy(&packagethumbnail),
                    core::mem::transmute_copy(&documentsequenceprintticket),
                    core::mem::transmute_copy(&discardcontrolpartname),
                    core::mem::transmute_copy(&documenttype),
                ) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: windows_core::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void, packagethumbnail: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePackageWriterOnStream1(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&interleaving), core::mem::transmute_copy(&documentsequencepartname), core::mem::transmute_copy(&coreproperties), core::mem::transmute_copy(&packagethumbnail), core::mem::transmute_copy(&documentsequenceprintticket), core::mem::transmute_copy(&discardcontrolpartname), core::mem::transmute_copy(&documenttype)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackage1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePackage1(this) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageFromStream1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, reuseobjects: windows_core::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePackageFromStream1(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&reuseobjects)) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageFromFile1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, reuseobjects: windows_core::BOOL, package: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePackageFromFile1(this, core::mem::transmute(&filename), core::mem::transmute_copy(&reuseobjects)) {
                    Ok(ok__) => {
                        package.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePage1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: windows_core::PCWSTR, parturi: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePage1(this, core::mem::transmute_copy(&pagedimensions), core::mem::transmute(&language), core::mem::transmute_copy(&parturi)) {
                    Ok(ok__) => {
                        page.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePageFromStream1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagemarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, reuseobjects: windows_core::BOOL, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreatePageFromStream1(this, core::mem::transmute_copy(&pagemarkupstream), core::mem::transmute_copy(&parturi), core::mem::transmute_copy(&resources), core::mem::transmute_copy(&reuseobjects)) {
                    Ok(ok__) => {
                        page.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Identity: IXpsOMObjectFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarymarkupstream: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void, resources: *mut core::ffi::c_void, dictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMObjectFactory1_Impl::CreateRemoteDictionaryResourceFromStream1(this, core::mem::transmute_copy(&dictionarymarkupstream), core::mem::transmute_copy(&parturi), core::mem::transmute_copy(&resources)) {
                    Ok(ok__) => {
                        dictionaryresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMObjectFactory_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentTypeFromFile: GetDocumentTypeFromFile::<Identity, OFFSET>,
            GetDocumentTypeFromStream: GetDocumentTypeFromStream::<Identity, OFFSET>,
            ConvertHDPhotoToJpegXR: ConvertHDPhotoToJpegXR::<Identity, OFFSET>,
            ConvertJpegXRToHDPhoto: ConvertJpegXRToHDPhoto::<Identity, OFFSET>,
            CreatePackageWriterOnFile1: CreatePackageWriterOnFile1::<Identity, OFFSET>,
            CreatePackageWriterOnStream1: CreatePackageWriterOnStream1::<Identity, OFFSET>,
            CreatePackage1: CreatePackage1::<Identity, OFFSET>,
            CreatePackageFromStream1: CreatePackageFromStream1::<Identity, OFFSET>,
            CreatePackageFromFile1: CreatePackageFromFile1::<Identity, OFFSET>,
            CreatePage1: CreatePage1::<Identity, OFFSET>,
            CreatePageFromStream1: CreatePageFromStream1::<Identity, OFFSET>,
            CreateRemoteDictionaryResourceFromStream1: CreateRemoteDictionaryResourceFromStream1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMObjectFactory1 as windows_core::Interface>::IID || iid == &<IXpsOMObjectFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMObjectFactory1 {}
windows_core::imp::define_interface!(IXpsOMPackage, IXpsOMPackage_Vtbl, 0x18c3df65_81e1_4674_91dc_fc452f5a416f);
windows_core::imp::interface_hierarchy!(IXpsOMPackage, windows_core::IUnknown);
impl IXpsOMPackage {
    pub unsafe fn GetDocumentSequence(&self) -> windows_core::Result<IXpsOMDocumentSequence> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentSequence)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentSequence<P0>(&self, documentsequence: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDocumentSequence>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentSequence)(windows_core::Interface::as_raw(self), documentsequence.param().abi()).ok() }
    }
    pub unsafe fn GetCoreProperties(&self) -> windows_core::Result<IXpsOMCoreProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCoreProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetCoreProperties<P0>(&self, coreproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMCoreProperties>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCoreProperties)(windows_core::Interface::as_raw(self), coreproperties.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDiscardControlPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<P0>(&self, discardcontrolparturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDiscardControlPartName)(windows_core::Interface::as_raw(self), discardcontrolparturi.param().abi()).ok() }
    }
    pub unsafe fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnailResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetThumbnailResource)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn WriteToFile<P0>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToFile)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteToStream<P0>(&self, stream: P0, optimizemarkupsize: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToStream)(windows_core::Interface::as_raw(self), stream.param().abi(), optimizemarkupsize.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub WriteToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    WriteToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteToStream: usize,
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage_Impl: windows_core::IUnknownImpl {
    fn GetDocumentSequence(&self) -> windows_core::Result<IXpsOMDocumentSequence>;
    fn SetDocumentSequence(&self, documentsequence: windows_core::Ref<IXpsOMDocumentSequence>) -> windows_core::Result<()>;
    fn GetCoreProperties(&self) -> windows_core::Result<IXpsOMCoreProperties>;
    fn SetCoreProperties(&self, coreproperties: windows_core::Ref<IXpsOMCoreProperties>) -> windows_core::Result<()>;
    fn GetDiscardControlPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetDiscardControlPartName(&self, discardcontrolparturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&self, imageresource: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn WriteToFile(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteToStream(&self, stream: windows_core::Ref<super::super::System::Com::ISequentialStream>, optimizemarkupsize: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage_Vtbl {
    pub const fn new<Identity: IXpsOMPackage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentSequence<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackage_Impl::GetDocumentSequence(this) {
                    Ok(ok__) => {
                        documentsequence.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentSequence<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequence: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage_Impl::SetDocumentSequence(this, core::mem::transmute_copy(&documentsequence)).into()
            }
        }
        unsafe extern "system" fn GetCoreProperties<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackage_Impl::GetCoreProperties(this) {
                    Ok(ok__) => {
                        coreproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCoreProperties<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coreproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage_Impl::SetCoreProperties(this, core::mem::transmute_copy(&coreproperties)).into()
            }
        }
        unsafe extern "system" fn GetDiscardControlPartName<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discardcontrolparturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackage_Impl::GetDiscardControlPartName(this) {
                    Ok(ok__) => {
                        discardcontrolparturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiscardControlPartName<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discardcontrolparturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage_Impl::SetDiscardControlPartName(this, core::mem::transmute_copy(&discardcontrolparturi)).into()
            }
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackage_Impl::GetThumbnailResource(this) {
                    Ok(ok__) => {
                        imageresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage_Impl::SetThumbnailResource(this, core::mem::transmute_copy(&imageresource)).into()
            }
        }
        unsafe extern "system" fn WriteToFile<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage_Impl::WriteToFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize)).into()
            }
        }
        unsafe extern "system" fn WriteToStream<Identity: IXpsOMPackage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage_Impl::WriteToStream(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&optimizemarkupsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentSequence: GetDocumentSequence::<Identity, OFFSET>,
            SetDocumentSequence: SetDocumentSequence::<Identity, OFFSET>,
            GetCoreProperties: GetCoreProperties::<Identity, OFFSET>,
            SetCoreProperties: SetCoreProperties::<Identity, OFFSET>,
            GetDiscardControlPartName: GetDiscardControlPartName::<Identity, OFFSET>,
            SetDiscardControlPartName: SetDiscardControlPartName::<Identity, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, OFFSET>,
            WriteToFile: WriteToFile::<Identity, OFFSET>,
            WriteToStream: WriteToStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackage {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn WriteToFile1<P0>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: bool, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToFile1)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), documenttype).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteToStream1<P0>(&self, outputstream: P0, optimizemarkupsize: bool, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteToStream1)(windows_core::Interface::as_raw(self), outputstream.param().abi(), optimizemarkupsize.into(), documenttype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage1_Vtbl {
    pub base__: IXpsOMPackage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub WriteToFile1: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::BOOL, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    WriteToFile1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteToStream1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteToStream1: usize,
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage1_Impl: IXpsOMPackage_Impl {
    fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn WriteToFile1(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
    fn WriteToStream1(&self, outputstream: windows_core::Ref<super::super::System::Com::ISequentialStream>, optimizemarkupsize: windows_core::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackage1_Vtbl {
    pub const fn new<Identity: IXpsOMPackage1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentType<Identity: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackage1_Impl::GetDocumentType(this) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteToFile1<Identity: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: windows_core::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage1_Impl::WriteToFile1(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
            }
        }
        unsafe extern "system" fn WriteToStream1<Identity: IXpsOMPackage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, optimizemarkupsize: windows_core::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackage1_Impl::WriteToStream1(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
            }
        }
        Self {
            base__: IXpsOMPackage_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, OFFSET>,
            WriteToFile1: WriteToFile1::<Identity, OFFSET>,
            WriteToStream1: WriteToStream1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackage1 as windows_core::Interface>::IID || iid == &<IXpsOMPackage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackage1 {}
windows_core::imp::define_interface!(IXpsOMPackageTarget, IXpsOMPackageTarget_Vtbl, 0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
windows_core::imp::interface_hierarchy!(IXpsOMPackageTarget, windows_core::IUnknown);
impl IXpsOMPackageTarget {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateXpsOMPackageWriter<P0, P1, P2>(&self, documentsequencepartname: P0, documentsequenceprintticket: P1, discardcontrolpartname: P2) -> windows_core::Result<IXpsOMPackageWriter>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<IXpsOMPrintTicketResource>,
        P2: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateXpsOMPackageWriter)(windows_core::Interface::as_raw(self), documentsequencepartname.param().abi(), documentsequenceprintticket.param().abi(), discardcontrolpartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateXpsOMPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateXpsOMPackageWriter: usize,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageTarget_Impl: windows_core::IUnknownImpl {
    fn CreateXpsOMPackageWriter(&self, documentsequencepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, documentsequenceprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, discardcontrolpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMPackageWriter>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageTarget_Vtbl {
    pub const fn new<Identity: IXpsOMPackageTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Identity: IXpsOMPackageTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequencepartname: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void, discardcontrolpartname: *mut core::ffi::c_void, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackageTarget_Impl::CreateXpsOMPackageWriter(this, core::mem::transmute_copy(&documentsequencepartname), core::mem::transmute_copy(&documentsequenceprintticket), core::mem::transmute_copy(&discardcontrolpartname)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateXpsOMPackageWriter: CreateXpsOMPackageWriter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackageTarget {}
windows_core::imp::define_interface!(IXpsOMPackageWriter, IXpsOMPackageWriter_Vtbl, 0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
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
        unsafe { (windows_core::Interface::vtable(self).StartNewDocument)(windows_core::Interface::as_raw(self), documentpartname.param().abi(), documentprintticket.param().abi(), documentstructure.param().abi(), signatureblockresources.param().abi(), restrictedfonts.param().abi()).ok() }
    }
    pub unsafe fn AddPage<P0, P2, P3, P4, P5>(&self, page: P0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: P2, storyfragments: P3, pageprintticket: P4, pagethumbnail: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPage>,
        P2: windows_core::Param<IXpsOMPartUriCollection>,
        P3: windows_core::Param<IXpsOMStoryFragmentsResource>,
        P4: windows_core::Param<IXpsOMPrintTicketResource>,
        P5: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), page.param().abi(), advisorypagedimensions, discardableresourceparts.param().abi(), storyfragments.param().abi(), pageprintticket.param().abi(), pagethumbnail.param().abi()).ok() }
    }
    pub unsafe fn AddResource<P0>(&self, resource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddResource)(windows_core::Interface::as_raw(self), resource.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsClosed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub StartNewDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    StartNewDocument: usize,
    pub AddPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const XPS_SIZE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter_Impl: windows_core::IUnknownImpl {
    fn StartNewDocument(&self, documentpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, documentprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, documentstructure: windows_core::Ref<IXpsOMDocumentStructureResource>, signatureblockresources: windows_core::Ref<IXpsOMSignatureBlockResourceCollection>, restrictedfonts: windows_core::Ref<IXpsOMPartUriCollection>) -> windows_core::Result<()>;
    fn AddPage(&self, page: windows_core::Ref<IXpsOMPage>, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: windows_core::Ref<IXpsOMPartUriCollection>, storyfragments: windows_core::Ref<IXpsOMStoryFragmentsResource>, pageprintticket: windows_core::Ref<IXpsOMPrintTicketResource>, pagethumbnail: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn AddResource(&self, resource: windows_core::Ref<IXpsOMResource>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn IsClosed(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter_Vtbl {
    pub const fn new<Identity: IXpsOMPackageWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartNewDocument<Identity: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentpartname: *mut core::ffi::c_void, documentprintticket: *mut core::ffi::c_void, documentstructure: *mut core::ffi::c_void, signatureblockresources: *mut core::ffi::c_void, restrictedfonts: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackageWriter_Impl::StartNewDocument(this, core::mem::transmute_copy(&documentpartname), core::mem::transmute_copy(&documentprintticket), core::mem::transmute_copy(&documentstructure), core::mem::transmute_copy(&signatureblockresources), core::mem::transmute_copy(&restrictedfonts)).into()
            }
        }
        unsafe extern "system" fn AddPage<Identity: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut core::ffi::c_void, storyfragments: *mut core::ffi::c_void, pageprintticket: *mut core::ffi::c_void, pagethumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackageWriter_Impl::AddPage(this, core::mem::transmute_copy(&page), core::mem::transmute_copy(&advisorypagedimensions), core::mem::transmute_copy(&discardableresourceparts), core::mem::transmute_copy(&storyfragments), core::mem::transmute_copy(&pageprintticket), core::mem::transmute_copy(&pagethumbnail)).into()
            }
        }
        unsafe extern "system" fn AddResource<Identity: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackageWriter_Impl::AddResource(this, core::mem::transmute_copy(&resource)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackageWriter_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn IsClosed<Identity: IXpsOMPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isclosed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPackageWriter_Impl::IsClosed(this) {
                    Ok(ok__) => {
                        isclosed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartNewDocument: StartNewDocument::<Identity, OFFSET>,
            AddPage: AddPage::<Identity, OFFSET>,
            AddResource: AddResource::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            IsClosed: IsClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackageWriter {}
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
        unsafe { (windows_core::Interface::vtable(self).AddModelTexture)(windows_core::Interface::as_raw(self), texturepartname.param().abi(), texturedata.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetModelPrintTicket<P0, P1>(&self, printticketpartname: P0, printticketdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetModelPrintTicket)(windows_core::Interface::as_raw(self), printticketpartname.param().abi(), printticketdata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter3D_Impl: IXpsOMPackageWriter_Impl {
    fn AddModelTexture(&self, texturepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, texturedata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetModelPrintTicket(&self, printticketpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, printticketdata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPackageWriter3D_Vtbl {
    pub const fn new<Identity: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddModelTexture<Identity: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturepartname: *mut core::ffi::c_void, texturedata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackageWriter3D_Impl::AddModelTexture(this, core::mem::transmute_copy(&texturepartname), core::mem::transmute_copy(&texturedata)).into()
            }
        }
        unsafe extern "system" fn SetModelPrintTicket<Identity: IXpsOMPackageWriter3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketpartname: *mut core::ffi::c_void, printticketdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPackageWriter3D_Impl::SetModelPrintTicket(this, core::mem::transmute_copy(&printticketpartname), core::mem::transmute_copy(&printticketdata)).into()
            }
        }
        Self {
            base__: IXpsOMPackageWriter_Vtbl::new::<Identity, OFFSET>(),
            AddModelTexture: AddModelTexture::<Identity, OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPackageWriter3D as windows_core::Interface>::IID || iid == &<IXpsOMPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPackageWriter3D {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisuals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPageDimensions(&self) -> windows_core::Result<XPS_SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageDimensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPageDimensions)(windows_core::Interface::as_raw(self), pagedimensions).ok() }
    }
    pub unsafe fn GetContentBox(&self) -> windows_core::Result<XPS_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentBox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContentBox)(windows_core::Interface::as_raw(self), contentbox).ok() }
    }
    pub unsafe fn GetBleedBox(&self) -> windows_core::Result<XPS_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBleedBox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBleedBox)(windows_core::Interface::as_raw(self), bleedbox).ok() }
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), language.param().abi()).ok() }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok() }
    }
    pub unsafe fn GetIsHyperlinkTarget(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsHyperlinkTarget(&self, ishyperlinktarget: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), ishyperlinktarget.into()).ok() }
    }
    pub unsafe fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionaryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDictionaryLocal)(windows_core::Interface::as_raw(self), resourcedictionary.param().abi()).ok() }
    }
    pub unsafe fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionaryResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDictionaryResource)(windows_core::Interface::as_raw(self), remotedictionaryresource.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<P0>(&self, stream: P0, optimizemarkupsize: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), stream.param().abi(), optimizemarkupsize.into()).ok() }
    }
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateUnusedLookupKey)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMPage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
    pub GenerateUnusedLookupKey: unsafe extern "system" fn(*mut core::ffi::c_void, XPS_OBJECT_TYPE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage_Impl: IXpsOMPart_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPageReference>;
    fn GetVisuals(&self) -> windows_core::Result<IXpsOMVisualCollection>;
    fn GetPageDimensions(&self) -> windows_core::Result<XPS_SIZE>;
    fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()>;
    fn GetContentBox(&self) -> windows_core::Result<XPS_RECT>;
    fn SetContentBox(&self, contentbox: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetBleedBox(&self) -> windows_core::Result<XPS_RECT>;
    fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLanguage(&self, language: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIsHyperlinkTarget(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsHyperlinkTarget(&self, ishyperlinktarget: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(&self, resourcedictionary: windows_core::Ref<IXpsOMDictionary>) -> windows_core::Result<()>;
    fn GetDictionaryResource(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(&self, remotedictionaryresource: windows_core::Ref<IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn Write(&self, stream: windows_core::Ref<super::super::System::Com::ISequentialStream>, optimizemarkupsize: windows_core::BOOL) -> windows_core::Result<()>;
    fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> windows_core::Result<windows_core::PWSTR>;
    fn Clone(&self) -> windows_core::Result<IXpsOMPage>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage_Vtbl {
    pub const fn new<Identity: IXpsOMPage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        pagereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVisuals<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visuals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetVisuals(this) {
                    Ok(ok__) => {
                        visuals.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPageDimensions<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetPageDimensions(this) {
                    Ok(ok__) => {
                        pagedimensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPageDimensions<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetPageDimensions(this, core::mem::transmute_copy(&pagedimensions)).into()
            }
        }
        unsafe extern "system" fn GetContentBox<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentbox: *mut XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetContentBox(this) {
                    Ok(ok__) => {
                        contentbox.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentBox<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentbox: *const XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetContentBox(this, core::mem::transmute_copy(&contentbox)).into()
            }
        }
        unsafe extern "system" fn GetBleedBox<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bleedbox: *mut XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetBleedBox(this) {
                    Ok(ok__) => {
                        bleedbox.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBleedBox<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bleedbox: *const XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetBleedBox(this, core::mem::transmute_copy(&bleedbox)).into()
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        language.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlinktarget: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetIsHyperlinkTarget(this) {
                    Ok(ok__) => {
                        ishyperlinktarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlinktarget: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetIsHyperlinkTarget(this, core::mem::transmute_copy(&ishyperlinktarget)).into()
            }
        }
        unsafe extern "system" fn GetDictionary<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetDictionary(this) {
                    Ok(ok__) => {
                        resourcedictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetDictionaryLocal(this) {
                    Ok(ok__) => {
                        resourcedictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetDictionaryLocal(this, core::mem::transmute_copy(&resourcedictionary)).into()
            }
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GetDictionaryResource(this) {
                    Ok(ok__) => {
                        remotedictionaryresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remotedictionaryresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::SetDictionaryResource(this, core::mem::transmute_copy(&remotedictionaryresource)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage_Impl::Write(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&optimizemarkupsize)).into()
            }
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::GenerateUnusedLookupKey(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage_Impl::Clone(this) {
                    Ok(ok__) => {
                        page.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetVisuals: GetVisuals::<Identity, OFFSET>,
            GetPageDimensions: GetPageDimensions::<Identity, OFFSET>,
            SetPageDimensions: SetPageDimensions::<Identity, OFFSET>,
            GetContentBox: GetContentBox::<Identity, OFFSET>,
            SetContentBox: SetContentBox::<Identity, OFFSET>,
            GetBleedBox: GetBleedBox::<Identity, OFFSET>,
            SetBleedBox: SetBleedBox::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, OFFSET>,
            GetDictionary: GetDictionary::<Identity, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            GenerateUnusedLookupKey: GenerateUnusedLookupKey::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPage as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPage {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<P0>(&self, stream: P0, optimizemarkupsize: bool, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write1)(windows_core::Interface::as_raw(self), stream.param().abi(), optimizemarkupsize.into(), documenttype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage1_Vtbl {
    pub base__: IXpsOMPage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage1_Impl: IXpsOMPage_Impl {
    fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&self, stream: windows_core::Ref<super::super::System::Com::ISequentialStream>, optimizemarkupsize: windows_core::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPage1_Vtbl {
    pub const fn new<Identity: IXpsOMPage1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentType<Identity: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPage1_Impl::GetDocumentType(this) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write1<Identity: IXpsOMPage1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, optimizemarkupsize: windows_core::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPage1_Impl::Write1(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&optimizemarkupsize), core::mem::transmute_copy(&documenttype)).into()
            }
        }
        Self { base__: IXpsOMPage_Vtbl::new::<Identity, OFFSET>(), GetDocumentType: GetDocumentType::<Identity, OFFSET>, Write1: Write1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPage1 as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMPage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPage1 {}
windows_core::imp::define_interface!(IXpsOMPageReference, IXpsOMPageReference_Vtbl, 0xed360180_6f92_4998_890d_2f208531a0a0);
windows_core::imp::interface_hierarchy!(IXpsOMPageReference, windows_core::IUnknown);
impl IXpsOMPageReference {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPage(&self) -> windows_core::Result<IXpsOMPage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPage<P0>(&self, page: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPage)(windows_core::Interface::as_raw(self), page.param().abi()).ok() }
    }
    pub unsafe fn DiscardPage(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DiscardPage)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsPageLoaded(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPageLoaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> windows_core::Result<XPS_SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdvisoryPageDimensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAdvisoryPageDimensions)(windows_core::Interface::as_raw(self), pagedimensions).ok() }
    }
    pub unsafe fn GetStoryFragmentsResource(&self) -> windows_core::Result<IXpsOMStoryFragmentsResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryFragmentsResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetStoryFragmentsResource<P0>(&self, storyfragmentsresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMStoryFragmentsResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStoryFragmentsResource)(windows_core::Interface::as_raw(self), storyfragmentsresource.param().abi()).ok() }
    }
    pub unsafe fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrintTicketResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPrintTicketResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrintTicketResource)(windows_core::Interface::as_raw(self), printticketresource.param().abi()).ok() }
    }
    pub unsafe fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThumbnailResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMImageResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetThumbnailResource)(windows_core::Interface::as_raw(self), imageresource.param().abi()).ok() }
    }
    pub unsafe fn CollectLinkTargets(&self) -> windows_core::Result<IXpsOMNameCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CollectLinkTargets)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CollectPartResources(&self) -> windows_core::Result<IXpsOMPartResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CollectPartResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn HasRestrictedFonts(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasRestrictedFonts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMPageReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscardPage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPageLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
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
    pub HasRestrictedFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMPageReference_Impl: windows_core::IUnknownImpl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument>;
    fn GetPage(&self) -> windows_core::Result<IXpsOMPage>;
    fn SetPage(&self, page: windows_core::Ref<IXpsOMPage>) -> windows_core::Result<()>;
    fn DiscardPage(&self) -> windows_core::Result<()>;
    fn IsPageLoaded(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetAdvisoryPageDimensions(&self) -> windows_core::Result<XPS_SIZE>;
    fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> windows_core::Result<()>;
    fn GetStoryFragmentsResource(&self) -> windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn SetStoryFragmentsResource(&self, storyfragmentsresource: windows_core::Ref<IXpsOMStoryFragmentsResource>) -> windows_core::Result<()>;
    fn GetPrintTicketResource(&self) -> windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(&self, printticketresource: windows_core::Ref<IXpsOMPrintTicketResource>) -> windows_core::Result<()>;
    fn GetThumbnailResource(&self) -> windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(&self, imageresource: windows_core::Ref<IXpsOMImageResource>) -> windows_core::Result<()>;
    fn CollectLinkTargets(&self) -> windows_core::Result<IXpsOMNameCollection>;
    fn CollectPartResources(&self) -> windows_core::Result<IXpsOMPartResources>;
    fn HasRestrictedFonts(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Clone(&self) -> windows_core::Result<IXpsOMPageReference>;
}
impl IXpsOMPageReference_Vtbl {
    pub const fn new<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        document.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPage<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::GetPage(this) {
                    Ok(ok__) => {
                        page.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPage<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReference_Impl::SetPage(this, core::mem::transmute_copy(&page)).into()
            }
        }
        unsafe extern "system" fn DiscardPage<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReference_Impl::DiscardPage(this).into()
            }
        }
        unsafe extern "system" fn IsPageLoaded<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispageloaded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::IsPageLoaded(this) {
                    Ok(ok__) => {
                        ispageloaded.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::GetAdvisoryPageDimensions(this) {
                    Ok(ok__) => {
                        pagedimensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReference_Impl::SetAdvisoryPageDimensions(this, core::mem::transmute_copy(&pagedimensions)).into()
            }
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyfragmentsresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::GetStoryFragmentsResource(this) {
                    Ok(ok__) => {
                        storyfragmentsresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyfragmentsresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReference_Impl::SetStoryFragmentsResource(this, core::mem::transmute_copy(&storyfragmentsresource)).into()
            }
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::GetPrintTicketResource(this) {
                    Ok(ok__) => {
                        printticketresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printticketresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReference_Impl::SetPrintTicketResource(this, core::mem::transmute_copy(&printticketresource)).into()
            }
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::GetThumbnailResource(this) {
                    Ok(ok__) => {
                        imageresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReference_Impl::SetThumbnailResource(this, core::mem::transmute_copy(&imageresource)).into()
            }
        }
        unsafe extern "system" fn CollectLinkTargets<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linktargets: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::CollectLinkTargets(this) {
                    Ok(ok__) => {
                        linktargets.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CollectPartResources<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::CollectPartResources(this) {
                    Ok(ok__) => {
                        partresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasRestrictedFonts<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictedfonts: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::HasRestrictedFonts(this) {
                    Ok(ok__) => {
                        restrictedfonts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMPageReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReference_Impl::Clone(this) {
                    Ok(ok__) => {
                        pagereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetPage: GetPage::<Identity, OFFSET>,
            SetPage: SetPage::<Identity, OFFSET>,
            DiscardPage: DiscardPage::<Identity, OFFSET>,
            IsPageLoaded: IsPageLoaded::<Identity, OFFSET>,
            GetAdvisoryPageDimensions: GetAdvisoryPageDimensions::<Identity, OFFSET>,
            SetAdvisoryPageDimensions: SetAdvisoryPageDimensions::<Identity, OFFSET>,
            GetStoryFragmentsResource: GetStoryFragmentsResource::<Identity, OFFSET>,
            SetStoryFragmentsResource: SetStoryFragmentsResource::<Identity, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, OFFSET>,
            CollectLinkTargets: CollectLinkTargets::<Identity, OFFSET>,
            CollectPartResources: CollectPartResources::<Identity, OFFSET>,
            HasRestrictedFonts: HasRestrictedFonts::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPageReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMPageReference {}
windows_core::imp::define_interface!(IXpsOMPageReferenceCollection, IXpsOMPageReferenceCollection_Vtbl, 0xca16ba4d_e7b9_45c5_958b_f98022473745);
windows_core::imp::interface_hierarchy!(IXpsOMPageReferenceCollection, windows_core::IUnknown);
impl IXpsOMPageReferenceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMPageReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, pagereference: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMPageReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, pagereference.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, pagereference: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMPageReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, pagereference.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, pagereference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMPageReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), pagereference.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMPageReferenceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMPageReference>;
    fn InsertAt(&self, index: u32, pagereference: windows_core::Ref<IXpsOMPageReference>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, pagereference: windows_core::Ref<IXpsOMPageReference>) -> windows_core::Result<()>;
    fn Append(&self, pagereference: windows_core::Ref<IXpsOMPageReference>) -> windows_core::Result<()>;
}
impl IXpsOMPageReferenceCollection_Vtbl {
    pub const fn new<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReferenceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPageReferenceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pagereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReferenceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pagereference)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReferenceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReferenceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pagereference)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMPageReferenceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPageReferenceCollection_Impl::Append(this, core::mem::transmute_copy(&pagereference)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPageReferenceCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMPageReferenceCollection {}
windows_core::imp::define_interface!(IXpsOMPart, IXpsOMPart_Vtbl, 0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
windows_core::imp::interface_hierarchy!(IXpsOMPart, windows_core::IUnknown);
impl IXpsOMPart {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPartName)(windows_core::Interface::as_raw(self), parturi.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPart_Impl: windows_core::IUnknownImpl {
    fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetPartName(&self, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPart_Vtbl {
    pub const fn new<Identity: IXpsOMPart_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPartName<Identity: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPart_Impl::GetPartName(this) {
                    Ok(ok__) => {
                        parturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPartName<Identity: IXpsOMPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPart_Impl::SetPartName(this, core::mem::transmute_copy(&parturi)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartName: GetPartName::<Identity, OFFSET>,
            SetPartName: SetPartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPart {}
windows_core::imp::define_interface!(IXpsOMPartResources, IXpsOMPartResources_Vtbl, 0xf4cf7729_4864_4275_99b3_a8717163ecaf);
windows_core::imp::interface_hierarchy!(IXpsOMPartResources, windows_core::IUnknown);
impl IXpsOMPartResources {
    pub unsafe fn GetFontResources(&self) -> windows_core::Result<IXpsOMFontResourceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetImageResources(&self) -> windows_core::Result<IXpsOMImageResourceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorProfileResources(&self) -> windows_core::Result<IXpsOMColorProfileResourceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorProfileResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRemoteDictionaryResources(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResourceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteDictionaryResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartResources_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImageResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorProfileResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteDictionaryResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMPartResources_Impl: windows_core::IUnknownImpl {
    fn GetFontResources(&self) -> windows_core::Result<IXpsOMFontResourceCollection>;
    fn GetImageResources(&self) -> windows_core::Result<IXpsOMImageResourceCollection>;
    fn GetColorProfileResources(&self) -> windows_core::Result<IXpsOMColorProfileResourceCollection>;
    fn GetRemoteDictionaryResources(&self) -> windows_core::Result<IXpsOMRemoteDictionaryResourceCollection>;
}
impl IXpsOMPartResources_Vtbl {
    pub const fn new<Identity: IXpsOMPartResources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontResources<Identity: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPartResources_Impl::GetFontResources(this) {
                    Ok(ok__) => {
                        fontresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImageResources<Identity: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPartResources_Impl::GetImageResources(this) {
                    Ok(ok__) => {
                        imageresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorProfileResources<Identity: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorprofileresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPartResources_Impl::GetColorProfileResources(this) {
                    Ok(ok__) => {
                        colorprofileresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Identity: IXpsOMPartResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPartResources_Impl::GetRemoteDictionaryResources(this) {
                    Ok(ok__) => {
                        dictionaryresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontResources: GetFontResources::<Identity, OFFSET>,
            GetImageResources: GetImageResources::<Identity, OFFSET>,
            GetColorProfileResources: GetColorProfileResources::<Identity, OFFSET>,
            GetRemoteDictionaryResources: GetRemoteDictionaryResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPartResources as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMPartResources {}
windows_core::imp::define_interface!(IXpsOMPartUriCollection, IXpsOMPartUriCollection_Vtbl, 0x57c650d4_067c_4893_8c33_f62a0633730f);
windows_core::imp::interface_hierarchy!(IXpsOMPartUriCollection, windows_core::IUnknown);
impl IXpsOMPartUriCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn InsertAt<P1>(&self, index: u32, parturi: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, parturi.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetAt<P1>(&self, index: u32, parturi: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, parturi.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn Append<P0>(&self, parturi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), parturi.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPartUriCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn InsertAt(&self, index: u32, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn Append(&self, parturi: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPartUriCollection_Vtbl {
    pub const fn new<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPartUriCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPartUriCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        parturi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPartUriCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&parturi)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPartUriCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPartUriCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&parturi)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMPartUriCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parturi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPartUriCollection_Impl::Append(this, core::mem::transmute_copy(&parturi)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPartUriCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPartUriCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetGeometryLocal<P0>(&self, geometry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometry>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGeometryLocal)(windows_core::Interface::as_raw(self), geometry.param().abi()).ok() }
    }
    pub unsafe fn GetGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometryLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGeometryLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGeometryLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok() }
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessibilityShortDescription)(windows_core::Interface::as_raw(self), shortdescription.param().abi()).ok() }
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessibilityLongDescription)(windows_core::Interface::as_raw(self), longdescription.param().abi()).ok() }
    }
    pub unsafe fn GetSnapsToPixels(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapsToPixels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSnapsToPixels(&self, snapstopixels: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSnapsToPixels)(windows_core::Interface::as_raw(self), snapstopixels.into()).ok() }
    }
    pub unsafe fn GetStrokeBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStrokeBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetStrokeBrushLocal<P0>(&self, brush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeBrushLocal)(windows_core::Interface::as_raw(self), brush.param().abi()).ok() }
    }
    pub unsafe fn GetStrokeBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeBrushLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeBrushLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok() }
    }
    pub unsafe fn GetStrokeDashes(&self) -> windows_core::Result<IXpsOMDashCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeDashes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStrokeDashCap(&self) -> windows_core::Result<XPS_DASH_CAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeDashCap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeDashCap)(windows_core::Interface::as_raw(self), strokedashcap).ok() }
    }
    pub unsafe fn GetStrokeDashOffset(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeDashOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeDashOffset)(windows_core::Interface::as_raw(self), strokedashoffset).ok() }
    }
    pub unsafe fn GetStrokeStartLineCap(&self) -> windows_core::Result<XPS_LINE_CAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeStartLineCap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeStartLineCap)(windows_core::Interface::as_raw(self), strokestartlinecap).ok() }
    }
    pub unsafe fn GetStrokeEndLineCap(&self) -> windows_core::Result<XPS_LINE_CAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeEndLineCap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeEndLineCap)(windows_core::Interface::as_raw(self), strokeendlinecap).ok() }
    }
    pub unsafe fn GetStrokeLineJoin(&self) -> windows_core::Result<XPS_LINE_JOIN> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeLineJoin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeLineJoin)(windows_core::Interface::as_raw(self), strokelinejoin).ok() }
    }
    pub unsafe fn GetStrokeMiterLimit(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeMiterLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeMiterLimit)(windows_core::Interface::as_raw(self), strokemiterlimit).ok() }
    }
    pub unsafe fn GetStrokeThickness(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrokeThickness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeThickness)(windows_core::Interface::as_raw(self), strokethickness).ok() }
    }
    pub unsafe fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, brush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFillBrushLocal)(windows_core::Interface::as_raw(self), brush.param().abi()).ok() }
    }
    pub unsafe fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFillBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFillBrushLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMPath> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetSnapsToPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetSnapsToPixels: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMPath_Impl: IXpsOMVisual_Impl {
    fn GetGeometry(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn GetGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn SetGeometryLocal(&self, geometry: windows_core::Ref<IXpsOMGeometry>) -> windows_core::Result<()>;
    fn GetGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetGeometryLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccessibilityShortDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(&self, shortdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccessibilityLongDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(&self, longdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSnapsToPixels(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetSnapsToPixels(&self, snapstopixels: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetStrokeBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetStrokeBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetStrokeBrushLocal(&self, brush: windows_core::Ref<IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetStrokeBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetStrokeBrushLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStrokeDashes(&self) -> windows_core::Result<IXpsOMDashCollection>;
    fn GetStrokeDashCap(&self) -> windows_core::Result<XPS_DASH_CAP>;
    fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> windows_core::Result<()>;
    fn GetStrokeDashOffset(&self) -> windows_core::Result<f32>;
    fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> windows_core::Result<()>;
    fn GetStrokeStartLineCap(&self) -> windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> windows_core::Result<()>;
    fn GetStrokeEndLineCap(&self) -> windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> windows_core::Result<()>;
    fn GetStrokeLineJoin(&self) -> windows_core::Result<XPS_LINE_JOIN>;
    fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> windows_core::Result<()>;
    fn GetStrokeMiterLimit(&self) -> windows_core::Result<f32>;
    fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> windows_core::Result<()>;
    fn GetStrokeThickness(&self) -> windows_core::Result<f32>;
    fn SetStrokeThickness(&self, strokethickness: f32) -> windows_core::Result<()>;
    fn GetFillBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(&self, brush: windows_core::Ref<IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetFillBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetFillBrushLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMPath>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMPath_Vtbl {
    pub const fn new<Identity: IXpsOMPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGeometry<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetGeometry(this) {
                    Ok(ok__) => {
                        geometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGeometryLocal<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetGeometryLocal(this) {
                    Ok(ok__) => {
                        geometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGeometryLocal<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetGeometryLocal(this, core::mem::transmute_copy(&geometry)).into()
            }
        }
        unsafe extern "system" fn GetGeometryLookup<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetGeometryLookup(this) {
                    Ok(ok__) => {
                        lookup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGeometryLookup<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetGeometryLookup(this, core::mem::transmute(&lookup)).into()
            }
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetAccessibilityShortDescription(this) {
                    Ok(ok__) => {
                        shortdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetAccessibilityShortDescription(this, core::mem::transmute(&shortdescription)).into()
            }
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetAccessibilityLongDescription(this) {
                    Ok(ok__) => {
                        longdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, longdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetAccessibilityLongDescription(this, core::mem::transmute(&longdescription)).into()
            }
        }
        unsafe extern "system" fn GetSnapsToPixels<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapstopixels: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetSnapsToPixels(this) {
                    Ok(ok__) => {
                        snapstopixels.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSnapsToPixels<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapstopixels: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetSnapsToPixels(this, core::mem::transmute_copy(&snapstopixels)).into()
            }
        }
        unsafe extern "system" fn GetStrokeBrush<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeBrush(this) {
                    Ok(ok__) => {
                        brush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeBrushLocal(this) {
                    Ok(ok__) => {
                        brush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeBrushLocal(this, core::mem::transmute_copy(&brush)).into()
            }
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeBrushLookup(this) {
                    Ok(ok__) => {
                        lookup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeBrushLookup(this, core::mem::transmute(&lookup)).into()
            }
        }
        unsafe extern "system" fn GetStrokeDashes<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeDashes(this) {
                    Ok(ok__) => {
                        strokedashes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStrokeDashCap<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeDashCap(this) {
                    Ok(ok__) => {
                        strokedashcap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeDashCap<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeDashCap(this, core::mem::transmute_copy(&strokedashcap)).into()
            }
        }
        unsafe extern "system" fn GetStrokeDashOffset<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashoffset: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeDashOffset(this) {
                    Ok(ok__) => {
                        strokedashoffset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeDashOffset<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokedashoffset: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeDashOffset(this, core::mem::transmute_copy(&strokedashoffset)).into()
            }
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeStartLineCap(this) {
                    Ok(ok__) => {
                        strokestartlinecap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeStartLineCap(this, core::mem::transmute_copy(&strokestartlinecap)).into()
            }
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeEndLineCap(this) {
                    Ok(ok__) => {
                        strokeendlinecap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeEndLineCap(this, core::mem::transmute_copy(&strokeendlinecap)).into()
            }
        }
        unsafe extern "system" fn GetStrokeLineJoin<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeLineJoin(this) {
                    Ok(ok__) => {
                        strokelinejoin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeLineJoin<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeLineJoin(this, core::mem::transmute_copy(&strokelinejoin)).into()
            }
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokemiterlimit: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeMiterLimit(this) {
                    Ok(ok__) => {
                        strokemiterlimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokemiterlimit: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeMiterLimit(this, core::mem::transmute_copy(&strokemiterlimit)).into()
            }
        }
        unsafe extern "system" fn GetStrokeThickness<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokethickness: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetStrokeThickness(this) {
                    Ok(ok__) => {
                        strokethickness.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrokeThickness<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokethickness: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetStrokeThickness(this, core::mem::transmute_copy(&strokethickness)).into()
            }
        }
        unsafe extern "system" fn GetFillBrush<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetFillBrush(this) {
                    Ok(ok__) => {
                        brush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetFillBrushLocal(this) {
                    Ok(ok__) => {
                        brush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetFillBrushLocal(this, core::mem::transmute_copy(&brush)).into()
            }
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::GetFillBrushLookup(this) {
                    Ok(ok__) => {
                        lookup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPath_Impl::SetFillBrushLookup(this, core::mem::transmute(&lookup)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPath_Impl::Clone(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMVisual_Vtbl::new::<Identity, OFFSET>(),
            GetGeometry: GetGeometry::<Identity, OFFSET>,
            GetGeometryLocal: GetGeometryLocal::<Identity, OFFSET>,
            SetGeometryLocal: SetGeometryLocal::<Identity, OFFSET>,
            GetGeometryLookup: GetGeometryLookup::<Identity, OFFSET>,
            SetGeometryLookup: SetGeometryLookup::<Identity, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, OFFSET>,
            GetSnapsToPixels: GetSnapsToPixels::<Identity, OFFSET>,
            SetSnapsToPixels: SetSnapsToPixels::<Identity, OFFSET>,
            GetStrokeBrush: GetStrokeBrush::<Identity, OFFSET>,
            GetStrokeBrushLocal: GetStrokeBrushLocal::<Identity, OFFSET>,
            SetStrokeBrushLocal: SetStrokeBrushLocal::<Identity, OFFSET>,
            GetStrokeBrushLookup: GetStrokeBrushLookup::<Identity, OFFSET>,
            SetStrokeBrushLookup: SetStrokeBrushLookup::<Identity, OFFSET>,
            GetStrokeDashes: GetStrokeDashes::<Identity, OFFSET>,
            GetStrokeDashCap: GetStrokeDashCap::<Identity, OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Identity, OFFSET>,
            GetStrokeDashOffset: GetStrokeDashOffset::<Identity, OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Identity, OFFSET>,
            GetStrokeStartLineCap: GetStrokeStartLineCap::<Identity, OFFSET>,
            SetStrokeStartLineCap: SetStrokeStartLineCap::<Identity, OFFSET>,
            GetStrokeEndLineCap: GetStrokeEndLineCap::<Identity, OFFSET>,
            SetStrokeEndLineCap: SetStrokeEndLineCap::<Identity, OFFSET>,
            GetStrokeLineJoin: GetStrokeLineJoin::<Identity, OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Identity, OFFSET>,
            GetStrokeMiterLimit: GetStrokeMiterLimit::<Identity, OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Identity, OFFSET>,
            GetStrokeThickness: GetStrokeThickness::<Identity, OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Identity, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPath as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMVisual as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMPath {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPrintTicketResource_Impl: IXpsOMResource_Impl {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMPrintTicketResource_Vtbl {
    pub const fn new<Identity: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStream<Identity: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMPrintTicketResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMPrintTicketResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMPrintTicketResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&partname)).into()
            }
        }
        Self { base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(), GetStream: GetStream::<Identity, OFFSET>, SetContent: SetContent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMPrintTicketResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMPrintTicketResource {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCenter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenter)(windows_core::Interface::as_raw(self), center).ok() }
    }
    pub unsafe fn GetRadiiSizes(&self) -> windows_core::Result<XPS_SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRadiiSizes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRadiiSizes)(windows_core::Interface::as_raw(self), radiisizes).ok() }
    }
    pub unsafe fn GetGradientOrigin(&self) -> windows_core::Result<XPS_POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGradientOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGradientOrigin)(windows_core::Interface::as_raw(self), origin).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMRadialGradientBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IXpsOMRadialGradientBrush_Impl: IXpsOMGradientBrush_Impl {
    fn GetCenter(&self) -> windows_core::Result<XPS_POINT>;
    fn SetCenter(&self, center: *const XPS_POINT) -> windows_core::Result<()>;
    fn GetRadiiSizes(&self) -> windows_core::Result<XPS_SIZE>;
    fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> windows_core::Result<()>;
    fn GetGradientOrigin(&self) -> windows_core::Result<XPS_POINT>;
    fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMRadialGradientBrush>;
}
impl IXpsOMRadialGradientBrush_Vtbl {
    pub const fn new<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCenter<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: *mut XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRadialGradientBrush_Impl::GetCenter(this) {
                    Ok(ok__) => {
                        center.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCenter<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: *const XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRadialGradientBrush_Impl::SetCenter(this, core::mem::transmute_copy(&center)).into()
            }
        }
        unsafe extern "system" fn GetRadiiSizes<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRadialGradientBrush_Impl::GetRadiiSizes(this) {
                    Ok(ok__) => {
                        radiisizes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRadiiSizes<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiisizes: *const XPS_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRadialGradientBrush_Impl::SetRadiiSizes(this, core::mem::transmute_copy(&radiisizes)).into()
            }
        }
        unsafe extern "system" fn GetGradientOrigin<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *mut XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRadialGradientBrush_Impl::GetGradientOrigin(this) {
                    Ok(ok__) => {
                        origin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGradientOrigin<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: *const XPS_POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRadialGradientBrush_Impl::SetGradientOrigin(this, core::mem::transmute_copy(&origin)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMRadialGradientBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRadialGradientBrush_Impl::Clone(this) {
                    Ok(ok__) => {
                        radialgradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMGradientBrush_Vtbl::new::<Identity, OFFSET>(),
            GetCenter: GetCenter::<Identity, OFFSET>,
            SetCenter: SetCenter::<Identity, OFFSET>,
            GetRadiiSizes: GetRadiiSizes::<Identity, OFFSET>,
            SetRadiiSizes: SetRadiiSizes::<Identity, OFFSET>,
            GetGradientOrigin: GetGradientOrigin::<Identity, OFFSET>,
            SetGradientOrigin: SetGradientOrigin::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRadialGradientBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMGradientBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMRadialGradientBrush {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDictionary<P0>(&self, dictionary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMDictionary>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDictionary)(windows_core::Interface::as_raw(self), dictionary.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource_Impl: IXpsOMResource_Impl {
    fn GetDictionary(&self) -> windows_core::Result<IXpsOMDictionary>;
    fn SetDictionary(&self, dictionary: windows_core::Ref<IXpsOMDictionary>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource_Vtbl {
    pub const fn new<Identity: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDictionary<Identity: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRemoteDictionaryResource_Impl::GetDictionary(this) {
                    Ok(ok__) => {
                        dictionary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDictionary<Identity: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionary: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRemoteDictionaryResource_Impl::SetDictionary(this, core::mem::transmute_copy(&dictionary)).into()
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetDictionary: GetDictionary::<Identity, OFFSET>,
            SetDictionary: SetDictionary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMRemoteDictionaryResource {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<P0>(&self, stream: P0, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write1)(windows_core::Interface::as_raw(self), stream.param().abi(), documenttype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource1_Vtbl {
    pub base__: IXpsOMRemoteDictionaryResource_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_DOCUMENT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1_Impl: IXpsOMRemoteDictionaryResource_Impl {
    fn GetDocumentType(&self) -> windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(&self, stream: windows_core::Ref<super::super::System::Com::ISequentialStream>, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResource1_Vtbl {
    pub const fn new<Identity: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentType<Identity: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRemoteDictionaryResource1_Impl::GetDocumentType(this) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write1<Identity: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRemoteDictionaryResource1_Impl::Write1(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&documenttype)).into()
            }
        }
        Self {
            base__: IXpsOMRemoteDictionaryResource_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentType: GetDocumentType::<Identity, OFFSET>,
            Write1: Write1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResource1 as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID || iid == &<IXpsOMRemoteDictionaryResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMRemoteDictionaryResource1 {}
windows_core::imp::define_interface!(IXpsOMRemoteDictionaryResourceCollection, IXpsOMRemoteDictionaryResourceCollection_Vtbl, 0x5c38db61_7fec_464a_87bd_41e3bef018be);
windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResourceCollection, windows_core::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMRemoteDictionaryResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMRemoteDictionaryResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn InsertAt(&self, index: u32, object: windows_core::Ref<IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: windows_core::Ref<IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn Append(&self, object: windows_core::Ref<IXpsOMRemoteDictionaryResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub const fn new<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRemoteDictionaryResourceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRemoteDictionaryResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        object.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRemoteDictionaryResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRemoteDictionaryResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRemoteDictionaryResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMRemoteDictionaryResourceCollection_Impl::Append(this, core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn GetByPartName<Identity: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, remotedictionaryresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMRemoteDictionaryResourceCollection_Impl::GetByPartName(this, core::mem::transmute_copy(&partname)) {
                    Ok(ok__) => {
                        remotedictionaryresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMRemoteDictionaryResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMRemoteDictionaryResourceCollection {}
windows_core::imp::define_interface!(IXpsOMResource, IXpsOMResource_Vtbl, 0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
impl core::ops::Deref for IXpsOMResource {
    type Target = IXpsOMPart;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXpsOMResource, windows_core::IUnknown, IXpsOMPart);
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMResource_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMResource_Impl: IXpsOMPart_Impl {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMResource_Vtbl {
    pub const fn new<Identity: IXpsOMResource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IXpsOMPart_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMResource {}
windows_core::imp::define_interface!(IXpsOMShareable, IXpsOMShareable_Vtbl, 0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
windows_core::imp::interface_hierarchy!(IXpsOMShareable, windows_core::IUnknown);
impl IXpsOMShareable {
    pub unsafe fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<XPS_OBJECT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMShareable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_OBJECT_TYPE) -> windows_core::HRESULT,
}
pub trait IXpsOMShareable_Impl: windows_core::IUnknownImpl {
    fn GetOwner(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetType(&self) -> windows_core::Result<XPS_OBJECT_TYPE>;
}
impl IXpsOMShareable_Vtbl {
    pub const fn new<Identity: IXpsOMShareable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMShareable_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IXpsOMShareable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMShareable_Impl::GetType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOwner: GetOwner::<Identity, OFFSET>, GetType: GetType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMShareable {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResource_Impl: IXpsOMResource_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMDocument>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResource_Vtbl {
    pub const fn new<Identity: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSignatureBlockResource_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSignatureBlockResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMSignatureBlockResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMSignatureBlockResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&partname)).into()
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMSignatureBlockResource {}
windows_core::imp::define_interface!(IXpsOMSignatureBlockResourceCollection, IXpsOMSignatureBlockResourceCollection_Vtbl, 0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
windows_core::imp::interface_hierarchy!(IXpsOMSignatureBlockResourceCollection, windows_core::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMSignatureBlockResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, signatureblockresource: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMSignatureBlockResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, signatureblockresource.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, signatureblockresource: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMSignatureBlockResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, signatureblockresource.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, signatureblockresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMSignatureBlockResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), signatureblockresource.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByPartName)(windows_core::Interface::as_raw(self), partname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMSignatureBlockResource>;
    fn InsertAt(&self, index: u32, signatureblockresource: windows_core::Ref<IXpsOMSignatureBlockResource>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, signatureblockresource: windows_core::Ref<IXpsOMSignatureBlockResource>) -> windows_core::Result<()>;
    fn Append(&self, signatureblockresource: windows_core::Ref<IXpsOMSignatureBlockResource>) -> windows_core::Result<()>;
    fn GetByPartName(&self, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMSignatureBlockResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub const fn new<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSignatureBlockResourceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSignatureBlockResourceCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        signatureblockresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMSignatureBlockResourceCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&signatureblockresource)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMSignatureBlockResourceCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMSignatureBlockResourceCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&signatureblockresource)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblockresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMSignatureBlockResourceCollection_Impl::Append(this, core::mem::transmute_copy(&signatureblockresource)).into()
            }
        }
        unsafe extern "system" fn GetByPartName<Identity: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, signatureblockresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSignatureBlockResourceCollection_Impl::GetByPartName(this, core::mem::transmute_copy(&partname)) {
                    Ok(ok__) => {
                        signatureblockresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            GetByPartName: GetByPartName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMSignatureBlockResourceCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMSignatureBlockResourceCollection {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), color as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetColor<P1>(&self, color: *const XPS_COLOR, colorprofile: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMColorProfileResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color, colorprofile.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMSolidColorBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSolidColorBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XPS_COLOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const XPS_COLOR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMSolidColorBrush_Impl: IXpsOMBrush_Impl {
    fn GetColor(&self, color: *mut XPS_COLOR) -> windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColor(&self, color: *const XPS_COLOR, colorprofile: windows_core::Ref<IXpsOMColorProfileResource>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMSolidColorBrush>;
}
impl IXpsOMSolidColorBrush_Vtbl {
    pub const fn new<Identity: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColor<Identity: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSolidColorBrush_Impl::GetColor(this, core::mem::transmute_copy(&color)) {
                    Ok(ok__) => {
                        colorprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColor<Identity: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMSolidColorBrush_Impl::SetColor(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&colorprofile)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMSolidColorBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMSolidColorBrush_Impl::Clone(this) {
                    Ok(ok__) => {
                        solidcolorbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, OFFSET>(),
            GetColor: GetColor::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMSolidColorBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMSolidColorBrush {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), sourcestream.param().abi(), partname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMStoryFragmentsResource_Impl: IXpsOMResource_Impl {
    fn GetOwner(&self) -> windows_core::Result<IXpsOMPageReference>;
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(&self, sourcestream: windows_core::Ref<super::super::System::Com::IStream>, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMStoryFragmentsResource_Vtbl {
    pub const fn new<Identity: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwner<Identity: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMStoryFragmentsResource_Impl::GetOwner(this) {
                    Ok(ok__) => {
                        owner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMStoryFragmentsResource_Impl::GetStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContent<Identity: IXpsOMStoryFragmentsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestream: *mut core::ffi::c_void, partname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMStoryFragmentsResource_Impl::SetContent(this, core::mem::transmute_copy(&sourcestream), core::mem::transmute_copy(&partname)).into()
            }
        }
        Self {
            base__: IXpsOMResource_Vtbl::new::<Identity, OFFSET>(),
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMStoryFragmentsResource as windows_core::Interface>::IID || iid == &<IXpsOMPart as windows_core::Interface>::IID || iid == &<IXpsOMResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMStoryFragmentsResource {}
windows_core::imp::define_interface!(IXpsOMThumbnailGenerator, IXpsOMThumbnailGenerator_Vtbl, 0x15b873d5_1971_41e8_83a3_6578403064c7);
windows_core::imp::interface_hierarchy!(IXpsOMThumbnailGenerator, windows_core::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateThumbnail<P0, P3>(&self, page: P0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: P3) -> windows_core::Result<IXpsOMImageResource>
    where
        P0: windows_core::Param<IXpsOMPage>,
        P3: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateThumbnail)(windows_core::Interface::as_raw(self), page.param().abi(), thumbnailtype, thumbnailsize, imageresourcepartname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMThumbnailGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GenerateThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XPS_IMAGE_TYPE, XPS_THUMBNAIL_SIZE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GenerateThumbnail: usize,
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMThumbnailGenerator_Impl: windows_core::IUnknownImpl {
    fn GenerateThumbnail(&self, page: windows_core::Ref<IXpsOMPage>, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsOMThumbnailGenerator_Vtbl {
    pub const fn new<Identity: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GenerateThumbnail<Identity: IXpsOMThumbnailGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, page: *mut core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut core::ffi::c_void, imageresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMThumbnailGenerator_Impl::GenerateThumbnail(this, core::mem::transmute_copy(&page), core::mem::transmute_copy(&thumbnailtype), core::mem::transmute_copy(&thumbnailsize), core::mem::transmute_copy(&imageresourcepartname)) {
                    Ok(ok__) => {
                        imageresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GenerateThumbnail: GenerateThumbnail::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMThumbnailGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsOMThumbnailGenerator {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), transform.param().abi()).ok() }
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok() }
    }
    pub unsafe fn GetViewbox(&self) -> windows_core::Result<XPS_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewbox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetViewbox)(windows_core::Interface::as_raw(self), viewbox).ok() }
    }
    pub unsafe fn GetViewport(&self) -> windows_core::Result<XPS_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetViewport)(windows_core::Interface::as_raw(self), viewport).ok() }
    }
    pub unsafe fn GetTileMode(&self) -> windows_core::Result<XPS_TILE_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTileMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTileMode)(windows_core::Interface::as_raw(self), tilemode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IXpsOMTileBrush_Impl: IXpsOMBrush_Impl {
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, transform: windows_core::Ref<IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetViewbox(&self) -> windows_core::Result<XPS_RECT>;
    fn SetViewbox(&self, viewbox: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetViewport(&self) -> windows_core::Result<XPS_RECT>;
    fn SetViewport(&self, viewport: *const XPS_RECT) -> windows_core::Result<()>;
    fn GetTileMode(&self) -> windows_core::Result<XPS_TILE_MODE>;
    fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> windows_core::Result<()>;
}
impl IXpsOMTileBrush_Vtbl {
    pub const fn new<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransform<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMTileBrush_Impl::GetTransform(this) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMTileBrush_Impl::GetTransformLocal(this) {
                    Ok(ok__) => {
                        transform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMTileBrush_Impl::SetTransformLocal(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetTransformLookup<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMTileBrush_Impl::GetTransformLookup(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMTileBrush_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn GetViewbox<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *mut XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMTileBrush_Impl::GetViewbox(this) {
                    Ok(ok__) => {
                        viewbox.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetViewbox<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewbox: *const XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMTileBrush_Impl::SetViewbox(this, core::mem::transmute_copy(&viewbox)).into()
            }
        }
        unsafe extern "system" fn GetViewport<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMTileBrush_Impl::GetViewport(this) {
                    Ok(ok__) => {
                        viewport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetViewport<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *const XPS_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMTileBrush_Impl::SetViewport(this, core::mem::transmute_copy(&viewport)).into()
            }
        }
        unsafe extern "system" fn GetTileMode<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMTileBrush_Impl::GetTileMode(this) {
                    Ok(ok__) => {
                        tilemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTileMode<Identity: IXpsOMTileBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tilemode: XPS_TILE_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMTileBrush_Impl::SetTileMode(this, core::mem::transmute_copy(&tilemode)).into()
            }
        }
        Self {
            base__: IXpsOMBrush_Vtbl::new::<Identity, OFFSET>(),
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            GetViewbox: GetViewbox::<Identity, OFFSET>,
            SetViewbox: SetViewbox::<Identity, OFFSET>,
            GetViewport: GetViewport::<Identity, OFFSET>,
            SetViewport: SetViewport::<Identity, OFFSET>,
            GetTileMode: GetTileMode::<Identity, OFFSET>,
            SetTileMode: SetTileMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMTileBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMTileBrush {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMMatrixTransform>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLocal)(windows_core::Interface::as_raw(self), matrixtransform.param().abi()).ok() }
    }
    pub unsafe fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransformLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok() }
    }
    pub unsafe fn GetClipGeometry(&self) -> windows_core::Result<IXpsOMGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipGeometryLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMGeometry>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipGeometryLocal)(windows_core::Interface::as_raw(self), clipgeometry.param().abi()).ok() }
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipGeometryLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipGeometryLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok() }
    }
    pub unsafe fn GetOpacity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity).ok() }
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOpacityMaskBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> windows_core::Result<IXpsOMBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOpacityMaskBrushLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMBrush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOpacityMaskBrushLocal)(windows_core::Interface::as_raw(self), opacitymaskbrush.param().abi()).ok() }
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOpacityMaskBrushLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOpacityMaskBrushLookup)(windows_core::Interface::as_raw(self), key.param().abi()).ok() }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok() }
    }
    pub unsafe fn GetIsHyperlinkTarget(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsHyperlinkTarget(&self, ishyperlink: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsHyperlinkTarget)(windows_core::Interface::as_raw(self), ishyperlink.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> windows_core::Result<super::super::System::Com::IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHyperlinkNavigateUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHyperlinkNavigateUri)(windows_core::Interface::as_raw(self), hyperlinkuri.param().abi()).ok() }
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), language.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsOMVisual_Impl: IXpsOMShareable_Impl {
    fn GetTransform(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(&self) -> windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(&self, matrixtransform: windows_core::Ref<IXpsOMMatrixTransform>) -> windows_core::Result<()>;
    fn GetTransformLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTransformLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetClipGeometry(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn GetClipGeometryLocal(&self) -> windows_core::Result<IXpsOMGeometry>;
    fn SetClipGeometryLocal(&self, clipgeometry: windows_core::Ref<IXpsOMGeometry>) -> windows_core::Result<()>;
    fn GetClipGeometryLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetClipGeometryLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOpacity(&self) -> windows_core::Result<f32>;
    fn SetOpacity(&self, opacity: f32) -> windows_core::Result<()>;
    fn GetOpacityMaskBrush(&self) -> windows_core::Result<IXpsOMBrush>;
    fn GetOpacityMaskBrushLocal(&self) -> windows_core::Result<IXpsOMBrush>;
    fn SetOpacityMaskBrushLocal(&self, opacitymaskbrush: windows_core::Ref<IXpsOMBrush>) -> windows_core::Result<()>;
    fn GetOpacityMaskBrushLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetOpacityMaskBrushLookup(&self, key: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIsHyperlinkTarget(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsHyperlinkTarget(&self, ishyperlink: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetHyperlinkNavigateUri(&self) -> windows_core::Result<super::super::System::Com::IUri>;
    fn SetHyperlinkNavigateUri(&self, hyperlinkuri: windows_core::Ref<super::super::System::Com::IUri>) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLanguage(&self, language: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXpsOMVisual_Vtbl {
    pub const fn new<Identity: IXpsOMVisual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransform<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetTransform(this) {
                    Ok(ok__) => {
                        matrixtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformLocal<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetTransformLocal(this) {
                    Ok(ok__) => {
                        matrixtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLocal<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetTransformLocal(this, core::mem::transmute_copy(&matrixtransform)).into()
            }
        }
        unsafe extern "system" fn GetTransformLookup<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetTransformLookup(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransformLookup<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetTransformLookup(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn GetClipGeometry<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetClipGeometry(this) {
                    Ok(ok__) => {
                        clipgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClipGeometryLocal<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetClipGeometryLocal(this) {
                    Ok(ok__) => {
                        clipgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipGeometryLocal<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipgeometry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetClipGeometryLocal(this, core::mem::transmute_copy(&clipgeometry)).into()
            }
        }
        unsafe extern "system" fn GetClipGeometryLookup<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetClipGeometryLookup(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipGeometryLookup<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetClipGeometryLookup(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn GetOpacity<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetOpacity(this) {
                    Ok(ok__) => {
                        opacity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOpacity<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity)).into()
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetOpacityMaskBrush(this) {
                    Ok(ok__) => {
                        opacitymaskbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetOpacityMaskBrushLocal(this) {
                    Ok(ok__) => {
                        opacitymaskbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymaskbrush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetOpacityMaskBrushLocal(this, core::mem::transmute_copy(&opacitymaskbrush)).into()
            }
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetOpacityMaskBrushLookup(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetOpacityMaskBrushLookup(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlink: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetIsHyperlinkTarget(this) {
                    Ok(ok__) => {
                        ishyperlink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishyperlink: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetIsHyperlinkTarget(this, core::mem::transmute_copy(&ishyperlink)).into()
            }
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hyperlinkuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetHyperlinkNavigateUri(this) {
                    Ok(ok__) => {
                        hyperlinkuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hyperlinkuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetHyperlinkNavigateUri(this, core::mem::transmute_copy(&hyperlinkuri)).into()
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisual_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        language.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguage<Identity: IXpsOMVisual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisual_Impl::SetLanguage(this, core::mem::transmute(&language)).into()
            }
        }
        Self {
            base__: IXpsOMShareable_Vtbl::new::<Identity, OFFSET>(),
            GetTransform: GetTransform::<Identity, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, OFFSET>,
            GetClipGeometry: GetClipGeometry::<Identity, OFFSET>,
            GetClipGeometryLocal: GetClipGeometryLocal::<Identity, OFFSET>,
            SetClipGeometryLocal: SetClipGeometryLocal::<Identity, OFFSET>,
            GetClipGeometryLookup: GetClipGeometryLookup::<Identity, OFFSET>,
            SetClipGeometryLookup: SetClipGeometryLookup::<Identity, OFFSET>,
            GetOpacity: GetOpacity::<Identity, OFFSET>,
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            GetOpacityMaskBrush: GetOpacityMaskBrush::<Identity, OFFSET>,
            GetOpacityMaskBrushLocal: GetOpacityMaskBrushLocal::<Identity, OFFSET>,
            SetOpacityMaskBrushLocal: SetOpacityMaskBrushLocal::<Identity, OFFSET>,
            GetOpacityMaskBrushLookup: GetOpacityMaskBrushLookup::<Identity, OFFSET>,
            SetOpacityMaskBrushLookup: SetOpacityMaskBrushLookup::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, OFFSET>,
            GetHyperlinkNavigateUri: GetHyperlinkNavigateUri::<Identity, OFFSET>,
            SetHyperlinkNavigateUri: SetHyperlinkNavigateUri::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            SetLanguage: SetLanguage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMVisual as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXpsOMVisual {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisual)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVisualLocal(&self) -> windows_core::Result<IXpsOMVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisualLocal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVisualLocal<P0>(&self, visual: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVisualLocal)(windows_core::Interface::as_raw(self), visual.param().abi()).ok() }
    }
    pub unsafe fn GetVisualLookup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisualLookup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVisualLookup<P0>(&self, lookup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVisualLookup)(windows_core::Interface::as_raw(self), lookup.param().abi()).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IXpsOMVisualBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisualLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVisualLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisualLookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetVisualLookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMVisualBrush_Impl: IXpsOMTileBrush_Impl {
    fn GetVisual(&self) -> windows_core::Result<IXpsOMVisual>;
    fn GetVisualLocal(&self) -> windows_core::Result<IXpsOMVisual>;
    fn SetVisualLocal(&self, visual: windows_core::Ref<IXpsOMVisual>) -> windows_core::Result<()>;
    fn GetVisualLookup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetVisualLookup(&self, lookup: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IXpsOMVisualBrush>;
}
impl IXpsOMVisualBrush_Vtbl {
    pub const fn new<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVisual<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisualBrush_Impl::GetVisual(this) {
                    Ok(ok__) => {
                        visual.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVisualLocal<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisualBrush_Impl::GetVisualLocal(this) {
                    Ok(ok__) => {
                        visual.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisualLocal<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisualBrush_Impl::SetVisualLocal(this, core::mem::transmute_copy(&visual)).into()
            }
        }
        unsafe extern "system" fn GetVisualLookup<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisualBrush_Impl::GetVisualLookup(this) {
                    Ok(ok__) => {
                        lookup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisualLookup<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookup: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisualBrush_Impl::SetVisualLookup(this, core::mem::transmute(&lookup)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IXpsOMVisualBrush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visualbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisualBrush_Impl::Clone(this) {
                    Ok(ok__) => {
                        visualbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXpsOMTileBrush_Vtbl::new::<Identity, OFFSET>(),
            GetVisual: GetVisual::<Identity, OFFSET>,
            GetVisualLocal: GetVisualLocal::<Identity, OFFSET>,
            SetVisualLocal: SetVisualLocal::<Identity, OFFSET>,
            GetVisualLookup: GetVisualLookup::<Identity, OFFSET>,
            SetVisualLookup: SetVisualLookup::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMVisualBrush as windows_core::Interface>::IID || iid == &<IXpsOMShareable as windows_core::Interface>::IID || iid == &<IXpsOMBrush as windows_core::Interface>::IID || iid == &<IXpsOMTileBrush as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMVisualBrush {}
windows_core::imp::define_interface!(IXpsOMVisualCollection, IXpsOMVisualCollection_Vtbl, 0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
windows_core::imp::interface_hierarchy!(IXpsOMVisualCollection, windows_core::IUnknown);
impl IXpsOMVisualCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMVisual> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn SetAt<P1>(&self, index: u32, object: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IXpsOMVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, object.param().abi()).ok() }
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXpsOMVisual>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), object.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXpsOMVisualCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsOMVisual>;
    fn InsertAt(&self, index: u32, object: windows_core::Ref<IXpsOMVisual>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn SetAt(&self, index: u32, object: windows_core::Ref<IXpsOMVisual>) -> windows_core::Result<()>;
    fn Append(&self, object: windows_core::Ref<IXpsOMVisual>) -> windows_core::Result<()>;
}
impl IXpsOMVisualCollection_Vtbl {
    pub const fn new<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisualCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsOMVisualCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        object.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisualCollection_Impl::InsertAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisualCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn SetAt<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisualCollection_Impl::SetAt(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IXpsOMVisualCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsOMVisualCollection_Impl::Append(this, core::mem::transmute_copy(&object)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            SetAt: SetAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsOMVisualCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsOMVisualCollection {}
windows_core::imp::define_interface!(IXpsSignature, IXpsSignature_Vtbl, 0x6ae4c93e_1ade_42fb_898b_3a5658284857);
windows_core::imp::interface_hierarchy!(IXpsSignature, windows_core::IUnknown);
impl IXpsSignature {
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSignatureValue)(windows_core::Interface::as_raw(self), signaturehashvalue as _, count as _).ok() }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<XPS_SIGNATURE_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Verify)(windows_core::Interface::as_raw(self), x509certificate, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomObjectEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomReferenceEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSignatureXml)(windows_core::Interface::as_raw(self), signaturexml as _, count as _).ok() }
    }
    pub unsafe fn SetSignatureXml(&self, signaturexml: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureXml)(windows_core::Interface::as_raw(self), core::mem::transmute(signaturexml.as_ptr()), signaturexml.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignature_Impl: windows_core::IUnknownImpl {
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn GetCertificateEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator>;
    fn GetSigningTime(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<XPS_SIGNATURE_STATUS>;
    fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY>;
    fn GetCustomObjectEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>;
    fn GetCustomReferenceEnumerator(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>;
    fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn SetSignatureXml(&self, signaturexml: *const u8, count: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignature_Vtbl {
    pub const fn new<Identity: IXpsSignature_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSignatureId<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sigid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetSignatureId(this) {
                    Ok(ok__) => {
                        sigid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureValue<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignature_Impl::GetSignatureValue(this, core::mem::transmute_copy(&signaturehashvalue), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetCertificateEnumerator(this) {
                    Ok(ok__) => {
                        certificateenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSigningTime<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sigdatetimestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetSigningTime(this) {
                    Ok(ok__) => {
                        sigdatetimestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetSigningTimeFormat(this) {
                    Ok(ok__) => {
                        timeformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetSignaturePartName(this) {
                    Ok(ok__) => {
                        signaturepartname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Verify<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::Verify(this, core::mem::transmute_copy(&x509certificate)) {
                    Ok(ok__) => {
                        sigstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetPolicy(this) {
                    Ok(ok__) => {
                        policy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetCustomObjectEnumerator(this) {
                    Ok(ok__) => {
                        customobjectenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignature_Impl::GetCustomReferenceEnumerator(this) {
                    Ok(ok__) => {
                        customreferenceenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureXml<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignature_Impl::GetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetSignatureXml<Identity: IXpsSignature_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturexml: *const u8, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignature_Impl::SetSignatureXml(this, core::mem::transmute_copy(&signaturexml), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            Verify: Verify::<Identity, OFFSET>,
            GetPolicy: GetPolicy::<Identity, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, OFFSET>,
            SetSignatureXml: SetSignatureXml::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignature as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignature {}
windows_core::imp::define_interface!(IXpsSignatureBlock, IXpsSignatureBlock_Vtbl, 0x151fac09_0b97_4ac6_a323_5e4297d4322b);
windows_core::imp::interface_hierarchy!(IXpsSignatureBlock, windows_core::IUnknown);
impl IXpsSignatureBlock {
    pub unsafe fn GetRequests(&self) -> windows_core::Result<IXpsSignatureRequestCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequests)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocumentName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRequest<P0>(&self, requestid: P0) -> windows_core::Result<IXpsSignatureRequest>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRequest)(windows_core::Interface::as_raw(self), requestid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureBlock_Impl: windows_core::IUnknownImpl {
    fn GetRequests(&self) -> windows_core::Result<IXpsSignatureRequestCollection>;
    fn GetPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn GetDocumentIndex(&self) -> windows_core::Result<u32>;
    fn GetDocumentName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateRequest(&self, requestid: &windows_core::PCWSTR) -> windows_core::Result<IXpsSignatureRequest>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureBlock_Vtbl {
    pub const fn new<Identity: IXpsSignatureBlock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRequests<Identity: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requests: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlock_Impl::GetRequests(this) {
                    Ok(ok__) => {
                        requests.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartName<Identity: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlock_Impl::GetPartName(this) {
                    Ok(ok__) => {
                        partname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentIndex<Identity: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixeddocumentindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlock_Impl::GetDocumentIndex(this) {
                    Ok(ok__) => {
                        fixeddocumentindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentName<Identity: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixeddocumentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlock_Impl::GetDocumentName(this) {
                    Ok(ok__) => {
                        fixeddocumentname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRequest<Identity: IXpsSignatureBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::PCWSTR, signaturerequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlock_Impl::CreateRequest(this, core::mem::transmute(&requestid)) {
                    Ok(ok__) => {
                        signaturerequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequests: GetRequests::<Identity, OFFSET>,
            GetPartName: GetPartName::<Identity, OFFSET>,
            GetDocumentIndex: GetDocumentIndex::<Identity, OFFSET>,
            GetDocumentName: GetDocumentName::<Identity, OFFSET>,
            CreateRequest: CreateRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureBlock as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignatureBlock {}
windows_core::imp::define_interface!(IXpsSignatureBlockCollection, IXpsSignatureBlockCollection_Vtbl, 0x23397050_fe99_467a_8dce_9237f074ffe4);
windows_core::imp::interface_hierarchy!(IXpsSignatureBlockCollection, windows_core::IUnknown);
impl IXpsSignatureBlockCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureBlock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IXpsSignatureBlockCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureBlock>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
}
impl IXpsSignatureBlockCollection_Vtbl {
    pub const fn new<Identity: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlockCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signatureblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureBlockCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        signatureblock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsSignatureBlockCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureBlockCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureBlockCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsSignatureBlockCollection {}
windows_core::imp::define_interface!(IXpsSignatureCollection, IXpsSignatureCollection_Vtbl, 0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
windows_core::imp::interface_hierarchy!(IXpsSignatureCollection, windows_core::IUnknown);
impl IXpsSignatureCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IXpsSignatureCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignature>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
}
impl IXpsSignatureCollection_Vtbl {
    pub const fn new<Identity: IXpsSignatureCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        signature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsSignatureCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsSignatureCollection {}
windows_core::imp::define_interface!(IXpsSignatureManager, IXpsSignatureManager_Vtbl, 0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
windows_core::imp::interface_hierarchy!(IXpsSignatureManager, windows_core::IUnknown);
impl IXpsSignatureManager {
    pub unsafe fn LoadPackageFile<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadPackageFile)(windows_core::Interface::as_raw(self), filename.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadPackageStream<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadPackageStream)(windows_core::Interface::as_raw(self), stream.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Security_Cryptography")]
    pub unsafe fn Sign<P0>(&self, signoptions: P0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<IXpsSignature>
    where
        P0: windows_core::Param<IXpsSigningOptions>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sign)(windows_core::Interface::as_raw(self), signoptions.param().abi(), x509certificate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignatureOriginPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureOriginPartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureOriginPartName)(windows_core::Interface::as_raw(self), signatureoriginpartname.param().abi()).ok() }
    }
    pub unsafe fn GetSignatures(&self) -> windows_core::Result<IXpsSignatureCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatures)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddSignatureBlock<P0>(&self, partname: P0, fixeddocumentindex: u32) -> windows_core::Result<IXpsSignatureBlock>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddSignatureBlock)(windows_core::Interface::as_raw(self), partname.param().abi(), fixeddocumentindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSignatureBlocks(&self) -> windows_core::Result<IXpsSignatureBlockCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSigningOptions(&self) -> windows_core::Result<IXpsSigningOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSigningOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn SavePackageToFile<P0>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SavePackageToFile)(windows_core::Interface::as_raw(self), filename.param().abi(), securityattributes, flagsandattributes).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SavePackageToStream<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SavePackageToStream)(windows_core::Interface::as_raw(self), stream.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureManager_Impl: windows_core::IUnknownImpl {
    fn LoadPackageFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LoadPackageStream(&self, stream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Sign(&self, signoptions: windows_core::Ref<IXpsSigningOptions>, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<IXpsSignature>;
    fn GetSignatureOriginPartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignatureOriginPartName(&self, signatureoriginpartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetSignatures(&self) -> windows_core::Result<IXpsSignatureCollection>;
    fn AddSignatureBlock(&self, partname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>, fixeddocumentindex: u32) -> windows_core::Result<IXpsSignatureBlock>;
    fn GetSignatureBlocks(&self) -> windows_core::Result<IXpsSignatureBlockCollection>;
    fn CreateSigningOptions(&self) -> windows_core::Result<IXpsSigningOptions>;
    fn SavePackageToFile(&self, filename: &windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::Result<()>;
    fn SavePackageToStream(&self, stream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureManager_Vtbl {
    pub const fn new<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadPackageFile<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureManager_Impl::LoadPackageFile(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn LoadPackageStream<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureManager_Impl::LoadPackageStream(this, core::mem::transmute_copy(&stream)).into()
            }
        }
        unsafe extern "system" fn Sign<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signoptions: *mut core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureManager_Impl::Sign(this, core::mem::transmute_copy(&signoptions), core::mem::transmute_copy(&x509certificate)) {
                    Ok(ok__) => {
                        signature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureManager_Impl::GetSignatureOriginPartName(this) {
                    Ok(ok__) => {
                        signatureoriginpartname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureoriginpartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureManager_Impl::SetSignatureOriginPartName(this, core::mem::transmute_copy(&signatureoriginpartname)).into()
            }
        }
        unsafe extern "system" fn GetSignatures<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatures: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureManager_Impl::GetSignatures(this) {
                    Ok(ok__) => {
                        signatures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSignatureBlock<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partname: *mut core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureManager_Impl::AddSignatureBlock(this, core::mem::transmute_copy(&partname), core::mem::transmute_copy(&fixeddocumentindex)) {
                    Ok(ok__) => {
                        signatureblock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignatureBlocks<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureblocks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureManager_Impl::GetSignatureBlocks(this) {
                    Ok(ok__) => {
                        signatureblocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signingoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureManager_Impl::CreateSigningOptions(this) {
                    Ok(ok__) => {
                        signingoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SavePackageToFile<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureManager_Impl::SavePackageToFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&securityattributes), core::mem::transmute_copy(&flagsandattributes)).into()
            }
        }
        unsafe extern "system" fn SavePackageToStream<Identity: IXpsSignatureManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureManager_Impl::SavePackageToStream(this, core::mem::transmute_copy(&stream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadPackageFile: LoadPackageFile::<Identity, OFFSET>,
            LoadPackageStream: LoadPackageStream::<Identity, OFFSET>,
            Sign: Sign::<Identity, OFFSET>,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, OFFSET>,
            GetSignatures: GetSignatures::<Identity, OFFSET>,
            AddSignatureBlock: AddSignatureBlock::<Identity, OFFSET>,
            GetSignatureBlocks: GetSignatureBlocks::<Identity, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, OFFSET>,
            SavePackageToFile: SavePackageToFile::<Identity, OFFSET>,
            SavePackageToStream: SavePackageToStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignatureManager {}
windows_core::imp::define_interface!(IXpsSignatureRequest, IXpsSignatureRequest_Vtbl, 0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
windows_core::imp::interface_hierarchy!(IXpsSignatureRequest, windows_core::IUnknown);
impl IXpsSignatureRequest {
    pub unsafe fn GetIntent(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIntent<P0>(&self, intent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIntent)(windows_core::Interface::as_raw(self), intent.param().abi()).ok() }
    }
    pub unsafe fn GetRequestedSigner(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestedSigner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRequestedSigner<P0>(&self, signername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRequestedSigner)(windows_core::Interface::as_raw(self), signername.param().abi()).ok() }
    }
    pub unsafe fn GetRequestSignByDate(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestSignByDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRequestSignByDate<P0>(&self, datestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRequestSignByDate)(windows_core::Interface::as_raw(self), datestring.param().abi()).ok() }
    }
    pub unsafe fn GetSigningLocale(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSigningLocale<P0>(&self, place: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSigningLocale)(windows_core::Interface::as_raw(self), place.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSpotLocation)(windows_core::Interface::as_raw(self), pageindex as _, core::mem::transmute(pagepartname), x as _, y as _).ok() }
    }
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpotLocation)(windows_core::Interface::as_raw(self), pageindex, x, y).ok() }
    }
    pub unsafe fn GetRequestId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignature(&self) -> windows_core::Result<IXpsSignature> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureRequest_Impl: windows_core::IUnknownImpl {
    fn GetIntent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIntent(&self, intent: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRequestedSigner(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRequestedSigner(&self, signername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRequestSignByDate(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRequestSignByDate(&self, datestring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSigningLocale(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSigningLocale(&self, place: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: windows_core::OutRef<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> windows_core::Result<()>;
    fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> windows_core::Result<()>;
    fn GetRequestId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignature(&self) -> windows_core::Result<IXpsSignature>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSignatureRequest_Vtbl {
    pub const fn new<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIntent<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intent: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequest_Impl::GetIntent(this) {
                    Ok(ok__) => {
                        intent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIntent<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intent: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequest_Impl::SetIntent(this, core::mem::transmute(&intent)).into()
            }
        }
        unsafe extern "system" fn GetRequestedSigner<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signername: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequest_Impl::GetRequestedSigner(this) {
                    Ok(ok__) => {
                        signername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRequestedSigner<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signername: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequest_Impl::SetRequestedSigner(this, core::mem::transmute(&signername)).into()
            }
        }
        unsafe extern "system" fn GetRequestSignByDate<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datestring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequest_Impl::GetRequestSignByDate(this) {
                    Ok(ok__) => {
                        datestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRequestSignByDate<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datestring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequest_Impl::SetRequestSignByDate(this, core::mem::transmute(&datestring)).into()
            }
        }
        unsafe extern "system" fn GetSigningLocale<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, place: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequest_Impl::GetSigningLocale(this) {
                    Ok(ok__) => {
                        place.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSigningLocale<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, place: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequest_Impl::SetSigningLocale(this, core::mem::transmute(&place)).into()
            }
        }
        unsafe extern "system" fn GetSpotLocation<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut core::ffi::c_void, x: *mut f32, y: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequest_Impl::GetSpotLocation(this, core::mem::transmute_copy(&pageindex), core::mem::transmute_copy(&pagepartname), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn SetSpotLocation<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequest_Impl::SetSpotLocation(this, core::mem::transmute_copy(&pageindex), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn GetRequestId<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequest_Impl::GetRequestId(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignature<Identity: IXpsSignatureRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequest_Impl::GetSignature(this) {
                    Ok(ok__) => {
                        signature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIntent: GetIntent::<Identity, OFFSET>,
            SetIntent: SetIntent::<Identity, OFFSET>,
            GetRequestedSigner: GetRequestedSigner::<Identity, OFFSET>,
            SetRequestedSigner: SetRequestedSigner::<Identity, OFFSET>,
            GetRequestSignByDate: GetRequestSignByDate::<Identity, OFFSET>,
            SetRequestSignByDate: SetRequestSignByDate::<Identity, OFFSET>,
            GetSigningLocale: GetSigningLocale::<Identity, OFFSET>,
            SetSigningLocale: SetSigningLocale::<Identity, OFFSET>,
            GetSpotLocation: GetSpotLocation::<Identity, OFFSET>,
            SetSpotLocation: SetSpotLocation::<Identity, OFFSET>,
            GetRequestId: GetRequestId::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSignatureRequest {}
windows_core::imp::define_interface!(IXpsSignatureRequestCollection, IXpsSignatureRequestCollection_Vtbl, 0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
windows_core::imp::interface_hierarchy!(IXpsSignatureRequestCollection, windows_core::IUnknown);
impl IXpsSignatureRequestCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IXpsSignatureRequestCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, index: u32) -> windows_core::Result<IXpsSignatureRequest>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
}
impl IXpsSignatureRequestCollection_Vtbl {
    pub const fn new<Identity: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequestCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, signaturerequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSignatureRequestCollection_Impl::GetAt(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        signaturerequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IXpsSignatureRequestCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSignatureRequestCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSignatureRequestCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXpsSignatureRequestCollection {}
windows_core::imp::define_interface!(IXpsSigningOptions, IXpsSigningOptions_Vtbl, 0x7718eae4_3215_49be_af5b_594fef7fcfa6);
windows_core::imp::interface_hierarchy!(IXpsSigningOptions, windows_core::IUnknown);
impl IXpsSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureId)(windows_core::Interface::as_raw(self), signatureid.param().abi()).ok() }
    }
    pub unsafe fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignatureMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignatureMethod)(windows_core::Interface::as_raw(self), signaturemethod.param().abi()).ok() }
    }
    pub unsafe fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDigestMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDigestMethod<P0>(&self, digestmethod: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDigestMethod)(windows_core::Interface::as_raw(self), digestmethod.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignaturePartName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Packaging::Opc::IOpcPartUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignaturePartName)(windows_core::Interface::as_raw(self), signaturepartname.param().abi()).ok() }
    }
    pub unsafe fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPolicy)(windows_core::Interface::as_raw(self), policy).ok() }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSigningTimeFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSigningTimeFormat)(windows_core::Interface::as_raw(self), timeformat).ok() }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjects(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferences(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomReferences)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateSet(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<XPS_SIGN_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSigningOptions_Impl: windows_core::IUnknownImpl {
    fn GetSignatureId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureId(&self, signatureid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignatureMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSignatureMethod(&self, signaturemethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDigestMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDigestMethod(&self, digestmethod: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignaturePartName(&self) -> windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignaturePartName(&self, signaturepartname: windows_core::Ref<super::Packaging::Opc::IOpcPartUri>) -> windows_core::Result<()>;
    fn GetPolicy(&self) -> windows_core::Result<XPS_SIGN_POLICY>;
    fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> windows_core::Result<()>;
    fn GetSigningTimeFormat(&self) -> windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::Result<()>;
    fn GetCustomObjects(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet>;
    fn GetCustomReferences(&self) -> windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet>;
    fn GetCertificateSet(&self) -> windows_core::Result<super::Packaging::Opc::IOpcCertificateSet>;
    fn GetFlags(&self) -> windows_core::Result<XPS_SIGN_FLAGS>;
    fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl IXpsSigningOptions_Vtbl {
    pub const fn new<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSignatureId<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetSignatureId(this) {
                    Ok(ok__) => {
                        signatureid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignatureId<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signatureid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetSignatureId(this, core::mem::transmute(&signatureid)).into()
            }
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetSignatureMethod(this) {
                    Ok(ok__) => {
                        signaturemethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturemethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetSignatureMethod(this, core::mem::transmute(&signaturemethod)).into()
            }
        }
        unsafe extern "system" fn GetDigestMethod<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetDigestMethod(this) {
                    Ok(ok__) => {
                        digestmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDigestMethod<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digestmethod: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetDigestMethod(this, core::mem::transmute(&digestmethod)).into()
            }
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetSignaturePartName(this) {
                    Ok(ok__) => {
                        signaturepartname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signaturepartname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetSignaturePartName(this, core::mem::transmute_copy(&signaturepartname)).into()
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetPolicy(this) {
                    Ok(ok__) => {
                        policy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPolicy<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: XPS_SIGN_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetPolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetSigningTimeFormat(this) {
                    Ok(ok__) => {
                        timeformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSigningTimeFormat<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetSigningTimeFormat(this, core::mem::transmute_copy(&timeformat)).into()
            }
        }
        unsafe extern "system" fn GetCustomObjects<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetCustomObjects(this) {
                    Ok(ok__) => {
                        customobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustomReferences<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customreferenceset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetCustomReferences(this) {
                    Ok(ok__) => {
                        customreferenceset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateSet<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificateset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetCertificateSet(this) {
                    Ok(ok__) => {
                        certificateset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXpsSigningOptions_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IXpsSigningOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXpsSigningOptions_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignatureId: GetSignatureId::<Identity, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, OFFSET>,
            SetDigestMethod: SetDigestMethod::<Identity, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, OFFSET>,
            GetPolicy: GetPolicy::<Identity, OFFSET>,
            SetPolicy: SetPolicy::<Identity, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, OFFSET>,
            SetSigningTimeFormat: SetSigningTimeFormat::<Identity, OFFSET>,
            GetCustomObjects: GetCustomObjects::<Identity, OFFSET>,
            GetCustomReferences: GetCustomReferences::<Identity, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXpsSigningOptions as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IXpsSigningOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINTER_DEVICE_CAPABILITIES(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: windows_core::BOOL,
    pub bSetPageDevice: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSINJECT_POINT(pub u16);
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
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_0,
    pub scRGB: XPS_COLOR_0_1,
    pub context: XPS_COLOR_0_2,
}
impl Default for XPS_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR_0_2 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_COLOR_0_0 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_COLOR_0_1 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_COLOR_TYPE(pub i32);
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_DASH_CAP(pub i32);
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_FILL_RULE(pub i32);
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_FONT_EMBEDDING(pub i32);
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_IMAGE_TYPE(pub i32);
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_INTERLEAVING(pub i32);
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_LINE_CAP(pub i32);
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_LINE_JOIN(pub i32);
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_OBJECT_TYPE(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_SEGMENT_TYPE(pub i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_SIGN_FLAGS(pub i32);
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
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_SIGN_POLICY(pub i32);
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
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_SPREAD_METHOD(pub i32);
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_STYLE_SIMULATION(pub i32);
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XPS_TILE_MODE(pub i32);
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
pub const XpsOMObjectFactory: windows_core::GUID = windows_core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: windows_core::GUID = windows_core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);

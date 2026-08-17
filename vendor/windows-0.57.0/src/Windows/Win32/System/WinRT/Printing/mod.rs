windows_core::imp::define_interface!(IPrintManagerInterop, IPrintManagerInterop_Vtbl, 0xc5435a42_8d43_4e7b_a68a_ef311e392087);
impl core::ops::Deref for IPrintManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintManagerInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShowPrintUIForWindowAsync<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrintManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowConfigurationNative, IPrintWorkflowConfigurationNative_Vtbl, 0xc056be0a_9ee2_450a_9823_964f0006f2bb);
impl core::ops::Deref for IPrintWorkflowConfigurationNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintWorkflowConfigurationNative, windows_core::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn PrinterQueue(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrinterQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn DriverProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn UserProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrintWorkflowConfigurationNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub PrinterQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    PrinterQueue: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub DriverProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    DriverProperties: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub UserProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    UserProperties: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowObjectModelSourceFileContentNative, IPrintWorkflowObjectModelSourceFileContentNative_Vtbl, 0x68c9e477_993e_4052_8ac6_454eff58db9d);
impl core::ops::Deref for IPrintWorkflowObjectModelSourceFileContentNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintWorkflowObjectModelSourceFileContentNative, windows_core::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    pub unsafe fn StartXpsOMGeneration<P0>(&self, receiver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintWorkflowXpsReceiver>,
    {
        (windows_core::Interface::vtable(self).StartXpsOMGeneration)(windows_core::Interface::as_raw(self), receiver.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn ObjectFactory(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartXpsOMGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowXpsObjectModelTargetPackageNative, IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl, 0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
impl core::ops::Deref for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsObjectModelTargetPackageNative, windows_core::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn DocumentPackageTarget(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DocumentPackageTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
windows_core::imp::define_interface!(IPrintWorkflowXpsReceiver, IPrintWorkflowXpsReceiver_Vtbl, 0x04097374_77b8_47f6_8167_aae29d4cf84b);
impl core::ops::Deref for IPrintWorkflowXpsReceiver {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsReceiver, windows_core::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<P0>(&self, documentsequenceprintticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SetDocumentSequencePrintTicket)(windows_core::Interface::as_raw(self), documentsequenceprintticket.param().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<P0>(&self, documentsequenceuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDocumentSequenceUri)(windows_core::Interface::as_raw(self), documentsequenceuri.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<P0, P1>(&self, documentid: u32, documentprintticket: P0, documenturi: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddDocumentData)(windows_core::Interface::as_raw(self), documentid, documentprintticket.param().abi(), documenturi.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<P0, P1>(&self, documentid: u32, pageid: u32, pagereference: P0, pageuri: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Xps::IXpsOMPageReference>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), documentid, pageid, pagereference.param().abi(), pageuri.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPrintWorkflowXpsReceiver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentSequencePrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentSequencePrintTicket: usize,
    pub SetDocumentSequenceUri: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDocumentData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDocumentData: usize,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub AddPage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintWorkflowXpsReceiver2, IPrintWorkflowXpsReceiver2_Vtbl, 0x023bcc0c_dfab_4a61_b074_490c6995580d);
impl core::ops::Deref for IPrintWorkflowXpsReceiver2 {
    type Target = IPrintWorkflowXpsReceiver;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsReceiver2, windows_core::IUnknown, IPrintWorkflowXpsReceiver);
impl IPrintWorkflowXpsReceiver2 {
    pub unsafe fn Failed(&self, xpserror: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Failed)(windows_core::Interface::as_raw(self), xpserror).ok()
    }
}
#[repr(C)]
pub struct IPrintWorkflowXpsReceiver2_Vtbl {
    pub base__: IPrintWorkflowXpsReceiver_Vtbl,
    pub Failed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DManagerInterop, IPrinting3DManagerInterop_Vtbl, 0x9ca31010_1484_4587_b26b_dddf9f9caecd);
impl core::ops::Deref for IPrinting3DManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrinting3DManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPrinting3DManagerInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShowPrintUIForWindowAsync<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

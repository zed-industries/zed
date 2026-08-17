pub trait IPrintManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintManagerInterop {}
impl IPrintManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintManagerInterop_Impl, const OFFSET: isize>() -> IPrintManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowConfigurationNative_Impl: Sized {
    fn PrinterQueue(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue>;
    fn DriverProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
    fn UserProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintWorkflowConfigurationNative {}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl IPrintWorkflowConfigurationNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>() -> IPrintWorkflowConfigurationNative_Vtbl {
        unsafe extern "system" fn PrinterQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintWorkflowConfigurationNative_Impl::PrinterQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintWorkflowConfigurationNative_Impl::DriverProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintWorkflowConfigurationNative_Impl::UserProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, Impl, OFFSET>,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowConfigurationNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowObjectModelSourceFileContentNative_Impl: Sized {
    fn StartXpsOMGeneration(&self, receiver: Option<&IPrintWorkflowXpsReceiver>) -> windows_core::Result<()>;
    fn ObjectFactory(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IPrintWorkflowObjectModelSourceFileContentNative {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
        unsafe extern "system" fn StartXpsOMGeneration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, receiver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowObjectModelSourceFileContentNative_Impl::StartXpsOMGeneration(this, windows_core::from_raw_borrowed(&receiver)).into()
        }
        unsafe extern "system" fn ObjectFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintWorkflowObjectModelSourceFileContentNative_Impl::ObjectFactory(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartXpsOMGeneration: StartXpsOMGeneration::<Identity, Impl, OFFSET>,
            ObjectFactory: ObjectFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowObjectModelSourceFileContentNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowXpsObjectModelTargetPackageNative_Impl: Sized {
    fn DocumentPackageTarget(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IPrintWorkflowXpsObjectModelTargetPackageNative {}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: isize>() -> IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
        unsafe extern "system" fn DocumentPackageTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintWorkflowXpsObjectModelTargetPackageNative_Impl::DocumentPackageTarget(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DocumentPackageTarget: DocumentPackageTarget::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsObjectModelTargetPackageNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver_Impl: Sized {
    fn SetDocumentSequencePrintTicket(&self, documentsequenceprintticket: Option<&super::super::Com::IStream>) -> windows_core::Result<()>;
    fn SetDocumentSequenceUri(&self, documentsequenceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDocumentData(&self, documentid: u32, documentprintticket: Option<&super::super::Com::IStream>, documenturi: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddPage(&self, documentid: u32, pageid: u32, pagereference: Option<&super::super::super::Storage::Xps::IXpsOMPageReference>, pageuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintWorkflowXpsReceiver {}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>() -> IPrintWorkflowXpsReceiver_Vtbl {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowXpsReceiver_Impl::SetDocumentSequencePrintTicket(this, windows_core::from_raw_borrowed(&documentsequenceprintticket)).into()
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequenceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowXpsReceiver_Impl::SetDocumentSequenceUri(this, core::mem::transmute(&documentsequenceuri)).into()
        }
        unsafe extern "system" fn AddDocumentData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u32, documentprintticket: *mut core::ffi::c_void, documenturi: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowXpsReceiver_Impl::AddDocumentData(this, core::mem::transmute_copy(&documentid), windows_core::from_raw_borrowed(&documentprintticket), core::mem::transmute(&documenturi)).into()
        }
        unsafe extern "system" fn AddPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u32, pageid: u32, pagereference: *mut core::ffi::c_void, pageuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowXpsReceiver_Impl::AddPage(this, core::mem::transmute_copy(&documentid), core::mem::transmute_copy(&pageid), windows_core::from_raw_borrowed(&pagereference), core::mem::transmute(&pageuri)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowXpsReceiver_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDocumentSequencePrintTicket: SetDocumentSequencePrintTicket::<Identity, Impl, OFFSET>,
            SetDocumentSequenceUri: SetDocumentSequenceUri::<Identity, Impl, OFFSET>,
            AddDocumentData: AddDocumentData::<Identity, Impl, OFFSET>,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver2_Impl: Sized + IPrintWorkflowXpsReceiver_Impl {
    fn Failed(&self, xpserror: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintWorkflowXpsReceiver2 {}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: isize>() -> IPrintWorkflowXpsReceiver2_Vtbl {
        unsafe extern "system" fn Failed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpserror: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintWorkflowXpsReceiver2_Impl::Failed(this, core::mem::transmute_copy(&xpserror)).into()
        }
        Self { base__: IPrintWorkflowXpsReceiver_Vtbl::new::<Identity, Impl, OFFSET>(), Failed: Failed::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver2 as windows_core::Interface>::IID || iid == &<IPrintWorkflowXpsReceiver as windows_core::Interface>::IID
    }
}
pub trait IPrinting3DManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrinting3DManagerInterop {}
impl IPrinting3DManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: isize>() -> IPrinting3DManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrinting3DManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrinting3DManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrinting3DManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinting3DManagerInterop as windows_core::Interface>::IID
    }
}

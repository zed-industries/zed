#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub trait IPrintDocumentPageSource_Impl: Sized {
    fn GetPreviewPageCollection(&self, docpackagetarget: Option<&super::super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>) -> windows_core::Result<IPrintPreviewPageCollection>;
    fn MakeDocument(&self, printtaskoptions: Option<&windows_core::IInspectable>, docpackagetarget: Option<&super::super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_Xps_Printing")]
impl windows_core::RuntimeName for IPrintDocumentPageSource {}
#[cfg(feature = "Win32_Storage_Xps_Printing")]
impl IPrintDocumentPageSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintDocumentPageSource_Vtbl
    where
        Identity: IPrintDocumentPageSource_Impl,
    {
        unsafe extern "system" fn GetPreviewPageCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, docpackagetarget: *mut core::ffi::c_void, docpagecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintDocumentPageSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintDocumentPageSource_Impl::GetPreviewPageCollection(this, windows_core::from_raw_borrowed(&docpackagetarget)) {
                Ok(ok__) => {
                    docpagecollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, printtaskoptions: *mut core::ffi::c_void, docpackagetarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintDocumentPageSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintDocumentPageSource_Impl::MakeDocument(this, windows_core::from_raw_borrowed(&printtaskoptions), windows_core::from_raw_borrowed(&docpackagetarget)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPreviewPageCollection: GetPreviewPageCollection::<Identity, OFFSET>,
            MakeDocument: MakeDocument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDocumentPageSource as windows_core::Interface>::IID
    }
}
pub trait IPrintManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintManagerInterop {}
impl IPrintManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintManagerInterop_Vtbl
    where
        Identity: IPrintManagerInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintManagerInterop as windows_core::Interface>::IID
    }
}
pub trait IPrintPreviewPageCollection_Impl: Sized {
    fn Paginate(&self, currentjobpage: u32, printtaskoptions: Option<&windows_core::IInspectable>) -> windows_core::Result<()>;
    fn MakePage(&self, desiredjobpage: u32, width: f32, height: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintPreviewPageCollection {}
impl IPrintPreviewPageCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintPreviewPageCollection_Vtbl
    where
        Identity: IPrintPreviewPageCollection_Impl,
    {
        unsafe extern "system" fn Paginate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentjobpage: u32, printtaskoptions: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintPreviewPageCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPreviewPageCollection_Impl::Paginate(this, core::mem::transmute_copy(&currentjobpage), windows_core::from_raw_borrowed(&printtaskoptions)).into()
        }
        unsafe extern "system" fn MakePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredjobpage: u32, width: f32, height: f32) -> windows_core::HRESULT
        where
            Identity: IPrintPreviewPageCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintPreviewPageCollection_Impl::MakePage(this, core::mem::transmute_copy(&desiredjobpage), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Paginate: Paginate::<Identity, OFFSET>, MakePage: MakePage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPreviewPageCollection as windows_core::Interface>::IID
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWorkflowConfigurationNative_Vtbl
    where
        Identity: IPrintWorkflowConfigurationNative_Impl,
    {
        unsafe extern "system" fn PrinterQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowConfigurationNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintWorkflowConfigurationNative_Impl::PrinterQueue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowConfigurationNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintWorkflowConfigurationNative_Impl::DriverProperties(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowConfigurationNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintWorkflowConfigurationNative_Impl::UserProperties(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, OFFSET>,
            DriverProperties: DriverProperties::<Identity, OFFSET>,
            UserProperties: UserProperties::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentNative_Vtbl
    where
        Identity: IPrintWorkflowObjectModelSourceFileContentNative_Impl,
    {
        unsafe extern "system" fn StartXpsOMGeneration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, receiver: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowObjectModelSourceFileContentNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowObjectModelSourceFileContentNative_Impl::StartXpsOMGeneration(this, windows_core::from_raw_borrowed(&receiver)).into()
        }
        unsafe extern "system" fn ObjectFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowObjectModelSourceFileContentNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintWorkflowObjectModelSourceFileContentNative_Impl::ObjectFactory(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartXpsOMGeneration: StartXpsOMGeneration::<Identity, OFFSET>,
            ObjectFactory: ObjectFactory::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl
    where
        Identity: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl,
    {
        unsafe extern "system" fn DocumentPackageTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrintWorkflowXpsObjectModelTargetPackageNative_Impl::DocumentPackageTarget(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DocumentPackageTarget: DocumentPackageTarget::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWorkflowXpsReceiver_Vtbl
    where
        Identity: IPrintWorkflowXpsReceiver_Impl,
    {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowXpsReceiver_Impl::SetDocumentSequencePrintTicket(this, windows_core::from_raw_borrowed(&documentsequenceprintticket)).into()
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequenceuri: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowXpsReceiver_Impl::SetDocumentSequenceUri(this, core::mem::transmute(&documentsequenceuri)).into()
        }
        unsafe extern "system" fn AddDocumentData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u32, documentprintticket: *mut core::ffi::c_void, documenturi: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowXpsReceiver_Impl::AddDocumentData(this, core::mem::transmute_copy(&documentid), windows_core::from_raw_borrowed(&documentprintticket), core::mem::transmute(&documenturi)).into()
        }
        unsafe extern "system" fn AddPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u32, pageid: u32, pagereference: *mut core::ffi::c_void, pageuri: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowXpsReceiver_Impl::AddPage(this, core::mem::transmute_copy(&documentid), core::mem::transmute_copy(&pageid), windows_core::from_raw_borrowed(&pagereference), core::mem::transmute(&pageuri)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowXpsReceiver_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDocumentSequencePrintTicket: SetDocumentSequencePrintTicket::<Identity, OFFSET>,
            SetDocumentSequenceUri: SetDocumentSequenceUri::<Identity, OFFSET>,
            AddDocumentData: AddDocumentData::<Identity, OFFSET>,
            AddPage: AddPage::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintWorkflowXpsReceiver2_Vtbl
    where
        Identity: IPrintWorkflowXpsReceiver2_Impl,
    {
        unsafe extern "system" fn Failed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpserror: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPrintWorkflowXpsReceiver2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintWorkflowXpsReceiver2_Impl::Failed(this, core::mem::transmute_copy(&xpserror)).into()
        }
        Self { base__: IPrintWorkflowXpsReceiver_Vtbl::new::<Identity, OFFSET>(), Failed: Failed::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrinting3DManagerInterop_Vtbl
    where
        Identity: IPrinting3DManagerInterop_Impl,
    {
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinting3DManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinting3DManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrinting3DManagerInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrinting3DManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrinting3DManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinting3DManagerInterop as windows_core::Interface>::IID
    }
}

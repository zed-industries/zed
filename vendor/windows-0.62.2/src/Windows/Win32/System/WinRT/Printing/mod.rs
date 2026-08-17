windows_core::imp::define_interface!(IPrintDocumentPageSource, IPrintDocumentPageSource_Vtbl, 0xa96bb1db_172e_4667_82b5_ad97a252318f);
windows_core::imp::interface_hierarchy!(IPrintDocumentPageSource, windows_core::IUnknown);
impl IPrintDocumentPageSource {
    #[cfg(feature = "Win32_Storage_Xps_Printing")]
    pub unsafe fn GetPreviewPageCollection<P0>(&self, docpackagetarget: P0) -> windows_core::Result<IPrintPreviewPageCollection>
    where
        P0: windows_core::Param<super::super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviewPageCollection)(windows_core::Interface::as_raw(self), docpackagetarget.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Storage_Xps_Printing")]
    pub unsafe fn MakeDocument<P0, P1>(&self, printtaskoptions: P0, docpackagetarget: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).MakeDocument)(windows_core::Interface::as_raw(self), printtaskoptions.param().abi(), docpackagetarget.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentPageSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps_Printing")]
    pub GetPreviewPageCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps_Printing"))]
    GetPreviewPageCollection: usize,
    #[cfg(feature = "Win32_Storage_Xps_Printing")]
    pub MakeDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps_Printing"))]
    MakeDocument: usize,
}
#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub trait IPrintDocumentPageSource_Impl: windows_core::IUnknownImpl {
    fn GetPreviewPageCollection(&self, docpackagetarget: windows_core::Ref<super::super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>) -> windows_core::Result<IPrintPreviewPageCollection>;
    fn MakeDocument(&self, printtaskoptions: windows_core::Ref<windows_core::IInspectable>, docpackagetarget: windows_core::Ref<super::super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_Xps_Printing")]
impl IPrintDocumentPageSource_Vtbl {
    pub const fn new<Identity: IPrintDocumentPageSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPreviewPageCollection<Identity: IPrintDocumentPageSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, docpackagetarget: *mut core::ffi::c_void, docpagecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintDocumentPageSource_Impl::GetPreviewPageCollection(this, core::mem::transmute_copy(&docpackagetarget)) {
                    Ok(ok__) => {
                        docpagecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeDocument<Identity: IPrintDocumentPageSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, printtaskoptions: *mut core::ffi::c_void, docpackagetarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDocumentPageSource_Impl::MakeDocument(this, core::mem::transmute_copy(&printtaskoptions), core::mem::transmute_copy(&docpackagetarget)).into()
            }
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
#[cfg(feature = "Win32_Storage_Xps_Printing")]
impl windows_core::RuntimeName for IPrintDocumentPageSource {}
windows_core::imp::define_interface!(IPrintManagerInterop, IPrintManagerInterop_Vtbl, 0xc5435a42_8d43_4e7b_a68a_ef311e392087);
windows_core::imp::interface_hierarchy!(IPrintManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintManagerInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ShowPrintUIForWindowAsync<T>(&self, appwindow: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IPrintManagerInterop_Vtbl {
    pub const fn new<Identity: IPrintManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
            }
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
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
impl windows_core::RuntimeName for IPrintManagerInterop {}
windows_core::imp::define_interface!(IPrintPreviewPageCollection, IPrintPreviewPageCollection_Vtbl, 0x0b31cc62_d7ec_4747_9d6e_f2537d870f2b);
windows_core::imp::interface_hierarchy!(IPrintPreviewPageCollection, windows_core::IUnknown);
impl IPrintPreviewPageCollection {
    pub unsafe fn Paginate<P1>(&self, currentjobpage: u32, printtaskoptions: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).Paginate)(windows_core::Interface::as_raw(self), currentjobpage, printtaskoptions.param().abi()).ok() }
    }
    pub unsafe fn MakePage(&self, desiredjobpage: u32, width: f32, height: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MakePage)(windows_core::Interface::as_raw(self), desiredjobpage, width, height).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPreviewPageCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Paginate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MakePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, f32) -> windows_core::HRESULT,
}
pub trait IPrintPreviewPageCollection_Impl: windows_core::IUnknownImpl {
    fn Paginate(&self, currentjobpage: u32, printtaskoptions: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>;
    fn MakePage(&self, desiredjobpage: u32, width: f32, height: f32) -> windows_core::Result<()>;
}
impl IPrintPreviewPageCollection_Vtbl {
    pub const fn new<Identity: IPrintPreviewPageCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Paginate<Identity: IPrintPreviewPageCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentjobpage: u32, printtaskoptions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPreviewPageCollection_Impl::Paginate(this, core::mem::transmute_copy(&currentjobpage), core::mem::transmute_copy(&printtaskoptions)).into()
            }
        }
        unsafe extern "system" fn MakePage<Identity: IPrintPreviewPageCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredjobpage: u32, width: f32, height: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintPreviewPageCollection_Impl::MakePage(this, core::mem::transmute_copy(&desiredjobpage), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Paginate: Paginate::<Identity, OFFSET>, MakePage: MakePage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintPreviewPageCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintPreviewPageCollection {}
windows_core::imp::define_interface!(IPrintWorkflowConfigurationNative, IPrintWorkflowConfigurationNative_Vtbl, 0xc056be0a_9ee2_450a_9823_964f0006f2bb);
windows_core::imp::interface_hierarchy!(IPrintWorkflowConfigurationNative, windows_core::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn PrinterQueue(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrinterQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn DriverProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn UserProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowConfigurationNative_Impl: windows_core::IUnknownImpl {
    fn PrinterQueue(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue>;
    fn DriverProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
    fn UserProperties(&self) -> windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl IPrintWorkflowConfigurationNative_Vtbl {
    pub const fn new<Identity: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrinterQueue<Identity: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintWorkflowConfigurationNative_Impl::PrinterQueue(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintWorkflowConfigurationNative_Impl::DriverProperties(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserProperties<Identity: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintWorkflowConfigurationNative_Impl::UserProperties(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintWorkflowConfigurationNative {}
windows_core::imp::define_interface!(IPrintWorkflowObjectModelSourceFileContentNative, IPrintWorkflowObjectModelSourceFileContentNative_Vtbl, 0x68c9e477_993e_4052_8ac6_454eff58db9d);
windows_core::imp::interface_hierarchy!(IPrintWorkflowObjectModelSourceFileContentNative, windows_core::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    pub unsafe fn StartXpsOMGeneration<P0>(&self, receiver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintWorkflowXpsReceiver>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartXpsOMGeneration)(windows_core::Interface::as_raw(self), receiver.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn ObjectFactory(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartXpsOMGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowObjectModelSourceFileContentNative_Impl: windows_core::IUnknownImpl {
    fn StartXpsOMGeneration(&self, receiver: windows_core::Ref<IPrintWorkflowXpsReceiver>) -> windows_core::Result<()>;
    fn ObjectFactory(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub const fn new<Identity: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartXpsOMGeneration<Identity: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, receiver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowObjectModelSourceFileContentNative_Impl::StartXpsOMGeneration(this, core::mem::transmute_copy(&receiver)).into()
            }
        }
        unsafe extern "system" fn ObjectFactory<Identity: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintWorkflowObjectModelSourceFileContentNative_Impl::ObjectFactory(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IPrintWorkflowObjectModelSourceFileContentNative {}
windows_core::imp::define_interface!(IPrintWorkflowXpsObjectModelTargetPackageNative, IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl, 0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsObjectModelTargetPackageNative, windows_core::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn DocumentPackageTarget(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentPackageTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowXpsObjectModelTargetPackageNative_Impl: windows_core::IUnknownImpl {
    fn DocumentPackageTarget(&self) -> windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub const fn new<Identity: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DocumentPackageTarget<Identity: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintWorkflowXpsObjectModelTargetPackageNative_Impl::DocumentPackageTarget(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DocumentPackageTarget: DocumentPackageTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsObjectModelTargetPackageNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
impl windows_core::RuntimeName for IPrintWorkflowXpsObjectModelTargetPackageNative {}
windows_core::imp::define_interface!(IPrintWorkflowXpsReceiver, IPrintWorkflowXpsReceiver_Vtbl, 0x04097374_77b8_47f6_8167_aae29d4cf84b);
windows_core::imp::interface_hierarchy!(IPrintWorkflowXpsReceiver, windows_core::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<P0>(&self, documentsequenceprintticket: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentSequencePrintTicket)(windows_core::Interface::as_raw(self), documentsequenceprintticket.param().abi()).ok() }
    }
    pub unsafe fn SetDocumentSequenceUri<P0>(&self, documentsequenceuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentSequenceUri)(windows_core::Interface::as_raw(self), documentsequenceuri.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<P1, P2>(&self, documentid: u32, documentprintticket: P1, documenturi: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Com::IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDocumentData)(windows_core::Interface::as_raw(self), documentid, documentprintticket.param().abi(), documenturi.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<P2, P3>(&self, documentid: u32, pageid: u32, pagereference: P2, pageuri: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::super::Storage::Xps::IXpsOMPageReference>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), documentid, pageid, pagereference.param().abi(), pageuri.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver_Impl: windows_core::IUnknownImpl {
    fn SetDocumentSequencePrintTicket(&self, documentsequenceprintticket: windows_core::Ref<super::super::Com::IStream>) -> windows_core::Result<()>;
    fn SetDocumentSequenceUri(&self, documentsequenceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDocumentData(&self, documentid: u32, documentprintticket: windows_core::Ref<super::super::Com::IStream>, documenturi: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddPage(&self, documentid: u32, pageid: u32, pagereference: windows_core::Ref<super::super::super::Storage::Xps::IXpsOMPageReference>, pageuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver_Vtbl {
    pub const fn new<Identity: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Identity: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequenceprintticket: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowXpsReceiver_Impl::SetDocumentSequencePrintTicket(this, core::mem::transmute_copy(&documentsequenceprintticket)).into()
            }
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Identity: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentsequenceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowXpsReceiver_Impl::SetDocumentSequenceUri(this, core::mem::transmute(&documentsequenceuri)).into()
            }
        }
        unsafe extern "system" fn AddDocumentData<Identity: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u32, documentprintticket: *mut core::ffi::c_void, documenturi: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowXpsReceiver_Impl::AddDocumentData(this, core::mem::transmute_copy(&documentid), core::mem::transmute_copy(&documentprintticket), core::mem::transmute(&documenturi)).into()
            }
        }
        unsafe extern "system" fn AddPage<Identity: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u32, pageid: u32, pagereference: *mut core::ffi::c_void, pageuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowXpsReceiver_Impl::AddPage(this, core::mem::transmute_copy(&documentid), core::mem::transmute_copy(&pageid), core::mem::transmute_copy(&pagereference), core::mem::transmute(&pageuri)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowXpsReceiver_Impl::Close(this).into()
            }
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
impl windows_core::RuntimeName for IPrintWorkflowXpsReceiver {}
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
        unsafe { (windows_core::Interface::vtable(self).Failed)(windows_core::Interface::as_raw(self), xpserror).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_Vtbl {
    pub base__: IPrintWorkflowXpsReceiver_Vtbl,
    pub Failed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver2_Impl: IPrintWorkflowXpsReceiver_Impl {
    fn Failed(&self, xpserror: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver2_Vtbl {
    pub const fn new<Identity: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Failed<Identity: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpserror: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintWorkflowXpsReceiver2_Impl::Failed(this, core::mem::transmute_copy(&xpserror)).into()
            }
        }
        Self { base__: IPrintWorkflowXpsReceiver_Vtbl::new::<Identity, OFFSET>(), Failed: Failed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver2 as windows_core::Interface>::IID || iid == &<IPrintWorkflowXpsReceiver as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPrintWorkflowXpsReceiver2 {}
windows_core::imp::define_interface!(IPrinting3DManagerInterop, IPrinting3DManagerInterop_Vtbl, 0x9ca31010_1484_4587_b26b_dddf9f9caecd);
windows_core::imp::interface_hierarchy!(IPrinting3DManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPrinting3DManagerInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ShowPrintUIForWindowAsync<T>(&self, appwindow: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrinting3DManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IPrinting3DManagerInterop_Vtbl {
    pub const fn new<Identity: IPrinting3DManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinting3DManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
            }
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinting3DManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
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
impl windows_core::RuntimeName for IPrinting3DManagerInterop {}

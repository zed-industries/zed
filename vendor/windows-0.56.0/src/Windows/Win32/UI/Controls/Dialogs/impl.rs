pub trait IPrintDialogCallback_Impl: Sized {
    fn InitDone(&self) -> windows_core::Result<()>;
    fn SelectionChange(&self) -> windows_core::Result<()>;
    fn HandleMessage(&self, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintDialogCallback {}
impl IPrintDialogCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>() -> IPrintDialogCallback_Vtbl {
        unsafe extern "system" fn InitDone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintDialogCallback_Impl::InitDone(this).into()
        }
        unsafe extern "system" fn SelectionChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintDialogCallback_Impl::SelectionChange(this).into()
        }
        unsafe extern "system" fn HandleMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintDialogCallback_Impl::HandleMessage(this, core::mem::transmute_copy(&hdlg), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitDone: InitDone::<Identity, Impl, OFFSET>,
            SelectionChange: SelectionChange::<Identity, Impl, OFFSET>,
            HandleMessage: HandleMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDialogCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IPrintDialogServices_Impl: Sized {
    fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentPrinterName(&self, pprintername: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentPortName(&self, pportname: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IPrintDialogServices {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IPrintDialogServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>() -> IPrintDialogServices_Vtbl {
        unsafe extern "system" fn GetCurrentDevMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintDialogServices_Impl::GetCurrentDevMode(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetCurrentPrinterName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintername: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintDialogServices_Impl::GetCurrentPrinterName(this, core::mem::transmute_copy(&pprintername), core::mem::transmute_copy(&pcchsize)).into()
        }
        unsafe extern "system" fn GetCurrentPortName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportname: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintDialogServices_Impl::GetCurrentPortName(this, core::mem::transmute_copy(&pportname), core::mem::transmute_copy(&pcchsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentDevMode: GetCurrentDevMode::<Identity, Impl, OFFSET>,
            GetCurrentPrinterName: GetCurrentPrinterName::<Identity, Impl, OFFSET>,
            GetCurrentPortName: GetCurrentPortName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDialogServices as windows_core::Interface>::IID
    }
}

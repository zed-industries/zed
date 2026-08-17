pub trait IEnumWIA_DEV_CAPS_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumWIA_DEV_CAPS {}
impl IEnumWIA_DEV_CAPS_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumWIA_DEV_CAPS_Vtbl
    where
        Identity: IEnumWIA_DEV_CAPS_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_CAPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_DEV_CAPS_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_CAPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_DEV_CAPS_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_CAPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_DEV_CAPS_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_CAPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWIA_DEV_CAPS_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_CAPS_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWIA_DEV_CAPS_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_CAPS as windows_core::Interface>::IID
    }
}
pub trait IEnumWIA_DEV_INFO_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumWIA_DEV_INFO {}
impl IEnumWIA_DEV_INFO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumWIA_DEV_INFO_Vtbl
    where
        Identity: IEnumWIA_DEV_INFO_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_DEV_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_DEV_INFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_DEV_INFO_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWIA_DEV_INFO_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_DEV_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWIA_DEV_INFO_Impl::GetCount(this) {
                Ok(ok__) => {
                    celt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_INFO as windows_core::Interface>::IID
    }
}
pub trait IEnumWIA_FORMAT_INFO_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumWIA_FORMAT_INFO {}
impl IEnumWIA_FORMAT_INFO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumWIA_FORMAT_INFO_Vtbl
    where
        Identity: IEnumWIA_FORMAT_INFO_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_FORMAT_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_FORMAT_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_FORMAT_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_FORMAT_INFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_FORMAT_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWIA_FORMAT_INFO_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_FORMAT_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWIA_FORMAT_INFO_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWIA_FORMAT_INFO_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWIA_FORMAT_INFO_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_FORMAT_INFO as windows_core::Interface>::IID
    }
}
pub trait IEnumWiaItem_Impl: Sized {
    fn Next(&self, celt: u32, ppiwiaitem: *mut Option<IWiaItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWiaItem>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumWiaItem {}
impl IEnumWiaItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumWiaItem_Vtbl
    where
        Identity: IEnumWiaItem_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppiwiaitem: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWiaItem_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWiaItem_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWiaItem_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWiaItem_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWiaItem_Impl::GetCount(this) {
                Ok(ok__) => {
                    celt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWiaItem as windows_core::Interface>::IID
    }
}
pub trait IEnumWiaItem2_Impl: Sized {
    fn Next(&self, celt: u32, ppiwiaitem2: *mut Option<IWiaItem2>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWiaItem2>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumWiaItem2 {}
impl IEnumWiaItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumWiaItem2_Vtbl
    where
        Identity: IEnumWiaItem2_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppiwiaitem2: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWiaItem2_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem2), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWiaItem2_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumWiaItem2_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWiaItem2_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumWiaItem2_Impl::GetCount(this) {
                Ok(ok__) => {
                    celt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWiaItem2 as windows_core::Interface>::IID
    }
}
pub trait IWiaAppErrorHandler_Impl: Sized {
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn ReportStatus(&self, lflags: i32, pwiaitem2: Option<&IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaAppErrorHandler {}
impl IWiaAppErrorHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaAppErrorHandler_Vtbl
    where
        Identity: IWiaAppErrorHandler_Impl,
    {
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWiaAppErrorHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaAppErrorHandler_Impl::GetWindow(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT
        where
            Identity: IWiaAppErrorHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaAppErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            ReportStatus: ReportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaAppErrorHandler as windows_core::Interface>::IID
    }
}
pub trait IWiaDataCallback_Impl: Sized {
    fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaDataCallback {}
impl IWiaDataCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaDataCallback_Vtbl
    where
        Identity: IWiaDataCallback_Impl,
    {
        unsafe extern "system" fn BandedDataCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWiaDataCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDataCallback_Impl::BandedDataCallback(this, core::mem::transmute_copy(&lmessage), core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lpercentcomplete), core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&llength), core::mem::transmute_copy(&lreserved), core::mem::transmute_copy(&lreslength), core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BandedDataCallback: BandedDataCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDataCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaDataTransfer_Impl: Sized {
    fn idtGetData(&self, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: Option<&IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtGetBandedData(&self, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: Option<&IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> windows_core::Result<()>;
    fn idtEnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IWiaDataTransfer {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaDataTransfer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaDataTransfer_Vtbl
    where
        Identity: IWiaDataTransfer_Impl,
    {
        unsafe extern "system" fn idtGetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDataTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDataTransfer_Impl::idtGetData(this, core::mem::transmute_copy(&pmedium), windows_core::from_raw_borrowed(&piwiadatacallback)).into()
        }
        unsafe extern "system" fn idtGetBandedData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDataTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDataTransfer_Impl::idtGetBandedData(this, core::mem::transmute_copy(&pwiadatatransinfo), windows_core::from_raw_borrowed(&piwiadatacallback)).into()
        }
        unsafe extern "system" fn idtQueryGetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> windows_core::HRESULT
        where
            Identity: IWiaDataTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDataTransfer_Impl::idtQueryGetData(this, core::mem::transmute_copy(&pfe)).into()
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDataTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDataTransfer_Impl::idtEnumWIA_FORMAT_INFO(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT
        where
            Identity: IWiaDataTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDataTransfer_Impl::idtGetExtendedTransferInfo(this, core::mem::transmute_copy(&pextendedtransferinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            idtGetData: idtGetData::<Identity, OFFSET>,
            idtGetBandedData: idtGetBandedData::<Identity, OFFSET>,
            idtQueryGetData: idtQueryGetData::<Identity, OFFSET>,
            idtEnumWIA_FORMAT_INFO: idtEnumWIA_FORMAT_INFO::<Identity, OFFSET>,
            idtGetExtendedTransferInfo: idtGetExtendedTransferInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDataTransfer as windows_core::Interface>::IID
    }
}
pub trait IWiaDevMgr_Impl: Sized {
    fn EnumDeviceInfo(&self, lflag: i32) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR, ppitemroot: *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetImageDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: Option<&IWiaItem>, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: Option<&IWiaEventCallback>) -> windows_core::Result<windows_core::IUnknown>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaDevMgr {}
impl IWiaDevMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaDevMgr_Vtbl
    where
        Identity: IWiaDevMgr_Impl,
    {
        unsafe extern "system" fn EnumDeviceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflag: i32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDevMgr_Impl::EnumDeviceInfo(this, core::mem::transmute_copy(&lflag)) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, ppwiaitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDevMgr_Impl::CreateDevice(this, core::mem::transmute(&bstrdeviceid)) {
                Ok(ok__) => {
                    ppwiaitemroot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr_Impl::SelectDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid), core::mem::transmute_copy(&ppitemroot)).into()
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr_Impl::SelectDeviceDlgID(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn GetImageDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr_Impl::GetImageDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), windows_core::from_raw_borrowed(&pitemroot), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&pguidformat)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, peventguid: *const windows_core::GUID, bstrcommandline: core::mem::MaybeUninit<windows_core::BSTR>, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, bstricon: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrcommandline), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDevMgr_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), windows_core::from_raw_borrowed(&piwiaeventcallback)) {
                Ok(ok__) => {
                    peventobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, bstricon: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn AddDeviceDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr_Impl::AddDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, OFFSET>,
            AddDeviceDlg: AddDeviceDlg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDevMgr as windows_core::Interface>::IID
    }
}
pub trait IWiaDevMgr2_Impl: Sized {
    fn EnumDeviceInfo(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR, ppitemroot: *mut Option<IWiaItem2>) -> windows_core::Result<()>;
    fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: Option<&IWiaEventCallback>) -> windows_core::Result<windows_core::IUnknown>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetImageDlg(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut Option<IWiaItem2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaDevMgr2 {}
impl IWiaDevMgr2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaDevMgr2_Vtbl
    where
        Identity: IWiaDevMgr2_Impl,
    {
        unsafe extern "system" fn EnumDeviceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDevMgr2_Impl::EnumDeviceInfo(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, ppwiaitem2root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDevMgr2_Impl::CreateDevice(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid)) {
                Ok(ok__) => {
                    ppwiaitem2root.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr2_Impl::SelectDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid), core::mem::transmute_copy(&ppitemroot)).into()
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr2_Impl::SelectDeviceDlgID(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDevMgr2_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), windows_core::from_raw_borrowed(&piwiaeventcallback)) {
                Ok(ok__) => {
                    peventobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, peventguid: *const windows_core::GUID, bstrfullappname: core::mem::MaybeUninit<windows_core::BSTR>, bstrcommandlinearg: core::mem::MaybeUninit<windows_core::BSTR>, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, bstricon: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr2_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrfullappname), core::mem::transmute(&bstrcommandlinearg), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, bstricon: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr2_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
        }
        unsafe extern "system" fn GetImageDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, hwndparent: super::super::Foundation::HWND, bstrfoldername: core::mem::MaybeUninit<windows_core::BSTR>, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDevMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDevMgr2_Impl::GetImageDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDevMgr2 as windows_core::Interface>::IID
    }
}
pub trait IWiaDrvItem_Impl: Sized {
    fn GetItemFlags(&self) -> windows_core::Result<i32>;
    fn GetDeviceSpecContext(&self) -> windows_core::Result<*mut u8>;
    fn GetFullItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddItemToFolder(&self, __midl__iwiadrvitem0004: Option<&IWiaDrvItem>) -> windows_core::Result<()>;
    fn UnlinkItemTree(&self, __midl__iwiadrvitem0005: i32) -> windows_core::Result<()>;
    fn RemoveItemFromFolder(&self, __midl__iwiadrvitem0006: i32) -> windows_core::Result<()>;
    fn FindItemByName(&self, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: &windows_core::BSTR) -> windows_core::Result<IWiaDrvItem>;
    fn FindChildItemByName(&self, __midl__iwiadrvitem0010: &windows_core::BSTR) -> windows_core::Result<IWiaDrvItem>;
    fn GetParentItem(&self) -> windows_core::Result<IWiaDrvItem>;
    fn GetFirstChildItem(&self) -> windows_core::Result<IWiaDrvItem>;
    fn GetNextSiblingItem(&self) -> windows_core::Result<IWiaDrvItem>;
    fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IWiaDrvItem {}
impl IWiaDrvItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaDrvItem_Vtbl
    where
        Identity: IWiaDrvItem_Impl,
    {
        unsafe extern "system" fn GetItemFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetItemFlags(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0000.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSpecContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetDeviceSpecContext(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0001.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullItemName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0002: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetFullItemName(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0002.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0003: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetItemName(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0003.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItemToFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0004: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDrvItem_Impl::AddItemToFolder(this, windows_core::from_raw_borrowed(&__midl__iwiadrvitem0004)).into()
        }
        unsafe extern "system" fn UnlinkItemTree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDrvItem_Impl::UnlinkItemTree(this, core::mem::transmute_copy(&__midl__iwiadrvitem0005)).into()
        }
        unsafe extern "system" fn RemoveItemFromFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaDrvItem_Impl::RemoveItemFromFolder(this, core::mem::transmute_copy(&__midl__iwiadrvitem0006)).into()
        }
        unsafe extern "system" fn FindItemByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: core::mem::MaybeUninit<windows_core::BSTR>, __midl__iwiadrvitem0009: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::FindItemByName(this, core::mem::transmute_copy(&__midl__iwiadrvitem0007), core::mem::transmute(&__midl__iwiadrvitem0008)) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0009.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindChildItemByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0010: core::mem::MaybeUninit<windows_core::BSTR>, __midl__iwiadrvitem0011: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::FindChildItemByName(this, core::mem::transmute(&__midl__iwiadrvitem0010)) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0011.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0012: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetParentItem(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0012.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0013: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetFirstChildItem(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0013.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0014: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::GetNextSiblingItem(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0014.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0015: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaDrvItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaDrvItem_Impl::DumpItemData(this) {
                Ok(ok__) => {
                    __midl__iwiadrvitem0015.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemFlags: GetItemFlags::<Identity, OFFSET>,
            GetDeviceSpecContext: GetDeviceSpecContext::<Identity, OFFSET>,
            GetFullItemName: GetFullItemName::<Identity, OFFSET>,
            GetItemName: GetItemName::<Identity, OFFSET>,
            AddItemToFolder: AddItemToFolder::<Identity, OFFSET>,
            UnlinkItemTree: UnlinkItemTree::<Identity, OFFSET>,
            RemoveItemFromFolder: RemoveItemFromFolder::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            FindChildItemByName: FindChildItemByName::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
            GetFirstChildItem: GetFirstChildItem::<Identity, OFFSET>,
            GetNextSiblingItem: GetNextSiblingItem::<Identity, OFFSET>,
            DumpItemData: DumpItemData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDrvItem as windows_core::Interface>::IID
    }
}
pub trait IWiaErrorHandler_Impl: Sized {
    fn ReportStatus(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: Option<&IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>;
    fn GetStatusDescription(&self, lflags: i32, pwiaitem2: Option<&IWiaItem2>, hrstatus: windows_core::HRESULT) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IWiaErrorHandler {}
impl IWiaErrorHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaErrorHandler_Vtbl
    where
        Identity: IWiaErrorHandler_Impl,
    {
        unsafe extern "system" fn ReportStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT
        where
            Identity: IWiaErrorHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
        }
        unsafe extern "system" fn GetStatusDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaErrorHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaErrorHandler_Impl::GetStatusDescription(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pwiaitem2), core::mem::transmute_copy(&hrstatus)) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportStatus: ReportStatus::<Identity, OFFSET>,
            GetStatusDescription: GetStatusDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaErrorHandler as windows_core::Interface>::IID
    }
}
pub trait IWiaEventCallback_Impl: Sized {
    fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: u32, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut u32, ulreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaEventCallback {}
impl IWiaEventCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaEventCallback_Vtbl
    where
        Identity: IWiaEventCallback_Impl,
    {
        unsafe extern "system" fn ImageEventCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstreventdescription: core::mem::MaybeUninit<windows_core::BSTR>, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrdevicedescription: core::mem::MaybeUninit<windows_core::BSTR>, dwdevicetype: u32, bstrfullitemname: core::mem::MaybeUninit<windows_core::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> windows_core::HRESULT
        where
            Identity: IWiaEventCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaEventCallback_Impl::ImageEventCallback(this, core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstreventdescription), core::mem::transmute(&bstrdeviceid), core::mem::transmute(&bstrdevicedescription), core::mem::transmute_copy(&dwdevicetype), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&puleventtype), core::mem::transmute_copy(&ulreserved)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ImageEventCallback: ImageEventCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaEventCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaImageFilter_Impl: Sized {
    fn InitializeFilter(&self, pwiaitem2: Option<&IWiaItem2>, pwiatransfercallback: Option<&IWiaTransferCallback>) -> windows_core::Result<()>;
    fn SetNewCallback(&self, pwiatransfercallback: Option<&IWiaTransferCallback>) -> windows_core::Result<()>;
    fn FilterPreviewImage(&self, lflags: i32, pwiachilditem2: Option<&IWiaItem2>, inputimageextents: &super::super::Foundation::RECT, pinputstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn ApplyProperties(&self, pwiapropertystorage: Option<&IWiaPropertyStorage>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaImageFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaImageFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaImageFilter_Vtbl
    where
        Identity: IWiaImageFilter_Impl,
    {
        unsafe extern "system" fn InitializeFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaImageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaImageFilter_Impl::InitializeFilter(this, windows_core::from_raw_borrowed(&pwiaitem2), windows_core::from_raw_borrowed(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn SetNewCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaImageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaImageFilter_Impl::SetNewCallback(this, windows_core::from_raw_borrowed(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn FilterPreviewImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiachilditem2: *mut core::ffi::c_void, inputimageextents: super::super::Foundation::RECT, pinputstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaImageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaImageFilter_Impl::FilterPreviewImage(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pwiachilditem2), core::mem::transmute(&inputimageextents), windows_core::from_raw_borrowed(&pinputstream)).into()
        }
        unsafe extern "system" fn ApplyProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiapropertystorage: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaImageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaImageFilter_Impl::ApplyProperties(this, windows_core::from_raw_borrowed(&pwiapropertystorage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, OFFSET>,
            SetNewCallback: SetNewCallback::<Identity, OFFSET>,
            FilterPreviewImage: FilterPreviewImage::<Identity, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaImageFilter as windows_core::Interface>::IID
    }
}
pub trait IWiaItem_Impl: Sized {
    fn GetItemType(&self) -> windows_core::Result<i32>;
    fn AnalyzeItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn EnumChildItems(&self) -> windows_core::Result<IEnumWiaItem>;
    fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn CreateChildItem(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn DeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn GetRootItem(&self) -> windows_core::Result<IWiaItem>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DumpDrvItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DumpTreeItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaItem {}
impl IWiaItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaItem_Vtbl
    where
        Identity: IWiaItem_Impl,
    {
        unsafe extern "system" fn GetItemType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::GetItemType(this) {
                Ok(ok__) => {
                    pitemtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem_Impl::AnalyzeItem(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn EnumChildItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::EnumChildItems(this) {
                Ok(ok__) => {
                    ppienumwiaitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateChildItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, bstrfullitemname: core::mem::MaybeUninit<windows_core::BSTR>, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::CreateChildItem(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                Ok(ok__) => {
                    ppiwiaitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid)) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrfullitemname: core::mem::MaybeUninit<windows_core::BSTR>, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname)) {
                Ok(ok__) => {
                    ppiwiaitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut Option<IWiaItem>) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem_Impl::DeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&plitemcount), core::mem::transmute_copy(&ppiwiaitem)).into()
        }
        unsafe extern "system" fn DeviceCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&piwiaitem)).into()
        }
        unsafe extern "system" fn GetRootItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::GetRootItem(this) {
                Ok(ok__) => {
                    ppiwiaitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppienumwia_dev_caps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::DumpItemData(this) {
                Ok(ok__) => {
                    bstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpDrvItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::DumpDrvItemData(this) {
                Ok(ok__) => {
                    bstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpTreeItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem_Impl::DumpTreeItemData(this) {
                Ok(ok__) => {
                    bstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT
        where
            Identity: IWiaItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem_Impl::Diagnostic(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, OFFSET>,
            AnalyzeItem: AnalyzeItem::<Identity, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            CreateChildItem: CreateChildItem::<Identity, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, OFFSET>,
            GetRootItem: GetRootItem::<Identity, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, OFFSET>,
            DumpItemData: DumpItemData::<Identity, OFFSET>,
            DumpDrvItemData: DumpDrvItemData::<Identity, OFFSET>,
            DumpTreeItemData: DumpTreeItemData::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItem as windows_core::Interface>::IID
    }
}
pub trait IWiaItem2_Impl: Sized {
    fn CreateChildItem(&self, litemflags: i32, lcreationflags: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn EnumChildItems(&self, pcategoryguid: *const windows_core::GUID) -> windows_core::Result<IEnumWiaItem2>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn GetItemCategory(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetItemType(&self) -> windows_core::Result<i32>;
    fn DeviceDlg(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut Option<IWiaItem2>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut Option<IWiaItem2>) -> windows_core::Result<()>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn CheckExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParentItem(&self) -> windows_core::Result<IWiaItem2>;
    fn GetRootItem(&self) -> windows_core::Result<IWiaItem2>;
    fn GetPreviewComponent(&self, lflags: i32) -> windows_core::Result<IWiaPreview>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaItem2 {}
impl IWiaItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaItem2_Vtbl
    where
        Identity: IWiaItem2_Impl,
    {
        unsafe extern "system" fn CreateChildItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::CreateChildItem(this, core::mem::transmute_copy(&litemflags), core::mem::transmute_copy(&lcreationflags), core::mem::transmute(&bstritemname)) {
                Ok(ok__) => {
                    ppiwiaitem2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem2_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn EnumChildItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryguid: *const windows_core::GUID, ppienumwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::EnumChildItems(this, core::mem::transmute_copy(&pcategoryguid)) {
                Ok(ok__) => {
                    ppienumwiaitem2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrfullitemname: core::mem::MaybeUninit<windows_core::BSTR>, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname)) {
                Ok(ok__) => {
                    ppiwiaitem2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemcategoryguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::GetItemCategory(this) {
                Ok(ok__) => {
                    pitemcategoryguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::GetItemType(this) {
                Ok(ok__) => {
                    pitemtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: core::mem::MaybeUninit<windows_core::BSTR>, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem2_Impl::DeviceDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn DeviceCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem2_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&ppiwiaitem2)).into()
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppienumwia_dev_caps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, riidextensioninterface: *const windows_core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem2_Impl::CheckExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&pbextensionexists)).into()
        }
        unsafe extern "system" fn GetExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem2_Impl::GetExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&ppout)).into()
        }
        unsafe extern "system" fn GetParentItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::GetParentItem(this) {
                Ok(ok__) => {
                    ppiwiaitem2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::GetRootItem(this) {
                Ok(ok__) => {
                    ppiwiaitem2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviewComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppwiapreview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::GetPreviewComponent(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppwiapreview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItem2_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid)) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT
        where
            Identity: IWiaItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItem2_Impl::Diagnostic(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateChildItem: CreateChildItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            GetItemCategory: GetItemCategory::<Identity, OFFSET>,
            GetItemType: GetItemType::<Identity, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, OFFSET>,
            CheckExtension: CheckExtension::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
            GetRootItem: GetRootItem::<Identity, OFFSET>,
            GetPreviewComponent: GetPreviewComponent::<Identity, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItem2 as windows_core::Interface>::IID
    }
}
pub trait IWiaItemExtras_Impl: Sized {
    fn GetExtendedErrorInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Escape(&self, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::Result<()>;
    fn CancelPendingIO(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaItemExtras {}
impl IWiaItemExtras_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaItemExtras_Vtbl
    where
        Identity: IWiaItemExtras_Impl,
    {
        unsafe extern "system" fn GetExtendedErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrerrortext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaItemExtras_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaItemExtras_Impl::GetExtendedErrorInfo(this) {
                Ok(ok__) => {
                    bstrerrortext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWiaItemExtras_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItemExtras_Impl::Escape(this, core::mem::transmute_copy(&dwescapecode), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdatasize)).into()
        }
        unsafe extern "system" fn CancelPendingIO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaItemExtras_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaItemExtras_Impl::CancelPendingIO(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetExtendedErrorInfo: GetExtendedErrorInfo::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            CancelPendingIO: CancelPendingIO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItemExtras as windows_core::Interface>::IID
    }
}
pub trait IWiaLog_Impl: Sized {
    fn InitializeLog(&self, hinstance: i32) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaLog {}
impl IWiaLog_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaLog_Vtbl
    where
        Identity: IWiaLog_Impl,
    {
        unsafe extern "system" fn InitializeLog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: i32) -> windows_core::HRESULT
        where
            Identity: IWiaLog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLog_Impl::InitializeLog(this, core::mem::transmute_copy(&hinstance)).into()
        }
        unsafe extern "system" fn hResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWiaLog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLog_Impl::hResult(this, core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn Log<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaLog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLog_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeLog: InitializeLog::<Identity, OFFSET>,
            hResult: hResult::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaLog as windows_core::Interface>::IID
    }
}
pub trait IWiaLogEx_Impl: Sized {
    fn InitializeLogEx(&self, hinstance: *const u8) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn hResultEx(&self, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn LogEx(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaLogEx {}
impl IWiaLogEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaLogEx_Vtbl
    where
        Identity: IWiaLogEx_Impl,
    {
        unsafe extern "system" fn InitializeLogEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: *const u8) -> windows_core::HRESULT
        where
            Identity: IWiaLogEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLogEx_Impl::InitializeLogEx(this, core::mem::transmute_copy(&hinstance)).into()
        }
        unsafe extern "system" fn hResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWiaLogEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLogEx_Impl::hResult(this, core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn Log<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaLogEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLogEx_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
        }
        unsafe extern "system" fn hResultEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWiaLogEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLogEx_Impl::hResultEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn LogEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaLogEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaLogEx_Impl::LogEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeLogEx: InitializeLogEx::<Identity, OFFSET>,
            hResult: hResult::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
            hResultEx: hResultEx::<Identity, OFFSET>,
            LogEx: LogEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaLogEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWiaMiniDrv_Impl: Sized {
    fn drvInitializeWia(&self, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: &windows_core::BSTR, __midl__iwiaminidrv0003: &windows_core::BSTR, __midl__iwiaminidrv0004: Option<&windows_core::IUnknown>, __midl__iwiaminidrv0005: Option<&windows_core::IUnknown>, __midl__iwiaminidrv0006: *mut Option<IWiaDrvItem>, __midl__iwiaminidrv0007: *mut Option<windows_core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> windows_core::Result<()>;
    fn drvAcquireItemData(&self, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> windows_core::Result<()>;
    fn drvInitItemProperties(&self, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> windows_core::Result<i32>;
    fn drvValidateItemProperties(&self, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<i32>;
    fn drvWriteItemProperties(&self, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> windows_core::Result<i32>;
    fn drvReadItemProperties(&self, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<i32>;
    fn drvLockWiaDevice(&self, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> windows_core::Result<i32>;
    fn drvUnLockWiaDevice(&self, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> windows_core::Result<i32>;
    fn drvAnalyzeItem(&self, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> windows_core::Result<()>;
    fn drvGetDeviceErrorStr(&self, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> windows_core::Result<()>;
    fn drvDeviceCommand(&self, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const windows_core::GUID, __midl__iwiaminidrv0046: *mut Option<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> windows_core::Result<()>;
    fn drvGetCapabilities(&self, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> windows_core::Result<()>;
    fn drvDeleteItem(&self, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> windows_core::Result<i32>;
    fn drvFreeDrvItemContext(&self, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> windows_core::Result<i32>;
    fn drvGetWiaFormatInfo(&self, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> windows_core::Result<()>;
    fn drvNotifyPnpEvent(&self, peventguid: *const windows_core::GUID, bstrdeviceid: &windows_core::BSTR, ulreserved: u32) -> windows_core::Result<()>;
    fn drvUnInitializeWia(&self, __midl__iwiaminidrv0064: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWiaMiniDrv {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWiaMiniDrv_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaMiniDrv_Vtbl
    where
        Identity: IWiaMiniDrv_Impl,
    {
        unsafe extern "system" fn drvInitializeWia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: core::mem::MaybeUninit<windows_core::BSTR>, __midl__iwiaminidrv0003: core::mem::MaybeUninit<windows_core::BSTR>, __midl__iwiaminidrv0004: *mut core::ffi::c_void, __midl__iwiaminidrv0005: *mut core::ffi::c_void, __midl__iwiaminidrv0006: *mut *mut core::ffi::c_void, __midl__iwiaminidrv0007: *mut *mut core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvInitializeWia(
                this,
                core::mem::transmute_copy(&__midl__iwiaminidrv0000),
                core::mem::transmute_copy(&__midl__iwiaminidrv0001),
                core::mem::transmute(&__midl__iwiaminidrv0002),
                core::mem::transmute(&__midl__iwiaminidrv0003),
                windows_core::from_raw_borrowed(&__midl__iwiaminidrv0004),
                windows_core::from_raw_borrowed(&__midl__iwiaminidrv0005),
                core::mem::transmute_copy(&__midl__iwiaminidrv0006),
                core::mem::transmute_copy(&__midl__iwiaminidrv0007),
                core::mem::transmute_copy(&__midl__iwiaminidrv0008),
            )
            .into()
        }
        unsafe extern "system" fn drvAcquireItemData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvAcquireItemData(this, core::mem::transmute_copy(&__midl__iwiaminidrv0009), core::mem::transmute_copy(&__midl__iwiaminidrv0010), core::mem::transmute_copy(&__midl__iwiaminidrv0011), core::mem::transmute_copy(&__midl__iwiaminidrv0012)).into()
        }
        unsafe extern "system" fn drvInitItemProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvInitItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0013), core::mem::transmute_copy(&__midl__iwiaminidrv0014)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0015.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvValidateItemProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvValidateItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0016), core::mem::transmute_copy(&__midl__iwiaminidrv0017), core::mem::transmute_copy(&__midl__iwiaminidrv0018), core::mem::transmute_copy(&__midl__iwiaminidrv0019)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0020.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvWriteItemProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvWriteItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0021), core::mem::transmute_copy(&__midl__iwiaminidrv0022), core::mem::transmute_copy(&__midl__iwiaminidrv0023)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0024.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvReadItemProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvReadItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0025), core::mem::transmute_copy(&__midl__iwiaminidrv0026), core::mem::transmute_copy(&__midl__iwiaminidrv0027), core::mem::transmute_copy(&__midl__iwiaminidrv0028)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0029.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvLockWiaDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvLockWiaDevice(this, core::mem::transmute_copy(&__midl__iwiaminidrv0030), core::mem::transmute_copy(&__midl__iwiaminidrv0031)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0032.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvUnLockWiaDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvUnLockWiaDevice(this, core::mem::transmute_copy(&__midl__iwiaminidrv0033), core::mem::transmute_copy(&__midl__iwiaminidrv0034)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0035.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvAnalyzeItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvAnalyzeItem(this, core::mem::transmute_copy(&__midl__iwiaminidrv0036), core::mem::transmute_copy(&__midl__iwiaminidrv0037), core::mem::transmute_copy(&__midl__iwiaminidrv0038)).into()
        }
        unsafe extern "system" fn drvGetDeviceErrorStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvGetDeviceErrorStr(this, core::mem::transmute_copy(&__midl__iwiaminidrv0039), core::mem::transmute_copy(&__midl__iwiaminidrv0040), core::mem::transmute_copy(&__midl__iwiaminidrv0041), core::mem::transmute_copy(&__midl__iwiaminidrv0042)).into()
        }
        unsafe extern "system" fn drvDeviceCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const windows_core::GUID, __midl__iwiaminidrv0046: *mut *mut core::ffi::c_void, __midl__iwiaminidrv0047: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvDeviceCommand(this, core::mem::transmute_copy(&__midl__iwiaminidrv0043), core::mem::transmute_copy(&__midl__iwiaminidrv0044), core::mem::transmute_copy(&__midl__iwiaminidrv0045), core::mem::transmute_copy(&__midl__iwiaminidrv0046), core::mem::transmute_copy(&__midl__iwiaminidrv0047)).into()
        }
        unsafe extern "system" fn drvGetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvGetCapabilities(this, core::mem::transmute_copy(&__midl__iwiaminidrv0048), core::mem::transmute_copy(&__midl__iwiaminidrv0049), core::mem::transmute_copy(&__midl__iwiaminidrv0050), core::mem::transmute_copy(&__midl__iwiaminidrv0051), core::mem::transmute_copy(&__midl__iwiaminidrv0052)).into()
        }
        unsafe extern "system" fn drvDeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvDeleteItem(this, core::mem::transmute_copy(&__midl__iwiaminidrv0053), core::mem::transmute_copy(&__midl__iwiaminidrv0054)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0055.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvFreeDrvItemContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrv_Impl::drvFreeDrvItemContext(this, core::mem::transmute_copy(&__midl__iwiaminidrv0056), core::mem::transmute_copy(&__midl__iwiaminidrv0057)) {
                Ok(ok__) => {
                    __midl__iwiaminidrv0058.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn drvGetWiaFormatInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvGetWiaFormatInfo(this, core::mem::transmute_copy(&__midl__iwiaminidrv0059), core::mem::transmute_copy(&__midl__iwiaminidrv0060), core::mem::transmute_copy(&__midl__iwiaminidrv0061), core::mem::transmute_copy(&__midl__iwiaminidrv0062), core::mem::transmute_copy(&__midl__iwiaminidrv0063)).into()
        }
        unsafe extern "system" fn drvNotifyPnpEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, ulreserved: u32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvNotifyPnpEvent(this, core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&ulreserved)).into()
        }
        unsafe extern "system" fn drvUnInitializeWia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrv_Impl::drvUnInitializeWia(this, core::mem::transmute_copy(&__midl__iwiaminidrv0064)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            drvInitializeWia: drvInitializeWia::<Identity, OFFSET>,
            drvAcquireItemData: drvAcquireItemData::<Identity, OFFSET>,
            drvInitItemProperties: drvInitItemProperties::<Identity, OFFSET>,
            drvValidateItemProperties: drvValidateItemProperties::<Identity, OFFSET>,
            drvWriteItemProperties: drvWriteItemProperties::<Identity, OFFSET>,
            drvReadItemProperties: drvReadItemProperties::<Identity, OFFSET>,
            drvLockWiaDevice: drvLockWiaDevice::<Identity, OFFSET>,
            drvUnLockWiaDevice: drvUnLockWiaDevice::<Identity, OFFSET>,
            drvAnalyzeItem: drvAnalyzeItem::<Identity, OFFSET>,
            drvGetDeviceErrorStr: drvGetDeviceErrorStr::<Identity, OFFSET>,
            drvDeviceCommand: drvDeviceCommand::<Identity, OFFSET>,
            drvGetCapabilities: drvGetCapabilities::<Identity, OFFSET>,
            drvDeleteItem: drvDeleteItem::<Identity, OFFSET>,
            drvFreeDrvItemContext: drvFreeDrvItemContext::<Identity, OFFSET>,
            drvGetWiaFormatInfo: drvGetWiaFormatInfo::<Identity, OFFSET>,
            drvNotifyPnpEvent: drvNotifyPnpEvent::<Identity, OFFSET>,
            drvUnInitializeWia: drvUnInitializeWia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaMiniDrv as windows_core::Interface>::IID
    }
}
pub trait IWiaMiniDrvCallBack_Impl: Sized {
    fn MiniDrvCallback(&self, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaMiniDrvCallBack {}
impl IWiaMiniDrvCallBack_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaMiniDrvCallBack_Vtbl
    where
        Identity: IWiaMiniDrvCallBack_Impl,
    {
        unsafe extern "system" fn MiniDrvCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrvCallBack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrvCallBack_Impl::MiniDrvCallback(this, core::mem::transmute_copy(&lreason), core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lpercentcomplete), core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&llength), core::mem::transmute_copy(&ptranctx), core::mem::transmute_copy(&lreserved)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MiniDrvCallback: MiniDrvCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaMiniDrvCallBack as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaMiniDrvTransferCallback_Impl: Sized {
    fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SendMessage(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaMiniDrvTransferCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaMiniDrvTransferCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaMiniDrvTransferCallback_Vtbl
    where
        Identity: IWiaMiniDrvTransferCallback_Impl,
    {
        unsafe extern "system" fn GetNextStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, bstrfullitemname: core::mem::MaybeUninit<windows_core::BSTR>, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrvTransferCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaMiniDrvTransferCallback_Impl::GetNextStream(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                Ok(ok__) => {
                    ppistream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT
        where
            Identity: IWiaMiniDrvTransferCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaMiniDrvTransferCallback_Impl::SendMessage(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiatransferparams)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNextStream: GetNextStream::<Identity, OFFSET>,
            SendMessage: SendMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaMiniDrvTransferCallback as windows_core::Interface>::IID
    }
}
pub trait IWiaNotifyDevMgr_Impl: Sized {
    fn NewDeviceArrival(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaNotifyDevMgr {}
impl IWiaNotifyDevMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaNotifyDevMgr_Vtbl
    where
        Identity: IWiaNotifyDevMgr_Impl,
    {
        unsafe extern "system" fn NewDeviceArrival<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaNotifyDevMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaNotifyDevMgr_Impl::NewDeviceArrival(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewDeviceArrival: NewDeviceArrival::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaNotifyDevMgr as windows_core::Interface>::IID
    }
}
pub trait IWiaPreview_Impl: Sized {
    fn GetNewPreview(&self, lflags: i32, pwiaitem2: Option<&IWiaItem2>, pwiatransfercallback: Option<&IWiaTransferCallback>) -> windows_core::Result<()>;
    fn UpdatePreview(&self, lflags: i32, pchildwiaitem2: Option<&IWiaItem2>, pwiatransfercallback: Option<&IWiaTransferCallback>) -> windows_core::Result<()>;
    fn DetectRegions(&self, lflags: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWiaPreview {}
impl IWiaPreview_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaPreview_Vtbl
    where
        Identity: IWiaPreview_Impl,
    {
        unsafe extern "system" fn GetNewPreview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPreview_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPreview_Impl::GetNewPreview(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pwiaitem2), windows_core::from_raw_borrowed(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn UpdatePreview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pchildwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPreview_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPreview_Impl::UpdatePreview(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pchildwiaitem2), windows_core::from_raw_borrowed(&pwiatransfercallback)).into()
        }
        unsafe extern "system" fn DetectRegions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IWiaPreview_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPreview_Impl::DetectRegions(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPreview_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPreview_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNewPreview: GetNewPreview::<Identity, OFFSET>,
            UpdatePreview: UpdatePreview::<Identity, OFFSET>,
            DetectRegions: DetectRegions::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaPreview as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWiaPropertyStorage_Impl: Sized {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const windows_core::PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> windows_core::Result<()>;
    fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWiaPropertyStorage {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWiaPropertyStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaPropertyStorage_Vtbl
    where
        Identity: IWiaPropertyStorage_Impl,
    {
        unsafe extern "system" fn ReadMultiple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn WriteMultiple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, propidnamefirst: u32) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
        }
        unsafe extern "system" fn DeleteMultiple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn WritePropertyNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::Revert(this).into()
        }
        unsafe extern "system" fn Enum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaPropertyStorage_Impl::Enum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
        }
        unsafe extern "system" fn SetClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Stat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::GetPropertyAttributes(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgflags), core::mem::transmute_copy(&rgpropvar)).into()
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnumprops: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaPropertyStorage_Impl::GetCount(this) {
                Ok(ok__) => {
                    pulnumprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::GetPropertyStream(this, core::mem::transmute_copy(&pcompatibilityid), core::mem::transmute_copy(&ppistream)).into()
        }
        unsafe extern "system" fn SetPropertyStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatibilityid: *mut windows_core::GUID, pistream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaPropertyStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaPropertyStorage_Impl::SetPropertyStream(this, core::mem::transmute_copy(&pcompatibilityid), windows_core::from_raw_borrowed(&pistream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            SetTimes: SetTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetPropertyStream: GetPropertyStream::<Identity, OFFSET>,
            SetPropertyStream: SetPropertyStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaPropertyStorage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaSegmentationFilter_Impl: Sized {
    fn DetectRegions(&self, lflags: i32, pinputstream: Option<&super::super::System::Com::IStream>, pwiaitem2: Option<&IWiaItem2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaSegmentationFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaSegmentationFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaSegmentationFilter_Vtbl
    where
        Identity: IWiaSegmentationFilter_Impl,
    {
        unsafe extern "system" fn DetectRegions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pinputstream: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaSegmentationFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaSegmentationFilter_Impl::DetectRegions(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pinputstream), windows_core::from_raw_borrowed(&pwiaitem2)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DetectRegions: DetectRegions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaSegmentationFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransfer_Impl: Sized {
    fn Download(&self, lflags: i32, piwiatransfercallback: Option<&IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Upload(&self, lflags: i32, psource: Option<&super::super::System::Com::IStream>, piwiatransfercallback: Option<&IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaTransfer {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaTransfer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaTransfer_Vtbl
    where
        Identity: IWiaTransfer_Impl,
    {
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaTransfer_Impl::Download(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&piwiatransfercallback)).into()
        }
        unsafe extern "system" fn Upload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, psource: *mut core::ffi::c_void, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaTransfer_Impl::Upload(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&psource), windows_core::from_raw_borrowed(&piwiatransfercallback)).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaTransfer_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaTransfer_Impl::EnumWIA_FORMAT_INFO(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Download: Download::<Identity, OFFSET>,
            Upload: Upload::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            EnumWIA_FORMAT_INFO: EnumWIA_FORMAT_INFO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaTransfer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransferCallback_Impl: Sized {
    fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()>;
    fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaTransferCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IWiaTransferCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaTransferCallback_Vtbl
    where
        Identity: IWiaTransferCallback_Impl,
    {
        unsafe extern "system" fn TransferCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT
        where
            Identity: IWiaTransferCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaTransferCallback_Impl::TransferCallback(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiatransferparams)).into()
        }
        unsafe extern "system" fn GetNextStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, bstrfullitemname: core::mem::MaybeUninit<windows_core::BSTR>, ppdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaTransferCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaTransferCallback_Impl::GetNextStream(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                Ok(ok__) => {
                    ppdestination.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransferCallback: TransferCallback::<Identity, OFFSET>,
            GetNextStream: GetNextStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaTransferCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension_Impl: Sized {
    fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA) -> windows_core::Result<()>;
    fn GetDeviceIcon(&self, bstrdeviceid: &windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::Result<()>;
    fn GetDeviceBitmapLogo(&self, bstrdeviceid: &windows_core::BSTR, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWiaUIExtension {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWiaUIExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaUIExtension_Vtbl
    where
        Identity: IWiaUIExtension_Impl,
    {
        unsafe extern "system" fn DeviceDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> windows_core::HRESULT
        where
            Identity: IWiaUIExtension_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaUIExtension_Impl::DeviceDialog(this, core::mem::transmute_copy(&pdevicedialogdata)).into()
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::HRESULT
        where
            Identity: IWiaUIExtension_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaUIExtension_Impl::GetDeviceIcon(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&phicon), core::mem::transmute_copy(&nsize)).into()
        }
        unsafe extern "system" fn GetDeviceBitmapLogo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> windows_core::HRESULT
        where
            Identity: IWiaUIExtension_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaUIExtension_Impl::GetDeviceBitmapLogo(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&phbitmap), core::mem::transmute_copy(&nmaxwidth), core::mem::transmute_copy(&nmaxheight)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceDialog: DeviceDialog::<Identity, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, OFFSET>,
            GetDeviceBitmapLogo: GetDeviceBitmapLogo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaUIExtension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IWiaUIExtension2_Impl: Sized {
    fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> windows_core::Result<()>;
    fn GetDeviceIcon(&self, bstrdeviceid: &windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IWiaUIExtension2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IWiaUIExtension2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaUIExtension2_Vtbl
    where
        Identity: IWiaUIExtension2_Impl,
    {
        unsafe extern "system" fn DeviceDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> windows_core::HRESULT
        where
            Identity: IWiaUIExtension2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaUIExtension2_Impl::DeviceDialog(this, core::mem::transmute_copy(&pdevicedialogdata)).into()
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: core::mem::MaybeUninit<windows_core::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::HRESULT
        where
            Identity: IWiaUIExtension2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaUIExtension2_Impl::GetDeviceIcon(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&phicon), core::mem::transmute_copy(&nsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceDialog: DeviceDialog::<Identity, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaUIExtension2 as windows_core::Interface>::IID
    }
}
pub trait IWiaVideo_Impl: Sized {
    fn PreviewVisible(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetPreviewVisible(&self, bpreviewvisible: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ImagesDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetImagesDirectory(&self, bstrimagedirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CreateVideoByWiaDevID(&self, bstrwiadeviceid: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CreateVideoByDevNum(&self, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CreateVideoByName(&self, bstrfriendlyname: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DestroyVideo(&self) -> windows_core::Result<()>;
    fn Play(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn TakePicture(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ResizeVideo(&self, bstretchtofitparent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetCurrentState(&self) -> windows_core::Result<WIAVIDEO_STATE>;
}
impl windows_core::RuntimeName for IWiaVideo {}
impl IWiaVideo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWiaVideo_Vtbl
    where
        Identity: IWiaVideo_Impl,
    {
        unsafe extern "system" fn PreviewVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpreviewvisible: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaVideo_Impl::PreviewVisible(this) {
                Ok(ok__) => {
                    pbpreviewvisible.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpreviewvisible: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::SetPreviewVisible(this, core::mem::transmute_copy(&bpreviewvisible)).into()
        }
        unsafe extern "system" fn ImagesDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrimagedirectory: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaVideo_Impl::ImagesDirectory(this) {
                Ok(ok__) => {
                    pbstrimagedirectory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImagesDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrimagedirectory: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::SetImagesDirectory(this, core::mem::transmute(&bstrimagedirectory)).into()
        }
        unsafe extern "system" fn CreateVideoByWiaDevID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrwiadeviceid: core::mem::MaybeUninit<windows_core::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::CreateVideoByWiaDevID(this, core::mem::transmute(&bstrwiadeviceid), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bstretchtofitparent), core::mem::transmute_copy(&bautobeginplayback)).into()
        }
        unsafe extern "system" fn CreateVideoByDevNum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::CreateVideoByDevNum(this, core::mem::transmute_copy(&uidevicenumber), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bstretchtofitparent), core::mem::transmute_copy(&bautobeginplayback)).into()
        }
        unsafe extern "system" fn CreateVideoByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::CreateVideoByName(this, core::mem::transmute(&bstrfriendlyname), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bstretchtofitparent), core::mem::transmute_copy(&bautobeginplayback)).into()
        }
        unsafe extern "system" fn DestroyVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::DestroyVideo(this).into()
        }
        unsafe extern "system" fn Play<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::Play(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::Pause(this).into()
        }
        unsafe extern "system" fn TakePicture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnewimagefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaVideo_Impl::TakePicture(this) {
                Ok(ok__) => {
                    pbstrnewimagefilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstretchtofitparent: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWiaVideo_Impl::ResizeVideo(this, core::mem::transmute_copy(&bstretchtofitparent)).into()
        }
        unsafe extern "system" fn GetCurrentState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> windows_core::HRESULT
        where
            Identity: IWiaVideo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWiaVideo_Impl::GetCurrentState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreviewVisible: PreviewVisible::<Identity, OFFSET>,
            SetPreviewVisible: SetPreviewVisible::<Identity, OFFSET>,
            ImagesDirectory: ImagesDirectory::<Identity, OFFSET>,
            SetImagesDirectory: SetImagesDirectory::<Identity, OFFSET>,
            CreateVideoByWiaDevID: CreateVideoByWiaDevID::<Identity, OFFSET>,
            CreateVideoByDevNum: CreateVideoByDevNum::<Identity, OFFSET>,
            CreateVideoByName: CreateVideoByName::<Identity, OFFSET>,
            DestroyVideo: DestroyVideo::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            TakePicture: TakePicture::<Identity, OFFSET>,
            ResizeVideo: ResizeVideo::<Identity, OFFSET>,
            GetCurrentState: GetCurrentState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaVideo as windows_core::Interface>::IID
    }
}

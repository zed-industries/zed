pub trait IActiveXUIHandlerSite_Impl: Sized {
    fn CreateScrollableContextMenu(&self) -> windows_core::Result<IScrollableContextMenu>;
    fn PickFileAndGetResult(&self, filepicker: Option<&windows_core::IUnknown>, allowmultipleselections: super::super::Foundation::BOOL) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IActiveXUIHandlerSite {}
impl IActiveXUIHandlerSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActiveXUIHandlerSite_Vtbl
    where
        Identity: IActiveXUIHandlerSite_Impl,
    {
        unsafe extern "system" fn CreateScrollableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollablecontextmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActiveXUIHandlerSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActiveXUIHandlerSite_Impl::CreateScrollableContextMenu(this) {
                Ok(ok__) => {
                    scrollablecontextmenu.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickFileAndGetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepicker: *mut core::ffi::c_void, allowmultipleselections: super::super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActiveXUIHandlerSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActiveXUIHandlerSite_Impl::PickFileAndGetResult(this, windows_core::from_raw_borrowed(&filepicker), core::mem::transmute_copy(&allowmultipleselections)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateScrollableContextMenu: CreateScrollableContextMenu::<Identity, OFFSET>,
            PickFileAndGetResult: PickFileAndGetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite as windows_core::Interface>::IID
    }
}
pub trait IActiveXUIHandlerSite2_Impl: Sized {
    fn AddSuspensionExemption(&self) -> windows_core::Result<u64>;
    fn RemoveSuspensionExemption(&self, ullcookie: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IActiveXUIHandlerSite2 {}
impl IActiveXUIHandlerSite2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActiveXUIHandlerSite2_Vtbl
    where
        Identity: IActiveXUIHandlerSite2_Impl,
    {
        unsafe extern "system" fn AddSuspensionExemption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullcookie: *mut u64) -> windows_core::HRESULT
        where
            Identity: IActiveXUIHandlerSite2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActiveXUIHandlerSite2_Impl::AddSuspensionExemption(this) {
                Ok(ok__) => {
                    pullcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspensionExemption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcookie: u64) -> windows_core::HRESULT
        where
            Identity: IActiveXUIHandlerSite2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActiveXUIHandlerSite2_Impl::RemoveSuspensionExemption(this, core::mem::transmute_copy(&ullcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddSuspensionExemption: AddSuspensionExemption::<Identity, OFFSET>,
            RemoveSuspensionExemption: RemoveSuspensionExemption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite2 as windows_core::Interface>::IID
    }
}
pub trait IActiveXUIHandlerSite3_Impl: Sized {
    fn MessageBoxW(&self, hwnd: super::super::Foundation::HWND, text: &windows_core::PCWSTR, caption: &windows_core::PCWSTR, r#type: u32) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IActiveXUIHandlerSite3 {}
impl IActiveXUIHandlerSite3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActiveXUIHandlerSite3_Vtbl
    where
        Identity: IActiveXUIHandlerSite3_Impl,
    {
        unsafe extern "system" fn MessageBoxW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, text: windows_core::PCWSTR, caption: windows_core::PCWSTR, r#type: u32, result: *mut i32) -> windows_core::HRESULT
        where
            Identity: IActiveXUIHandlerSite3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActiveXUIHandlerSite3_Impl::MessageBoxW(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&text), core::mem::transmute(&caption), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MessageBoxW: MessageBoxW::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAnchorClick_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProcOnClick(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAnchorClick {}
#[cfg(feature = "Win32_System_Com")]
impl IAnchorClick_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAnchorClick_Vtbl
    where
        Identity: IAnchorClick_Impl,
    {
        unsafe extern "system" fn ProcOnClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAnchorClick_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAnchorClick_Impl::ProcOnClick(this).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ProcOnClick: ProcOnClick::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnchorClick as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionSite_Impl: Sized {
    fn GetAudioSessionGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn OnAudioStreamCreated(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnAudioStreamDestroyed(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSessionSite {}
impl IAudioSessionSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionSite_Vtbl
    where
        Identity: IAudioSessionSite_Impl,
    {
        unsafe extern "system" fn GetAudioSessionGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionSite_Impl::GetAudioSessionGuid(this) {
                Ok(ok__) => {
                    audiosessionguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAudioStreamCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioSessionSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionSite_Impl::OnAudioStreamCreated(this, core::mem::transmute(&endpointid)).into()
        }
        unsafe extern "system" fn OnAudioStreamDestroyed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioSessionSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionSite_Impl::OnAudioStreamDestroyed(this, core::mem::transmute(&endpointid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAudioSessionGuid: GetAudioSessionGuid::<Identity, OFFSET>,
            OnAudioStreamCreated: OnAudioStreamCreated::<Identity, OFFSET>,
            OnAudioStreamDestroyed: OnAudioStreamDestroyed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionSite as windows_core::Interface>::IID
    }
}
pub trait ICaretPositionProvider_Impl: Sized {
    fn GetCaretPosition(&self, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICaretPositionProvider {}
impl ICaretPositionProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICaretPositionProvider_Vtbl
    where
        Identity: ICaretPositionProvider_Impl,
    {
        unsafe extern "system" fn GetCaretPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> windows_core::HRESULT
        where
            Identity: ICaretPositionProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICaretPositionProvider_Impl::GetCaretPosition(this, core::mem::transmute_copy(&pptcaret), core::mem::transmute_copy(&pflheight)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCaretPosition: GetCaretPosition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICaretPositionProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDeviceRect_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDeviceRect {}
#[cfg(feature = "Win32_System_Com")]
impl IDeviceRect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeviceRect_Vtbl
    where
        Identity: IDeviceRect_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceRect as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDithererImpl_Impl: Sized {
    fn SetDestColorTable(&self, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::Result<()>;
    fn SetEventSink(&self, peventsink: Option<&IImageDecodeEventSink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDithererImpl {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDithererImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDithererImpl_Vtbl
    where
        Identity: IDithererImpl_Impl,
    {
        unsafe extern "system" fn SetDestColorTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::HRESULT
        where
            Identity: IDithererImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDithererImpl_Impl::SetDestColorTable(this, core::mem::transmute_copy(&ncolors), core::mem::transmute_copy(&prgbcolors)).into()
        }
        unsafe extern "system" fn SetEventSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDithererImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDithererImpl_Impl::SetEventSink(this, windows_core::from_raw_borrowed(&peventsink)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDestColorTable: SetDestColorTable::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDithererImpl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadBehavior_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn startDownload(&self, bstrurl: &windows_core::BSTR, pdispcallback: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadBehavior {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadBehavior_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadBehavior_Vtbl
    where
        Identity: IDownloadBehavior_Impl,
    {
        unsafe extern "system" fn startDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, pdispcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDownloadBehavior_Impl::startDownload(this, core::mem::transmute(&bstrurl), windows_core::from_raw_borrowed(&pdispcallback)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), startDownload: startDownload::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadBehavior as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDownloadManager_Impl: Sized {
    fn Download(&self, pmk: Option<&super::super::System::Com::IMoniker>, pbc: Option<&super::super::System::Com::IBindCtx>, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: &windows_core::PCWSTR, pszredir: &windows_core::PCWSTR, uicp: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IDownloadManager {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IDownloadManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadManager_Vtbl
    where
        Identity: IDownloadManager_Impl,
    {
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: windows_core::PCWSTR, pszredir: windows_core::PCWSTR, uicp: u32) -> windows_core::HRESULT
        where
            Identity: IDownloadManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDownloadManager_Impl::Download(this, windows_core::from_raw_borrowed(&pmk), windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&dwbindverb), core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute(&pszheaders), core::mem::transmute(&pszredir), core::mem::transmute_copy(&uicp)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Download: Download::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadManager as windows_core::Interface>::IID
    }
}
pub trait IEnumManagerFrames_Impl: Sized {
    fn Next(&self, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<u32>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumManagerFrames>;
}
impl windows_core::RuntimeName for IEnumManagerFrames {}
impl IEnumManagerFrames_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumManagerFrames_Vtbl
    where
        Identity: IEnumManagerFrames_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumManagerFrames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumManagerFrames_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppwindows), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumManagerFrames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumManagerFrames_Impl::Count(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumManagerFrames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumManagerFrames_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumManagerFrames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumManagerFrames_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumManagerFrames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumManagerFrames_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumManagerFrames as windows_core::Interface>::IID
    }
}
pub trait IEnumOpenServiceActivity_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOpenServiceActivity>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivity>;
}
impl windows_core::RuntimeName for IEnumOpenServiceActivity {}
impl IEnumOpenServiceActivity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumOpenServiceActivity_Vtbl
    where
        Identity: IEnumOpenServiceActivity_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOpenServiceActivity_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOpenServiceActivity_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOpenServiceActivity_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumOpenServiceActivity_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOpenServiceActivity as windows_core::Interface>::IID
    }
}
pub trait IEnumOpenServiceActivityCategory_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOpenServiceActivityCategory>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivityCategory>;
}
impl windows_core::RuntimeName for IEnumOpenServiceActivityCategory {}
impl IEnumOpenServiceActivityCategory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumOpenServiceActivityCategory_Vtbl
    where
        Identity: IEnumOpenServiceActivityCategory_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOpenServiceActivityCategory_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOpenServiceActivityCategory_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOpenServiceActivityCategory_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumOpenServiceActivityCategory_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOpenServiceActivityCategory as windows_core::Interface>::IID
    }
}
pub trait IEnumSTATURL_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATURL>;
    fn SetFilter(&self, poszfilter: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumSTATURL {}
impl IEnumSTATURL_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSTATURL_Vtbl
    where
        Identity: IEnumSTATURL_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSTATURL_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATURL_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSTATURL_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATURL_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSTATURL_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATURL_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSTATURL_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSTATURL_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poszfilter: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSTATURL_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSTATURL_Impl::SetFilter(this, core::mem::transmute(&poszfilter), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATURL as windows_core::Interface>::IID
    }
}
pub trait IHTMLPersistData_Impl: Sized {
    fn save(&self, punk: Option<&windows_core::IUnknown>, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn load(&self, punk: Option<&windows_core::IUnknown>, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn queryType(&self, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl windows_core::RuntimeName for IHTMLPersistData {}
impl IHTMLPersistData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHTMLPersistData_Vtbl
    where
        Identity: IHTMLPersistData_Impl,
    {
        unsafe extern "system" fn save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ltype: i32, fcontinuebroacast: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLPersistData_Impl::save(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&ltype)) {
                Ok(ok__) => {
                    fcontinuebroacast.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ltype: i32, fdodefault: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLPersistData_Impl::load(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&ltype)) {
                Ok(ok__) => {
                    fdodefault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn queryType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltype: i32, pfsupportstype: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLPersistData_Impl::queryType(this, core::mem::transmute_copy(&ltype)) {
                Ok(ok__) => {
                    pfsupportstype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            save: save::<Identity, OFFSET>,
            load: load::<Identity, OFFSET>,
            queryType: queryType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHTMLPersistData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHTMLPersistDataOM_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn setAttribute(&self, name: &windows_core::BSTR, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHTMLPersistDataOM {}
#[cfg(feature = "Win32_System_Com")]
impl IHTMLPersistDataOM_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHTMLPersistDataOM_Vtbl
    where
        Identity: IHTMLPersistDataOM_Impl,
    {
        unsafe extern "system" fn XMLDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLPersistDataOM_Impl::XMLDocument(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLPersistDataOM_Impl::getAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLPersistDataOM_Impl::setAttribute(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHTMLPersistDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLPersistDataOM_Impl::removeAttribute(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHTMLPersistDataOM as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHTMLUserDataOM_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn save(&self, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn load(&self, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn setAttribute(&self, name: &windows_core::BSTR, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Setexpires(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn expires(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHTMLUserDataOM {}
#[cfg(feature = "Win32_System_Com")]
impl IHTMLUserDataOM_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHTMLUserDataOM_Vtbl
    where
        Identity: IHTMLUserDataOM_Impl,
    {
        unsafe extern "system" fn XMLDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLUserDataOM_Impl::XMLDocument(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLUserDataOM_Impl::save(this, core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLUserDataOM_Impl::load(this, core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLUserDataOM_Impl::getAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLUserDataOM_Impl::setAttribute(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLUserDataOM_Impl::removeAttribute(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Setexpires<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHTMLUserDataOM_Impl::Setexpires(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn expires<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHTMLUserDataOM_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHTMLUserDataOM_Impl::expires(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, OFFSET>,
            save: save::<Identity, OFFSET>,
            load: load::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
            Setexpires: Setexpires::<Identity, OFFSET>,
            expires: expires::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHTMLUserDataOM as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHeaderFooter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn htmlHead(&self) -> windows_core::Result<windows_core::BSTR>;
    fn htmlFoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettextHead(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn textHead(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettextFoot(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn textFoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setpage(&self, v: u32) -> windows_core::Result<()>;
    fn page(&self) -> windows_core::Result<u32>;
    fn SetpageTotal(&self, v: u32) -> windows_core::Result<()>;
    fn pageTotal(&self) -> windows_core::Result<u32>;
    fn SetURL(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settitle(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetdateShort(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn dateShort(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetdateLong(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn dateLong(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettimeShort(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn timeShort(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettimeLong(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn timeLong(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHeaderFooter {}
#[cfg(feature = "Win32_System_Com")]
impl IHeaderFooter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHeaderFooter_Vtbl
    where
        Identity: IHeaderFooter_Impl,
    {
        unsafe extern "system" fn htmlHead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::htmlHead(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn htmlFoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::htmlFoot(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettextHead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SettextHead(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn textHead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::textHead(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettextFoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SettextFoot(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn textFoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::textFoot(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setpage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: u32) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::Setpage(this, core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn page<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::page(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetpageTotal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: u32) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SetpageTotal(this, core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn pageTotal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::pageTotal(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SetURL(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn URL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::URL(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::Settitle(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::title(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdateShort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SetdateShort(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn dateShort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::dateShort(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdateLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SetdateLong(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn dateLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::dateLong(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettimeShort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SettimeShort(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn timeShort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::timeShort(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettimeLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter_Impl::SettimeLong(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn timeLong<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter_Impl::timeLong(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            htmlHead: htmlHead::<Identity, OFFSET>,
            htmlFoot: htmlFoot::<Identity, OFFSET>,
            SettextHead: SettextHead::<Identity, OFFSET>,
            textHead: textHead::<Identity, OFFSET>,
            SettextFoot: SettextFoot::<Identity, OFFSET>,
            textFoot: textFoot::<Identity, OFFSET>,
            Setpage: Setpage::<Identity, OFFSET>,
            page: page::<Identity, OFFSET>,
            SetpageTotal: SetpageTotal::<Identity, OFFSET>,
            pageTotal: pageTotal::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            URL: URL::<Identity, OFFSET>,
            Settitle: Settitle::<Identity, OFFSET>,
            title: title::<Identity, OFFSET>,
            SetdateShort: SetdateShort::<Identity, OFFSET>,
            dateShort: dateShort::<Identity, OFFSET>,
            SetdateLong: SetdateLong::<Identity, OFFSET>,
            dateLong: dateLong::<Identity, OFFSET>,
            SettimeShort: SettimeShort::<Identity, OFFSET>,
            timeShort: timeShort::<Identity, OFFSET>,
            SettimeLong: SettimeLong::<Identity, OFFSET>,
            timeLong: timeLong::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHeaderFooter as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHeaderFooter2_Impl: Sized + IHeaderFooter_Impl {
    fn Setfont(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn font(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHeaderFooter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IHeaderFooter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHeaderFooter2_Vtbl
    where
        Identity: IHeaderFooter2_Impl,
    {
        unsafe extern "system" fn Setfont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHeaderFooter2_Impl::Setfont(this, core::mem::transmute(&v)).into()
        }
        unsafe extern "system" fn font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHeaderFooter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHeaderFooter2_Impl::font(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IHeaderFooter_Vtbl::new::<Identity, OFFSET>(), Setfont: Setfont::<Identity, OFFSET>, font: font::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHeaderFooter2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IHeaderFooter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHomePage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn navigateHomePage(&self) -> windows_core::Result<()>;
    fn setHomePage(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn isHomePage(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHomePage {}
#[cfg(feature = "Win32_System_Com")]
impl IHomePage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHomePage_Vtbl
    where
        Identity: IHomePage_Impl,
    {
        unsafe extern "system" fn navigateHomePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHomePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHomePage_Impl::navigateHomePage(this).into()
        }
        unsafe extern "system" fn setHomePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IHomePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHomePage_Impl::setHomePage(this, core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn isHomePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, p: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IHomePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHomePage_Impl::isHomePage(this, core::mem::transmute(&bstrurl)) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            navigateHomePage: navigateHomePage::<Identity, OFFSET>,
            setHomePage: setHomePage::<Identity, OFFSET>,
            isHomePage: isHomePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHomePage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IHomePageSetting_Impl: Sized {
    fn SetHomePage(&self, hwnd: super::super::Foundation::HWND, homepageuri: &windows_core::PCWSTR, brandingmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsHomePage(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetHomePageToBrowserDefault(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHomePageSetting {}
impl IHomePageSetting_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHomePageSetting_Vtbl
    where
        Identity: IHomePageSetting_Impl,
    {
        unsafe extern "system" fn SetHomePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, homepageuri: windows_core::PCWSTR, brandingmessage: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IHomePageSetting_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHomePageSetting_Impl::SetHomePage(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&homepageuri), core::mem::transmute(&brandingmessage)).into()
        }
        unsafe extern "system" fn IsHomePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, isdefault: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHomePageSetting_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHomePageSetting_Impl::IsHomePage(this, core::mem::transmute(&uri)) {
                Ok(ok__) => {
                    isdefault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePageToBrowserDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHomePageSetting_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHomePageSetting_Impl::SetHomePageToBrowserDefault(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHomePage: SetHomePage::<Identity, OFFSET>,
            IsHomePage: IsHomePage::<Identity, OFFSET>,
            SetHomePageToBrowserDefault: SetHomePageToBrowserDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHomePageSetting as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIEWebDriverManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExecuteCommand(&self, command: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IIEWebDriverManager {}
#[cfg(feature = "Win32_System_Com")]
impl IIEWebDriverManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIEWebDriverManager_Vtbl
    where
        Identity: IIEWebDriverManager_Impl,
    {
        unsafe extern "system" fn ExecuteCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: windows_core::PCWSTR, response: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IIEWebDriverManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIEWebDriverManager_Impl::ExecuteCommand(this, core::mem::transmute(&command)) {
                Ok(ok__) => {
                    response.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ExecuteCommand: ExecuteCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIEWebDriverManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIEWebDriverSite_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WindowOperation(&self, operationcode: u32, hwnd: u32) -> windows_core::Result<()>;
    fn DetachWebdriver(&self, punkwd: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCapabilityValue(&self, punkwd: Option<&windows_core::IUnknown>, capname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IIEWebDriverSite {}
#[cfg(feature = "Win32_System_Com")]
impl IIEWebDriverSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIEWebDriverSite_Vtbl
    where
        Identity: IIEWebDriverSite_Impl,
    {
        unsafe extern "system" fn WindowOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationcode: u32, hwnd: u32) -> windows_core::HRESULT
        where
            Identity: IIEWebDriverSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIEWebDriverSite_Impl::WindowOperation(this, core::mem::transmute_copy(&operationcode), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn DetachWebdriver<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkwd: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIEWebDriverSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIEWebDriverSite_Impl::DetachWebdriver(this, windows_core::from_raw_borrowed(&punkwd)).into()
        }
        unsafe extern "system" fn GetCapabilityValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkwd: *mut core::ffi::c_void, capname: windows_core::PCWSTR, capvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IIEWebDriverSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIEWebDriverSite_Impl::GetCapabilityValue(this, windows_core::from_raw_borrowed(&punkwd), core::mem::transmute(&capname)) {
                Ok(ok__) => {
                    capvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WindowOperation: WindowOperation::<Identity, OFFSET>,
            DetachWebdriver: DetachWebdriver::<Identity, OFFSET>,
            GetCapabilityValue: GetCapabilityValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIEWebDriverSite as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IImageDecodeEventSink_Impl: Sized {
    fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const windows_core::GUID, npasses: u32, dwhints: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn OnBitsComplete(&self) -> windows_core::Result<()>;
    fn OnDecodeComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnPalette(&self) -> windows_core::Result<()>;
    fn OnProgress(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IImageDecodeEventSink {}
impl IImageDecodeEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImageDecodeEventSink_Vtbl
    where
        Identity: IImageDecodeEventSink_Impl,
    {
        unsafe extern "system" fn GetSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nwidth: i32, nheight: i32, bfid: *const windows_core::GUID, npasses: u32, dwhints: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageDecodeEventSink_Impl::GetSurface(this, core::mem::transmute_copy(&nwidth), core::mem::transmute_copy(&nheight), core::mem::transmute_copy(&bfid), core::mem::transmute_copy(&npasses), core::mem::transmute_copy(&dwhints)) {
                Ok(ok__) => {
                    ppsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnBeginDecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeEventSink_Impl::OnBeginDecode(this, core::mem::transmute_copy(&pdwevents), core::mem::transmute_copy(&pnformats), core::mem::transmute_copy(&ppformats)).into()
        }
        unsafe extern "system" fn OnBitsComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeEventSink_Impl::OnBitsComplete(this).into()
        }
        unsafe extern "system" fn OnDecodeComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeEventSink_Impl::OnDecodeComplete(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn OnPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeEventSink_Impl::OnPalette(this).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeEventSink_Impl::OnProgress(this, core::mem::transmute_copy(&pbounds), core::mem::transmute_copy(&bcomplete)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSurface: GetSurface::<Identity, OFFSET>,
            OnBeginDecode: OnBeginDecode::<Identity, OFFSET>,
            OnBitsComplete: OnBitsComplete::<Identity, OFFSET>,
            OnDecodeComplete: OnDecodeComplete::<Identity, OFFSET>,
            OnPalette: OnPalette::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageDecodeEventSink as windows_core::Interface>::IID
    }
}
pub trait IImageDecodeEventSink2_Impl: Sized + IImageDecodeEventSink_Impl {
    fn IsAlphaPremultRequired(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IImageDecodeEventSink2 {}
impl IImageDecodeEventSink2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImageDecodeEventSink2_Vtbl
    where
        Identity: IImageDecodeEventSink2_Impl,
    {
        unsafe extern "system" fn IsAlphaPremultRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpremultalpha: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IImageDecodeEventSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageDecodeEventSink2_Impl::IsAlphaPremultRequired(this) {
                Ok(ok__) => {
                    pfpremultalpha.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IImageDecodeEventSink_Vtbl::new::<Identity, OFFSET>(), IsAlphaPremultRequired: IsAlphaPremultRequired::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageDecodeEventSink2 as windows_core::Interface>::IID || iid == &<IImageDecodeEventSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImageDecodeFilter_Impl: Sized {
    fn Initialize(&self, peventsink: Option<&IImageDecodeEventSink>) -> windows_core::Result<()>;
    fn Process(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Terminate(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IImageDecodeFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IImageDecodeFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImageDecodeFilter_Vtbl
    where
        Identity: IImageDecodeFilter_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageDecodeFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeFilter_Impl::Initialize(this, windows_core::from_raw_borrowed(&peventsink)).into()
        }
        unsafe extern "system" fn Process<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageDecodeFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeFilter_Impl::Process(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IImageDecodeFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageDecodeFilter_Impl::Terminate(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageDecodeFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIntelliForms_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Setenabled(&self, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IIntelliForms {}
#[cfg(feature = "Win32_System_Com")]
impl IIntelliForms_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIntelliForms_Vtbl
    where
        Identity: IIntelliForms_Impl,
    {
        unsafe extern "system" fn enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IIntelliForms_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIntelliForms_Impl::enabled(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setenabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IIntelliForms_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIntelliForms_Impl::Setenabled(this, core::mem::transmute_copy(&bval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            enabled: enabled::<Identity, OFFSET>,
            Setenabled: Setenabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIntelliForms as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IInternetExplorerManager_Impl: Sized {
    fn CreateObject(&self, dwconfig: u32, pszurl: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetExplorerManager {}
impl IInternetExplorerManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetExplorerManager_Vtbl
    where
        Identity: IInternetExplorerManager_Impl,
    {
        unsafe extern "system" fn CreateObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfig: u32, pszurl: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetExplorerManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetExplorerManager_Impl::CreateObject(this, core::mem::transmute_copy(&dwconfig), core::mem::transmute(&pszurl), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetExplorerManager as windows_core::Interface>::IID
    }
}
pub trait IInternetExplorerManager2_Impl: Sized {
    fn EnumFrameWindows(&self) -> windows_core::Result<IEnumManagerFrames>;
}
impl windows_core::RuntimeName for IInternetExplorerManager2 {}
impl IInternetExplorerManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetExplorerManager2_Vtbl
    where
        Identity: IInternetExplorerManager2_Impl,
    {
        unsafe extern "system" fn EnumFrameWindows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetExplorerManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInternetExplorerManager2_Impl::EnumFrameWindows(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumFrameWindows: EnumFrameWindows::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetExplorerManager2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILayoutRect_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetnextRect(&self, bstrelementid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn nextRect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetcontentSrc(&self, varcontentsrc: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn contentSrc(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SethonorPageBreaks(&self, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn honorPageBreaks(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SethonorPageRules(&self, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn honorPageRules(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetnextRectElement(&self, pelem: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn nextRectElement(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn contentDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ILayoutRect {}
#[cfg(feature = "Win32_System_Com")]
impl ILayoutRect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILayoutRect_Vtbl
    where
        Identity: ILayoutRect_Impl,
    {
        unsafe extern "system" fn SetnextRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrelementid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILayoutRect_Impl::SetnextRect(this, core::mem::transmute(&bstrelementid)).into()
        }
        unsafe extern "system" fn nextRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrelementid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILayoutRect_Impl::nextRect(this) {
                Ok(ok__) => {
                    pbstrelementid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcontentSrc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varcontentsrc: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILayoutRect_Impl::SetcontentSrc(this, core::mem::transmute(&varcontentsrc)).into()
        }
        unsafe extern "system" fn contentSrc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcontentsrc: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILayoutRect_Impl::contentSrc(this) {
                Ok(ok__) => {
                    pvarcontentsrc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethonorPageBreaks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILayoutRect_Impl::SethonorPageBreaks(this, core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn honorPageBreaks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILayoutRect_Impl::honorPageBreaks(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethonorPageRules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILayoutRect_Impl::SethonorPageRules(this, core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn honorPageRules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILayoutRect_Impl::honorPageRules(this) {
                Ok(ok__) => {
                    p.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnextRectElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILayoutRect_Impl::SetnextRectElement(this, windows_core::from_raw_borrowed(&pelem)).into()
        }
        unsafe extern "system" fn nextRectElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppelem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILayoutRect_Impl::nextRectElement(this) {
                Ok(ok__) => {
                    ppelem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILayoutRect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILayoutRect_Impl::contentDocument(this) {
                Ok(ok__) => {
                    pdoc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetnextRect: SetnextRect::<Identity, OFFSET>,
            nextRect: nextRect::<Identity, OFFSET>,
            SetcontentSrc: SetcontentSrc::<Identity, OFFSET>,
            contentSrc: contentSrc::<Identity, OFFSET>,
            SethonorPageBreaks: SethonorPageBreaks::<Identity, OFFSET>,
            honorPageBreaks: honorPageBreaks::<Identity, OFFSET>,
            SethonorPageRules: SethonorPageRules::<Identity, OFFSET>,
            honorPageRules: honorPageRules::<Identity, OFFSET>,
            SetnextRectElement: SetnextRectElement::<Identity, OFFSET>,
            nextRectElement: nextRectElement::<Identity, OFFSET>,
            contentDocument: contentDocument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILayoutRect as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IMapMIMEToCLSID_Impl: Sized {
    fn EnableDefaultMappings(&self, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn MapMIMEToCLSID(&self, pszmimetype: &windows_core::PCWSTR, pclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetMapping(&self, pszmimetype: &windows_core::PCWSTR, dwmapmode: u32, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMapMIMEToCLSID {}
impl IMapMIMEToCLSID_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMapMIMEToCLSID_Vtbl
    where
        Identity: IMapMIMEToCLSID_Impl,
    {
        unsafe extern "system" fn EnableDefaultMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMapMIMEToCLSID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMapMIMEToCLSID_Impl::EnableDefaultMappings(this, core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn MapMIMEToCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmimetype: windows_core::PCWSTR, pclsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMapMIMEToCLSID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMapMIMEToCLSID_Impl::MapMIMEToCLSID(this, core::mem::transmute(&pszmimetype), core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmimetype: windows_core::PCWSTR, dwmapmode: u32, clsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMapMIMEToCLSID_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMapMIMEToCLSID_Impl::SetMapping(this, core::mem::transmute(&pszmimetype), core::mem::transmute_copy(&dwmapmode), core::mem::transmute_copy(&clsid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableDefaultMappings: EnableDefaultMappings::<Identity, OFFSET>,
            MapMIMEToCLSID: MapMIMEToCLSID::<Identity, OFFSET>,
            SetMapping: SetMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapMIMEToCLSID as windows_core::Interface>::IID
    }
}
pub trait IMediaActivityNotifySite_Impl: Sized {
    fn OnMediaActivityStarted(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()>;
    fn OnMediaActivityStopped(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMediaActivityNotifySite {}
impl IMediaActivityNotifySite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaActivityNotifySite_Vtbl
    where
        Identity: IMediaActivityNotifySite_Impl,
    {
        unsafe extern "system" fn OnMediaActivityStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT
        where
            Identity: IMediaActivityNotifySite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaActivityNotifySite_Impl::OnMediaActivityStarted(this, core::mem::transmute_copy(&mediaactivitytype)).into()
        }
        unsafe extern "system" fn OnMediaActivityStopped<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT
        where
            Identity: IMediaActivityNotifySite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaActivityNotifySite_Impl::OnMediaActivityStopped(this, core::mem::transmute_copy(&mediaactivitytype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMediaActivityStarted: OnMediaActivityStarted::<Identity, OFFSET>,
            OnMediaActivityStopped: OnMediaActivityStopped::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaActivityNotifySite as windows_core::Interface>::IID
    }
}
pub trait IOpenService_Impl: Sized {
    fn IsDefault(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDefault(&self, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetID(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IOpenService {}
impl IOpenService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenService_Vtbl
    where
        Identity: IOpenService_Impl,
    {
        unsafe extern "system" fn IsDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenService_Impl::IsDefault(this) {
                Ok(ok__) => {
                    pfisdefault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IOpenService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenService_Impl::SetDefault(this, core::mem::transmute_copy(&fdefault), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenService_Impl::GetID(this) {
                Ok(ok__) => {
                    pbstrid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDefault: IsDefault::<Identity, OFFSET>,
            SetDefault: SetDefault::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOpenServiceActivity_Impl: Sized + IOpenService_Impl {
    fn Execute(&self, pinput: Option<&IOpenServiceActivityInput>, poutput: Option<&IOpenServiceActivityOutputContext>) -> windows_core::Result<()>;
    fn CanExecute(&self, pinput: Option<&IOpenServiceActivityInput>, poutput: Option<&IOpenServiceActivityOutputContext>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CanExecuteType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Preview(&self, pinput: Option<&IOpenServiceActivityInput>, poutput: Option<&IOpenServiceActivityOutputContext>) -> windows_core::Result<()>;
    fn CanPreview(&self, pinput: Option<&IOpenServiceActivityInput>, poutput: Option<&IOpenServiceActivityOutputContext>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CanPreviewType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetStatusText(&self, pinput: Option<&IOpenServiceActivityInput>) -> windows_core::Result<windows_core::BSTR>;
    fn GetHomepageUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCategoryName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIconPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIcon(&self, fsmallicon: super::super::Foundation::BOOL) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetDescriptionFilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetInstallUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOpenServiceActivity {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOpenServiceActivity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenServiceActivity_Vtbl
    where
        Identity: IOpenServiceActivity_Impl,
    {
        unsafe extern "system" fn Execute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenServiceActivity_Impl::Execute(this, windows_core::from_raw_borrowed(&pinput), windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn CanExecute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void, pfcanexecute: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::CanExecute(this, windows_core::from_raw_borrowed(&pinput), windows_core::from_raw_borrowed(&poutput)) {
                Ok(ok__) => {
                    pfcanexecute.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanExecuteType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanexecute: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::CanExecuteType(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    pfcanexecute.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenServiceActivity_Impl::Preview(this, windows_core::from_raw_borrowed(&pinput), windows_core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn CanPreview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void, pfcanpreview: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::CanPreview(this, windows_core::from_raw_borrowed(&pinput), windows_core::from_raw_borrowed(&poutput)) {
                Ok(ok__) => {
                    pfcanpreview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPreviewType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanpreview: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::CanPreviewType(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    pfcanpreview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetStatusText(this, windows_core::from_raw_borrowed(&pinput)) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomepageUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhomepageurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetHomepageUrl(this) {
                Ok(ok__) => {
                    pbstrhomepageurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    pbstrdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategoryName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcategoryname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetCategoryName(this) {
                Ok(ok__) => {
                    pbstrcategoryname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstriconpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetIconPath(this) {
                Ok(ok__) => {
                    pbstriconpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsmallicon: super::super::Foundation::BOOL, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetIcon(this, core::mem::transmute_copy(&fsmallicon)) {
                Ok(ok__) => {
                    phicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionFilePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxmlpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetDescriptionFilePath(this) {
                Ok(ok__) => {
                    pbstrxmlpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxmluri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetDownloadUrl(this) {
                Ok(ok__) => {
                    pbstrxmluri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstallUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinstalluri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::GetInstallUrl(this) {
                Ok(ok__) => {
                    pbstrinstalluri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivity_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    pfisenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenServiceActivity_Impl::SetEnabled(this, core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: IOpenService_Vtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, OFFSET>,
            CanExecute: CanExecute::<Identity, OFFSET>,
            CanExecuteType: CanExecuteType::<Identity, OFFSET>,
            Preview: Preview::<Identity, OFFSET>,
            CanPreview: CanPreview::<Identity, OFFSET>,
            CanPreviewType: CanPreviewType::<Identity, OFFSET>,
            GetStatusText: GetStatusText::<Identity, OFFSET>,
            GetHomepageUrl: GetHomepageUrl::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetCategoryName: GetCategoryName::<Identity, OFFSET>,
            GetIconPath: GetIconPath::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetDescriptionFilePath: GetDescriptionFilePath::<Identity, OFFSET>,
            GetDownloadUrl: GetDownloadUrl::<Identity, OFFSET>,
            GetInstallUrl: GetInstallUrl::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivity as windows_core::Interface>::IID || iid == &<IOpenService as windows_core::Interface>::IID
    }
}
pub trait IOpenServiceActivityCategory_Impl: Sized {
    fn HasDefaultActivity(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetDefaultActivity(&self) -> windows_core::Result<IOpenServiceActivity>;
    fn SetDefaultActivity(&self, pactivity: Option<&IOpenServiceActivity>, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetActivityEnumerator(&self, pinput: Option<&IOpenServiceActivityInput>, poutput: Option<&IOpenServiceActivityOutputContext>) -> windows_core::Result<IEnumOpenServiceActivity>;
}
impl windows_core::RuntimeName for IOpenServiceActivityCategory {}
impl IOpenServiceActivityCategory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenServiceActivityCategory_Vtbl
    where
        Identity: IOpenServiceActivityCategory_Impl,
    {
        unsafe extern "system" fn HasDefaultActivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasdefaultactivity: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityCategory_Impl::HasDefaultActivity(this) {
                Ok(ok__) => {
                    pfhasdefaultactivity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultActivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdefaultactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityCategory_Impl::GetDefaultActivity(this) {
                Ok(ok__) => {
                    ppdefaultactivity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultActivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivity: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenServiceActivityCategory_Impl::SetDefaultActivity(this, windows_core::from_raw_borrowed(&pactivity), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityCategory_Impl::GetName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void, ppenumactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityCategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityCategory_Impl::GetActivityEnumerator(this, windows_core::from_raw_borrowed(&pinput), windows_core::from_raw_borrowed(&poutput)) {
                Ok(ok__) => {
                    ppenumactivity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HasDefaultActivity: HasDefaultActivity::<Identity, OFFSET>,
            GetDefaultActivity: GetDefaultActivity::<Identity, OFFSET>,
            SetDefaultActivity: SetDefaultActivity::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetActivityEnumerator: GetActivityEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityCategory as windows_core::Interface>::IID
    }
}
pub trait IOpenServiceActivityInput_Impl: Sized {
    fn GetVariable(&self, pwzvariablename: &windows_core::PCWSTR, pwzvariabletype: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn HasVariable(&self, pwzvariablename: &windows_core::PCWSTR, pwzvariabletype: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetType(&self) -> windows_core::Result<OpenServiceActivityContentType>;
}
impl windows_core::RuntimeName for IOpenServiceActivityInput {}
impl IOpenServiceActivityInput_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenServiceActivityInput_Vtbl
    where
        Identity: IOpenServiceActivityInput_Impl,
    {
        unsafe extern "system" fn GetVariable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzvariablename: windows_core::PCWSTR, pwzvariabletype: windows_core::PCWSTR, pbstrvariablecontent: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityInput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityInput_Impl::GetVariable(this, core::mem::transmute(&pwzvariablename), core::mem::transmute(&pwzvariabletype)) {
                Ok(ok__) => {
                    pbstrvariablecontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasVariable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzvariablename: windows_core::PCWSTR, pwzvariabletype: windows_core::PCWSTR, pfhasvariable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityInput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityInput_Impl::HasVariable(this, core::mem::transmute(&pwzvariablename), core::mem::transmute(&pwzvariabletype)) {
                Ok(ok__) => {
                    pfhasvariable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut OpenServiceActivityContentType) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityInput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityInput_Impl::GetType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVariable: GetVariable::<Identity, OFFSET>,
            HasVariable: HasVariable::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityInput as windows_core::Interface>::IID
    }
}
pub trait IOpenServiceActivityManager_Impl: Sized {
    fn GetCategoryEnumerator(&self, etype: OpenServiceActivityContentType) -> windows_core::Result<IEnumOpenServiceActivityCategory>;
    fn GetActivityByID(&self, pwzactivityid: &windows_core::PCWSTR) -> windows_core::Result<IOpenServiceActivity>;
    fn GetActivityByHomepageAndCategory(&self, pwzhomepage: &windows_core::PCWSTR, pwzcategory: &windows_core::PCWSTR) -> windows_core::Result<IOpenServiceActivity>;
    fn GetVersionCookie(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IOpenServiceActivityManager {}
impl IOpenServiceActivityManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenServiceActivityManager_Vtbl
    where
        Identity: IOpenServiceActivityManager_Impl,
    {
        unsafe extern "system" fn GetCategoryEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, etype: OpenServiceActivityContentType, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityManager_Impl::GetCategoryEnumerator(this, core::mem::transmute_copy(&etype)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityByID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzactivityid: windows_core::PCWSTR, ppactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityManager_Impl::GetActivityByID(this, core::mem::transmute(&pwzactivityid)) {
                Ok(ok__) => {
                    ppactivity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityByHomepageAndCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzhomepage: windows_core::PCWSTR, pwzcategory: windows_core::PCWSTR, ppactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityManager_Impl::GetActivityByHomepageAndCategory(this, core::mem::transmute(&pwzhomepage), core::mem::transmute(&pwzcategory)) {
                Ok(ok__) => {
                    ppactivity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversioncookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityManager_Impl::GetVersionCookie(this) {
                Ok(ok__) => {
                    pdwversioncookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCategoryEnumerator: GetCategoryEnumerator::<Identity, OFFSET>,
            GetActivityByID: GetActivityByID::<Identity, OFFSET>,
            GetActivityByHomepageAndCategory: GetActivityByHomepageAndCategory::<Identity, OFFSET>,
            GetVersionCookie: GetVersionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpenServiceActivityOutputContext_Impl: Sized {
    fn Navigate(&self, pwzuri: &windows_core::PCWSTR, pwzmethod: &windows_core::PCWSTR, pwzheaders: &windows_core::PCWSTR, ppostdata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CanNavigate(&self, pwzuri: &windows_core::PCWSTR, pwzmethod: &windows_core::PCWSTR, pwzheaders: &windows_core::PCWSTR, ppostdata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpenServiceActivityOutputContext {}
#[cfg(feature = "Win32_System_Com")]
impl IOpenServiceActivityOutputContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenServiceActivityOutputContext_Vtbl
    where
        Identity: IOpenServiceActivityOutputContext_Impl,
    {
        unsafe extern "system" fn Navigate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzuri: windows_core::PCWSTR, pwzmethod: windows_core::PCWSTR, pwzheaders: windows_core::PCWSTR, ppostdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityOutputContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenServiceActivityOutputContext_Impl::Navigate(this, core::mem::transmute(&pwzuri), core::mem::transmute(&pwzmethod), core::mem::transmute(&pwzheaders), windows_core::from_raw_borrowed(&ppostdata)).into()
        }
        unsafe extern "system" fn CanNavigate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzuri: windows_core::PCWSTR, pwzmethod: windows_core::PCWSTR, pwzheaders: windows_core::PCWSTR, ppostdata: *mut core::ffi::c_void, pfcannavigate: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpenServiceActivityOutputContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceActivityOutputContext_Impl::CanNavigate(this, core::mem::transmute(&pwzuri), core::mem::transmute(&pwzmethod), core::mem::transmute(&pwzheaders), windows_core::from_raw_borrowed(&ppostdata)) {
                Ok(ok__) => {
                    pfcannavigate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Navigate: Navigate::<Identity, OFFSET>,
            CanNavigate: CanNavigate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityOutputContext as windows_core::Interface>::IID
    }
}
pub trait IOpenServiceManager_Impl: Sized {
    fn InstallService(&self, pwzserviceurl: &windows_core::PCWSTR) -> windows_core::Result<IOpenService>;
    fn UninstallService(&self, pservice: Option<&IOpenService>) -> windows_core::Result<()>;
    fn GetServiceByID(&self, pwzid: &windows_core::PCWSTR) -> windows_core::Result<IOpenService>;
}
impl windows_core::RuntimeName for IOpenServiceManager {}
impl IOpenServiceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenServiceManager_Vtbl
    where
        Identity: IOpenServiceManager_Impl,
    {
        unsafe extern "system" fn InstallService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzserviceurl: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceManager_Impl::InstallService(this, core::mem::transmute(&pwzserviceurl)) {
                Ok(ok__) => {
                    ppservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenServiceManager_Impl::UninstallService(this, windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn GetServiceByID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzid: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpenServiceManager_Impl::GetServiceByID(this, core::mem::transmute(&pwzid)) {
                Ok(ok__) => {
                    ppservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallService: InstallService::<Identity, OFFSET>,
            UninstallService: UninstallService::<Identity, OFFSET>,
            GetServiceByID: GetServiceByID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceManager as windows_core::Interface>::IID
    }
}
pub trait IPeerFactory_Impl: Sized {}
impl windows_core::RuntimeName for IPeerFactory {}
impl IPeerFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPeerFactory_Vtbl
    where
        Identity: IPeerFactory_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPeerFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistHistory_Impl: Sized + super::super::System::Com::IPersist_Impl {
    fn LoadHistory(&self, pstream: Option<&super::super::System::Com::IStream>, pbc: Option<&super::super::System::Com::IBindCtx>) -> windows_core::Result<()>;
    fn SaveHistory(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetPositionCookie(&self, dwpositioncookie: u32) -> windows_core::Result<()>;
    fn GetPositionCookie(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPersistHistory {}
#[cfg(feature = "Win32_System_Com")]
impl IPersistHistory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistHistory_Vtbl
    where
        Identity: IPersistHistory_Impl,
    {
        unsafe extern "system" fn LoadHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistHistory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistHistory_Impl::LoadHistory(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&pbc)).into()
        }
        unsafe extern "system" fn SaveHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistHistory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistHistory_Impl::SaveHistory(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn SetPositionCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpositioncookie: u32) -> windows_core::HRESULT
        where
            Identity: IPersistHistory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistHistory_Impl::SetPositionCookie(this, core::mem::transmute_copy(&dwpositioncookie)).into()
        }
        unsafe extern "system" fn GetPositionCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpositioncookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPersistHistory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistHistory_Impl::GetPositionCookie(this) {
                Ok(ok__) => {
                    pdwpositioncookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IPersist_Vtbl::new::<Identity, OFFSET>(),
            LoadHistory: LoadHistory::<Identity, OFFSET>,
            SaveHistory: SaveHistory::<Identity, OFFSET>,
            SetPositionCookie: SetPositionCookie::<Identity, OFFSET>,
            GetPositionCookie: GetPositionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistHistory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as windows_core::Interface>::IID
    }
}
pub trait IPrintTaskRequestFactory_Impl: Sized {
    fn CreatePrintTaskRequest(&self, pprinttaskrequesthandler: Option<&IPrintTaskRequestHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintTaskRequestFactory {}
impl IPrintTaskRequestFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintTaskRequestFactory_Vtbl
    where
        Identity: IPrintTaskRequestFactory_Impl,
    {
        unsafe extern "system" fn CreatePrintTaskRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprinttaskrequesthandler: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTaskRequestFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskRequestFactory_Impl::CreatePrintTaskRequest(this, windows_core::from_raw_borrowed(&pprinttaskrequesthandler)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreatePrintTaskRequest: CreatePrintTaskRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskRequestFactory as windows_core::Interface>::IID
    }
}
pub trait IPrintTaskRequestHandler_Impl: Sized {
    fn HandlePrintTaskRequest(&self, pprinttaskrequest: Option<&windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrintTaskRequestHandler {}
impl IPrintTaskRequestHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrintTaskRequestHandler_Vtbl
    where
        Identity: IPrintTaskRequestHandler_Impl,
    {
        unsafe extern "system" fn HandlePrintTaskRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprinttaskrequest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPrintTaskRequestHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrintTaskRequestHandler_Impl::HandlePrintTaskRequest(this, windows_core::from_raw_borrowed(&pprinttaskrequest)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandlePrintTaskRequest: HandlePrintTaskRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskRequestHandler as windows_core::Interface>::IID
    }
}
pub trait IScrollableContextMenu_Impl: Sized {
    fn AddItem(&self, itemtext: &windows_core::PCWSTR, cmdid: u32) -> windows_core::Result<()>;
    fn ShowModal(&self, x: i32, y: i32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IScrollableContextMenu {}
impl IScrollableContextMenu_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IScrollableContextMenu_Vtbl
    where
        Identity: IScrollableContextMenu_Impl,
    {
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemtext: windows_core::PCWSTR, cmdid: u32) -> windows_core::HRESULT
        where
            Identity: IScrollableContextMenu_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScrollableContextMenu_Impl::AddItem(this, core::mem::transmute(&itemtext), core::mem::transmute_copy(&cmdid)).into()
        }
        unsafe extern "system" fn ShowModal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, cmdid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IScrollableContextMenu_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IScrollableContextMenu_Impl::ShowModal(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                Ok(ok__) => {
                    cmdid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddItem: AddItem::<Identity, OFFSET>, ShowModal: ShowModal::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollableContextMenu as windows_core::Interface>::IID
    }
}
pub trait IScrollableContextMenu2_Impl: Sized + IScrollableContextMenu_Impl {
    fn AddSeparator(&self) -> windows_core::Result<()>;
    fn SetPlacement(&self, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IScrollableContextMenu2 {}
impl IScrollableContextMenu2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IScrollableContextMenu2_Vtbl
    where
        Identity: IScrollableContextMenu2_Impl,
    {
        unsafe extern "system" fn AddSeparator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScrollableContextMenu2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScrollableContextMenu2_Impl::AddSeparator(this).into()
        }
        unsafe extern "system" fn SetPlacement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::HRESULT
        where
            Identity: IScrollableContextMenu2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScrollableContextMenu2_Impl::SetPlacement(this, core::mem::transmute_copy(&scmp)).into()
        }
        Self {
            base__: IScrollableContextMenu_Vtbl::new::<Identity, OFFSET>(),
            AddSeparator: AddSeparator::<Identity, OFFSET>,
            SetPlacement: SetPlacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollableContextMenu2 as windows_core::Interface>::IID || iid == &<IScrollableContextMenu as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISniffStream_Impl: Sized {
    fn Init(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Peek(&self, pbuffer: *mut core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISniffStream {}
#[cfg(feature = "Win32_System_Com")]
impl ISniffStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISniffStream_Vtbl
    where
        Identity: ISniffStream_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISniffStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISniffStream_Impl::Init(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn Peek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISniffStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISniffStream_Impl::Peek(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&nbytes), core::mem::transmute_copy(&pnbytesread)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Init: Init::<Identity, OFFSET>, Peek: Peek::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISniffStream as windows_core::Interface>::IID
    }
}
pub trait ISurfacePresenterFlip_Impl: Sized {
    fn Present(&self) -> windows_core::Result<()>;
    fn GetBuffer(&self, backbufferindex: u32, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISurfacePresenterFlip {}
impl ISurfacePresenterFlip_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISurfacePresenterFlip_Vtbl
    where
        Identity: ISurfacePresenterFlip_Impl,
    {
        unsafe extern "system" fn Present<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISurfacePresenterFlip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurfacePresenterFlip_Impl::Present(this).into()
        }
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, backbufferindex: u32, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISurfacePresenterFlip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurfacePresenterFlip_Impl::GetBuffer(this, core::mem::transmute_copy(&backbufferindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppbuffer)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Present: Present::<Identity, OFFSET>, GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlip as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ISurfacePresenterFlip2_Impl: Sized {
    fn SetRotation(&self, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ISurfacePresenterFlip2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ISurfacePresenterFlip2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISurfacePresenterFlip2_Vtbl
    where
        Identity: ISurfacePresenterFlip2_Impl,
    {
        unsafe extern "system" fn SetRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT
        where
            Identity: ISurfacePresenterFlip2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurfacePresenterFlip2_Impl::SetRotation(this, core::mem::transmute_copy(&dxgirotation)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRotation: SetRotation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlip2 as windows_core::Interface>::IID
    }
}
pub trait ISurfacePresenterFlipBuffer_Impl: Sized {
    fn BeginDraw(&self, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISurfacePresenterFlipBuffer {}
impl ISurfacePresenterFlipBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISurfacePresenterFlipBuffer_Vtbl
    where
        Identity: ISurfacePresenterFlipBuffer_Impl,
    {
        unsafe extern "system" fn BeginDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISurfacePresenterFlipBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurfacePresenterFlipBuffer_Impl::BeginDraw(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISurfacePresenterFlipBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISurfacePresenterFlipBuffer_Impl::EndDraw(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BeginDraw: BeginDraw::<Identity, OFFSET>, EndDraw: EndDraw::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlipBuffer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetContainer_Impl: Sized {
    fn GetFrameUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for ITargetContainer {}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetContainer_Vtbl
    where
        Identity: ITargetContainer_Impl,
    {
        unsafe extern "system" fn GetFrameUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframesrc: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetContainer_Impl::GetFrameUrl(this) {
                Ok(ok__) => {
                    ppszframesrc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetContainer_Impl::GetFramesContainer(this) {
                Ok(ok__) => {
                    ppcontainer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameUrl: GetFrameUrl::<Identity, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetContainer as windows_core::Interface>::IID
    }
}
pub trait ITargetEmbedding_Impl: Sized {
    fn GetTargetFrame(&self) -> windows_core::Result<ITargetFrame>;
}
impl windows_core::RuntimeName for ITargetEmbedding {}
impl ITargetEmbedding_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetEmbedding_Vtbl
    where
        Identity: ITargetEmbedding_Impl,
    {
        unsafe extern "system" fn GetTargetFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetEmbedding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetEmbedding_Impl::GetTargetFrame(this) {
                Ok(ok__) => {
                    pptargetframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTargetFrame: GetTargetFrame::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetEmbedding as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame_Impl: Sized {
    fn SetFrameName(&self, pszframename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn FindFrame(&self, psztargetname: &windows_core::PCWSTR, ppunkcontextframe: Option<&windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn SetFrameSrc(&self, pszframesrc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrameOptions(&self) -> windows_core::Result<u32>;
    fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()>;
    fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
    fn RemoteNavigate(&self, clength: u32, puldata: *const u32) -> windows_core::Result<()>;
    fn OnChildFrameActivate(&self, punkchildframe: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnChildFrameDeactivate(&self, punkchildframe: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for ITargetFrame {}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetFrame_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetFrame_Vtbl
    where
        Identity: ITargetFrame_Impl,
    {
        unsafe extern "system" fn SetFrameName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::SetFrameName(this, core::mem::transmute(&pszframename)).into()
        }
        unsafe extern "system" fn GetFrameName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame_Impl::GetFrameName(this) {
                Ok(ok__) => {
                    ppszframename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame_Impl::GetParentFrame(this) {
                Ok(ok__) => {
                    ppunkparent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, ppunkcontextframe: *mut core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame_Impl::FindFrame(this, core::mem::transmute(&psztargetname), windows_core::from_raw_borrowed(&ppunkcontextframe), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppunktargetframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameSrc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframesrc: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::SetFrameSrc(this, core::mem::transmute(&pszframesrc)).into()
        }
        unsafe extern "system" fn GetFrameSrc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframesrc: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame_Impl::GetFrameSrc(this) {
                Ok(ok__) => {
                    ppszframesrc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame_Impl::GetFramesContainer(this) {
                Ok(ok__) => {
                    ppcontainer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::SetFrameOptions(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrameOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame_Impl::GetFrameOptions(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameMargins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwidth: u32, dwheight: u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::SetFrameMargins(this, core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight)).into()
        }
        unsafe extern "system" fn GetFrameMargins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::GetFrameMargins(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
        }
        unsafe extern "system" fn RemoteNavigate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clength: u32, puldata: *const u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::RemoteNavigate(this, core::mem::transmute_copy(&clength), core::mem::transmute_copy(&puldata)).into()
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::OnChildFrameActivate(this, windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame_Impl::OnChildFrameDeactivate(this, windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameName: SetFrameName::<Identity, OFFSET>,
            GetFrameName: GetFrameName::<Identity, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, OFFSET>,
            FindFrame: FindFrame::<Identity, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, OFFSET>,
            RemoteNavigate: RemoteNavigate::<Identity, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFrame as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame2_Impl: Sized {
    fn SetFrameName(&self, pszframename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetFrameSrc(&self, pszframesrc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrameOptions(&self) -> windows_core::Result<u32>;
    fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()>;
    fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
    fn FindFrame(&self, psztargetname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn GetTargetAlias(&self, psztargetname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for ITargetFrame2 {}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetFrame2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetFrame2_Vtbl
    where
        Identity: ITargetFrame2_Impl,
    {
        unsafe extern "system" fn SetFrameName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame2_Impl::SetFrameName(this, core::mem::transmute(&pszframename)).into()
        }
        unsafe extern "system" fn GetFrameName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::GetFrameName(this) {
                Ok(ok__) => {
                    ppszframename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::GetParentFrame(this) {
                Ok(ok__) => {
                    ppunkparent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameSrc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframesrc: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame2_Impl::SetFrameSrc(this, core::mem::transmute(&pszframesrc)).into()
        }
        unsafe extern "system" fn GetFrameSrc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframesrc: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::GetFrameSrc(this) {
                Ok(ok__) => {
                    ppszframesrc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::GetFramesContainer(this) {
                Ok(ok__) => {
                    ppcontainer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame2_Impl::SetFrameOptions(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFrameOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::GetFrameOptions(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameMargins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwidth: u32, dwheight: u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame2_Impl::SetFrameMargins(this, core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight)).into()
        }
        unsafe extern "system" fn GetFrameMargins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFrame2_Impl::GetFrameMargins(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
        }
        unsafe extern "system" fn FindFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::FindFrame(this, core::mem::transmute(&psztargetname), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppunktargetframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetAlias<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, ppsztargetalias: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFrame2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFrame2_Impl::GetTargetAlias(this, core::mem::transmute(&psztargetname)) {
                Ok(ok__) => {
                    ppsztargetalias.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameName: SetFrameName::<Identity, OFFSET>,
            GetFrameName: GetFrameName::<Identity, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, OFFSET>,
            FindFrame: FindFrame::<Identity, OFFSET>,
            GetTargetAlias: GetTargetAlias::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFrame2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv_Impl: Sized {
    fn FindFrameDownwards(&self, psztargetname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn FindFrameInContext(&self, psztargetname: &windows_core::PCWSTR, punkcontextframe: Option<&windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn OnChildFrameActivate(&self, punkchildframe: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnChildFrameDeactivate(&self, punkchildframe: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn NavigateHack(&self, grfhlnf: u32, pbc: Option<&super::super::System::Com::IBindCtx>, pibsc: Option<&super::super::System::Com::IBindStatusCallback>, psztargetname: &windows_core::PCWSTR, pszurl: &windows_core::PCWSTR, pszlocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FindBrowserByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITargetFramePriv {}
#[cfg(feature = "Win32_System_Com")]
impl ITargetFramePriv_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetFramePriv_Vtbl
    where
        Identity: ITargetFramePriv_Impl,
    {
        unsafe extern "system" fn FindFrameDownwards<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFramePriv_Impl::FindFrameDownwards(this, core::mem::transmute(&psztargetname), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppunktargetframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFrameInContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, punkcontextframe: *mut core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFramePriv_Impl::FindFrameInContext(this, core::mem::transmute(&psztargetname), windows_core::from_raw_borrowed(&punkcontextframe), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppunktargetframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFramePriv_Impl::OnChildFrameActivate(this, windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFramePriv_Impl::OnChildFrameDeactivate(this, windows_core::from_raw_borrowed(&punkchildframe)).into()
        }
        unsafe extern "system" fn NavigateHack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: u32, pbc: *mut core::ffi::c_void, pibsc: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, pszurl: windows_core::PCWSTR, pszlocation: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFramePriv_Impl::NavigateHack(this, core::mem::transmute_copy(&grfhlnf), windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pibsc), core::mem::transmute(&psztargetname), core::mem::transmute(&pszurl), core::mem::transmute(&pszlocation)).into()
        }
        unsafe extern "system" fn FindBrowserByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, ppunkbrowser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetFramePriv_Impl::FindBrowserByIndex(this, core::mem::transmute_copy(&dwid)) {
                Ok(ok__) => {
                    ppunkbrowser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFrameDownwards: FindFrameDownwards::<Identity, OFFSET>,
            FindFrameInContext: FindFrameInContext::<Identity, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, OFFSET>,
            NavigateHack: NavigateHack::<Identity, OFFSET>,
            FindBrowserByIndex: FindBrowserByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFramePriv as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv2_Impl: Sized + ITargetFramePriv_Impl {
    fn AggregatedNavigation2(&self, grfhlnf: u32, pbc: Option<&super::super::System::Com::IBindCtx>, pibsc: Option<&super::super::System::Com::IBindStatusCallback>, psztargetname: &windows_core::PCWSTR, puri: Option<&super::super::System::Com::IUri>, pszlocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITargetFramePriv2 {}
#[cfg(feature = "Win32_System_Com")]
impl ITargetFramePriv2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetFramePriv2_Vtbl
    where
        Identity: ITargetFramePriv2_Impl,
    {
        unsafe extern "system" fn AggregatedNavigation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: u32, pbc: *mut core::ffi::c_void, pibsc: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, puri: *mut core::ffi::c_void, pszlocation: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetFramePriv2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetFramePriv2_Impl::AggregatedNavigation2(this, core::mem::transmute_copy(&grfhlnf), windows_core::from_raw_borrowed(&pbc), windows_core::from_raw_borrowed(&pibsc), core::mem::transmute(&psztargetname), windows_core::from_raw_borrowed(&puri), core::mem::transmute(&pszlocation)).into()
        }
        Self { base__: ITargetFramePriv_Vtbl::new::<Identity, OFFSET>(), AggregatedNavigation2: AggregatedNavigation2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFramePriv2 as windows_core::Interface>::IID || iid == &<ITargetFramePriv as windows_core::Interface>::IID
    }
}
pub trait ITargetNotify_Impl: Sized {
    fn OnCreate(&self, punkdestination: Option<&windows_core::IUnknown>, cbcookie: u32) -> windows_core::Result<()>;
    fn OnReuse(&self, punkdestination: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITargetNotify {}
impl ITargetNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetNotify_Vtbl
    where
        Identity: ITargetNotify_Impl,
    {
        unsafe extern "system" fn OnCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdestination: *mut core::ffi::c_void, cbcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITargetNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetNotify_Impl::OnCreate(this, windows_core::from_raw_borrowed(&punkdestination), core::mem::transmute_copy(&cbcookie)).into()
        }
        unsafe extern "system" fn OnReuse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdestination: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetNotify_Impl::OnReuse(this, windows_core::from_raw_borrowed(&punkdestination)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCreate: OnCreate::<Identity, OFFSET>, OnReuse: OnReuse::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetNotify as windows_core::Interface>::IID
    }
}
pub trait ITargetNotify2_Impl: Sized + ITargetNotify_Impl {
    fn GetOptionString(&self, pbstroptions: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITargetNotify2 {}
impl ITargetNotify2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetNotify2_Vtbl
    where
        Identity: ITargetNotify2_Impl,
    {
        unsafe extern "system" fn GetOptionString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstroptions: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetNotify2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetNotify2_Impl::GetOptionString(this, core::mem::transmute_copy(&pbstroptions)).into()
        }
        Self { base__: ITargetNotify_Vtbl::new::<Identity, OFFSET>(), GetOptionString: GetOptionString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetNotify2 as windows_core::Interface>::IID || iid == &<ITargetNotify as windows_core::Interface>::IID
    }
}
pub trait ITimer_Impl: Sized {
    fn Advise(&self, vtimemin: &windows_core::VARIANT, vtimemax: &windows_core::VARIANT, vtimeinterval: &windows_core::VARIANT, dwflags: u32, ptimersink: Option<&ITimerSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn Freeze(&self, ffreeze: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetTime(&self) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ITimer {}
impl ITimer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITimer_Vtbl
    where
        Identity: ITimer_Impl,
    {
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtimemin: core::mem::MaybeUninit<windows_core::VARIANT>, vtimemax: core::mem::MaybeUninit<windows_core::VARIANT>, vtimeinterval: core::mem::MaybeUninit<windows_core::VARIANT>, dwflags: u32, ptimersink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITimer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITimer_Impl::Advise(this, core::mem::transmute(&vtimemin), core::mem::transmute(&vtimemax), core::mem::transmute(&vtimeinterval), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&ptimersink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ITimer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimer_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Freeze<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffreeze: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITimer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimer_Impl::Freeze(this, core::mem::transmute_copy(&ffreeze)).into()
        }
        unsafe extern "system" fn GetTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITimer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITimer_Impl::GetTime(this) {
                Ok(ok__) => {
                    pvtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            Freeze: Freeze::<Identity, OFFSET>,
            GetTime: GetTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimer as windows_core::Interface>::IID
    }
}
pub trait ITimerEx_Impl: Sized + ITimer_Impl {
    fn SetMode(&self, dwmode: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITimerEx {}
impl ITimerEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITimerEx_Vtbl
    where
        Identity: ITimerEx_Impl,
    {
        unsafe extern "system" fn SetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32) -> windows_core::HRESULT
        where
            Identity: ITimerEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimerEx_Impl::SetMode(this, core::mem::transmute_copy(&dwmode)).into()
        }
        Self { base__: ITimer_Vtbl::new::<Identity, OFFSET>(), SetMode: SetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimerEx as windows_core::Interface>::IID || iid == &<ITimer as windows_core::Interface>::IID
    }
}
pub trait ITimerService_Impl: Sized {
    fn CreateTimer(&self, preferencetimer: Option<&ITimer>) -> windows_core::Result<ITimer>;
    fn GetNamedTimer(&self, rguidname: *const windows_core::GUID) -> windows_core::Result<ITimer>;
    fn SetNamedTimerReference(&self, rguidname: *const windows_core::GUID, preferencetimer: Option<&ITimer>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITimerService {}
impl ITimerService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITimerService_Vtbl
    where
        Identity: ITimerService_Impl,
    {
        unsafe extern "system" fn CreateTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferencetimer: *mut core::ffi::c_void, ppnewtimer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITimerService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITimerService_Impl::CreateTimer(this, windows_core::from_raw_borrowed(&preferencetimer)) {
                Ok(ok__) => {
                    ppnewtimer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidname: *const windows_core::GUID, pptimer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITimerService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITimerService_Impl::GetNamedTimer(this, core::mem::transmute_copy(&rguidname)) {
                Ok(ok__) => {
                    pptimer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedTimerReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidname: *const windows_core::GUID, preferencetimer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITimerService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimerService_Impl::SetNamedTimerReference(this, core::mem::transmute_copy(&rguidname), windows_core::from_raw_borrowed(&preferencetimer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTimer: CreateTimer::<Identity, OFFSET>,
            GetNamedTimer: GetNamedTimer::<Identity, OFFSET>,
            SetNamedTimerReference: SetNamedTimerReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimerService as windows_core::Interface>::IID
    }
}
pub trait ITimerSink_Impl: Sized {
    fn OnTimer(&self, vtimeadvise: &windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITimerSink {}
impl ITimerSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITimerSink_Vtbl
    where
        Identity: ITimerSink_Impl,
    {
        unsafe extern "system" fn OnTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtimeadvise: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITimerSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITimerSink_Impl::OnTimer(this, core::mem::transmute(&vtimeadvise)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTimer: OnTimer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimerSink as windows_core::Interface>::IID
    }
}
pub trait ITridentTouchInput_Impl: Sized {
    fn OnPointerMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITridentTouchInput {}
impl ITridentTouchInput_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITridentTouchInput_Vtbl
    where
        Identity: ITridentTouchInput_Impl,
    {
        unsafe extern "system" fn OnPointerMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfallowmanipulations: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITridentTouchInput_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITridentTouchInput_Impl::OnPointerMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    pfallowmanipulations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPointerMessage: OnPointerMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITridentTouchInput as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IUrlHistoryNotify_Impl: Sized + super::super::System::Ole::IOleCommandTarget_Impl {}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IUrlHistoryNotify {}
#[cfg(feature = "Win32_System_Ole")]
impl IUrlHistoryNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlHistoryNotify_Vtbl
    where
        Identity: IUrlHistoryNotify_Impl,
    {
        Self { base__: super::super::System::Ole::IOleCommandTarget_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlHistoryNotify as windows_core::Interface>::IID || iid == &<super::super::System::Ole::IOleCommandTarget as windows_core::Interface>::IID
    }
}
pub trait IUrlHistoryStg_Impl: Sized {
    fn AddUrl(&self, pocsurl: &windows_core::PCWSTR, pocstitle: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn DeleteUrl(&self, pocsurl: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn QueryUrl(&self, pocsurl: &windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> windows_core::Result<()>;
    fn BindToObject(&self, pocsurl: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumUrls(&self) -> windows_core::Result<IEnumSTATURL>;
}
impl windows_core::RuntimeName for IUrlHistoryStg {}
impl IUrlHistoryStg_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlHistoryStg_Vtbl
    where
        Identity: IUrlHistoryStg_Impl,
    {
        unsafe extern "system" fn AddUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, pocstitle: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlHistoryStg_Impl::AddUrl(this, core::mem::transmute(&pocsurl), core::mem::transmute(&pocstitle), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlHistoryStg_Impl::DeleteUrl(this, core::mem::transmute(&pocsurl), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn QueryUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlHistoryStg_Impl::QueryUrl(this, core::mem::transmute(&pocsurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&lpstaturl)).into()
        }
        unsafe extern "system" fn BindToObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlHistoryStg_Impl::BindToObject(this, core::mem::transmute(&pocsurl), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvout)).into()
        }
        unsafe extern "system" fn EnumUrls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlHistoryStg_Impl::EnumUrls(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddUrl: AddUrl::<Identity, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, OFFSET>,
            QueryUrl: QueryUrl::<Identity, OFFSET>,
            BindToObject: BindToObject::<Identity, OFFSET>,
            EnumUrls: EnumUrls::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlHistoryStg as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IUrlHistoryStg2_Impl: Sized + IUrlHistoryStg_Impl {
    fn AddUrlAndNotify(&self, pocsurl: &windows_core::PCWSTR, pocstitle: &windows_core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: Option<&super::super::System::Ole::IOleCommandTarget>, punkisfolder: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ClearHistory(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IUrlHistoryStg2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IUrlHistoryStg2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlHistoryStg2_Vtbl
    where
        Identity: IUrlHistoryStg2_Impl,
    {
        unsafe extern "system" fn AddUrlAndNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, pocstitle: windows_core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: *mut core::ffi::c_void, punkisfolder: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlHistoryStg2_Impl::AddUrlAndNotify(this, core::mem::transmute(&pocsurl), core::mem::transmute(&pocstitle), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&fwritehistory), windows_core::from_raw_borrowed(&poctnotify), windows_core::from_raw_borrowed(&punkisfolder)).into()
        }
        unsafe extern "system" fn ClearHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlHistoryStg2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlHistoryStg2_Impl::ClearHistory(this).into()
        }
        Self {
            base__: IUrlHistoryStg_Vtbl::new::<Identity, OFFSET>(),
            AddUrlAndNotify: AddUrlAndNotify::<Identity, OFFSET>,
            ClearHistory: ClearHistory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlHistoryStg2 as windows_core::Interface>::IID || iid == &<IUrlHistoryStg as windows_core::Interface>::IID
    }
}
pub trait IViewObjectPresentFlip_Impl: Sized {
    fn NotifyRender(&self, frecreatepresenter: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RenderObjectToBitmap(&self, pbitmap: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RenderObjectToSharedBuffer(&self, pbuffer: Option<&ISurfacePresenterFlipBuffer>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IViewObjectPresentFlip {}
impl IViewObjectPresentFlip_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewObjectPresentFlip_Vtbl
    where
        Identity: IViewObjectPresentFlip_Impl,
    {
        unsafe extern "system" fn NotifyRender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecreatepresenter: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IViewObjectPresentFlip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObjectPresentFlip_Impl::NotifyRender(this, core::mem::transmute_copy(&frecreatepresenter)).into()
        }
        unsafe extern "system" fn RenderObjectToBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewObjectPresentFlip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObjectPresentFlip_Impl::RenderObjectToBitmap(this, windows_core::from_raw_borrowed(&pbitmap)).into()
        }
        unsafe extern "system" fn RenderObjectToSharedBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewObjectPresentFlip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObjectPresentFlip_Impl::RenderObjectToSharedBuffer(this, windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyRender: NotifyRender::<Identity, OFFSET>,
            RenderObjectToBitmap: RenderObjectToBitmap::<Identity, OFFSET>,
            RenderObjectToSharedBuffer: RenderObjectToSharedBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlip as windows_core::Interface>::IID
    }
}
pub trait IViewObjectPresentFlip2_Impl: Sized {
    fn NotifyLeavingView(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IViewObjectPresentFlip2 {}
impl IViewObjectPresentFlip2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewObjectPresentFlip2_Vtbl
    where
        Identity: IViewObjectPresentFlip2_Impl,
    {
        unsafe extern "system" fn NotifyLeavingView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewObjectPresentFlip2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObjectPresentFlip2_Impl::NotifyLeavingView(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyLeavingView: NotifyLeavingView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlip2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IViewObjectPresentFlipSite2_Impl: Sized {
    fn GetRotationForCurrentOutput(&self) -> windows_core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IViewObjectPresentFlipSite2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IViewObjectPresentFlipSite2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewObjectPresentFlipSite2_Vtbl
    where
        Identity: IViewObjectPresentFlipSite2_Impl,
    {
        unsafe extern "system" fn GetRotationForCurrentOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdxgirotation: *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT
        where
            Identity: IViewObjectPresentFlipSite2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObjectPresentFlipSite2_Impl::GetRotationForCurrentOutput(this) {
                Ok(ok__) => {
                    pdxgirotation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRotationForCurrentOutput: GetRotationForCurrentOutput::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlipSite2 as windows_core::Interface>::IID
    }
}
pub trait IWebBrowserEventsService_Impl: Sized {
    fn FireBeforeNavigate2Event(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn FireNavigateComplete2Event(&self) -> windows_core::Result<()>;
    fn FireDownloadBeginEvent(&self) -> windows_core::Result<()>;
    fn FireDownloadCompleteEvent(&self) -> windows_core::Result<()>;
    fn FireDocumentCompleteEvent(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWebBrowserEventsService {}
impl IWebBrowserEventsService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebBrowserEventsService_Vtbl
    where
        Identity: IWebBrowserEventsService_Impl,
    {
        unsafe extern "system" fn FireBeforeNavigate2Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcancel: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWebBrowserEventsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebBrowserEventsService_Impl::FireBeforeNavigate2Event(this) {
                Ok(ok__) => {
                    pfcancel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireNavigateComplete2Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebBrowserEventsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebBrowserEventsService_Impl::FireNavigateComplete2Event(this).into()
        }
        unsafe extern "system" fn FireDownloadBeginEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebBrowserEventsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebBrowserEventsService_Impl::FireDownloadBeginEvent(this).into()
        }
        unsafe extern "system" fn FireDownloadCompleteEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebBrowserEventsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebBrowserEventsService_Impl::FireDownloadCompleteEvent(this).into()
        }
        unsafe extern "system" fn FireDocumentCompleteEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebBrowserEventsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebBrowserEventsService_Impl::FireDocumentCompleteEvent(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FireBeforeNavigate2Event: FireBeforeNavigate2Event::<Identity, OFFSET>,
            FireNavigateComplete2Event: FireNavigateComplete2Event::<Identity, OFFSET>,
            FireDownloadBeginEvent: FireDownloadBeginEvent::<Identity, OFFSET>,
            FireDownloadCompleteEvent: FireDownloadCompleteEvent::<Identity, OFFSET>,
            FireDocumentCompleteEvent: FireDocumentCompleteEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowserEventsService as windows_core::Interface>::IID
    }
}
pub trait IWebBrowserEventsUrlService_Impl: Sized {
    fn GetUrlForEvents(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IWebBrowserEventsUrlService {}
impl IWebBrowserEventsUrlService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebBrowserEventsUrlService_Vtbl
    where
        Identity: IWebBrowserEventsUrlService_Impl,
    {
        unsafe extern "system" fn GetUrlForEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebBrowserEventsUrlService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebBrowserEventsUrlService_Impl::GetUrlForEvents(this) {
                Ok(ok__) => {
                    purl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUrlForEvents: GetUrlForEvents::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowserEventsUrlService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait Iwfolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn navigate(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn navigateFrame(&self, bstrurl: &windows_core::BSTR, bstrtargetframe: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn navigateNoSite(&self, bstrurl: &windows_core::BSTR, bstrtargetframe: &windows_core::BSTR, dwhwnd: u32, pwb: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for Iwfolders {}
#[cfg(feature = "Win32_System_Com")]
impl Iwfolders_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Iwfolders_Vtbl
    where
        Identity: Iwfolders_Impl,
    {
        unsafe extern "system" fn navigate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, pbstrretval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: Iwfolders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match Iwfolders_Impl::navigate(this, core::mem::transmute(&bstrurl)) {
                Ok(ok__) => {
                    pbstrretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn navigateFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, bstrtargetframe: core::mem::MaybeUninit<windows_core::BSTR>, pbstrretval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: Iwfolders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match Iwfolders_Impl::navigateFrame(this, core::mem::transmute(&bstrurl), core::mem::transmute(&bstrtargetframe)) {
                Ok(ok__) => {
                    pbstrretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn navigateNoSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, bstrtargetframe: core::mem::MaybeUninit<windows_core::BSTR>, dwhwnd: u32, pwb: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: Iwfolders_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            Iwfolders_Impl::navigateNoSite(this, core::mem::transmute(&bstrurl), core::mem::transmute(&bstrtargetframe), core::mem::transmute_copy(&dwhwnd), windows_core::from_raw_borrowed(&pwb)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            navigate: navigate::<Identity, OFFSET>,
            navigateFrame: navigateFrame::<Identity, OFFSET>,
            navigateNoSite: navigateNoSite::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Iwfolders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}

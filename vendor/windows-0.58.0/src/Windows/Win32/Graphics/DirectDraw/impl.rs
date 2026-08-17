pub trait IDDVideoPortContainer_Impl: Sized {
    fn CreateVideoPort(&self, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut Option<IDirectDrawVideoPort>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumVideoPorts(&self, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> windows_core::Result<()>;
    fn GetVideoPortConnectInfo(&self, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> windows_core::Result<()>;
    fn QueryVideoPortStatus(&self, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDDVideoPortContainer {}
impl IDDVideoPortContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDDVideoPortContainer_Vtbl
    where
        Identity: IDDVideoPortContainer_Impl,
    {
        unsafe extern "system" fn CreateVideoPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDDVideoPortContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDDVideoPortContainer_Impl::CreateVideoPort(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumVideoPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDDVideoPortContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDDVideoPortContainer_Impl::EnumVideoPorts(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> windows_core::HRESULT
        where
            Identity: IDDVideoPortContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDDVideoPortContainer_Impl::GetVideoPortConnectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&pcinfo), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn QueryVideoPortStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> windows_core::HRESULT
        where
            Identity: IDDVideoPortContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDDVideoPortContainer_Impl::QueryVideoPortStatus(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoPort: CreateVideoPort::<Identity, OFFSET>,
            EnumVideoPorts: EnumVideoPorts::<Identity, OFFSET>,
            GetVideoPortConnectInfo: GetVideoPortConnectInfo::<Identity, OFFSET>,
            QueryVideoPortStatus: QueryVideoPortStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDDVideoPortContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw_Impl: Sized {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC, param1: *mut Option<IDirectDrawSurface>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: Option<&IDirectDrawSurface>) -> windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Initialize(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDraw {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDraw_Vtbl
    where
        Identity: IDirectDraw_Impl,
    {
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::Compact(this).into()
        }
        unsafe extern "system" fn CreateClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw_Impl::DuplicateSurface(this, windows_core::from_raw_borrowed(&param0)) {
                Ok(ok__) => {
                    param1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::FlipToGDISurface(this).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw_Impl::GetGDISurface(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::GetMonitorFrequency(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::GetScanLine(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::GetVerticalBlankStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::Initialize(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::RestoreDisplayMode(this).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDraw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw2_Impl: Sized {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC, param1: *mut Option<IDirectDrawSurface>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: Option<&IDirectDrawSurface>) -> windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Initialize(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDraw2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDraw2_Vtbl
    where
        Identity: IDirectDraw2_Impl,
    {
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::Compact(this).into()
        }
        unsafe extern "system" fn CreateClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw2_Impl::DuplicateSurface(this, windows_core::from_raw_borrowed(&param0)) {
                Ok(ok__) => {
                    param1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::FlipToGDISurface(this).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw2_Impl::GetGDISurface(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetMonitorFrequency(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetScanLine(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetVerticalBlankStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::Initialize(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::RestoreDisplayMode(this).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw2_Impl::GetAvailableVidMem(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw4_Impl: Sized {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC2, param1: *mut Option<IDirectDrawSurface4>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: Option<&IDirectDrawSurface4>) -> windows_core::Result<IDirectDrawSurface4>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface4>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Initialize(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::Result<()>;
    fn GetSurfaceFromDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<IDirectDrawSurface4>;
    fn RestoreAllSurfaces(&self) -> windows_core::Result<()>;
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDraw4 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDraw4_Vtbl
    where
        Identity: IDirectDraw4_Impl,
    {
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::Compact(this).into()
        }
        unsafe extern "system" fn CreateClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw4_Impl::DuplicateSurface(this, windows_core::from_raw_borrowed(&param0)) {
                Ok(ok__) => {
                    param1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::FlipToGDISurface(this).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw4_Impl::GetGDISurface(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetMonitorFrequency(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetScanLine(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetVerticalBlankStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::Initialize(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::RestoreDisplayMode(this).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetAvailableVidMem(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw4_Impl::GetSurfaceFromDC(this, core::mem::transmute_copy(&param0)) {
                Ok(ok__) => {
                    param1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::RestoreAllSurfaces(this).into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::TestCooperativeLevel(this).into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw4_Impl::GetDeviceIdentifier(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDraw7_Impl: Sized {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC2, param1: *mut Option<IDirectDrawSurface7>, param2: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: Option<&IDirectDrawSurface7>) -> windows_core::Result<IDirectDrawSurface7>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface7>;
    fn GetMonitorFrequency(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetScanLine(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetVerticalBlankStatus(&self, param0: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Initialize(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::Result<()>;
    fn GetSurfaceFromDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<IDirectDrawSurface7>;
    fn RestoreAllSurfaces(&self) -> windows_core::Result<()>;
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> windows_core::Result<()>;
    fn StartModeTest(&self, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn EvaluateMode(&self, param0: u32, param1: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDraw7 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDraw7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDraw7_Vtbl
    where
        Identity: IDirectDraw7_Impl,
    {
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::Compact(this).into()
        }
        unsafe extern "system" fn CreateClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn CreatePalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), windows_core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn DuplicateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw7_Impl::DuplicateSurface(this, windows_core::from_raw_borrowed(&param0)) {
                Ok(ok__) => {
                    param1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::FlipToGDISurface(this).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetGDISurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw7_Impl::GetGDISurface(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetMonitorFrequency(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetScanLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetScanLine(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetVerticalBlankStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::Initialize(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::RestoreDisplayMode(this).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetAvailableVidMem(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDraw7_Impl::GetSurfaceFromDC(this, core::mem::transmute_copy(&param0)) {
                Ok(ok__) => {
                    param1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::RestoreAllSurfaces(this).into()
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::TestCooperativeLevel(this).into()
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::GetDeviceIdentifier(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartModeTest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::StartModeTest(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EvaluateMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDraw7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDraw7_Impl::EvaluateMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, OFFSET>,
            StartModeTest: StartModeTest::<Identity, OFFSET>,
            EvaluateMode: EvaluateMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw7 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawClipper_Impl: Sized {
    fn GetClipList(&self, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> windows_core::Result<()>;
    fn GetHWnd(&self, param0: *mut super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: u32) -> windows_core::Result<()>;
    fn IsClipListChanged(&self, param0: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetClipList(&self, param0: *mut super::Gdi::RGNDATA, param1: u32) -> windows_core::Result<()>;
    fn SetHWnd(&self, param0: u32, param1: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawClipper {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawClipper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawClipper_Vtbl
    where
        Identity: IDirectDrawClipper_Impl,
    {
        unsafe extern "system" fn GetClipList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawClipper_Impl::GetClipList(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetHWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDirectDrawClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawClipper_Impl::GetHWnd(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawClipper_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsClipListChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectDrawClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawClipper_Impl::IsClipListChanged(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetClipList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawClipper_Impl::SetClipList(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetHWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDirectDrawClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawClipper_Impl::SetHWnd(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClipList: GetClipList::<Identity, OFFSET>,
            GetHWnd: GetHWnd::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsClipListChanged: IsClipListChanged::<Identity, OFFSET>,
            SetClipList: SetClipList::<Identity, OFFSET>,
            SetHWnd: SetHWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawClipper as windows_core::Interface>::IID
    }
}
pub trait IDirectDrawColorControl_Impl: Sized {
    fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::Result<()>;
    fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectDrawColorControl {}
impl IDirectDrawColorControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawColorControl_Vtbl
    where
        Identity: IDirectDrawColorControl_Impl,
    {
        unsafe extern "system" fn GetColorControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT
        where
            Identity: IDirectDrawColorControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawColorControl_Impl::GetColorControls(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT
        where
            Identity: IDirectDrawColorControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawColorControl_Impl::SetColorControls(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColorControls: GetColorControls::<Identity, OFFSET>,
            SetColorControls: SetColorControls::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawColorControl as windows_core::Interface>::IID
    }
}
pub trait IDirectDrawGammaControl_Impl: Sized {
    fn GetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectDrawGammaControl {}
impl IDirectDrawGammaControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawGammaControl_Vtbl
    where
        Identity: IDirectDrawGammaControl_Impl,
    {
        unsafe extern "system" fn GetGammaRamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::HRESULT
        where
            Identity: IDirectDrawGammaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawGammaControl_Impl::GetGammaRamp(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetGammaRamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::HRESULT
        where
            Identity: IDirectDrawGammaControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawGammaControl_Impl::SetGammaRamp(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGammaRamp: GetGammaRamp::<Identity, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawGammaControl as windows_core::Interface>::IID
    }
}
pub trait IDirectDrawKernel_Impl: Sized {
    fn GetCaps(&self, param0: *mut DDKERNELCAPS) -> windows_core::Result<()>;
    fn GetKernelHandle(&self, param0: *mut usize) -> windows_core::Result<()>;
    fn ReleaseKernelHandle(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectDrawKernel {}
impl IDirectDrawKernel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawKernel_Vtbl
    where
        Identity: IDirectDrawKernel_Impl,
    {
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDKERNELCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectDrawKernel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawKernel_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetKernelHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDirectDrawKernel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawKernel_Impl::GetKernelHandle(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawKernel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawKernel_Impl::ReleaseKernelHandle(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetKernelHandle: GetKernelHandle::<Identity, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawKernel as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawPalette_Impl: Sized {
    fn GetCaps(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn SetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawPalette {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawPalette_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawPalette_Vtbl
    where
        Identity: IDirectDrawPalette_Impl,
    {
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawPalette_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawPalette_Impl::GetEntries(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawPalette_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawPalette_Impl::SetEntries(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetEntries: GetEntries::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            SetEntries: SetEntries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawPalette as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface_Impl: Sized {
    fn AddAttachedSurface(&self, param0: Option<&IDirectDrawSurface>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: Option<&IDirectDrawSurface>, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: Option<&IDirectDrawSurface>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn Flip(&self, param0: Option<&IDirectDrawSurface>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut Option<IDirectDrawSurface>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS) -> windows_core::Result<()>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: Option<&IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: Option<&IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: Option<&IDirectDrawSurface>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawSurface {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawSurface_Vtbl
    where
        Identity: IDirectDrawSurface_Impl,
    {
        unsafe extern "system" fn AddAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::AddAttachedSurface(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::AddOverlayDirtyRect(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::Blt(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::Flip(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface_Impl::GetClipper(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface_Impl::GetPalette(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::IsLost(this).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::Restore(this).into()
        }
        unsafe extern "system" fn SetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::SetClipper(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::SetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::SetPalette(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface2_Impl: Sized {
    fn AddAttachedSurface(&self, param0: Option<&IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: Option<&IDirectDrawSurface2>, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: Option<&IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn Flip(&self, param0: Option<&IDirectDrawSurface2>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut Option<IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS) -> windows_core::Result<()>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: Option<&IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: Option<&IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: Option<&IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawSurface2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawSurface2_Vtbl
    where
        Identity: IDirectDrawSurface2_Impl,
    {
        unsafe extern "system" fn AddAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::AddAttachedSurface(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::AddOverlayDirtyRect(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::Blt(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::Flip(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface2_Impl::GetClipper(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface2_Impl::GetPalette(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::IsLost(this).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::Restore(this).into()
        }
        unsafe extern "system" fn SetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::SetClipper(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::SetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::SetPalette(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface2_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface3_Impl: Sized {
    fn AddAttachedSurface(&self, param0: Option<&IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: Option<&IDirectDrawSurface3>, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: Option<&IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn Flip(&self, param0: Option<&IDirectDrawSurface3>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut Option<IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS) -> windows_core::Result<()>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: Option<&IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: Option<&IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: Option<&IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC, param1: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawSurface3 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawSurface3_Vtbl
    where
        Identity: IDirectDrawSurface3_Impl,
    {
        unsafe extern "system" fn AddAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::AddAttachedSurface(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::AddOverlayDirtyRect(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::Blt(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::Flip(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface3_Impl::GetClipper(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface3_Impl::GetPalette(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::IsLost(this).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::Restore(this).into()
        }
        unsafe extern "system" fn SetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::SetClipper(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::SetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::SetPalette(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface3_Impl::SetSurfaceDesc(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface4_Impl: Sized {
    fn AddAttachedSurface(&self, param0: Option<&IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: Option<&IDirectDrawSurface4>, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: Option<&IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> windows_core::Result<()>;
    fn Flip(&self, param0: Option<&IDirectDrawSurface4>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut Option<IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS2) -> windows_core::Result<()>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: Option<&IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: Option<&IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: Option<&IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::Result<()>;
    fn SetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetUniquenessValue(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn ChangeUniquenessValue(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawSurface4 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawSurface4_Vtbl
    where
        Identity: IDirectDrawSurface4_Impl,
    {
        unsafe extern "system" fn AddAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::AddAttachedSurface(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::AddOverlayDirtyRect(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::Blt(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::Flip(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface4_Impl::GetClipper(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface4_Impl::GetPalette(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::IsLost(this).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::Restore(this).into()
        }
        unsafe extern "system" fn SetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::SetClipper(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::SetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::SetPalette(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::SetSurfaceDesc(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::SetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::FreePrivateData(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::GetUniquenessValue(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface4_Impl::ChangeUniquenessValue(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawSurface7_Impl: Sized {
    fn AddAttachedSurface(&self, param0: Option<&IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Blt(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: Option<&IDirectDrawSurface7>, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: Option<&IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> windows_core::Result<()>;
    fn Flip(&self, param0: Option<&IDirectDrawSurface7>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut Option<IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDSCAPS2) -> windows_core::Result<()>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn GetDC(&self, param0: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn Initialize(&self, param0: Option<&IDirectDraw>, param1: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::Gdi::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: Option<&IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::Result<()>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: Option<&IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::super::Foundation::RECT, param1: Option<&IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: Option<&IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::Result<()>;
    fn SetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetUniquenessValue(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn ChangeUniquenessValue(&self) -> windows_core::Result<()>;
    fn SetPriority(&self, param0: u32) -> windows_core::Result<()>;
    fn GetPriority(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SetLOD(&self, param0: u32) -> windows_core::Result<()>;
    fn GetLOD(&self, param0: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirectDrawSurface7 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirectDrawSurface7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawSurface7_Vtbl
    where
        Identity: IDirectDrawSurface7_Impl,
    {
        unsafe extern "system" fn AddAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::AddAttachedSurface(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::AddOverlayDirtyRect(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Blt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::Blt(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn BltBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn BltFast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), windows_core::from_raw_borrowed(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Flip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::Flip(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBltStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetCaps(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface7_Impl::GetClipper(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetFlipStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectDrawSurface7_Impl::GetPalette(this) {
                Ok(ok__) => {
                    param0.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::Initialize(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn IsLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::IsLost(this).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::Restore(this).into()
        }
        unsafe extern "system" fn SetClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetClipper(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn SetColorKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetColorKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetPalette(this, windows_core::from_raw_borrowed(&param0)).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), windows_core::from_raw_borrowed(&param1)).into()
        }
        unsafe extern "system" fn GetDDInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn PageUnlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetSurfaceDesc(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn FreePrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::FreePrivateData(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetUniquenessValue(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::ChangeUniquenessValue(this).into()
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetPriority(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetPriority(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetLOD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::SetLOD(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetLOD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurface7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurface7_Impl::GetLOD(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface7 as windows_core::Interface>::IID
    }
}
pub trait IDirectDrawSurfaceKernel_Impl: Sized {
    fn GetKernelHandle(&self, param0: *mut usize) -> windows_core::Result<()>;
    fn ReleaseKernelHandle(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectDrawSurfaceKernel {}
impl IDirectDrawSurfaceKernel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawSurfaceKernel_Vtbl
    where
        Identity: IDirectDrawSurfaceKernel_Impl,
    {
        unsafe extern "system" fn GetKernelHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurfaceKernel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurfaceKernel_Impl::GetKernelHandle(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawSurfaceKernel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawSurfaceKernel_Impl::ReleaseKernelHandle(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetKernelHandle: GetKernelHandle::<Identity, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurfaceKernel as windows_core::Interface>::IID
    }
}
pub trait IDirectDrawVideoPort_Impl: Sized {
    fn Flip(&self, param0: Option<&IDirectDrawSurface>, param1: u32) -> windows_core::Result<()>;
    fn GetBandwidthInfo(&self, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> windows_core::Result<()>;
    fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::Result<()>;
    fn GetInputFormats(&self, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> windows_core::Result<()>;
    fn GetOutputFormats(&self, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> windows_core::Result<()>;
    fn GetFieldPolarity(&self, param0: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetVideoLine(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn GetVideoSignalStatus(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::Result<()>;
    fn SetTargetSurface(&self, param0: Option<&IDirectDrawSurface>, param1: u32) -> windows_core::Result<()>;
    fn StartVideo(&self, param0: *mut DDVIDEOPORTINFO) -> windows_core::Result<()>;
    fn StopVideo(&self) -> windows_core::Result<()>;
    fn UpdateVideo(&self, param0: *mut DDVIDEOPORTINFO) -> windows_core::Result<()>;
    fn WaitForSync(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectDrawVideoPort {}
impl IDirectDrawVideoPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawVideoPort_Vtbl
    where
        Identity: IDirectDrawVideoPort_Impl,
    {
        unsafe extern "system" fn Flip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::Flip(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetBandwidthInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetBandwidthInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetColorControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetColorControls(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetInputFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetInputFormats(this, core::mem::transmute_copy(&lpnumformats), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetOutputFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetOutputFormats(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&lpnumformats), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetFieldPolarity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetFieldPolarity(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetVideoLine(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetVideoSignalStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::GetVideoSignalStatus(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetColorControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::SetColorControls(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetTargetSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::SetTargetSurface(this, windows_core::from_raw_borrowed(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::StartVideo(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn StopVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::StopVideo(this).into()
        }
        unsafe extern "system" fn UpdateVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::UpdateVideo(this, core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WaitForSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPort_Impl::WaitForSync(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flip: Flip::<Identity, OFFSET>,
            GetBandwidthInfo: GetBandwidthInfo::<Identity, OFFSET>,
            GetColorControls: GetColorControls::<Identity, OFFSET>,
            GetInputFormats: GetInputFormats::<Identity, OFFSET>,
            GetOutputFormats: GetOutputFormats::<Identity, OFFSET>,
            GetFieldPolarity: GetFieldPolarity::<Identity, OFFSET>,
            GetVideoLine: GetVideoLine::<Identity, OFFSET>,
            GetVideoSignalStatus: GetVideoSignalStatus::<Identity, OFFSET>,
            SetColorControls: SetColorControls::<Identity, OFFSET>,
            SetTargetSurface: SetTargetSurface::<Identity, OFFSET>,
            StartVideo: StartVideo::<Identity, OFFSET>,
            StopVideo: StopVideo::<Identity, OFFSET>,
            UpdateVideo: UpdateVideo::<Identity, OFFSET>,
            WaitForSync: WaitForSync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawVideoPort as windows_core::Interface>::IID
    }
}
pub trait IDirectDrawVideoPortNotify_Impl: Sized {
    fn AcquireNotification(&self, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> windows_core::Result<()>;
    fn ReleaseNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectDrawVideoPortNotify {}
impl IDirectDrawVideoPortNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectDrawVideoPortNotify_Vtbl
    where
        Identity: IDirectDrawVideoPortNotify_Impl,
    {
        unsafe extern "system" fn AcquireNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPortNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPortNotify_Impl::AcquireNotification(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn ReleaseNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectDrawVideoPortNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectDrawVideoPortNotify_Impl::ReleaseNotification(this, core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireNotification: AcquireNotification::<Identity, OFFSET>,
            ReleaseNotification: ReleaseNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawVideoPortNotify as windows_core::Interface>::IID
    }
}

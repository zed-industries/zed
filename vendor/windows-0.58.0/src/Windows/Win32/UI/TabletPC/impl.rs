#[cfg(feature = "Win32_System_Com")]
pub trait IDynamicRenderer_Impl: Sized {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, benabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HWND(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetHWND(&self, hwnd: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn ClipRectangle(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn SetClipRectangle(&self, prccliprect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn ClipRegion(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetClipRegion(&self, hcliprgn: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn DrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, pida: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn DataCacheEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDataCacheEnabled(&self, fcachedata: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReleaseCachedData(&self, strokeid: u32) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Draw(&self, hdc: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDynamicRenderer {}
#[cfg(feature = "Win32_System_Com")]
impl IDynamicRenderer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDynamicRenderer_Vtbl
    where
        Identity: IDynamicRenderer_Impl,
    {
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicRenderer_Impl::Enabled(this) {
                Ok(ok__) => {
                    benabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::SetEnabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn HWND<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicRenderer_Impl::HWND(this) {
                Ok(ok__) => {
                    hwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::SetHWND(this, core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn ClipRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prccliprect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicRenderer_Impl::ClipRectangle(this) {
                Ok(ok__) => {
                    prccliprect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prccliprect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::SetClipRectangle(this, core::mem::transmute_copy(&prccliprect)).into()
        }
        unsafe extern "system" fn ClipRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicRenderer_Impl::ClipRegion(this) {
                Ok(ok__) => {
                    phcliprgn.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcliprgn: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::SetClipRegion(this, core::mem::transmute_copy(&hcliprgn)).into()
        }
        unsafe extern "system" fn DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppida: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicRenderer_Impl::DrawingAttributes(this) {
                Ok(ok__) => {
                    ppida.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pida: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::putref_DrawingAttributes(this, windows_core::from_raw_borrowed(&pida)).into()
        }
        unsafe extern "system" fn DataCacheEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcachedata: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicRenderer_Impl::DataCacheEnabled(this) {
                Ok(ok__) => {
                    pfcachedata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCacheEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcachedata: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::SetDataCacheEnabled(this, core::mem::transmute_copy(&fcachedata)).into()
        }
        unsafe extern "system" fn ReleaseCachedData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeid: u32) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::ReleaseCachedData(this, core::mem::transmute_copy(&strokeid)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IDynamicRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicRenderer_Impl::Draw(this, core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            HWND: HWND::<Identity, OFFSET>,
            SetHWND: SetHWND::<Identity, OFFSET>,
            ClipRectangle: ClipRectangle::<Identity, OFFSET>,
            SetClipRectangle: SetClipRectangle::<Identity, OFFSET>,
            ClipRegion: ClipRegion::<Identity, OFFSET>,
            SetClipRegion: SetClipRegion::<Identity, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, OFFSET>,
            DataCacheEnabled: DataCacheEnabled::<Identity, OFFSET>,
            SetDataCacheEnabled: SetDataCacheEnabled::<Identity, OFFSET>,
            ReleaseCachedData: ReleaseCachedData::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicRenderer as windows_core::Interface>::IID
    }
}
pub trait IGestureRecognizer_Impl: Sized {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn MaxStrokeCount(&self) -> windows_core::Result<i32>;
    fn SetMaxStrokeCount(&self, cstrokes: i32) -> windows_core::Result<()>;
    fn EnableGestures(&self, cgestures: u32, pgestures: *const i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGestureRecognizer {}
impl IGestureRecognizer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGestureRecognizer_Vtbl
    where
        Identity: IGestureRecognizer_Impl,
    {
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IGestureRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGestureRecognizer_Impl::Enabled(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IGestureRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGestureRecognizer_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn MaxStrokeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcstrokes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IGestureRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGestureRecognizer_Impl::MaxStrokeCount(this) {
                Ok(ok__) => {
                    pcstrokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStrokeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cstrokes: i32) -> windows_core::HRESULT
        where
            Identity: IGestureRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGestureRecognizer_Impl::SetMaxStrokeCount(this, core::mem::transmute_copy(&cstrokes)).into()
        }
        unsafe extern "system" fn EnableGestures<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cgestures: u32, pgestures: *const i32) -> windows_core::HRESULT
        where
            Identity: IGestureRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGestureRecognizer_Impl::EnableGestures(this, core::mem::transmute_copy(&cgestures), core::mem::transmute_copy(&pgestures)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGestureRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGestureRecognizer_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            MaxStrokeCount: MaxStrokeCount::<Identity, OFFSET>,
            SetMaxStrokeCount: SetMaxStrokeCount::<Identity, OFFSET>,
            EnableGestures: EnableGestures::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGestureRecognizer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHandwrittenTextInsertion_Impl: Sized {
    fn InsertRecognitionResultsArray(&self, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InsertInkRecognitionResult(&self, piinkrecoresult: Option<&IInkRecognitionResult>, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHandwrittenTextInsertion {}
#[cfg(feature = "Win32_System_Com")]
impl IHandwrittenTextInsertion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHandwrittenTextInsertion_Vtbl
    where
        Identity: IHandwrittenTextInsertion_Impl,
    {
        unsafe extern "system" fn InsertRecognitionResultsArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHandwrittenTextInsertion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHandwrittenTextInsertion_Impl::InsertRecognitionResultsArray(this, core::mem::transmute_copy(&psaalternates), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piinkrecoresult: *mut core::ffi::c_void, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHandwrittenTextInsertion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHandwrittenTextInsertion_Impl::InsertInkRecognitionResult(this, windows_core::from_raw_borrowed(&piinkrecoresult), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertRecognitionResultsArray: InsertRecognitionResultsArray::<Identity, OFFSET>,
            InsertInkRecognitionResult: InsertInkRecognitionResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHandwrittenTextInsertion as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInk_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInk {}
#[cfg(feature = "Win32_System_Com")]
impl IInk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInk_Vtbl
    where
        Identity: IInk_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInk as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkCollector_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&self) -> windows_core::Result<isize>;
    fn SethWnd(&self, newwindow: isize) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, collecting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DefaultDrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&self, newattributes: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn Renderer(&self) -> windows_core::Result<IInkRenderer>;
    fn putref_Renderer(&self, newinkrenderer: Option<&IInkRenderer>) -> windows_core::Result<()>;
    fn Ink(&self) -> windows_core::Result<IInkDisp>;
    fn putref_Ink(&self, newink: Option<&IInkDisp>) -> windows_core::Result<()>;
    fn AutoRedraw(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoRedraw(&self, autoredraw: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CollectingInk(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CollectionMode(&self) -> windows_core::Result<InkCollectionMode>;
    fn SetCollectionMode(&self, mode: InkCollectionMode) -> windows_core::Result<()>;
    fn DynamicRendering(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDynamicRendering(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DesiredPacketDescription(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDesiredPacketDescription(&self, packetguids: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MouseIcon(&self) -> windows_core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn MousePointer(&self) -> windows_core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> windows_core::Result<()>;
    fn Cursors(&self) -> windows_core::Result<IInkCursors>;
    fn MarginX(&self) -> windows_core::Result<i32>;
    fn SetMarginX(&self, marginx: i32) -> windows_core::Result<()>;
    fn MarginY(&self) -> windows_core::Result<i32>;
    fn SetMarginY(&self, marginy: i32) -> windows_core::Result<()>;
    fn Tablet(&self) -> windows_core::Result<IInkTablet>;
    fn SupportHighContrastInk(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSupportHighContrastInk(&self, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetWindowInputRectangle(&self, windowinputrectangle: *mut Option<IInkRectangle>) -> windows_core::Result<()>;
    fn SetWindowInputRectangle(&self, windowinputrectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn SetAllTabletsMode(&self, usemouseforinput: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetSingleTabletIntegratedMode(&self, tablet: Option<&IInkTablet>) -> windows_core::Result<()>;
    fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IInkCollector {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkCollector_Vtbl
    where
        Identity: IInkCollector_Impl,
    {
        unsafe extern "system" fn hWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentwindow: *mut isize) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::hWnd(this) {
                Ok(ok__) => {
                    currentwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newwindow: isize) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SethWnd(this, core::mem::transmute_copy(&newwindow)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::Enabled(this) {
                Ok(ok__) => {
                    collecting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetEnabled(this, core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::DefaultDrawingAttributes(this) {
                Ok(ok__) => {
                    currentattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newattributes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::putref_DefaultDrawingAttributes(this, windows_core::from_raw_borrowed(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentinkrenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::Renderer(this) {
                Ok(ok__) => {
                    currentinkrenderer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinkrenderer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::putref_Renderer(this, windows_core::from_raw_borrowed(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::Ink(this) {
                Ok(ok__) => {
                    ink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::putref_Ink(this, windows_core::from_raw_borrowed(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoredraw: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::AutoRedraw(this) {
                Ok(ok__) => {
                    autoredraw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoredraw: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetAutoRedraw(this, core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::CollectingInk(this) {
                Ok(ok__) => {
                    collecting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut InkCollectionMode) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::CollectionMode(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: InkCollectionMode) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetCollectionMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::DynamicRendering(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetDynamicRendering(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetguids: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::DesiredPacketDescription(this) {
                Ok(ok__) => {
                    packetguids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetguids: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetDesiredPacketDescription(this, core::mem::transmute(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::MouseIcon(this) {
                Ok(ok__) => {
                    mouseicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetMouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::putref_MouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: *mut InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::MousePointer(this) {
                Ok(ok__) => {
                    mousepointer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetMousePointer(this, core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn Cursors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cursors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::Cursors(this) {
                Ok(ok__) => {
                    cursors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginx: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::MarginX(this) {
                Ok(ok__) => {
                    marginx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginx: i32) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetMarginX(this, core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::MarginY(this) {
                Ok(ok__) => {
                    marginy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginy: i32) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetMarginY(this, core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, singletablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::Tablet(this) {
                Ok(ok__) => {
                    singletablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::SupportHighContrastInk(this) {
                Ok(ok__) => {
                    support.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetSupportHighContrastInk(this, core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetGestureStatus(this, core::mem::transmute_copy(&gesture), core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::GetGestureStatus(this, core::mem::transmute_copy(&gesture)) {
                Ok(ok__) => {
                    listening.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinputrectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::GetWindowInputRectangle(this, core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinputrectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetWindowInputRectangle(this, windows_core::from_raw_borrowed(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usemouseforinput: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetAllTabletsMode(this, core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablet: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetSingleTabletIntegratedMode(this, windows_core::from_raw_borrowed(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCollector_Impl::GetEventInterest(this, core::mem::transmute_copy(&eventid)) {
                Ok(ok__) => {
                    listen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: InkCollectorEventInterest, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCollector_Impl::SetEventInterest(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&listen)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            hWnd: hWnd::<Identity, OFFSET>,
            SethWnd: SethWnd::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            DefaultDrawingAttributes: DefaultDrawingAttributes::<Identity, OFFSET>,
            putref_DefaultDrawingAttributes: putref_DefaultDrawingAttributes::<Identity, OFFSET>,
            Renderer: Renderer::<Identity, OFFSET>,
            putref_Renderer: putref_Renderer::<Identity, OFFSET>,
            Ink: Ink::<Identity, OFFSET>,
            putref_Ink: putref_Ink::<Identity, OFFSET>,
            AutoRedraw: AutoRedraw::<Identity, OFFSET>,
            SetAutoRedraw: SetAutoRedraw::<Identity, OFFSET>,
            CollectingInk: CollectingInk::<Identity, OFFSET>,
            CollectionMode: CollectionMode::<Identity, OFFSET>,
            SetCollectionMode: SetCollectionMode::<Identity, OFFSET>,
            DynamicRendering: DynamicRendering::<Identity, OFFSET>,
            SetDynamicRendering: SetDynamicRendering::<Identity, OFFSET>,
            DesiredPacketDescription: DesiredPacketDescription::<Identity, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, OFFSET>,
            MouseIcon: MouseIcon::<Identity, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, OFFSET>,
            MousePointer: MousePointer::<Identity, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, OFFSET>,
            Cursors: Cursors::<Identity, OFFSET>,
            MarginX: MarginX::<Identity, OFFSET>,
            SetMarginX: SetMarginX::<Identity, OFFSET>,
            MarginY: MarginY::<Identity, OFFSET>,
            SetMarginY: SetMarginY::<Identity, OFFSET>,
            Tablet: Tablet::<Identity, OFFSET>,
            SupportHighContrastInk: SupportHighContrastInk::<Identity, OFFSET>,
            SetSupportHighContrastInk: SetSupportHighContrastInk::<Identity, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, OFFSET>,
            GetWindowInputRectangle: GetWindowInputRectangle::<Identity, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, OFFSET>,
            SetSingleTabletIntegratedMode: SetSingleTabletIntegratedMode::<Identity, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCollector as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<i32>;
    fn Inverted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, attributes: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn Tablet(&self) -> windows_core::Result<IInkTablet>;
    fn Buttons(&self) -> windows_core::Result<IInkCursorButtons>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkCursor {}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkCursor_Vtbl
    where
        Identity: IInkCursor_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursor_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursor_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursor_Impl::Inverted(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursor_Impl::DrawingAttributes(this) {
                Ok(ok__) => {
                    attributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCursor_Impl::putref_DrawingAttributes(this, windows_core::from_raw_borrowed(&attributes)).into()
        }
        unsafe extern "system" fn Tablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursor_Impl::Tablet(this) {
                Ok(ok__) => {
                    tablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buttons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buttons: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursor_Impl::Buttons(this) {
                Ok(ok__) => {
                    buttons.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Inverted: Inverted::<Identity, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, OFFSET>,
            Tablet: Tablet::<Identity, OFFSET>,
            Buttons: Buttons::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCursor as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorButton_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<InkCursorButtonState>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkCursorButton {}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursorButton_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkCursorButton_Vtbl
    where
        Identity: IInkCursorButton_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkCursorButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursorButton_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkCursorButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursorButton_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentstate: *mut InkCursorButtonState) -> windows_core::HRESULT
        where
            Identity: IInkCursorButton_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursorButton_Impl::State(this) {
                Ok(ok__) => {
                    currentstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCursorButton as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorButtons_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, identifier: &windows_core::VARIANT) -> windows_core::Result<IInkCursorButton>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkCursorButtons {}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursorButtons_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkCursorButtons_Vtbl
    where
        Identity: IInkCursorButtons_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkCursorButtons_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursorButtons_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursorButtons_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursorButtons_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: core::mem::MaybeUninit<windows_core::VARIANT>, button: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursorButtons_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursorButtons_Impl::Item(this, core::mem::transmute(&identifier)) {
                Ok(ok__) => {
                    button.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCursorButtons as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursors_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<IInkCursor>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkCursors {}
#[cfg(feature = "Win32_System_Com")]
impl IInkCursors_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkCursors_Vtbl
    where
        Identity: IInkCursors_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkCursors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursors_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursors_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, cursor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCursors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCursors_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    cursor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCursors as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCustomStrokes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, identifier: &windows_core::VARIANT) -> windows_core::Result<IInkStrokes>;
    fn Add(&self, name: &windows_core::BSTR, strokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn Remove(&self, identifier: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkCustomStrokes {}
#[cfg(feature = "Win32_System_Com")]
impl IInkCustomStrokes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkCustomStrokes_Vtbl
    where
        Identity: IInkCustomStrokes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkCustomStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCustomStrokes_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCustomStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCustomStrokes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: core::mem::MaybeUninit<windows_core::VARIANT>, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCustomStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkCustomStrokes_Impl::Item(this, core::mem::transmute(&identifier)) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, strokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCustomStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCustomStrokes_Impl::Add(this, core::mem::transmute(&name), windows_core::from_raw_borrowed(&strokes)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkCustomStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCustomStrokes_Impl::Remove(this, core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkCustomStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkCustomStrokes_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkCustomStrokes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDisp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn ExtendedProperties(&self) -> windows_core::Result<IInkExtendedProperties>;
    fn Dirty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDirty(&self, dirty: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CustomStrokes(&self) -> windows_core::Result<IInkCustomStrokes>;
    fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> windows_core::Result<IInkRectangle>;
    fn DeleteStrokes(&self, strokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn DeleteStroke(&self, stroke: Option<&IInkStrokeDisp>) -> windows_core::Result<()>;
    fn ExtractStrokes(&self, strokes: Option<&IInkStrokes>, extractflags: InkExtractFlags) -> windows_core::Result<IInkDisp>;
    fn ExtractWithRectangle(&self, rectangle: Option<&IInkRectangle>, extractflags: InkExtractFlags) -> windows_core::Result<IInkDisp>;
    fn Clip(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IInkDisp>;
    fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> windows_core::Result<IInkStrokes>;
    fn HitTestWithRectangle(&self, selectionrectangle: Option<&IInkRectangle>, intersectpercent: f32) -> windows_core::Result<IInkStrokes>;
    fn HitTestWithLasso(&self, points: &windows_core::VARIANT, intersectpercent: f32, lassopoints: *mut windows_core::VARIANT, strokes: *mut Option<IInkStrokes>) -> windows_core::Result<()>;
    fn NearestPoint(&self, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut Option<IInkStrokeDisp>) -> windows_core::Result<()>;
    fn CreateStrokes(&self, strokeids: &windows_core::VARIANT) -> windows_core::Result<IInkStrokes>;
    fn AddStrokesAtRectangle(&self, sourcestrokes: Option<&IInkStrokes>, targetrectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn Save(&self, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode) -> windows_core::Result<windows_core::VARIANT>;
    fn Load(&self, data: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn CreateStroke(&self, packetdata: &windows_core::VARIANT, packetdescription: &windows_core::VARIANT) -> windows_core::Result<IInkStrokeDisp>;
    fn ClipboardCopyWithRectangle(&self, rectangle: Option<&IInkRectangle>, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn ClipboardCopy(&self, strokes: Option<&IInkStrokes>, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> windows_core::Result<super::super::System::Com::IDataObject>;
    fn CanPaste(&self, dataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClipboardPaste(&self, x: i32, y: i32, dataobject: Option<&super::super::System::Com::IDataObject>) -> windows_core::Result<IInkStrokes>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkDisp {}
#[cfg(feature = "Win32_System_Com")]
impl IInkDisp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkDisp_Vtbl
    where
        Identity: IInkDisp_Impl,
    {
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::ExtendedProperties(this) {
                Ok(ok__) => {
                    properties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::Dirty(this) {
                Ok(ok__) => {
                    dirty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirty: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::SetDirty(this, core::mem::transmute_copy(&dirty)).into()
        }
        unsafe extern "system" fn CustomStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkinkcustomstrokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::CustomStrokes(this) {
                Ok(ok__) => {
                    ppunkinkcustomstrokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::GetBoundingBox(this, core::mem::transmute_copy(&boundingboxmode)) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::DeleteStrokes(this, windows_core::from_raw_borrowed(&strokes)).into()
        }
        unsafe extern "system" fn DeleteStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stroke: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::DeleteStroke(this, windows_core::from_raw_borrowed(&stroke)).into()
        }
        unsafe extern "system" fn ExtractStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void, extractflags: InkExtractFlags, extractedink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::ExtractStrokes(this, windows_core::from_raw_borrowed(&strokes), core::mem::transmute_copy(&extractflags)) {
                Ok(ok__) => {
                    extractedink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractWithRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void, extractflags: InkExtractFlags, extractedink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::ExtractWithRectangle(this, windows_core::from_raw_borrowed(&rectangle), core::mem::transmute_copy(&extractflags)) {
                Ok(ok__) => {
                    extractedink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::Clip(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::Clone(this) {
                Ok(ok__) => {
                    newink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestCircle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, radius: f32, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::HitTestCircle(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&radius)) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionrectangle: *mut core::ffi::c_void, intersectpercent: f32, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::HitTestWithRectangle(this, windows_core::from_raw_borrowed(&selectionrectangle), core::mem::transmute_copy(&intersectpercent)) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestWithLasso<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: core::mem::MaybeUninit<windows_core::VARIANT>, intersectpercent: f32, lassopoints: *mut core::mem::MaybeUninit<windows_core::VARIANT>, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::HitTestWithLasso(this, core::mem::transmute(&points), core::mem::transmute_copy(&intersectpercent), core::mem::transmute_copy(&lassopoints), core::mem::transmute_copy(&strokes)).into()
        }
        unsafe extern "system" fn NearestPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::NearestPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&pointonstroke), core::mem::transmute_copy(&distancefrompacket), core::mem::transmute_copy(&stroke)).into()
        }
        unsafe extern "system" fn CreateStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokeids: core::mem::MaybeUninit<windows_core::VARIANT>, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::CreateStrokes(this, core::mem::transmute(&strokeids)) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStrokesAtRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestrokes: *mut core::ffi::c_void, targetrectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::AddStrokesAtRectangle(this, windows_core::from_raw_borrowed(&sourcestrokes), windows_core::from_raw_borrowed(&targetrectangle)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::Save(this, core::mem::transmute_copy(&persistenceformat), core::mem::transmute_copy(&compressionmode)) {
                Ok(ok__) => {
                    data.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDisp_Impl::Load(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn CreateStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetdata: core::mem::MaybeUninit<windows_core::VARIANT>, packetdescription: core::mem::MaybeUninit<windows_core::VARIANT>, stroke: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::CreateStroke(this, core::mem::transmute(&packetdata), core::mem::transmute(&packetdescription)) {
                Ok(ok__) => {
                    stroke.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopyWithRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::ClipboardCopyWithRectangle(this, windows_core::from_raw_borrowed(&rectangle), core::mem::transmute_copy(&clipboardformats), core::mem::transmute_copy(&clipboardmodes)) {
                Ok(ok__) => {
                    dataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardCopy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::ClipboardCopy(this, windows_core::from_raw_borrowed(&strokes), core::mem::transmute_copy(&clipboardformats), core::mem::transmute_copy(&clipboardmodes)) {
                Ok(ok__) => {
                    dataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataobject: *mut core::ffi::c_void, canpaste: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::CanPaste(this, windows_core::from_raw_borrowed(&dataobject)) {
                Ok(ok__) => {
                    canpaste.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipboardPaste<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, dataobject: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDisp_Impl::ClipboardPaste(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), windows_core::from_raw_borrowed(&dataobject)) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Strokes: Strokes::<Identity, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, OFFSET>,
            Dirty: Dirty::<Identity, OFFSET>,
            SetDirty: SetDirty::<Identity, OFFSET>,
            CustomStrokes: CustomStrokes::<Identity, OFFSET>,
            GetBoundingBox: GetBoundingBox::<Identity, OFFSET>,
            DeleteStrokes: DeleteStrokes::<Identity, OFFSET>,
            DeleteStroke: DeleteStroke::<Identity, OFFSET>,
            ExtractStrokes: ExtractStrokes::<Identity, OFFSET>,
            ExtractWithRectangle: ExtractWithRectangle::<Identity, OFFSET>,
            Clip: Clip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            HitTestCircle: HitTestCircle::<Identity, OFFSET>,
            HitTestWithRectangle: HitTestWithRectangle::<Identity, OFFSET>,
            HitTestWithLasso: HitTestWithLasso::<Identity, OFFSET>,
            NearestPoint: NearestPoint::<Identity, OFFSET>,
            CreateStrokes: CreateStrokes::<Identity, OFFSET>,
            AddStrokesAtRectangle: AddStrokesAtRectangle::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            CreateStroke: CreateStroke::<Identity, OFFSET>,
            ClipboardCopyWithRectangle: ClipboardCopyWithRectangle::<Identity, OFFSET>,
            ClipboardCopy: ClipboardCopy::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            ClipboardPaste: ClipboardPaste::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDisp as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivider_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn putref_Strokes(&self, strokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn RecognizerContext(&self) -> windows_core::Result<IInkRecognizerContext>;
    fn putref_RecognizerContext(&self, recognizercontext: Option<&IInkRecognizerContext>) -> windows_core::Result<()>;
    fn LineHeight(&self) -> windows_core::Result<i32>;
    fn SetLineHeight(&self, lineheight: i32) -> windows_core::Result<()>;
    fn Divide(&self) -> windows_core::Result<IInkDivisionResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkDivider {}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkDivider_Vtbl
    where
        Identity: IInkDivider_Impl,
    {
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivider_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDivider_Impl::putref_Strokes(this, windows_core::from_raw_borrowed(&strokes)).into()
        }
        unsafe extern "system" fn RecognizerContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizercontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivider_Impl::RecognizerContext(this) {
                Ok(ok__) => {
                    recognizercontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RecognizerContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizercontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDivider_Impl::putref_RecognizerContext(this, windows_core::from_raw_borrowed(&recognizercontext)).into()
        }
        unsafe extern "system" fn LineHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineheight: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivider_Impl::LineHeight(this) {
                Ok(ok__) => {
                    lineheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineheight: i32) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDivider_Impl::SetLineHeight(this, core::mem::transmute_copy(&lineheight)).into()
        }
        unsafe extern "system" fn Divide<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkdivisionresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivider_Impl::Divide(this) {
                Ok(ok__) => {
                    inkdivisionresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Strokes: Strokes::<Identity, OFFSET>,
            putref_Strokes: putref_Strokes::<Identity, OFFSET>,
            RecognizerContext: RecognizerContext::<Identity, OFFSET>,
            putref_RecognizerContext: putref_RecognizerContext::<Identity, OFFSET>,
            LineHeight: LineHeight::<Identity, OFFSET>,
            SetLineHeight: SetLineHeight::<Identity, OFFSET>,
            Divide: Divide::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDivider as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn ResultByType(&self, divisiontype: InkDivisionType) -> windows_core::Result<IInkDivisionUnits>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkDivisionResult {}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivisionResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkDivisionResult_Vtbl
    where
        Identity: IInkDivisionResult_Impl,
    {
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivisionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionResult_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, divisiontype: InkDivisionType, inkdivisionunits: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivisionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionResult_Impl::ResultByType(this, core::mem::transmute_copy(&divisiontype)) {
                Ok(ok__) => {
                    inkdivisionunits.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Strokes: Strokes::<Identity, OFFSET>,
            ResultByType: ResultByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDivisionResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionUnit_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn DivisionType(&self) -> windows_core::Result<InkDivisionType>;
    fn RecognizedString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RotationTransform(&self) -> windows_core::Result<IInkTransform>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkDivisionUnit {}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivisionUnit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkDivisionUnit_Vtbl
    where
        Identity: IInkDivisionUnit_Impl,
    {
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnit_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DivisionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, divisiontype: *mut InkDivisionType) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnit_Impl::DivisionType(this) {
                Ok(ok__) => {
                    divisiontype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizedString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recostring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnit_Impl::RecognizedString(this) {
                Ok(ok__) => {
                    recostring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotationtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnit_Impl::RotationTransform(this) {
                Ok(ok__) => {
                    rotationtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Strokes: Strokes::<Identity, OFFSET>,
            DivisionType: DivisionType::<Identity, OFFSET>,
            RecognizedString: RecognizedString::<Identity, OFFSET>,
            RotationTransform: RotationTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDivisionUnit as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionUnits_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<IInkDivisionUnit>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkDivisionUnits {}
#[cfg(feature = "Win32_System_Com")]
impl IInkDivisionUnits_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkDivisionUnits_Vtbl
    where
        Identity: IInkDivisionUnits_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnits_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnits_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, inkdivisionunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDivisionUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDivisionUnits_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    inkdivisionunit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDivisionUnits as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDrawingAttributes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Color(&self) -> windows_core::Result<i32>;
    fn SetColor(&self, newcolor: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<f32>;
    fn SetWidth(&self, newwidth: f32) -> windows_core::Result<()>;
    fn Height(&self) -> windows_core::Result<f32>;
    fn SetHeight(&self, newheight: f32) -> windows_core::Result<()>;
    fn FitToCurve(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFitToCurve(&self, flag: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IgnorePressure(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIgnorePressure(&self, flag: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AntiAliased(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAntiAliased(&self, flag: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Transparency(&self) -> windows_core::Result<i32>;
    fn SetTransparency(&self, newtransparency: i32) -> windows_core::Result<()>;
    fn RasterOperation(&self) -> windows_core::Result<InkRasterOperation>;
    fn SetRasterOperation(&self, newrasteroperation: InkRasterOperation) -> windows_core::Result<()>;
    fn PenTip(&self) -> windows_core::Result<InkPenTip>;
    fn SetPenTip(&self, newpentip: InkPenTip) -> windows_core::Result<()>;
    fn ExtendedProperties(&self) -> windows_core::Result<IInkExtendedProperties>;
    fn Clone(&self) -> windows_core::Result<IInkDrawingAttributes>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkDrawingAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl IInkDrawingAttributes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkDrawingAttributes_Vtbl
    where
        Identity: IInkDrawingAttributes_Impl,
    {
        unsafe extern "system" fn Color<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentcolor: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::Color(this) {
                Ok(ok__) => {
                    currentcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcolor: i32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetColor(this, core::mem::transmute_copy(&newcolor)).into()
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentwidth: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::Width(this) {
                Ok(ok__) => {
                    currentwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newwidth: f32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetWidth(this, core::mem::transmute_copy(&newwidth)).into()
        }
        unsafe extern "system" fn Height<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentheight: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::Height(this) {
                Ok(ok__) => {
                    currentheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newheight: f32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetHeight(this, core::mem::transmute_copy(&newheight)).into()
        }
        unsafe extern "system" fn FitToCurve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::FitToCurve(this) {
                Ok(ok__) => {
                    flag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFitToCurve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetFitToCurve(this, core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn IgnorePressure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::IgnorePressure(this) {
                Ok(ok__) => {
                    flag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnorePressure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetIgnorePressure(this, core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn AntiAliased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::AntiAliased(this) {
                Ok(ok__) => {
                    flag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiAliased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetAntiAliased(this, core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn Transparency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currenttransparency: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::Transparency(this) {
                Ok(ok__) => {
                    currenttransparency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransparency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newtransparency: i32) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetTransparency(this, core::mem::transmute_copy(&newtransparency)).into()
        }
        unsafe extern "system" fn RasterOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentrasteroperation: *mut InkRasterOperation) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::RasterOperation(this) {
                Ok(ok__) => {
                    currentrasteroperation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRasterOperation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newrasteroperation: InkRasterOperation) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetRasterOperation(this, core::mem::transmute_copy(&newrasteroperation)).into()
        }
        unsafe extern "system" fn PenTip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpentip: *mut InkPenTip) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::PenTip(this) {
                Ok(ok__) => {
                    currentpentip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPenTip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newpentip: InkPenTip) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkDrawingAttributes_Impl::SetPenTip(this, core::mem::transmute_copy(&newpentip)).into()
        }
        unsafe extern "system" fn ExtendedProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::ExtendedProperties(this) {
                Ok(ok__) => {
                    properties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkDrawingAttributes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkDrawingAttributes_Impl::Clone(this) {
                Ok(ok__) => {
                    drawingattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Color: Color::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            SetHeight: SetHeight::<Identity, OFFSET>,
            FitToCurve: FitToCurve::<Identity, OFFSET>,
            SetFitToCurve: SetFitToCurve::<Identity, OFFSET>,
            IgnorePressure: IgnorePressure::<Identity, OFFSET>,
            SetIgnorePressure: SetIgnorePressure::<Identity, OFFSET>,
            AntiAliased: AntiAliased::<Identity, OFFSET>,
            SetAntiAliased: SetAntiAliased::<Identity, OFFSET>,
            Transparency: Transparency::<Identity, OFFSET>,
            SetTransparency: SetTransparency::<Identity, OFFSET>,
            RasterOperation: RasterOperation::<Identity, OFFSET>,
            SetRasterOperation: SetRasterOperation::<Identity, OFFSET>,
            PenTip: PenTip::<Identity, OFFSET>,
            SetPenTip: SetPenTip::<Identity, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkDrawingAttributes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkEdit_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> windows_core::Result<InkEditStatus>;
    fn UseMouseForInput(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseMouseForInput(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn InkMode(&self) -> windows_core::Result<InkMode>;
    fn SetInkMode(&self, newval: InkMode) -> windows_core::Result<()>;
    fn InkInsertMode(&self) -> windows_core::Result<InkInsertMode>;
    fn SetInkInsertMode(&self, newval: InkInsertMode) -> windows_core::Result<()>;
    fn DrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, newval: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn RecognitionTimeout(&self) -> windows_core::Result<i32>;
    fn SetRecognitionTimeout(&self, newval: i32) -> windows_core::Result<()>;
    fn Recognizer(&self) -> windows_core::Result<IInkRecognizer>;
    fn putref_Recognizer(&self, newval: Option<&IInkRecognizer>) -> windows_core::Result<()>;
    fn Factoid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFactoid(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SelInks(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelInks(&self, selink: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelInksDisplayMode(&self) -> windows_core::Result<InkDisplayMode>;
    fn SetSelInksDisplayMode(&self, inkdisplaymode: InkDisplayMode) -> windows_core::Result<()>;
    fn Recognize(&self) -> windows_core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetBackColor(&self, clr: u32) -> windows_core::Result<()>;
    fn BackColor(&self) -> windows_core::Result<u32>;
    fn Appearance(&self) -> windows_core::Result<AppearanceConstants>;
    fn SetAppearance(&self, pappearance: AppearanceConstants) -> windows_core::Result<()>;
    fn BorderStyle(&self) -> windows_core::Result<BorderStyleConstants>;
    fn SetBorderStyle(&self, pborderstyle: BorderStyleConstants) -> windows_core::Result<()>;
    fn Hwnd(&self) -> windows_core::Result<super::super::System::Ole::OLE_HANDLE>;
    fn Font(&self) -> windows_core::Result<super::super::System::Ole::IFontDisp>;
    fn putref_Font(&self, ppfont: Option<&super::super::System::Ole::IFontDisp>) -> windows_core::Result<()>;
    fn Text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetText(&self, pbstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MouseIcon(&self) -> windows_core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn MousePointer(&self) -> windows_core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> windows_core::Result<()>;
    fn Locked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLocked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MaxLength(&self) -> windows_core::Result<i32>;
    fn SetMaxLength(&self, lmaxlength: i32) -> windows_core::Result<()>;
    fn MultiLine(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMultiLine(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ScrollBars(&self) -> windows_core::Result<ScrollBarsConstants>;
    fn SetScrollBars(&self, newval: ScrollBarsConstants) -> windows_core::Result<()>;
    fn DisableNoScroll(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableNoScroll(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SelAlignment(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelAlignment(&self, pvarselalignment: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelBold(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelBold(&self, pvarselbold: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelItalic(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelItalic(&self, pvarselitalic: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelUnderline(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelUnderline(&self, pvarselunderline: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelColor(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelColor(&self, pvarselcolor: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelFontName(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelFontName(&self, pvarselfontname: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelFontSize(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelFontSize(&self, pvarselfontsize: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SelCharOffset(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSelCharOffset(&self, pvarselcharoffset: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn TextRTF(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTextRTF(&self, pbstrtextrtf: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SelStart(&self) -> windows_core::Result<i32>;
    fn SetSelStart(&self, plselstart: i32) -> windows_core::Result<()>;
    fn SelLength(&self) -> windows_core::Result<i32>;
    fn SetSelLength(&self, plsellength: i32) -> windows_core::Result<()>;
    fn SelText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSelText(&self, pbstrseltext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SelRTF(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSelRTF(&self, pbstrselrtf: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IInkEdit {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkEdit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkEdit_Vtbl
    where
        Identity: IInkEdit_Impl,
    {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut InkEditStatus) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Status(this) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseMouseForInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::UseMouseForInput(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseMouseForInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetUseMouseForInput(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn InkMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut InkMode) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::InkMode(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: InkMode) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetInkMode(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn InkInsertMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut InkInsertMode) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::InkInsertMode(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkInsertMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: InkInsertMode) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetInkInsertMode(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::DrawingAttributes(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::putref_DrawingAttributes(this, windows_core::from_raw_borrowed(&newval)).into()
        }
        unsafe extern "system" fn RecognitionTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::RecognitionTimeout(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetRecognitionTimeout(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Recognizer(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Recognizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::putref_Recognizer(this, windows_core::from_raw_borrowed(&newval)).into()
        }
        unsafe extern "system" fn Factoid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Factoid(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetFactoid(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SelInks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pselink: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelInks(this) {
                Ok(ok__) => {
                    pselink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selink: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelInks(this, core::mem::transmute(&selink)).into()
        }
        unsafe extern "system" fn SelInksDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinkdisplaymode: *mut InkDisplayMode) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelInksDisplayMode(this) {
                Ok(ok__) => {
                    pinkdisplaymode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelInksDisplayMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkdisplaymode: InkDisplayMode) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelInksDisplayMode(this, core::mem::transmute_copy(&inkdisplaymode)).into()
        }
        unsafe extern "system" fn Recognize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::Recognize(this).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, plisten: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::GetGestureStatus(this, core::mem::transmute_copy(&gesture)) {
                Ok(ok__) => {
                    plisten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetGestureStatus(this, core::mem::transmute_copy(&gesture), core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn SetBackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clr: u32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetBackColor(this, core::mem::transmute_copy(&clr)).into()
        }
        unsafe extern "system" fn BackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclr: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::BackColor(this) {
                Ok(ok__) => {
                    pclr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappearance: *mut AppearanceConstants) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Appearance(this) {
                Ok(ok__) => {
                    pappearance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappearance: AppearanceConstants) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetAppearance(this, core::mem::transmute_copy(&pappearance)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pborderstyle: *mut BorderStyleConstants) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::BorderStyle(this) {
                Ok(ok__) => {
                    pborderstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pborderstyle: BorderStyleConstants) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetBorderStyle(this, core::mem::transmute_copy(&pborderstyle)).into()
        }
        unsafe extern "system" fn Hwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pohhwnd: *mut super::super::System::Ole::OLE_HANDLE) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Hwnd(this) {
                Ok(ok__) => {
                    pohhwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Font(this) {
                Ok(ok__) => {
                    ppfont.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::putref_Font(this, windows_core::from_raw_borrowed(&ppfont)).into()
        }
        unsafe extern "system" fn Text<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Text(this) {
                Ok(ok__) => {
                    pbstrtext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetText(this, core::mem::transmute(&pbstrtext)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::MouseIcon(this) {
                Ok(ok__) => {
                    mouseicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetMouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::putref_MouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: *mut InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::MousePointer(this) {
                Ok(ok__) => {
                    mousepointer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetMousePointer(this, core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn Locked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Locked(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetLocked(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::Enabled(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetEnabled(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxlength: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::MaxLength(this) {
                Ok(ok__) => {
                    plmaxlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxlength: i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetMaxLength(this, core::mem::transmute_copy(&lmaxlength)).into()
        }
        unsafe extern "system" fn MultiLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::MultiLine(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetMultiLine(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ScrollBars<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut ScrollBarsConstants) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::ScrollBars(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollBars<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: ScrollBarsConstants) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetScrollBars(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DisableNoScroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::DisableNoScroll(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableNoScroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetDisableNoScroll(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SelAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselalignment: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelAlignment(this) {
                Ok(ok__) => {
                    pvarselalignment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselalignment: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelAlignment(this, core::mem::transmute(&pvarselalignment)).into()
        }
        unsafe extern "system" fn SelBold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselbold: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelBold(this) {
                Ok(ok__) => {
                    pvarselbold.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelBold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselbold: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelBold(this, core::mem::transmute(&pvarselbold)).into()
        }
        unsafe extern "system" fn SelItalic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselitalic: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelItalic(this) {
                Ok(ok__) => {
                    pvarselitalic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelItalic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselitalic: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelItalic(this, core::mem::transmute(&pvarselitalic)).into()
        }
        unsafe extern "system" fn SelUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselunderline: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelUnderline(this) {
                Ok(ok__) => {
                    pvarselunderline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselunderline: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelUnderline(this, core::mem::transmute(&pvarselunderline)).into()
        }
        unsafe extern "system" fn SelColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselcolor: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelColor(this) {
                Ok(ok__) => {
                    pvarselcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselcolor: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelColor(this, core::mem::transmute(&pvarselcolor)).into()
        }
        unsafe extern "system" fn SelFontName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselfontname: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelFontName(this) {
                Ok(ok__) => {
                    pvarselfontname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselfontname: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelFontName(this, core::mem::transmute(&pvarselfontname)).into()
        }
        unsafe extern "system" fn SelFontSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselfontsize: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelFontSize(this) {
                Ok(ok__) => {
                    pvarselfontsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelFontSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselfontsize: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelFontSize(this, core::mem::transmute(&pvarselfontsize)).into()
        }
        unsafe extern "system" fn SelCharOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselcharoffset: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelCharOffset(this) {
                Ok(ok__) => {
                    pvarselcharoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelCharOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselcharoffset: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelCharOffset(this, core::mem::transmute(&pvarselcharoffset)).into()
        }
        unsafe extern "system" fn TextRTF<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtextrtf: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::TextRTF(this) {
                Ok(ok__) => {
                    pbstrtextrtf.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextRTF<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtextrtf: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetTextRTF(this, core::mem::transmute(&pbstrtextrtf)).into()
        }
        unsafe extern "system" fn SelStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plselstart: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelStart(this) {
                Ok(ok__) => {
                    plselstart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plselstart: i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelStart(this, core::mem::transmute_copy(&plselstart)).into()
        }
        unsafe extern "system" fn SelLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsellength: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelLength(this) {
                Ok(ok__) => {
                    plsellength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsellength: i32) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelLength(this, core::mem::transmute_copy(&plsellength)).into()
        }
        unsafe extern "system" fn SelText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrseltext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelText(this) {
                Ok(ok__) => {
                    pbstrseltext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrseltext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelText(this, core::mem::transmute(&pbstrseltext)).into()
        }
        unsafe extern "system" fn SelRTF<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrselrtf: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkEdit_Impl::SelRTF(this) {
                Ok(ok__) => {
                    pbstrselrtf.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelRTF<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrselrtf: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::SetSelRTF(this, core::mem::transmute(&pbstrselrtf)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkEdit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkEdit_Impl::Refresh(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            UseMouseForInput: UseMouseForInput::<Identity, OFFSET>,
            SetUseMouseForInput: SetUseMouseForInput::<Identity, OFFSET>,
            InkMode: InkMode::<Identity, OFFSET>,
            SetInkMode: SetInkMode::<Identity, OFFSET>,
            InkInsertMode: InkInsertMode::<Identity, OFFSET>,
            SetInkInsertMode: SetInkInsertMode::<Identity, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, OFFSET>,
            RecognitionTimeout: RecognitionTimeout::<Identity, OFFSET>,
            SetRecognitionTimeout: SetRecognitionTimeout::<Identity, OFFSET>,
            Recognizer: Recognizer::<Identity, OFFSET>,
            putref_Recognizer: putref_Recognizer::<Identity, OFFSET>,
            Factoid: Factoid::<Identity, OFFSET>,
            SetFactoid: SetFactoid::<Identity, OFFSET>,
            SelInks: SelInks::<Identity, OFFSET>,
            SetSelInks: SetSelInks::<Identity, OFFSET>,
            SelInksDisplayMode: SelInksDisplayMode::<Identity, OFFSET>,
            SetSelInksDisplayMode: SetSelInksDisplayMode::<Identity, OFFSET>,
            Recognize: Recognize::<Identity, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            BackColor: BackColor::<Identity, OFFSET>,
            Appearance: Appearance::<Identity, OFFSET>,
            SetAppearance: SetAppearance::<Identity, OFFSET>,
            BorderStyle: BorderStyle::<Identity, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, OFFSET>,
            Hwnd: Hwnd::<Identity, OFFSET>,
            Font: Font::<Identity, OFFSET>,
            putref_Font: putref_Font::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            MouseIcon: MouseIcon::<Identity, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, OFFSET>,
            MousePointer: MousePointer::<Identity, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, OFFSET>,
            Locked: Locked::<Identity, OFFSET>,
            SetLocked: SetLocked::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            MaxLength: MaxLength::<Identity, OFFSET>,
            SetMaxLength: SetMaxLength::<Identity, OFFSET>,
            MultiLine: MultiLine::<Identity, OFFSET>,
            SetMultiLine: SetMultiLine::<Identity, OFFSET>,
            ScrollBars: ScrollBars::<Identity, OFFSET>,
            SetScrollBars: SetScrollBars::<Identity, OFFSET>,
            DisableNoScroll: DisableNoScroll::<Identity, OFFSET>,
            SetDisableNoScroll: SetDisableNoScroll::<Identity, OFFSET>,
            SelAlignment: SelAlignment::<Identity, OFFSET>,
            SetSelAlignment: SetSelAlignment::<Identity, OFFSET>,
            SelBold: SelBold::<Identity, OFFSET>,
            SetSelBold: SetSelBold::<Identity, OFFSET>,
            SelItalic: SelItalic::<Identity, OFFSET>,
            SetSelItalic: SetSelItalic::<Identity, OFFSET>,
            SelUnderline: SelUnderline::<Identity, OFFSET>,
            SetSelUnderline: SetSelUnderline::<Identity, OFFSET>,
            SelColor: SelColor::<Identity, OFFSET>,
            SetSelColor: SetSelColor::<Identity, OFFSET>,
            SelFontName: SelFontName::<Identity, OFFSET>,
            SetSelFontName: SetSelFontName::<Identity, OFFSET>,
            SelFontSize: SelFontSize::<Identity, OFFSET>,
            SetSelFontSize: SetSelFontSize::<Identity, OFFSET>,
            SelCharOffset: SelCharOffset::<Identity, OFFSET>,
            SetSelCharOffset: SetSelCharOffset::<Identity, OFFSET>,
            TextRTF: TextRTF::<Identity, OFFSET>,
            SetTextRTF: SetTextRTF::<Identity, OFFSET>,
            SelStart: SelStart::<Identity, OFFSET>,
            SetSelStart: SetSelStart::<Identity, OFFSET>,
            SelLength: SelLength::<Identity, OFFSET>,
            SetSelLength: SetSelLength::<Identity, OFFSET>,
            SelText: SelText::<Identity, OFFSET>,
            SetSelText: SetSelText::<Identity, OFFSET>,
            SelRTF: SelRTF::<Identity, OFFSET>,
            SetSelRTF: SetSelRTF::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkEdit as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkExtendedProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, identifier: &windows_core::VARIANT) -> windows_core::Result<IInkExtendedProperty>;
    fn Add(&self, guid: &windows_core::BSTR, data: &windows_core::VARIANT) -> windows_core::Result<IInkExtendedProperty>;
    fn Remove(&self, identifier: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn DoesPropertyExist(&self, guid: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkExtendedProperties {}
#[cfg(feature = "Win32_System_Com")]
impl IInkExtendedProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkExtendedProperties_Vtbl
    where
        Identity: IInkExtendedProperties_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperties_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperties_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: core::mem::MaybeUninit<windows_core::VARIANT>, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperties_Impl::Item(this, core::mem::transmute(&identifier)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: core::mem::MaybeUninit<windows_core::BSTR>, data: core::mem::MaybeUninit<windows_core::VARIANT>, inkextendedproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperties_Impl::Add(this, core::mem::transmute(&guid), core::mem::transmute(&data)) {
                Ok(ok__) => {
                    inkextendedproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkExtendedProperties_Impl::Remove(this, core::mem::transmute(&identifier)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkExtendedProperties_Impl::Clear(this).into()
        }
        unsafe extern "system" fn DoesPropertyExist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: core::mem::MaybeUninit<windows_core::BSTR>, doespropertyexist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperties_Impl::DoesPropertyExist(this, core::mem::transmute(&guid)) {
                Ok(ok__) => {
                    doespropertyexist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            DoesPropertyExist: DoesPropertyExist::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkExtendedProperties as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkExtendedProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Guid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Data(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetData(&self, data: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkExtendedProperty {}
#[cfg(feature = "Win32_System_Com")]
impl IInkExtendedProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkExtendedProperty_Vtbl
    where
        Identity: IInkExtendedProperty_Impl,
    {
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperty_Impl::Guid(this) {
                Ok(ok__) => {
                    guid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkExtendedProperty_Impl::Data(this) {
                Ok(ok__) => {
                    data.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkExtendedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkExtendedProperty_Impl::SetData(this, core::mem::transmute(&data)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Guid: Guid::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkExtendedProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkGesture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Confidence(&self) -> windows_core::Result<InkRecognitionConfidence>;
    fn Id(&self) -> windows_core::Result<InkApplicationGesture>;
    fn GetHotPoint(&self, x: *mut i32, y: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkGesture {}
#[cfg(feature = "Win32_System_Com")]
impl IInkGesture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkGesture_Vtbl
    where
        Identity: IInkGesture_Impl,
    {
        unsafe extern "system" fn Confidence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> windows_core::HRESULT
        where
            Identity: IInkGesture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkGesture_Impl::Confidence(this) {
                Ok(ok__) => {
                    confidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut InkApplicationGesture) -> windows_core::HRESULT
        where
            Identity: IInkGesture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkGesture_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHotPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut i32, y: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkGesture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkGesture_Impl::GetHotPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Confidence: Confidence::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            GetHotPoint: GetHotPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkGesture as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IInkLineInfo_Impl: Sized {
    fn SetFormat(&self, pim: *const INKMETRIC) -> windows_core::Result<()>;
    fn GetFormat(&self, pim: *const INKMETRIC) -> windows_core::Result<()>;
    fn GetInkExtent(&self, pim: *const INKMETRIC, pnwidth: *const u32) -> windows_core::Result<()>;
    fn GetCandidate(&self, ncandidatenum: u32, pwcrecogword: &windows_core::PCWSTR, pcwcrecogword: *const u32, dwflags: u32) -> windows_core::Result<()>;
    fn SetCandidate(&self, ncandidatenum: u32, strrecogword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Recognize(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInkLineInfo {}
impl IInkLineInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkLineInfo_Vtbl
    where
        Identity: IInkLineInfo_Impl,
    {
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pim: *const INKMETRIC) -> windows_core::HRESULT
        where
            Identity: IInkLineInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkLineInfo_Impl::SetFormat(this, core::mem::transmute_copy(&pim)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pim: *const INKMETRIC) -> windows_core::HRESULT
        where
            Identity: IInkLineInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkLineInfo_Impl::GetFormat(this, core::mem::transmute_copy(&pim)).into()
        }
        unsafe extern "system" fn GetInkExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pim: *const INKMETRIC, pnwidth: *const u32) -> windows_core::HRESULT
        where
            Identity: IInkLineInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkLineInfo_Impl::GetInkExtent(this, core::mem::transmute_copy(&pim), core::mem::transmute_copy(&pnwidth)).into()
        }
        unsafe extern "system" fn GetCandidate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncandidatenum: u32, pwcrecogword: windows_core::PCWSTR, pcwcrecogword: *const u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInkLineInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkLineInfo_Impl::GetCandidate(this, core::mem::transmute_copy(&ncandidatenum), core::mem::transmute(&pwcrecogword), core::mem::transmute_copy(&pcwcrecogword), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetCandidate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncandidatenum: u32, strrecogword: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IInkLineInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkLineInfo_Impl::SetCandidate(this, core::mem::transmute_copy(&ncandidatenum), core::mem::transmute(&strrecogword)).into()
        }
        unsafe extern "system" fn Recognize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkLineInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkLineInfo_Impl::Recognize(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFormat: SetFormat::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetInkExtent: GetInkExtent::<Identity, OFFSET>,
            GetCandidate: GetCandidate::<Identity, OFFSET>,
            SetCandidate: SetCandidate::<Identity, OFFSET>,
            Recognize: Recognize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkLineInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkOverlay_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&self) -> windows_core::Result<isize>;
    fn SethWnd(&self, newwindow: isize) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, collecting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DefaultDrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&self, newattributes: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn Renderer(&self) -> windows_core::Result<IInkRenderer>;
    fn putref_Renderer(&self, newinkrenderer: Option<&IInkRenderer>) -> windows_core::Result<()>;
    fn Ink(&self) -> windows_core::Result<IInkDisp>;
    fn putref_Ink(&self, newink: Option<&IInkDisp>) -> windows_core::Result<()>;
    fn AutoRedraw(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoRedraw(&self, autoredraw: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CollectingInk(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CollectionMode(&self) -> windows_core::Result<InkCollectionMode>;
    fn SetCollectionMode(&self, mode: InkCollectionMode) -> windows_core::Result<()>;
    fn DynamicRendering(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDynamicRendering(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DesiredPacketDescription(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDesiredPacketDescription(&self, packetguids: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MouseIcon(&self) -> windows_core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn MousePointer(&self) -> windows_core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> windows_core::Result<()>;
    fn EditingMode(&self) -> windows_core::Result<InkOverlayEditingMode>;
    fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> windows_core::Result<()>;
    fn Selection(&self) -> windows_core::Result<IInkStrokes>;
    fn SetSelection(&self, selection: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn EraserMode(&self) -> windows_core::Result<InkOverlayEraserMode>;
    fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> windows_core::Result<()>;
    fn EraserWidth(&self) -> windows_core::Result<i32>;
    fn SetEraserWidth(&self, neweraserwidth: i32) -> windows_core::Result<()>;
    fn AttachMode(&self) -> windows_core::Result<InkOverlayAttachMode>;
    fn SetAttachMode(&self, attachmode: InkOverlayAttachMode) -> windows_core::Result<()>;
    fn Cursors(&self) -> windows_core::Result<IInkCursors>;
    fn MarginX(&self) -> windows_core::Result<i32>;
    fn SetMarginX(&self, marginx: i32) -> windows_core::Result<()>;
    fn MarginY(&self) -> windows_core::Result<i32>;
    fn SetMarginY(&self, marginy: i32) -> windows_core::Result<()>;
    fn Tablet(&self) -> windows_core::Result<IInkTablet>;
    fn SupportHighContrastInk(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSupportHighContrastInk(&self, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SupportHighContrastSelectionUI(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSupportHighContrastSelectionUI(&self, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HitTestSelection(&self, x: i32, y: i32) -> windows_core::Result<SelectionHitResult>;
    fn Draw(&self, rect: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetWindowInputRectangle(&self, windowinputrectangle: *mut Option<IInkRectangle>) -> windows_core::Result<()>;
    fn SetWindowInputRectangle(&self, windowinputrectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn SetAllTabletsMode(&self, usemouseforinput: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetSingleTabletIntegratedMode(&self, tablet: Option<&IInkTablet>) -> windows_core::Result<()>;
    fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IInkOverlay {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkOverlay_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkOverlay_Vtbl
    where
        Identity: IInkOverlay_Impl,
    {
        unsafe extern "system" fn hWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentwindow: *mut isize) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::hWnd(this) {
                Ok(ok__) => {
                    currentwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newwindow: isize) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SethWnd(this, core::mem::transmute_copy(&newwindow)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::Enabled(this) {
                Ok(ok__) => {
                    collecting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetEnabled(this, core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::DefaultDrawingAttributes(this) {
                Ok(ok__) => {
                    currentattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newattributes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::putref_DefaultDrawingAttributes(this, windows_core::from_raw_borrowed(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentinkrenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::Renderer(this) {
                Ok(ok__) => {
                    currentinkrenderer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinkrenderer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::putref_Renderer(this, windows_core::from_raw_borrowed(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::Ink(this) {
                Ok(ok__) => {
                    ink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::putref_Ink(this, windows_core::from_raw_borrowed(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoredraw: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::AutoRedraw(this) {
                Ok(ok__) => {
                    autoredraw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoredraw: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetAutoRedraw(this, core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::CollectingInk(this) {
                Ok(ok__) => {
                    collecting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut InkCollectionMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::CollectionMode(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: InkCollectionMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetCollectionMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::DynamicRendering(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetDynamicRendering(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetguids: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::DesiredPacketDescription(this) {
                Ok(ok__) => {
                    packetguids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetguids: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetDesiredPacketDescription(this, core::mem::transmute(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::MouseIcon(this) {
                Ok(ok__) => {
                    mouseicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetMouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::putref_MouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: *mut InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::MousePointer(this) {
                Ok(ok__) => {
                    mousepointer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetMousePointer(this, core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn EditingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::EditingMode(this) {
                Ok(ok__) => {
                    editingmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, editingmode: InkOverlayEditingMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetEditingMode(this, core::mem::transmute_copy(&editingmode)).into()
        }
        unsafe extern "system" fn Selection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::Selection(this) {
                Ok(ok__) => {
                    selection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetSelection(this, windows_core::from_raw_borrowed(&selection)).into()
        }
        unsafe extern "system" fn EraserMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::EraserMode(this) {
                Ok(ok__) => {
                    erasermode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, erasermode: InkOverlayEraserMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetEraserMode(this, core::mem::transmute_copy(&erasermode)).into()
        }
        unsafe extern "system" fn EraserWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eraserwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::EraserWidth(this) {
                Ok(ok__) => {
                    eraserwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, neweraserwidth: i32) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetEraserWidth(this, core::mem::transmute_copy(&neweraserwidth)).into()
        }
        unsafe extern "system" fn AttachMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmode: *mut InkOverlayAttachMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::AttachMode(this) {
                Ok(ok__) => {
                    attachmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmode: InkOverlayAttachMode) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetAttachMode(this, core::mem::transmute_copy(&attachmode)).into()
        }
        unsafe extern "system" fn Cursors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cursors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::Cursors(this) {
                Ok(ok__) => {
                    cursors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginx: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::MarginX(this) {
                Ok(ok__) => {
                    marginx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginx: i32) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetMarginX(this, core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::MarginY(this) {
                Ok(ok__) => {
                    marginy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginy: i32) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetMarginY(this, core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, singletablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::Tablet(this) {
                Ok(ok__) => {
                    singletablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::SupportHighContrastInk(this) {
                Ok(ok__) => {
                    support.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetSupportHighContrastInk(this, core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::SupportHighContrastSelectionUI(this) {
                Ok(ok__) => {
                    support.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetSupportHighContrastSelectionUI(this, core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn HitTestSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::HitTestSelection(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                Ok(ok__) => {
                    selarea.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::Draw(this, windows_core::from_raw_borrowed(&rect)).into()
        }
        unsafe extern "system" fn SetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetGestureStatus(this, core::mem::transmute_copy(&gesture), core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::GetGestureStatus(this, core::mem::transmute_copy(&gesture)) {
                Ok(ok__) => {
                    listening.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinputrectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::GetWindowInputRectangle(this, core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinputrectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetWindowInputRectangle(this, windows_core::from_raw_borrowed(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usemouseforinput: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetAllTabletsMode(this, core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablet: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetSingleTabletIntegratedMode(this, windows_core::from_raw_borrowed(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkOverlay_Impl::GetEventInterest(this, core::mem::transmute_copy(&eventid)) {
                Ok(ok__) => {
                    listen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: InkCollectorEventInterest, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkOverlay_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkOverlay_Impl::SetEventInterest(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&listen)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            hWnd: hWnd::<Identity, OFFSET>,
            SethWnd: SethWnd::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            DefaultDrawingAttributes: DefaultDrawingAttributes::<Identity, OFFSET>,
            putref_DefaultDrawingAttributes: putref_DefaultDrawingAttributes::<Identity, OFFSET>,
            Renderer: Renderer::<Identity, OFFSET>,
            putref_Renderer: putref_Renderer::<Identity, OFFSET>,
            Ink: Ink::<Identity, OFFSET>,
            putref_Ink: putref_Ink::<Identity, OFFSET>,
            AutoRedraw: AutoRedraw::<Identity, OFFSET>,
            SetAutoRedraw: SetAutoRedraw::<Identity, OFFSET>,
            CollectingInk: CollectingInk::<Identity, OFFSET>,
            CollectionMode: CollectionMode::<Identity, OFFSET>,
            SetCollectionMode: SetCollectionMode::<Identity, OFFSET>,
            DynamicRendering: DynamicRendering::<Identity, OFFSET>,
            SetDynamicRendering: SetDynamicRendering::<Identity, OFFSET>,
            DesiredPacketDescription: DesiredPacketDescription::<Identity, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, OFFSET>,
            MouseIcon: MouseIcon::<Identity, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, OFFSET>,
            MousePointer: MousePointer::<Identity, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, OFFSET>,
            EditingMode: EditingMode::<Identity, OFFSET>,
            SetEditingMode: SetEditingMode::<Identity, OFFSET>,
            Selection: Selection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            EraserMode: EraserMode::<Identity, OFFSET>,
            SetEraserMode: SetEraserMode::<Identity, OFFSET>,
            EraserWidth: EraserWidth::<Identity, OFFSET>,
            SetEraserWidth: SetEraserWidth::<Identity, OFFSET>,
            AttachMode: AttachMode::<Identity, OFFSET>,
            SetAttachMode: SetAttachMode::<Identity, OFFSET>,
            Cursors: Cursors::<Identity, OFFSET>,
            MarginX: MarginX::<Identity, OFFSET>,
            SetMarginX: SetMarginX::<Identity, OFFSET>,
            MarginY: MarginY::<Identity, OFFSET>,
            SetMarginY: SetMarginY::<Identity, OFFSET>,
            Tablet: Tablet::<Identity, OFFSET>,
            SupportHighContrastInk: SupportHighContrastInk::<Identity, OFFSET>,
            SetSupportHighContrastInk: SetSupportHighContrastInk::<Identity, OFFSET>,
            SupportHighContrastSelectionUI: SupportHighContrastSelectionUI::<Identity, OFFSET>,
            SetSupportHighContrastSelectionUI: SetSupportHighContrastSelectionUI::<Identity, OFFSET>,
            HitTestSelection: HitTestSelection::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, OFFSET>,
            GetWindowInputRectangle: GetWindowInputRectangle::<Identity, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, OFFSET>,
            SetSingleTabletIntegratedMode: SetSingleTabletIntegratedMode::<Identity, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkOverlay as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInkPicture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn hWnd(&self) -> windows_core::Result<isize>;
    fn DefaultDrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DefaultDrawingAttributes(&self, newattributes: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn Renderer(&self) -> windows_core::Result<IInkRenderer>;
    fn putref_Renderer(&self, newinkrenderer: Option<&IInkRenderer>) -> windows_core::Result<()>;
    fn Ink(&self) -> windows_core::Result<IInkDisp>;
    fn putref_Ink(&self, newink: Option<&IInkDisp>) -> windows_core::Result<()>;
    fn AutoRedraw(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoRedraw(&self, autoredraw: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CollectingInk(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CollectionMode(&self) -> windows_core::Result<InkCollectionMode>;
    fn SetCollectionMode(&self, mode: InkCollectionMode) -> windows_core::Result<()>;
    fn DynamicRendering(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDynamicRendering(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DesiredPacketDescription(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDesiredPacketDescription(&self, packetguids: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MouseIcon(&self) -> windows_core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetMouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn putref_MouseIcon(&self, mouseicon: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn MousePointer(&self) -> windows_core::Result<InkMousePointer>;
    fn SetMousePointer(&self, mousepointer: InkMousePointer) -> windows_core::Result<()>;
    fn EditingMode(&self) -> windows_core::Result<InkOverlayEditingMode>;
    fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> windows_core::Result<()>;
    fn Selection(&self) -> windows_core::Result<IInkStrokes>;
    fn SetSelection(&self, selection: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn EraserMode(&self) -> windows_core::Result<InkOverlayEraserMode>;
    fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> windows_core::Result<()>;
    fn EraserWidth(&self) -> windows_core::Result<i32>;
    fn SetEraserWidth(&self, neweraserwidth: i32) -> windows_core::Result<()>;
    fn putref_Picture(&self, ppicture: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn SetPicture(&self, ppicture: Option<&super::super::System::Ole::IPictureDisp>) -> windows_core::Result<()>;
    fn Picture(&self) -> windows_core::Result<super::super::System::Ole::IPictureDisp>;
    fn SetSizeMode(&self, smnewsizemode: InkPictureSizeMode) -> windows_core::Result<()>;
    fn SizeMode(&self) -> windows_core::Result<InkPictureSizeMode>;
    fn SetBackColor(&self, newcolor: u32) -> windows_core::Result<()>;
    fn BackColor(&self) -> windows_core::Result<u32>;
    fn Cursors(&self) -> windows_core::Result<IInkCursors>;
    fn MarginX(&self) -> windows_core::Result<i32>;
    fn SetMarginX(&self, marginx: i32) -> windows_core::Result<()>;
    fn MarginY(&self) -> windows_core::Result<i32>;
    fn SetMarginY(&self, marginy: i32) -> windows_core::Result<()>;
    fn Tablet(&self) -> windows_core::Result<IInkTablet>;
    fn SupportHighContrastInk(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSupportHighContrastInk(&self, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SupportHighContrastSelectionUI(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSupportHighContrastSelectionUI(&self, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HitTestSelection(&self, x: i32, y: i32) -> windows_core::Result<SelectionHitResult>;
    fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetWindowInputRectangle(&self, windowinputrectangle: *mut Option<IInkRectangle>) -> windows_core::Result<()>;
    fn SetWindowInputRectangle(&self, windowinputrectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn SetAllTabletsMode(&self, usemouseforinput: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetSingleTabletIntegratedMode(&self, tablet: Option<&IInkTablet>) -> windows_core::Result<()>;
    fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn InkEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetInkEnabled(&self, collecting: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, vbool: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IInkPicture {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInkPicture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkPicture_Vtbl
    where
        Identity: IInkPicture_Impl,
    {
        unsafe extern "system" fn hWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentwindow: *mut isize) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::hWnd(this) {
                Ok(ok__) => {
                    currentwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::DefaultDrawingAttributes(this) {
                Ok(ok__) => {
                    currentattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DefaultDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newattributes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::putref_DefaultDrawingAttributes(this, windows_core::from_raw_borrowed(&newattributes)).into()
        }
        unsafe extern "system" fn Renderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentinkrenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Renderer(this) {
                Ok(ok__) => {
                    currentinkrenderer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Renderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinkrenderer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::putref_Renderer(this, windows_core::from_raw_borrowed(&newinkrenderer)).into()
        }
        unsafe extern "system" fn Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Ink(this) {
                Ok(ok__) => {
                    ink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::putref_Ink(this, windows_core::from_raw_borrowed(&newink)).into()
        }
        unsafe extern "system" fn AutoRedraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoredraw: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::AutoRedraw(this) {
                Ok(ok__) => {
                    autoredraw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoRedraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoredraw: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetAutoRedraw(this, core::mem::transmute_copy(&autoredraw)).into()
        }
        unsafe extern "system" fn CollectingInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::CollectingInk(this) {
                Ok(ok__) => {
                    collecting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut InkCollectionMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::CollectionMode(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: InkCollectionMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetCollectionMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn DynamicRendering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::DynamicRendering(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicRendering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetDynamicRendering(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetguids: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::DesiredPacketDescription(this) {
                Ok(ok__) => {
                    packetguids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetguids: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetDesiredPacketDescription(this, core::mem::transmute(&packetguids)).into()
        }
        unsafe extern "system" fn MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::MouseIcon(this) {
                Ok(ok__) => {
                    mouseicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetMouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn putref_MouseIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mouseicon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::putref_MouseIcon(this, windows_core::from_raw_borrowed(&mouseicon)).into()
        }
        unsafe extern "system" fn MousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: *mut InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::MousePointer(this) {
                Ok(ok__) => {
                    mousepointer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMousePointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mousepointer: InkMousePointer) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetMousePointer(this, core::mem::transmute_copy(&mousepointer)).into()
        }
        unsafe extern "system" fn EditingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, editingmode: *mut InkOverlayEditingMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::EditingMode(this) {
                Ok(ok__) => {
                    editingmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEditingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, editingmode: InkOverlayEditingMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetEditingMode(this, core::mem::transmute_copy(&editingmode)).into()
        }
        unsafe extern "system" fn Selection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Selection(this) {
                Ok(ok__) => {
                    selection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetSelection(this, windows_core::from_raw_borrowed(&selection)).into()
        }
        unsafe extern "system" fn EraserMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, erasermode: *mut InkOverlayEraserMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::EraserMode(this) {
                Ok(ok__) => {
                    erasermode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, erasermode: InkOverlayEraserMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetEraserMode(this, core::mem::transmute_copy(&erasermode)).into()
        }
        unsafe extern "system" fn EraserWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eraserwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::EraserWidth(this) {
                Ok(ok__) => {
                    eraserwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEraserWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, neweraserwidth: i32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetEraserWidth(this, core::mem::transmute_copy(&neweraserwidth)).into()
        }
        unsafe extern "system" fn putref_Picture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicture: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::putref_Picture(this, windows_core::from_raw_borrowed(&ppicture)).into()
        }
        unsafe extern "system" fn SetPicture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicture: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetPicture(this, windows_core::from_raw_borrowed(&ppicture)).into()
        }
        unsafe extern "system" fn Picture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppicture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Picture(this) {
                Ok(ok__) => {
                    pppicture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, smnewsizemode: InkPictureSizeMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetSizeMode(this, core::mem::transmute_copy(&smnewsizemode)).into()
        }
        unsafe extern "system" fn SizeMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsizemode: *mut InkPictureSizeMode) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::SizeMode(this) {
                Ok(ok__) => {
                    smsizemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcolor: u32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetBackColor(this, core::mem::transmute_copy(&newcolor)).into()
        }
        unsafe extern "system" fn BackColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::BackColor(this) {
                Ok(ok__) => {
                    pcolor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cursors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cursors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Cursors(this) {
                Ok(ok__) => {
                    cursors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginx: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::MarginX(this) {
                Ok(ok__) => {
                    marginx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginx: i32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetMarginX(this, core::mem::transmute_copy(&marginx)).into()
        }
        unsafe extern "system" fn MarginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::MarginY(this) {
                Ok(ok__) => {
                    marginy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMarginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, marginy: i32) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetMarginY(this, core::mem::transmute_copy(&marginy)).into()
        }
        unsafe extern "system" fn Tablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, singletablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Tablet(this) {
                Ok(ok__) => {
                    singletablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportHighContrastInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::SupportHighContrastInk(this) {
                Ok(ok__) => {
                    support.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetSupportHighContrastInk(this, core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn SupportHighContrastSelectionUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::SupportHighContrastSelectionUI(this) {
                Ok(ok__) => {
                    support.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportHighContrastSelectionUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, support: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetSupportHighContrastSelectionUI(this, core::mem::transmute_copy(&support)).into()
        }
        unsafe extern "system" fn HitTestSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, selarea: *mut SelectionHitResult) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::HitTestSelection(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                Ok(ok__) => {
                    selarea.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetGestureStatus(this, core::mem::transmute_copy(&gesture), core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn GetGestureStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gesture: InkApplicationGesture, listening: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::GetGestureStatus(this, core::mem::transmute_copy(&gesture)) {
                Ok(ok__) => {
                    listening.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinputrectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::GetWindowInputRectangle(this, core::mem::transmute_copy(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowinputrectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetWindowInputRectangle(this, windows_core::from_raw_borrowed(&windowinputrectangle)).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usemouseforinput: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetAllTabletsMode(this, core::mem::transmute_copy(&usemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletIntegratedMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablet: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetSingleTabletIntegratedMode(this, windows_core::from_raw_borrowed(&tablet)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: InkCollectorEventInterest, listen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::GetEventInterest(this, core::mem::transmute_copy(&eventid)) {
                Ok(ok__) => {
                    listen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: InkCollectorEventInterest, listen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetEventInterest(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&listen)).into()
        }
        unsafe extern "system" fn InkEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::InkEnabled(this) {
                Ok(ok__) => {
                    collecting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInkEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collecting: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetInkEnabled(this, core::mem::transmute_copy(&collecting)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkPicture_Impl::Enabled(this) {
                Ok(ok__) => {
                    pbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbool: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkPicture_Impl::SetEnabled(this, core::mem::transmute_copy(&vbool)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            hWnd: hWnd::<Identity, OFFSET>,
            DefaultDrawingAttributes: DefaultDrawingAttributes::<Identity, OFFSET>,
            putref_DefaultDrawingAttributes: putref_DefaultDrawingAttributes::<Identity, OFFSET>,
            Renderer: Renderer::<Identity, OFFSET>,
            putref_Renderer: putref_Renderer::<Identity, OFFSET>,
            Ink: Ink::<Identity, OFFSET>,
            putref_Ink: putref_Ink::<Identity, OFFSET>,
            AutoRedraw: AutoRedraw::<Identity, OFFSET>,
            SetAutoRedraw: SetAutoRedraw::<Identity, OFFSET>,
            CollectingInk: CollectingInk::<Identity, OFFSET>,
            CollectionMode: CollectionMode::<Identity, OFFSET>,
            SetCollectionMode: SetCollectionMode::<Identity, OFFSET>,
            DynamicRendering: DynamicRendering::<Identity, OFFSET>,
            SetDynamicRendering: SetDynamicRendering::<Identity, OFFSET>,
            DesiredPacketDescription: DesiredPacketDescription::<Identity, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, OFFSET>,
            MouseIcon: MouseIcon::<Identity, OFFSET>,
            SetMouseIcon: SetMouseIcon::<Identity, OFFSET>,
            putref_MouseIcon: putref_MouseIcon::<Identity, OFFSET>,
            MousePointer: MousePointer::<Identity, OFFSET>,
            SetMousePointer: SetMousePointer::<Identity, OFFSET>,
            EditingMode: EditingMode::<Identity, OFFSET>,
            SetEditingMode: SetEditingMode::<Identity, OFFSET>,
            Selection: Selection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            EraserMode: EraserMode::<Identity, OFFSET>,
            SetEraserMode: SetEraserMode::<Identity, OFFSET>,
            EraserWidth: EraserWidth::<Identity, OFFSET>,
            SetEraserWidth: SetEraserWidth::<Identity, OFFSET>,
            putref_Picture: putref_Picture::<Identity, OFFSET>,
            SetPicture: SetPicture::<Identity, OFFSET>,
            Picture: Picture::<Identity, OFFSET>,
            SetSizeMode: SetSizeMode::<Identity, OFFSET>,
            SizeMode: SizeMode::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            BackColor: BackColor::<Identity, OFFSET>,
            Cursors: Cursors::<Identity, OFFSET>,
            MarginX: MarginX::<Identity, OFFSET>,
            SetMarginX: SetMarginX::<Identity, OFFSET>,
            MarginY: MarginY::<Identity, OFFSET>,
            SetMarginY: SetMarginY::<Identity, OFFSET>,
            Tablet: Tablet::<Identity, OFFSET>,
            SupportHighContrastInk: SupportHighContrastInk::<Identity, OFFSET>,
            SetSupportHighContrastInk: SetSupportHighContrastInk::<Identity, OFFSET>,
            SupportHighContrastSelectionUI: SupportHighContrastSelectionUI::<Identity, OFFSET>,
            SetSupportHighContrastSelectionUI: SetSupportHighContrastSelectionUI::<Identity, OFFSET>,
            HitTestSelection: HitTestSelection::<Identity, OFFSET>,
            SetGestureStatus: SetGestureStatus::<Identity, OFFSET>,
            GetGestureStatus: GetGestureStatus::<Identity, OFFSET>,
            GetWindowInputRectangle: GetWindowInputRectangle::<Identity, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, OFFSET>,
            SetSingleTabletIntegratedMode: SetSingleTabletIntegratedMode::<Identity, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, OFFSET>,
            InkEnabled: InkEnabled::<Identity, OFFSET>,
            SetInkEnabled: SetInkEnabled::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkPicture as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionAlternate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn String(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Confidence(&self) -> windows_core::Result<InkRecognitionConfidence>;
    fn Baseline(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Midline(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Ascender(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Descender(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn LineNumber(&self) -> windows_core::Result<i32>;
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn LineAlternates(&self) -> windows_core::Result<IInkRecognitionAlternates>;
    fn ConfidenceAlternates(&self) -> windows_core::Result<IInkRecognitionAlternates>;
    fn GetStrokesFromStrokeRanges(&self, strokes: Option<&IInkStrokes>) -> windows_core::Result<IInkStrokes>;
    fn GetStrokesFromTextRange(&self, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut Option<IInkStrokes>) -> windows_core::Result<()>;
    fn GetTextRangeFromStrokes(&self, strokes: Option<&IInkStrokes>, selectionstart: *mut i32, selectionlength: *mut i32) -> windows_core::Result<()>;
    fn AlternatesWithConstantPropertyValues(&self, propertytype: &windows_core::BSTR) -> windows_core::Result<IInkRecognitionAlternates>;
    fn GetPropertyValue(&self, propertytype: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognitionAlternate {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognitionAlternate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognitionAlternate_Vtbl
    where
        Identity: IInkRecognitionAlternate_Impl,
    {
        unsafe extern "system" fn String<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recostring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::String(this) {
                Ok(ok__) => {
                    recostring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, confidence: *mut InkRecognitionConfidence) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::Confidence(this) {
                Ok(ok__) => {
                    confidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseline: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::Baseline(this) {
                Ok(ok__) => {
                    baseline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Midline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, midline: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::Midline(this) {
                Ok(ok__) => {
                    midline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ascender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ascender: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::Ascender(this) {
                Ok(ok__) => {
                    ascender.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, descender: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::Descender(this) {
                Ok(ok__) => {
                    descender.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linenumber: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::LineNumber(this) {
                Ok(ok__) => {
                    linenumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineAlternates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linealternates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::LineAlternates(this) {
                Ok(ok__) => {
                    linealternates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfidenceAlternates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, confidencealternates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::ConfidenceAlternates(this) {
                Ok(ok__) => {
                    confidencealternates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromStrokeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void, getstrokesfromstrokeranges: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::GetStrokesFromStrokeRanges(this, windows_core::from_raw_borrowed(&strokes)) {
                Ok(ok__) => {
                    getstrokesfromstrokeranges.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokesFromTextRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognitionAlternate_Impl::GetStrokesFromTextRange(this, core::mem::transmute_copy(&selectionstart), core::mem::transmute_copy(&selectionlength), core::mem::transmute_copy(&getstrokesfromtextrange)).into()
        }
        unsafe extern "system" fn GetTextRangeFromStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognitionAlternate_Impl::GetTextRangeFromStrokes(this, windows_core::from_raw_borrowed(&strokes), core::mem::transmute_copy(&selectionstart), core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn AlternatesWithConstantPropertyValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertytype: core::mem::MaybeUninit<windows_core::BSTR>, alternateswithconstantpropertyvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::AlternatesWithConstantPropertyValues(this, core::mem::transmute(&propertytype)) {
                Ok(ok__) => {
                    alternateswithconstantpropertyvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertytype: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternate_Impl::GetPropertyValue(this, core::mem::transmute(&propertytype)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            String: String::<Identity, OFFSET>,
            Confidence: Confidence::<Identity, OFFSET>,
            Baseline: Baseline::<Identity, OFFSET>,
            Midline: Midline::<Identity, OFFSET>,
            Ascender: Ascender::<Identity, OFFSET>,
            Descender: Descender::<Identity, OFFSET>,
            LineNumber: LineNumber::<Identity, OFFSET>,
            Strokes: Strokes::<Identity, OFFSET>,
            LineAlternates: LineAlternates::<Identity, OFFSET>,
            ConfidenceAlternates: ConfidenceAlternates::<Identity, OFFSET>,
            GetStrokesFromStrokeRanges: GetStrokesFromStrokeRanges::<Identity, OFFSET>,
            GetStrokesFromTextRange: GetStrokesFromTextRange::<Identity, OFFSET>,
            GetTextRangeFromStrokes: GetTextRangeFromStrokes::<Identity, OFFSET>,
            AlternatesWithConstantPropertyValues: AlternatesWithConstantPropertyValues::<Identity, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognitionAlternate as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionAlternates_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn Item(&self, index: i32) -> windows_core::Result<IInkRecognitionAlternate>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognitionAlternates {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognitionAlternates_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognitionAlternates_Vtbl
    where
        Identity: IInkRecognitionAlternates_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternates_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternates_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternates_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, inkrecoalternate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionAlternates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionAlternates_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    inkrecoalternate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Strokes: Strokes::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognitionAlternates as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TopString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TopAlternate(&self) -> windows_core::Result<IInkRecognitionAlternate>;
    fn TopConfidence(&self) -> windows_core::Result<InkRecognitionConfidence>;
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn AlternatesFromSelection(&self, selectionstart: i32, selectionlength: i32, maximumalternates: i32) -> windows_core::Result<IInkRecognitionAlternates>;
    fn ModifyTopAlternate(&self, alternate: Option<&IInkRecognitionAlternate>) -> windows_core::Result<()>;
    fn SetResultOnStrokes(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognitionResult {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognitionResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognitionResult_Vtbl
    where
        Identity: IInkRecognitionResult_Impl,
    {
        unsafe extern "system" fn TopString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, topstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionResult_Impl::TopString(this) {
                Ok(ok__) => {
                    topstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopAlternate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, topalternate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionResult_Impl::TopAlternate(this) {
                Ok(ok__) => {
                    topalternate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopConfidence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, topconfidence: *mut InkRecognitionConfidence) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionResult_Impl::TopConfidence(this) {
                Ok(ok__) => {
                    topconfidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionResult_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternatesFromSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognitionResult_Impl::AlternatesFromSelection(this, core::mem::transmute_copy(&selectionstart), core::mem::transmute_copy(&selectionlength), core::mem::transmute_copy(&maximumalternates)) {
                Ok(ok__) => {
                    alternatesfromselection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTopAlternate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alternate: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognitionResult_Impl::ModifyTopAlternate(this, windows_core::from_raw_borrowed(&alternate)).into()
        }
        unsafe extern "system" fn SetResultOnStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognitionResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognitionResult_Impl::SetResultOnStrokes(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TopString: TopString::<Identity, OFFSET>,
            TopAlternate: TopAlternate::<Identity, OFFSET>,
            TopConfidence: TopConfidence::<Identity, OFFSET>,
            Strokes: Strokes::<Identity, OFFSET>,
            AlternatesFromSelection: AlternatesFromSelection::<Identity, OFFSET>,
            ModifyTopAlternate: ModifyTopAlternate::<Identity, OFFSET>,
            SetResultOnStrokes: SetResultOnStrokes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognitionResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Vendor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Capabilities(&self) -> windows_core::Result<InkRecognizerCapabilities>;
    fn Languages(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SupportedProperties(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn PreferredPacketDescription(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn CreateRecognizerContext(&self) -> windows_core::Result<IInkRecognizerContext>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognizer {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognizer_Vtbl
    where
        Identity: IInkRecognizer_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Vendor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendor: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::Vendor(this) {
                Ok(ok__) => {
                    vendor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilitiesflags: *mut InkRecognizerCapabilities) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::Capabilities(this) {
                Ok(ok__) => {
                    capabilitiesflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languages: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::Languages(this) {
                Ok(ok__) => {
                    languages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedproperties: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::SupportedProperties(this) {
                Ok(ok__) => {
                    supportedproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredpacketdescription: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::PreferredPacketDescription(this) {
                Ok(ok__) => {
                    preferredpacketdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecognizerContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer_Impl::CreateRecognizerContext(this) {
                Ok(ok__) => {
                    context.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Vendor: Vendor::<Identity, OFFSET>,
            Capabilities: Capabilities::<Identity, OFFSET>,
            Languages: Languages::<Identity, OFFSET>,
            SupportedProperties: SupportedProperties::<Identity, OFFSET>,
            PreferredPacketDescription: PreferredPacketDescription::<Identity, OFFSET>,
            CreateRecognizerContext: CreateRecognizerContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognizer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizer2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UnicodeRanges(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognizer2 {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognizer2_Vtbl
    where
        Identity: IInkRecognizer2_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer2_Impl::Id(this) {
                Ok(ok__) => {
                    pbstrid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicoderanges: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizer2_Impl::UnicodeRanges(this) {
                Ok(ok__) => {
                    unicoderanges.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            UnicodeRanges: UnicodeRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognizer2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Strokes(&self) -> windows_core::Result<IInkStrokes>;
    fn putref_Strokes(&self, strokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn CharacterAutoCompletionMode(&self) -> windows_core::Result<InkRecognizerCharacterAutoCompletionMode>;
    fn SetCharacterAutoCompletionMode(&self, mode: InkRecognizerCharacterAutoCompletionMode) -> windows_core::Result<()>;
    fn Factoid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFactoid(&self, factoid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Guide(&self) -> windows_core::Result<IInkRecognizerGuide>;
    fn putref_Guide(&self, recognizerguide: Option<&IInkRecognizerGuide>) -> windows_core::Result<()>;
    fn PrefixText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrefixText(&self, prefix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SuffixText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSuffixText(&self, suffix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RecognitionFlags(&self) -> windows_core::Result<InkRecognitionModes>;
    fn SetRecognitionFlags(&self, modes: InkRecognitionModes) -> windows_core::Result<()>;
    fn WordList(&self) -> windows_core::Result<IInkWordList>;
    fn putref_WordList(&self, wordlist: Option<&IInkWordList>) -> windows_core::Result<()>;
    fn Recognizer(&self) -> windows_core::Result<IInkRecognizer>;
    fn Recognize(&self, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut Option<IInkRecognitionResult>) -> windows_core::Result<()>;
    fn StopBackgroundRecognition(&self) -> windows_core::Result<()>;
    fn EndInkInput(&self) -> windows_core::Result<()>;
    fn BackgroundRecognize(&self, customdata: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn BackgroundRecognizeWithAlternates(&self, customdata: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IInkRecognizerContext>;
    fn IsStringSupported(&self, string: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognizerContext {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognizerContext_Vtbl
    where
        Identity: IInkRecognizerContext_Impl,
    {
        unsafe extern "system" fn Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::Strokes(this) {
                Ok(ok__) => {
                    strokes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Strokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::putref_Strokes(this, windows_core::from_raw_borrowed(&strokes)).into()
        }
        unsafe extern "system" fn CharacterAutoCompletionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::CharacterAutoCompletionMode(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterAutoCompletionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: InkRecognizerCharacterAutoCompletionMode) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::SetCharacterAutoCompletionMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Factoid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::Factoid(this) {
                Ok(ok__) => {
                    factoid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::SetFactoid(this, core::mem::transmute(&factoid)).into()
        }
        unsafe extern "system" fn Guide<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizerguide: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::Guide(this) {
                Ok(ok__) => {
                    recognizerguide.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Guide<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizerguide: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::putref_Guide(this, windows_core::from_raw_borrowed(&recognizerguide)).into()
        }
        unsafe extern "system" fn PrefixText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::PrefixText(this) {
                Ok(ok__) => {
                    prefix.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefixText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::SetPrefixText(this, core::mem::transmute(&prefix)).into()
        }
        unsafe extern "system" fn SuffixText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, suffix: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::SuffixText(this) {
                Ok(ok__) => {
                    suffix.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuffixText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, suffix: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::SetSuffixText(this, core::mem::transmute(&suffix)).into()
        }
        unsafe extern "system" fn RecognitionFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modes: *mut InkRecognitionModes) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::RecognitionFlags(this) {
                Ok(ok__) => {
                    modes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecognitionFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modes: InkRecognitionModes) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::SetRecognitionFlags(this, core::mem::transmute_copy(&modes)).into()
        }
        unsafe extern "system" fn WordList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::WordList(this) {
                Ok(ok__) => {
                    wordlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_WordList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordlist: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::putref_WordList(this, windows_core::from_raw_borrowed(&wordlist)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::Recognizer(this) {
                Ok(ok__) => {
                    recognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::Recognize(this, core::mem::transmute_copy(&recognitionstatus), core::mem::transmute_copy(&recognitionresult)).into()
        }
        unsafe extern "system" fn StopBackgroundRecognition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::StopBackgroundRecognition(this).into()
        }
        unsafe extern "system" fn EndInkInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::EndInkInput(this).into()
        }
        unsafe extern "system" fn BackgroundRecognize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customdata: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::BackgroundRecognize(this, core::mem::transmute(&customdata)).into()
        }
        unsafe extern "system" fn BackgroundRecognizeWithAlternates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customdata: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext_Impl::BackgroundRecognizeWithAlternates(this, core::mem::transmute(&customdata)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recocontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::Clone(this) {
                Ok(ok__) => {
                    recocontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStringSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: core::mem::MaybeUninit<windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext_Impl::IsStringSupported(this, core::mem::transmute(&string)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Strokes: Strokes::<Identity, OFFSET>,
            putref_Strokes: putref_Strokes::<Identity, OFFSET>,
            CharacterAutoCompletionMode: CharacterAutoCompletionMode::<Identity, OFFSET>,
            SetCharacterAutoCompletionMode: SetCharacterAutoCompletionMode::<Identity, OFFSET>,
            Factoid: Factoid::<Identity, OFFSET>,
            SetFactoid: SetFactoid::<Identity, OFFSET>,
            Guide: Guide::<Identity, OFFSET>,
            putref_Guide: putref_Guide::<Identity, OFFSET>,
            PrefixText: PrefixText::<Identity, OFFSET>,
            SetPrefixText: SetPrefixText::<Identity, OFFSET>,
            SuffixText: SuffixText::<Identity, OFFSET>,
            SetSuffixText: SetSuffixText::<Identity, OFFSET>,
            RecognitionFlags: RecognitionFlags::<Identity, OFFSET>,
            SetRecognitionFlags: SetRecognitionFlags::<Identity, OFFSET>,
            WordList: WordList::<Identity, OFFSET>,
            putref_WordList: putref_WordList::<Identity, OFFSET>,
            Recognizer: Recognizer::<Identity, OFFSET>,
            Recognize: Recognize::<Identity, OFFSET>,
            StopBackgroundRecognition: StopBackgroundRecognition::<Identity, OFFSET>,
            EndInkInput: EndInkInput::<Identity, OFFSET>,
            BackgroundRecognize: BackgroundRecognize::<Identity, OFFSET>,
            BackgroundRecognizeWithAlternates: BackgroundRecognizeWithAlternates::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            IsStringSupported: IsStringSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognizerContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerContext2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnabledUnicodeRanges(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetEnabledUnicodeRanges(&self, unicoderanges: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognizerContext2 {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerContext2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognizerContext2_Vtbl
    where
        Identity: IInkRecognizerContext2_Impl,
    {
        unsafe extern "system" fn EnabledUnicodeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicoderanges: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerContext2_Impl::EnabledUnicodeRanges(this) {
                Ok(ok__) => {
                    unicoderanges.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabledUnicodeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicoderanges: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerContext2_Impl::SetEnabledUnicodeRanges(this, core::mem::transmute(&unicoderanges)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnabledUnicodeRanges: EnabledUnicodeRanges::<Identity, OFFSET>,
            SetEnabledUnicodeRanges: SetEnabledUnicodeRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognizerContext2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerGuide_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WritingBox(&self) -> windows_core::Result<IInkRectangle>;
    fn SetWritingBox(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn DrawnBox(&self) -> windows_core::Result<IInkRectangle>;
    fn SetDrawnBox(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn Rows(&self) -> windows_core::Result<i32>;
    fn SetRows(&self, units: i32) -> windows_core::Result<()>;
    fn Columns(&self) -> windows_core::Result<i32>;
    fn SetColumns(&self, units: i32) -> windows_core::Result<()>;
    fn Midline(&self) -> windows_core::Result<i32>;
    fn SetMidline(&self, units: i32) -> windows_core::Result<()>;
    fn GuideData(&self, precoguide: *mut InkRecoGuide) -> windows_core::Result<()>;
    fn SetGuideData(&self, recoguide: &InkRecoGuide) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognizerGuide {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizerGuide_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognizerGuide_Vtbl
    where
        Identity: IInkRecognizerGuide_Impl,
    {
        unsafe extern "system" fn WritingBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerGuide_Impl::WritingBox(this) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWritingBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::SetWritingBox(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn DrawnBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerGuide_Impl::DrawnBox(this) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDrawnBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::SetDrawnBox(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn Rows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerGuide_Impl::Rows(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::SetRows(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Columns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerGuide_Impl::Columns(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::SetColumns(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Midline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizerGuide_Impl::Midline(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMidline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::SetMidline(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn GuideData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precoguide: *mut InkRecoGuide) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::GuideData(this, core::mem::transmute_copy(&precoguide)).into()
        }
        unsafe extern "system" fn SetGuideData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recoguide: InkRecoGuide) -> windows_core::HRESULT
        where
            Identity: IInkRecognizerGuide_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRecognizerGuide_Impl::SetGuideData(this, core::mem::transmute(&recoguide)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WritingBox: WritingBox::<Identity, OFFSET>,
            SetWritingBox: SetWritingBox::<Identity, OFFSET>,
            DrawnBox: DrawnBox::<Identity, OFFSET>,
            SetDrawnBox: SetDrawnBox::<Identity, OFFSET>,
            Rows: Rows::<Identity, OFFSET>,
            SetRows: SetRows::<Identity, OFFSET>,
            Columns: Columns::<Identity, OFFSET>,
            SetColumns: SetColumns::<Identity, OFFSET>,
            Midline: Midline::<Identity, OFFSET>,
            SetMidline: SetMidline::<Identity, OFFSET>,
            GuideData: GuideData::<Identity, OFFSET>,
            SetGuideData: SetGuideData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognizerGuide as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizers_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDefaultRecognizer(&self, lcid: i32) -> windows_core::Result<IInkRecognizer>;
    fn Item(&self, index: i32) -> windows_core::Result<IInkRecognizer>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRecognizers {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRecognizers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRecognizers_Vtbl
    where
        Identity: IInkRecognizers_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRecognizers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizers_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizers_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultRecognizer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: i32, defaultrecognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizers_Impl::GetDefaultRecognizer(this, core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    defaultrecognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, inkrecognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRecognizers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRecognizers_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    inkrecognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            GetDefaultRecognizer: GetDefaultRecognizer::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRecognizers as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRectangle_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Top(&self) -> windows_core::Result<i32>;
    fn SetTop(&self, units: i32) -> windows_core::Result<()>;
    fn Left(&self) -> windows_core::Result<i32>;
    fn SetLeft(&self, units: i32) -> windows_core::Result<()>;
    fn Bottom(&self) -> windows_core::Result<i32>;
    fn SetBottom(&self, units: i32) -> windows_core::Result<()>;
    fn Right(&self) -> windows_core::Result<i32>;
    fn SetRight(&self, units: i32) -> windows_core::Result<()>;
    fn Data(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn SetData(&self, rect: &super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetRectangle(&self, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> windows_core::Result<()>;
    fn SetRectangle(&self, top: i32, left: i32, bottom: i32, right: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRectangle {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRectangle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRectangle_Vtbl
    where
        Identity: IInkRectangle_Impl,
    {
        unsafe extern "system" fn Top<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRectangle_Impl::Top(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::SetTop(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Left<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRectangle_Impl::Left(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeft<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::SetLeft(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Bottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRectangle_Impl::Bottom(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::SetBottom(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Right<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRectangle_Impl::Right(this) {
                Ok(ok__) => {
                    units.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, units: i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::SetRight(this, core::mem::transmute_copy(&units)).into()
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRectangle_Impl::Data(this) {
                Ok(ok__) => {
                    rect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::SetData(this, core::mem::transmute(&rect)).into()
        }
        unsafe extern "system" fn GetRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::GetRectangle(this, core::mem::transmute_copy(&top), core::mem::transmute_copy(&left), core::mem::transmute_copy(&bottom), core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: i32, left: i32, bottom: i32, right: i32) -> windows_core::HRESULT
        where
            Identity: IInkRectangle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRectangle_Impl::SetRectangle(this, core::mem::transmute_copy(&top), core::mem::transmute_copy(&left), core::mem::transmute_copy(&bottom), core::mem::transmute_copy(&right)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Top: Top::<Identity, OFFSET>,
            SetTop: SetTop::<Identity, OFFSET>,
            Left: Left::<Identity, OFFSET>,
            SetLeft: SetLeft::<Identity, OFFSET>,
            Bottom: Bottom::<Identity, OFFSET>,
            SetBottom: SetBottom::<Identity, OFFSET>,
            Right: Right::<Identity, OFFSET>,
            SetRight: SetRight::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            GetRectangle: GetRectangle::<Identity, OFFSET>,
            SetRectangle: SetRectangle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRectangle as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRenderer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetViewTransform(&self, viewtransform: Option<&IInkTransform>) -> windows_core::Result<()>;
    fn SetViewTransform(&self, viewtransform: Option<&IInkTransform>) -> windows_core::Result<()>;
    fn GetObjectTransform(&self, objecttransform: Option<&IInkTransform>) -> windows_core::Result<()>;
    fn SetObjectTransform(&self, objecttransform: Option<&IInkTransform>) -> windows_core::Result<()>;
    fn Draw(&self, hdc: isize, strokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn DrawStroke(&self, hdc: isize, stroke: Option<&IInkStrokeDisp>, drawingattributes: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn PixelToInkSpace(&self, hdc: isize, x: *mut i32, y: *mut i32) -> windows_core::Result<()>;
    fn InkSpaceToPixel(&self, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> windows_core::Result<()>;
    fn PixelToInkSpaceFromPoints(&self, hdc: isize, points: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn InkSpaceToPixelFromPoints(&self, hdc: isize, points: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn Measure(&self, strokes: Option<&IInkStrokes>) -> windows_core::Result<IInkRectangle>;
    fn MeasureStroke(&self, stroke: Option<&IInkStrokeDisp>, drawingattributes: Option<&IInkDrawingAttributes>) -> windows_core::Result<IInkRectangle>;
    fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkRenderer {}
#[cfg(feature = "Win32_System_Com")]
impl IInkRenderer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkRenderer_Vtbl
    where
        Identity: IInkRenderer_Impl,
    {
        unsafe extern "system" fn GetViewTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewtransform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::GetViewTransform(this, windows_core::from_raw_borrowed(&viewtransform)).into()
        }
        unsafe extern "system" fn SetViewTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewtransform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::SetViewTransform(this, windows_core::from_raw_borrowed(&viewtransform)).into()
        }
        unsafe extern "system" fn GetObjectTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objecttransform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::GetObjectTransform(this, windows_core::from_raw_borrowed(&objecttransform)).into()
        }
        unsafe extern "system" fn SetObjectTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objecttransform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::SetObjectTransform(this, windows_core::from_raw_borrowed(&objecttransform)).into()
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: isize, strokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::Draw(this, core::mem::transmute_copy(&hdc), windows_core::from_raw_borrowed(&strokes)).into()
        }
        unsafe extern "system" fn DrawStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: isize, stroke: *mut core::ffi::c_void, drawingattributes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::DrawStroke(this, core::mem::transmute_copy(&hdc), windows_core::from_raw_borrowed(&stroke), windows_core::from_raw_borrowed(&drawingattributes)).into()
        }
        unsafe extern "system" fn PixelToInkSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: isize, x: *mut i32, y: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::PixelToInkSpace(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InkSpaceToPixel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::InkSpaceToPixel(this, core::mem::transmute_copy(&hdcdisplay), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn PixelToInkSpaceFromPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: isize, points: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::PixelToInkSpaceFromPoints(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&points)).into()
        }
        unsafe extern "system" fn InkSpaceToPixelFromPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: isize, points: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::InkSpaceToPixelFromPoints(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&points)).into()
        }
        unsafe extern "system" fn Measure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRenderer_Impl::Measure(this, windows_core::from_raw_borrowed(&strokes)) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stroke: *mut core::ffi::c_void, drawingattributes: *mut core::ffi::c_void, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkRenderer_Impl::MeasureStroke(this, windows_core::from_raw_borrowed(&stroke), windows_core::from_raw_borrowed(&drawingattributes)) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::Move(this, core::mem::transmute_copy(&horizontalcomponent), core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, degrees: f32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::Rotate(this, core::mem::transmute_copy(&degrees), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkRenderer_Impl::ScaleTransform(this, core::mem::transmute_copy(&horizontalmultiplier), core::mem::transmute_copy(&verticalmultiplier), core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetViewTransform: GetViewTransform::<Identity, OFFSET>,
            SetViewTransform: SetViewTransform::<Identity, OFFSET>,
            GetObjectTransform: GetObjectTransform::<Identity, OFFSET>,
            SetObjectTransform: SetObjectTransform::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            DrawStroke: DrawStroke::<Identity, OFFSET>,
            PixelToInkSpace: PixelToInkSpace::<Identity, OFFSET>,
            InkSpaceToPixel: InkSpaceToPixel::<Identity, OFFSET>,
            PixelToInkSpaceFromPoints: PixelToInkSpaceFromPoints::<Identity, OFFSET>,
            InkSpaceToPixelFromPoints: InkSpaceToPixelFromPoints::<Identity, OFFSET>,
            Measure: Measure::<Identity, OFFSET>,
            MeasureStroke: MeasureStroke::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Rotate: Rotate::<Identity, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkRenderer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkStrokeDisp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ID(&self) -> windows_core::Result<i32>;
    fn BezierPoints(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DrawingAttributes(&self) -> windows_core::Result<IInkDrawingAttributes>;
    fn putref_DrawingAttributes(&self, drawattrs: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn Ink(&self) -> windows_core::Result<IInkDisp>;
    fn ExtendedProperties(&self) -> windows_core::Result<IInkExtendedProperties>;
    fn PolylineCusps(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn BezierCusps(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SelfIntersections(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn PacketCount(&self) -> windows_core::Result<i32>;
    fn PacketSize(&self) -> windows_core::Result<i32>;
    fn PacketDescription(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Deleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> windows_core::Result<IInkRectangle>;
    fn FindIntersections(&self, strokes: Option<&IInkStrokes>) -> windows_core::Result<windows_core::VARIANT>;
    fn GetRectangleIntersections(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<windows_core::VARIANT>;
    fn Clip(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn NearestPoint(&self, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> windows_core::Result<()>;
    fn Split(&self, splitat: f32) -> windows_core::Result<IInkStrokeDisp>;
    fn GetPacketDescriptionPropertyMetrics(&self, propertyname: &windows_core::BSTR, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> windows_core::Result<()>;
    fn GetPoints(&self, index: i32, count: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPoints(&self, points: &windows_core::VARIANT, index: i32, count: i32) -> windows_core::Result<i32>;
    fn GetPacketData(&self, index: i32, count: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn GetPacketValuesByProperty(&self, propertyname: &windows_core::BSTR, index: i32, count: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPacketValuesByProperty(&self, bstrpropertyname: &windows_core::BSTR, packetvalues: &windows_core::VARIANT, index: i32, count: i32) -> windows_core::Result<i32>;
    fn GetFlattenedBezierPoints(&self, fittingerror: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn Transform(&self, transform: Option<&IInkTransform>, applyonpenwidth: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ScaleToRectangle(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> windows_core::Result<()>;
    fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkStrokeDisp {}
#[cfg(feature = "Win32_System_Com")]
impl IInkStrokeDisp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkStrokeDisp_Vtbl
    where
        Identity: IInkStrokeDisp_Impl,
    {
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::ID(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::BezierPoints(this) {
                Ok(ok__) => {
                    points.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawattrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::DrawingAttributes(this) {
                Ok(ok__) => {
                    drawattrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_DrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawattrs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::putref_DrawingAttributes(this, windows_core::from_raw_borrowed(&drawattrs)).into()
        }
        unsafe extern "system" fn Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::Ink(this) {
                Ok(ok__) => {
                    ink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::ExtendedProperties(this) {
                Ok(ok__) => {
                    properties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolylineCusps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cusps: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::PolylineCusps(this) {
                Ok(ok__) => {
                    cusps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BezierCusps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cusps: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::BezierCusps(this) {
                Ok(ok__) => {
                    cusps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelfIntersections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, intersections: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::SelfIntersections(this) {
                Ok(ok__) => {
                    intersections.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::PacketCount(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::PacketSize(this) {
                Ok(ok__) => {
                    plsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetdescription: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::PacketDescription(this) {
                Ok(ok__) => {
                    packetdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deleted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::Deleted(this) {
                Ok(ok__) => {
                    deleted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::GetBoundingBox(this, core::mem::transmute_copy(&boundingboxmode)) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindIntersections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokes: *mut core::ffi::c_void, intersections: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::FindIntersections(this, windows_core::from_raw_borrowed(&strokes)) {
                Ok(ok__) => {
                    intersections.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRectangleIntersections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void, intersections: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::GetRectangleIntersections(this, windows_core::from_raw_borrowed(&rectangle)) {
                Ok(ok__) => {
                    intersections.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::Clip(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn HitTestCircle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, radius: f32, intersects: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::HitTestCircle(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&radius)) {
                Ok(ok__) => {
                    intersects.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearestPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::NearestPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&distance), core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn Split<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, splitat: f32, newstroke: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::Split(this, core::mem::transmute_copy(&splitat)) {
                Ok(ok__) => {
                    newstroke.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketDescriptionPropertyMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::GetPacketDescriptionPropertyMetrics(this, core::mem::transmute(&propertyname), core::mem::transmute_copy(&minimum), core::mem::transmute_copy(&maximum), core::mem::transmute_copy(&units), core::mem::transmute_copy(&resolution)).into()
        }
        unsafe extern "system" fn GetPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, count: i32, points: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::GetPoints(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    points.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: core::mem::MaybeUninit<windows_core::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::SetPoints(this, core::mem::transmute(&points), core::mem::transmute_copy(&index), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    numberofpointsset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, count: i32, packetdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::GetPacketData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    packetdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketValuesByProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, index: i32, count: i32, packetvalues: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::GetPacketValuesByProperty(this, core::mem::transmute(&propertyname), core::mem::transmute_copy(&index), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    packetvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPacketValuesByProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, packetvalues: core::mem::MaybeUninit<windows_core::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::SetPacketValuesByProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute(&packetvalues), core::mem::transmute_copy(&index), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    numberofpacketsset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlattenedBezierPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fittingerror: i32, flattenedbezierpoints: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokeDisp_Impl::GetFlattenedBezierPoints(this, core::mem::transmute_copy(&fittingerror)) {
                Ok(ok__) => {
                    flattenedbezierpoints.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void, applyonpenwidth: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::Transform(this, windows_core::from_raw_borrowed(&transform), core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        unsafe extern "system" fn ScaleToRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::ScaleToRectangle(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::Move(this, core::mem::transmute_copy(&horizontalcomponent), core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, degrees: f32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::Rotate(this, core::mem::transmute_copy(&degrees), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Shear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::Shear(this, core::mem::transmute_copy(&horizontalmultiplier), core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokeDisp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokeDisp_Impl::ScaleTransform(this, core::mem::transmute_copy(&horizontalmultiplier), core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ID: ID::<Identity, OFFSET>,
            BezierPoints: BezierPoints::<Identity, OFFSET>,
            DrawingAttributes: DrawingAttributes::<Identity, OFFSET>,
            putref_DrawingAttributes: putref_DrawingAttributes::<Identity, OFFSET>,
            Ink: Ink::<Identity, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, OFFSET>,
            PolylineCusps: PolylineCusps::<Identity, OFFSET>,
            BezierCusps: BezierCusps::<Identity, OFFSET>,
            SelfIntersections: SelfIntersections::<Identity, OFFSET>,
            PacketCount: PacketCount::<Identity, OFFSET>,
            PacketSize: PacketSize::<Identity, OFFSET>,
            PacketDescription: PacketDescription::<Identity, OFFSET>,
            Deleted: Deleted::<Identity, OFFSET>,
            GetBoundingBox: GetBoundingBox::<Identity, OFFSET>,
            FindIntersections: FindIntersections::<Identity, OFFSET>,
            GetRectangleIntersections: GetRectangleIntersections::<Identity, OFFSET>,
            Clip: Clip::<Identity, OFFSET>,
            HitTestCircle: HitTestCircle::<Identity, OFFSET>,
            NearestPoint: NearestPoint::<Identity, OFFSET>,
            Split: Split::<Identity, OFFSET>,
            GetPacketDescriptionPropertyMetrics: GetPacketDescriptionPropertyMetrics::<Identity, OFFSET>,
            GetPoints: GetPoints::<Identity, OFFSET>,
            SetPoints: SetPoints::<Identity, OFFSET>,
            GetPacketData: GetPacketData::<Identity, OFFSET>,
            GetPacketValuesByProperty: GetPacketValuesByProperty::<Identity, OFFSET>,
            SetPacketValuesByProperty: SetPacketValuesByProperty::<Identity, OFFSET>,
            GetFlattenedBezierPoints: GetFlattenedBezierPoints::<Identity, OFFSET>,
            Transform: Transform::<Identity, OFFSET>,
            ScaleToRectangle: ScaleToRectangle::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Rotate: Rotate::<Identity, OFFSET>,
            Shear: Shear::<Identity, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkStrokeDisp as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkStrokes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Ink(&self) -> windows_core::Result<IInkDisp>;
    fn RecognitionResult(&self) -> windows_core::Result<IInkRecognitionResult>;
    fn ToString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Item(&self, index: i32) -> windows_core::Result<IInkStrokeDisp>;
    fn Add(&self, inkstroke: Option<&IInkStrokeDisp>) -> windows_core::Result<()>;
    fn AddStrokes(&self, inkstrokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn Remove(&self, inkstroke: Option<&IInkStrokeDisp>) -> windows_core::Result<()>;
    fn RemoveStrokes(&self, inkstrokes: Option<&IInkStrokes>) -> windows_core::Result<()>;
    fn ModifyDrawingAttributes(&self, drawattrs: Option<&IInkDrawingAttributes>) -> windows_core::Result<()>;
    fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> windows_core::Result<IInkRectangle>;
    fn Transform(&self, transform: Option<&IInkTransform>, applyonpenwidth: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ScaleToRectangle(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> windows_core::Result<()>;
    fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::Result<()>;
    fn Clip(&self, rectangle: Option<&IInkRectangle>) -> windows_core::Result<()>;
    fn RemoveRecognitionResult(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkStrokes {}
#[cfg(feature = "Win32_System_Com")]
impl IInkStrokes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkStrokes_Vtbl
    where
        Identity: IInkStrokes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::Ink(this) {
                Ok(ok__) => {
                    ink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognitionresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::RecognitionResult(this) {
                Ok(ok__) => {
                    recognitionresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tostring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::ToString(this) {
                Ok(ok__) => {
                    tostring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, stroke: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    stroke.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstroke: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Add(this, windows_core::from_raw_borrowed(&inkstroke)).into()
        }
        unsafe extern "system" fn AddStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstrokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::AddStrokes(this, windows_core::from_raw_borrowed(&inkstrokes)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstroke: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Remove(this, windows_core::from_raw_borrowed(&inkstroke)).into()
        }
        unsafe extern "system" fn RemoveStrokes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstrokes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::RemoveStrokes(this, windows_core::from_raw_borrowed(&inkstrokes)).into()
        }
        unsafe extern "system" fn ModifyDrawingAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawattrs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::ModifyDrawingAttributes(this, windows_core::from_raw_borrowed(&drawattrs)).into()
        }
        unsafe extern "system" fn GetBoundingBox<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkStrokes_Impl::GetBoundingBox(this, core::mem::transmute_copy(&boundingboxmode)) {
                Ok(ok__) => {
                    boundingbox.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void, applyonpenwidth: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Transform(this, windows_core::from_raw_borrowed(&transform), core::mem::transmute_copy(&applyonpenwidth)).into()
        }
        unsafe extern "system" fn ScaleToRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::ScaleToRectangle(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Move(this, core::mem::transmute_copy(&horizontalcomponent), core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, degrees: f32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Rotate(this, core::mem::transmute_copy(&degrees), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Shear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Shear(this, core::mem::transmute_copy(&horizontalmultiplier), core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::ScaleTransform(this, core::mem::transmute_copy(&horizontalmultiplier), core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn Clip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::Clip(this, windows_core::from_raw_borrowed(&rectangle)).into()
        }
        unsafe extern "system" fn RemoveRecognitionResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkStrokes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkStrokes_Impl::RemoveRecognitionResult(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Ink: Ink::<Identity, OFFSET>,
            RecognitionResult: RecognitionResult::<Identity, OFFSET>,
            ToString: ToString::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            AddStrokes: AddStrokes::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveStrokes: RemoveStrokes::<Identity, OFFSET>,
            ModifyDrawingAttributes: ModifyDrawingAttributes::<Identity, OFFSET>,
            GetBoundingBox: GetBoundingBox::<Identity, OFFSET>,
            Transform: Transform::<Identity, OFFSET>,
            ScaleToRectangle: ScaleToRectangle::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Rotate: Rotate::<Identity, OFFSET>,
            Shear: Shear::<Identity, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, OFFSET>,
            Clip: Clip::<Identity, OFFSET>,
            RemoveRecognitionResult: RemoveRecognitionResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkStrokes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PlugAndPlayId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MaximumInputRectangle(&self) -> windows_core::Result<IInkRectangle>;
    fn HardwareCapabilities(&self) -> windows_core::Result<TabletHardwareCapabilities>;
    fn IsPacketPropertySupported(&self, packetpropertyname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyMetrics(&self, propertyname: &windows_core::BSTR, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkTablet {}
#[cfg(feature = "Win32_System_Com")]
impl IInkTablet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkTablet_Vtbl
    where
        Identity: IInkTablet_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkTablet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlugAndPlayId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkTablet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet_Impl::PlugAndPlayId(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkTablet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet_Impl::MaximumInputRectangle(this) {
                Ok(ok__) => {
                    rectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: *mut TabletHardwareCapabilities) -> windows_core::HRESULT
        where
            Identity: IInkTablet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet_Impl::HardwareCapabilities(this) {
                Ok(ok__) => {
                    capabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkTablet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet_Impl::IsPacketPropertySupported(this, core::mem::transmute(&packetpropertyname)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTablet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTablet_Impl::GetPropertyMetrics(this, core::mem::transmute(&propertyname), core::mem::transmute_copy(&minimum), core::mem::transmute_copy(&maximum), core::mem::transmute_copy(&units), core::mem::transmute_copy(&resolution)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            PlugAndPlayId: PlugAndPlayId::<Identity, OFFSET>,
            MaximumInputRectangle: MaximumInputRectangle::<Identity, OFFSET>,
            HardwareCapabilities: HardwareCapabilities::<Identity, OFFSET>,
            IsPacketPropertySupported: IsPacketPropertySupported::<Identity, OFFSET>,
            GetPropertyMetrics: GetPropertyMetrics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkTablet as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeviceKind(&self) -> windows_core::Result<TabletDeviceKind>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkTablet2 {}
#[cfg(feature = "Win32_System_Com")]
impl IInkTablet2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkTablet2_Vtbl
    where
        Identity: IInkTablet2_Impl,
    {
        unsafe extern "system" fn DeviceKind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: *mut TabletDeviceKind) -> windows_core::HRESULT
        where
            Identity: IInkTablet2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet2_Impl::DeviceKind(this) {
                Ok(ok__) => {
                    kind.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), DeviceKind: DeviceKind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkTablet2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet3_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsMultiTouch(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MaximumCursors(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkTablet3 {}
#[cfg(feature = "Win32_System_Com")]
impl IInkTablet3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkTablet3_Vtbl
    where
        Identity: IInkTablet3_Impl,
    {
        unsafe extern "system" fn IsMultiTouch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pismultitouch: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkTablet3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet3_Impl::IsMultiTouch(this) {
                Ok(ok__) => {
                    pismultitouch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumCursors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaximumcursors: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInkTablet3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablet3_Impl::MaximumCursors(this) {
                Ok(ok__) => {
                    pmaximumcursors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsMultiTouch: IsMultiTouch::<Identity, OFFSET>,
            MaximumCursors: MaximumCursors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkTablet3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablets_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn DefaultTablet(&self) -> windows_core::Result<IInkTablet>;
    fn Item(&self, index: i32) -> windows_core::Result<IInkTablet>;
    fn IsPacketPropertySupported(&self, packetpropertyname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkTablets {}
#[cfg(feature = "Win32_System_Com")]
impl IInkTablets_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkTablets_Vtbl
    where
        Identity: IInkTablets_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInkTablets_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablets_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkTablets_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablets_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    _newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaulttablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkTablets_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablets_Impl::DefaultTablet(this) {
                Ok(ok__) => {
                    defaulttablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, tablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkTablets_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablets_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    tablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPacketPropertySupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkTablets_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTablets_Impl::IsPacketPropertySupported(this, core::mem::transmute(&packetpropertyname)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            DefaultTablet: DefaultTablet::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            IsPacketPropertySupported: IsPacketPropertySupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkTablets as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IInkTransform_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Translate(&self, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f32, x: f32, y: f32) -> windows_core::Result<()>;
    fn Reflect(&self, horizontally: super::super::Foundation::VARIANT_BOOL, vertically: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Shear(&self, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::Result<()>;
    fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::Result<()>;
    fn GetTransform(&self, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> windows_core::Result<()>;
    fn SetTransform(&self, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> windows_core::Result<()>;
    fn eM11(&self) -> windows_core::Result<f32>;
    fn SeteM11(&self, value: f32) -> windows_core::Result<()>;
    fn eM12(&self) -> windows_core::Result<f32>;
    fn SeteM12(&self, value: f32) -> windows_core::Result<()>;
    fn eM21(&self) -> windows_core::Result<f32>;
    fn SeteM21(&self, value: f32) -> windows_core::Result<()>;
    fn eM22(&self) -> windows_core::Result<f32>;
    fn SeteM22(&self, value: f32) -> windows_core::Result<()>;
    fn eDx(&self) -> windows_core::Result<f32>;
    fn SeteDx(&self, value: f32) -> windows_core::Result<()>;
    fn eDy(&self) -> windows_core::Result<f32>;
    fn SeteDy(&self, value: f32) -> windows_core::Result<()>;
    fn Data(&self, xform: *mut super::super::Graphics::Gdi::XFORM) -> windows_core::Result<()>;
    fn SetData(&self, xform: &super::super::Graphics::Gdi::XFORM) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IInkTransform {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IInkTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkTransform_Vtbl
    where
        Identity: IInkTransform_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Translate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::Translate(this, core::mem::transmute_copy(&horizontalcomponent), core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn Rotate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, degrees: f32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::Rotate(this, core::mem::transmute_copy(&degrees), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Reflect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontally: super::super::Foundation::VARIANT_BOOL, vertically: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::Reflect(this, core::mem::transmute_copy(&horizontally), core::mem::transmute_copy(&vertically)).into()
        }
        unsafe extern "system" fn Shear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalcomponent: f32, verticalcomponent: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::Shear(this, core::mem::transmute_copy(&horizontalcomponent), core::mem::transmute_copy(&verticalcomponent)).into()
        }
        unsafe extern "system" fn ScaleTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalmultiplier: f32, verticalmultiplier: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::ScaleTransform(this, core::mem::transmute_copy(&horizontalmultiplier), core::mem::transmute_copy(&verticalmultiplier)).into()
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::GetTransform(this, core::mem::transmute_copy(&em11), core::mem::transmute_copy(&em12), core::mem::transmute_copy(&em21), core::mem::transmute_copy(&em22), core::mem::transmute_copy(&edx), core::mem::transmute_copy(&edy)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SetTransform(this, core::mem::transmute_copy(&em11), core::mem::transmute_copy(&em12), core::mem::transmute_copy(&em21), core::mem::transmute_copy(&em22), core::mem::transmute_copy(&edx), core::mem::transmute_copy(&edy)).into()
        }
        unsafe extern "system" fn eM11<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTransform_Impl::eM11(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM11<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SeteM11(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM12<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTransform_Impl::eM12(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM12<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SeteM12(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM21<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTransform_Impl::eM21(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM21<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SeteM21(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eM22<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTransform_Impl::eM22(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteM22<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SeteM22(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eDx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTransform_Impl::eDx(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SeteDx(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn eDy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInkTransform_Impl::eDy(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeteDy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SeteDy(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xform: *mut super::super::Graphics::Gdi::XFORM) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::Data(this, core::mem::transmute_copy(&xform)).into()
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xform: super::super::Graphics::Gdi::XFORM) -> windows_core::HRESULT
        where
            Identity: IInkTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkTransform_Impl::SetData(this, core::mem::transmute(&xform)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Translate: Translate::<Identity, OFFSET>,
            Rotate: Rotate::<Identity, OFFSET>,
            Reflect: Reflect::<Identity, OFFSET>,
            Shear: Shear::<Identity, OFFSET>,
            ScaleTransform: ScaleTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            eM11: eM11::<Identity, OFFSET>,
            SeteM11: SeteM11::<Identity, OFFSET>,
            eM12: eM12::<Identity, OFFSET>,
            SeteM12: SeteM12::<Identity, OFFSET>,
            eM21: eM21::<Identity, OFFSET>,
            SeteM21: SeteM21::<Identity, OFFSET>,
            eM22: eM22::<Identity, OFFSET>,
            SeteM22: SeteM22::<Identity, OFFSET>,
            eDx: eDx::<Identity, OFFSET>,
            SeteDx: SeteDx::<Identity, OFFSET>,
            eDy: eDy::<Identity, OFFSET>,
            SeteDy: SeteDy::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkTransform as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkWordList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddWord(&self, newword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveWord(&self, removeword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Merge(&self, mergewordlist: Option<&IInkWordList>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkWordList {}
#[cfg(feature = "Win32_System_Com")]
impl IInkWordList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkWordList_Vtbl
    where
        Identity: IInkWordList_Impl,
    {
        unsafe extern "system" fn AddWord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkWordList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkWordList_Impl::AddWord(this, core::mem::transmute(&newword)).into()
        }
        unsafe extern "system" fn RemoveWord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, removeword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkWordList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkWordList_Impl::RemoveWord(this, core::mem::transmute(&removeword)).into()
        }
        unsafe extern "system" fn Merge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mergewordlist: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInkWordList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkWordList_Impl::Merge(this, windows_core::from_raw_borrowed(&mergewordlist)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AddWord: AddWord::<Identity, OFFSET>,
            RemoveWord: RemoveWord::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkWordList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkWordList2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddWords(&self, newwords: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInkWordList2 {}
#[cfg(feature = "Win32_System_Com")]
impl IInkWordList2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInkWordList2_Vtbl
    where
        Identity: IInkWordList2_Impl,
    {
        unsafe extern "system" fn AddWords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newwords: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInkWordList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInkWordList2_Impl::AddWords(this, core::mem::transmute(&newwords)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), AddWords: AddWords::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkWordList2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IInputPanelWindowHandle_Impl: Sized {
    fn AttachedEditWindow32(&self) -> windows_core::Result<i32>;
    fn SetAttachedEditWindow32(&self, attachededitwindow: i32) -> windows_core::Result<()>;
    fn AttachedEditWindow64(&self) -> windows_core::Result<i64>;
    fn SetAttachedEditWindow64(&self, attachededitwindow: i64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInputPanelWindowHandle {}
impl IInputPanelWindowHandle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInputPanelWindowHandle_Vtbl
    where
        Identity: IInputPanelWindowHandle_Impl,
    {
        unsafe extern "system" fn AttachedEditWindow32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInputPanelWindowHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInputPanelWindowHandle_Impl::AttachedEditWindow32(this) {
                Ok(ok__) => {
                    attachededitwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: i32) -> windows_core::HRESULT
        where
            Identity: IInputPanelWindowHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInputPanelWindowHandle_Impl::SetAttachedEditWindow32(this, core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn AttachedEditWindow64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut i64) -> windows_core::HRESULT
        where
            Identity: IInputPanelWindowHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInputPanelWindowHandle_Impl::AttachedEditWindow64(this) {
                Ok(ok__) => {
                    attachededitwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: i64) -> windows_core::HRESULT
        where
            Identity: IInputPanelWindowHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInputPanelWindowHandle_Impl::SetAttachedEditWindow64(this, core::mem::transmute_copy(&attachededitwindow)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachedEditWindow32: AttachedEditWindow32::<Identity, OFFSET>,
            SetAttachedEditWindow32: SetAttachedEditWindow32::<Identity, OFFSET>,
            AttachedEditWindow64: AttachedEditWindow64::<Identity, OFFSET>,
            SetAttachedEditWindow64: SetAttachedEditWindow64::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPanelWindowHandle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMathInputControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Show(&self) -> windows_core::Result<()>;
    fn Hide(&self) -> windows_core::Result<()>;
    fn IsVisible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPosition(&self, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> windows_core::Result<()>;
    fn SetPosition(&self, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn SetCustomPaint(&self, element: i32, paint: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetCaptionText(&self, captiontext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadInk(&self, ink: Option<&IInkDisp>) -> windows_core::Result<()>;
    fn SetOwnerWindow(&self, ownerwindow: isize) -> windows_core::Result<()>;
    fn EnableExtendedButtons(&self, extended: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetPreviewHeight(&self) -> windows_core::Result<i32>;
    fn SetPreviewHeight(&self, height: i32) -> windows_core::Result<()>;
    fn EnableAutoGrow(&self, autogrow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AddFunctionName(&self, functionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveFunctionName(&self, functionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetHoverIcon(&self) -> windows_core::Result<super::super::System::Ole::IPictureDisp>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IMathInputControl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMathInputControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMathInputControl_Vtbl
    where
        Identity: IMathInputControl_Impl,
    {
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::Show(this).into()
        }
        unsafe extern "system" fn Hide<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::Hide(this).into()
        }
        unsafe extern "system" fn IsVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbshown: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMathInputControl_Impl::IsVisible(this) {
                Ok(ok__) => {
                    pvbshown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::GetPosition(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&right), core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::SetPosition(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&right), core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::Clear(this).into()
        }
        unsafe extern "system" fn SetCustomPaint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: i32, paint: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::SetCustomPaint(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&paint)).into()
        }
        unsafe extern "system" fn SetCaptionText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, captiontext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::SetCaptionText(this, core::mem::transmute(&captiontext)).into()
        }
        unsafe extern "system" fn LoadInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::LoadInk(this, windows_core::from_raw_borrowed(&ink)).into()
        }
        unsafe extern "system" fn SetOwnerWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ownerwindow: isize) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::SetOwnerWindow(this, core::mem::transmute_copy(&ownerwindow)).into()
        }
        unsafe extern "system" fn EnableExtendedButtons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extended: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::EnableExtendedButtons(this, core::mem::transmute_copy(&extended)).into()
        }
        unsafe extern "system" fn GetPreviewHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMathInputControl_Impl::GetPreviewHeight(this) {
                Ok(ok__) => {
                    height.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreviewHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: i32) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::SetPreviewHeight(this, core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn EnableAutoGrow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autogrow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::EnableAutoGrow(this, core::mem::transmute_copy(&autogrow)).into()
        }
        unsafe extern "system" fn AddFunctionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::AddFunctionName(this, core::mem::transmute(&functionname)).into()
        }
        unsafe extern "system" fn RemoveFunctionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMathInputControl_Impl::RemoveFunctionName(this, core::mem::transmute(&functionname)).into()
        }
        unsafe extern "system" fn GetHoverIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hoverimage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMathInputControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMathInputControl_Impl::GetHoverIcon(this) {
                Ok(ok__) => {
                    hoverimage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
            Hide: Hide::<Identity, OFFSET>,
            IsVisible: IsVisible::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetCustomPaint: SetCustomPaint::<Identity, OFFSET>,
            SetCaptionText: SetCaptionText::<Identity, OFFSET>,
            LoadInk: LoadInk::<Identity, OFFSET>,
            SetOwnerWindow: SetOwnerWindow::<Identity, OFFSET>,
            EnableExtendedButtons: EnableExtendedButtons::<Identity, OFFSET>,
            GetPreviewHeight: GetPreviewHeight::<Identity, OFFSET>,
            SetPreviewHeight: SetPreviewHeight::<Identity, OFFSET>,
            EnableAutoGrow: EnableAutoGrow::<Identity, OFFSET>,
            AddFunctionName: AddFunctionName::<Identity, OFFSET>,
            RemoveFunctionName: RemoveFunctionName::<Identity, OFFSET>,
            GetHoverIcon: GetHoverIcon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMathInputControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPenInputPanel_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Busy(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Factoid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFactoid(&self, factoid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AttachedEditWindow(&self) -> windows_core::Result<i32>;
    fn SetAttachedEditWindow(&self, attachededitwindow: i32) -> windows_core::Result<()>;
    fn CurrentPanel(&self) -> windows_core::Result<PanelType>;
    fn SetCurrentPanel(&self, currentpanel: PanelType) -> windows_core::Result<()>;
    fn DefaultPanel(&self) -> windows_core::Result<PanelType>;
    fn SetDefaultPanel(&self, defaultpanel: PanelType) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVisible(&self, visible: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Top(&self) -> windows_core::Result<i32>;
    fn Left(&self) -> windows_core::Result<i32>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn VerticalOffset(&self) -> windows_core::Result<i32>;
    fn SetVerticalOffset(&self, verticaloffset: i32) -> windows_core::Result<()>;
    fn HorizontalOffset(&self) -> windows_core::Result<i32>;
    fn SetHorizontalOffset(&self, horizontaloffset: i32) -> windows_core::Result<()>;
    fn AutoShow(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoShow(&self, autoshow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MoveTo(&self, left: i32, top: i32) -> windows_core::Result<()>;
    fn CommitPendingInput(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn EnableTsf(&self, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPenInputPanel {}
#[cfg(feature = "Win32_System_Com")]
impl IPenInputPanel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPenInputPanel_Vtbl
    where
        Identity: IPenInputPanel_Impl,
    {
        unsafe extern "system" fn Busy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, busy: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Busy(this) {
                Ok(ok__) => {
                    busy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Factoid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Factoid(this) {
                Ok(ok__) => {
                    factoid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetFactoid(this, core::mem::transmute(&factoid)).into()
        }
        unsafe extern "system" fn AttachedEditWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::AttachedEditWindow(this) {
                Ok(ok__) => {
                    attachededitwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetAttachedEditWindow(this, core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn CurrentPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpanel: *mut PanelType) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::CurrentPanel(this) {
                Ok(ok__) => {
                    currentpanel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpanel: PanelType) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetCurrentPanel(this, core::mem::transmute_copy(&currentpanel)).into()
        }
        unsafe extern "system" fn DefaultPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaultpanel: *mut PanelType) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::DefaultPanel(this) {
                Ok(ok__) => {
                    pdefaultpanel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPanel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultpanel: PanelType) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetDefaultPanel(this, core::mem::transmute_copy(&defaultpanel)).into()
        }
        unsafe extern "system" fn Visible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Visible(this) {
                Ok(ok__) => {
                    visible.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetVisible(this, core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn Top<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Top(this) {
                Ok(ok__) => {
                    top.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Left<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Left(this) {
                Ok(ok__) => {
                    left.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Width(this) {
                Ok(ok__) => {
                    width.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::Height(this) {
                Ok(ok__) => {
                    height.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, verticaloffset: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::VerticalOffset(this) {
                Ok(ok__) => {
                    verticaloffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, verticaloffset: i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetVerticalOffset(this, core::mem::transmute_copy(&verticaloffset)).into()
        }
        unsafe extern "system" fn HorizontalOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontaloffset: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::HorizontalOffset(this) {
                Ok(ok__) => {
                    horizontaloffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontaloffset: i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetHorizontalOffset(this, core::mem::transmute_copy(&horizontaloffset)).into()
        }
        unsafe extern "system" fn AutoShow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pautoshow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPenInputPanel_Impl::AutoShow(this) {
                Ok(ok__) => {
                    pautoshow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoShow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::SetAutoShow(this, core::mem::transmute_copy(&autoshow)).into()
        }
        unsafe extern "system" fn MoveTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::MoveTo(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn CommitPendingInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::CommitPendingInput(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn EnableTsf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IPenInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPenInputPanel_Impl::EnableTsf(this, core::mem::transmute_copy(&enable)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Busy: Busy::<Identity, OFFSET>,
            Factoid: Factoid::<Identity, OFFSET>,
            SetFactoid: SetFactoid::<Identity, OFFSET>,
            AttachedEditWindow: AttachedEditWindow::<Identity, OFFSET>,
            SetAttachedEditWindow: SetAttachedEditWindow::<Identity, OFFSET>,
            CurrentPanel: CurrentPanel::<Identity, OFFSET>,
            SetCurrentPanel: SetCurrentPanel::<Identity, OFFSET>,
            DefaultPanel: DefaultPanel::<Identity, OFFSET>,
            SetDefaultPanel: SetDefaultPanel::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Top: Top::<Identity, OFFSET>,
            Left: Left::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            VerticalOffset: VerticalOffset::<Identity, OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Identity, OFFSET>,
            HorizontalOffset: HorizontalOffset::<Identity, OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Identity, OFFSET>,
            AutoShow: AutoShow::<Identity, OFFSET>,
            SetAutoShow: SetAutoShow::<Identity, OFFSET>,
            MoveTo: MoveTo::<Identity, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            EnableTsf: EnableTsf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPenInputPanel as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRealTimeStylus_Impl: Sized {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HWND(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR>;
    fn SetHWND(&self, hwnd: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn WindowInputRectangle(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn SetWindowInputRectangle(&self, prcwndinputrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn AddStylusSyncPlugin(&self, iindex: u32, piplugin: Option<&IStylusSyncPlugin>) -> windows_core::Result<()>;
    fn RemoveStylusSyncPlugin(&self, iindex: u32, ppiplugin: *mut Option<IStylusSyncPlugin>) -> windows_core::Result<()>;
    fn RemoveAllStylusSyncPlugins(&self) -> windows_core::Result<()>;
    fn GetStylusSyncPlugin(&self, iindex: u32) -> windows_core::Result<IStylusSyncPlugin>;
    fn GetStylusSyncPluginCount(&self) -> windows_core::Result<u32>;
    fn AddStylusAsyncPlugin(&self, iindex: u32, piplugin: Option<&IStylusAsyncPlugin>) -> windows_core::Result<()>;
    fn RemoveStylusAsyncPlugin(&self, iindex: u32, ppiplugin: *mut Option<IStylusAsyncPlugin>) -> windows_core::Result<()>;
    fn RemoveAllStylusAsyncPlugins(&self) -> windows_core::Result<()>;
    fn GetStylusAsyncPlugin(&self, iindex: u32) -> windows_core::Result<IStylusAsyncPlugin>;
    fn GetStylusAsyncPluginCount(&self) -> windows_core::Result<u32>;
    fn ChildRealTimeStylusPlugin(&self) -> windows_core::Result<IRealTimeStylus>;
    fn putref_ChildRealTimeStylusPlugin(&self, pirts: Option<&IRealTimeStylus>) -> windows_core::Result<()>;
    fn AddCustomStylusDataToQueue(&self, sq: StylusQueue, pguidid: *const windows_core::GUID, cbdata: u32, pbdata: *const u8) -> windows_core::Result<()>;
    fn ClearStylusQueues(&self) -> windows_core::Result<()>;
    fn SetAllTabletsMode(&self, fusemouseforinput: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetSingleTabletMode(&self, pitablet: Option<&IInkTablet>) -> windows_core::Result<()>;
    fn GetTablet(&self) -> windows_core::Result<IInkTablet>;
    fn GetTabletContextIdFromTablet(&self, pitablet: Option<&IInkTablet>) -> windows_core::Result<u32>;
    fn GetTabletFromTabletContextId(&self, tcid: u32) -> windows_core::Result<IInkTablet>;
    fn GetAllTabletContextIds(&self, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> windows_core::Result<()>;
    fn GetStyluses(&self) -> windows_core::Result<IInkCursors>;
    fn GetStylusForId(&self, sid: u32) -> windows_core::Result<IInkCursor>;
    fn SetDesiredPacketDescription(&self, cproperties: u32, ppropertyguids: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetDesiredPacketDescription(&self, pcproperties: *mut u32, pppropertyguids: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPacketDescriptionData(&self, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRealTimeStylus {}
#[cfg(feature = "Win32_System_Com")]
impl IRealTimeStylus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRealTimeStylus_Vtbl
    where
        Identity: IRealTimeStylus_Impl,
    {
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::Enabled(this) {
                Ok(ok__) => {
                    pfenable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::SetEnabled(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn HWND<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::HWND(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHWND<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::SetHWND(this, core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn WindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcwndinputrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::WindowInputRectangle(this) {
                Ok(ok__) => {
                    prcwndinputrect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcwndinputrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::SetWindowInputRectangle(this, core::mem::transmute_copy(&prcwndinputrect)).into()
        }
        unsafe extern "system" fn AddStylusSyncPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, piplugin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::AddStylusSyncPlugin(this, core::mem::transmute_copy(&iindex), windows_core::from_raw_borrowed(&piplugin)).into()
        }
        unsafe extern "system" fn RemoveStylusSyncPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::RemoveStylusSyncPlugin(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&ppiplugin)).into()
        }
        unsafe extern "system" fn RemoveAllStylusSyncPlugins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::RemoveAllStylusSyncPlugins(this).into()
        }
        unsafe extern "system" fn GetStylusSyncPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetStylusSyncPlugin(this, core::mem::transmute_copy(&iindex)) {
                Ok(ok__) => {
                    ppiplugin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusSyncPluginCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcplugins: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetStylusSyncPluginCount(this) {
                Ok(ok__) => {
                    pcplugins.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStylusAsyncPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, piplugin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::AddStylusAsyncPlugin(this, core::mem::transmute_copy(&iindex), windows_core::from_raw_borrowed(&piplugin)).into()
        }
        unsafe extern "system" fn RemoveStylusAsyncPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::RemoveStylusAsyncPlugin(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&ppiplugin)).into()
        }
        unsafe extern "system" fn RemoveAllStylusAsyncPlugins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::RemoveAllStylusAsyncPlugins(this).into()
        }
        unsafe extern "system" fn GetStylusAsyncPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppiplugin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetStylusAsyncPlugin(this, core::mem::transmute_copy(&iindex)) {
                Ok(ok__) => {
                    ppiplugin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusAsyncPluginCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcplugins: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetStylusAsyncPluginCount(this) {
                Ok(ok__) => {
                    pcplugins.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildRealTimeStylusPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppirts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::ChildRealTimeStylusPlugin(this) {
                Ok(ok__) => {
                    ppirts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ChildRealTimeStylusPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirts: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::putref_ChildRealTimeStylusPlugin(this, windows_core::from_raw_borrowed(&pirts)).into()
        }
        unsafe extern "system" fn AddCustomStylusDataToQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sq: StylusQueue, pguidid: *const windows_core::GUID, cbdata: u32, pbdata: *const u8) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::AddCustomStylusDataToQueue(this, core::mem::transmute_copy(&sq), core::mem::transmute_copy(&pguidid), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn ClearStylusQueues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::ClearStylusQueues(this).into()
        }
        unsafe extern "system" fn SetAllTabletsMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusemouseforinput: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::SetAllTabletsMode(this, core::mem::transmute_copy(&fusemouseforinput)).into()
        }
        unsafe extern "system" fn SetSingleTabletMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitablet: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::SetSingleTabletMode(this, windows_core::from_raw_borrowed(&pitablet)).into()
        }
        unsafe extern "system" fn GetTablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppisingletablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetTablet(this) {
                Ok(ok__) => {
                    ppisingletablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletContextIdFromTablet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitablet: *mut core::ffi::c_void, ptcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetTabletContextIdFromTablet(this, windows_core::from_raw_borrowed(&pitablet)) {
                Ok(ok__) => {
                    ptcid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabletFromTabletContextId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tcid: u32, ppitablet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetTabletFromTabletContextId(this, core::mem::transmute_copy(&tcid)) {
                Ok(ok__) => {
                    ppitablet.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllTabletContextIds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::GetAllTabletContextIds(this, core::mem::transmute_copy(&pctcidcount), core::mem::transmute_copy(&pptcids)).into()
        }
        unsafe extern "system" fn GetStyluses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiinkcursors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetStyluses(this) {
                Ok(ok__) => {
                    ppiinkcursors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStylusForId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sid: u32, ppiinkcursor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus_Impl::GetStylusForId(this, core::mem::transmute_copy(&sid)) {
                Ok(ok__) => {
                    ppiinkcursor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppropertyguids: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::SetDesiredPacketDescription(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppropertyguids)).into()
        }
        unsafe extern "system" fn GetDesiredPacketDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcproperties: *mut u32, pppropertyguids: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::GetDesiredPacketDescription(this, core::mem::transmute_copy(&pcproperties), core::mem::transmute_copy(&pppropertyguids)).into()
        }
        unsafe extern "system" fn GetPacketDescriptionData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus_Impl::GetPacketDescriptionData(this, core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&pfinktodevicescalex), core::mem::transmute_copy(&pfinktodevicescaley), core::mem::transmute_copy(&pcpacketproperties), core::mem::transmute_copy(&pppacketproperties)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            HWND: HWND::<Identity, OFFSET>,
            SetHWND: SetHWND::<Identity, OFFSET>,
            WindowInputRectangle: WindowInputRectangle::<Identity, OFFSET>,
            SetWindowInputRectangle: SetWindowInputRectangle::<Identity, OFFSET>,
            AddStylusSyncPlugin: AddStylusSyncPlugin::<Identity, OFFSET>,
            RemoveStylusSyncPlugin: RemoveStylusSyncPlugin::<Identity, OFFSET>,
            RemoveAllStylusSyncPlugins: RemoveAllStylusSyncPlugins::<Identity, OFFSET>,
            GetStylusSyncPlugin: GetStylusSyncPlugin::<Identity, OFFSET>,
            GetStylusSyncPluginCount: GetStylusSyncPluginCount::<Identity, OFFSET>,
            AddStylusAsyncPlugin: AddStylusAsyncPlugin::<Identity, OFFSET>,
            RemoveStylusAsyncPlugin: RemoveStylusAsyncPlugin::<Identity, OFFSET>,
            RemoveAllStylusAsyncPlugins: RemoveAllStylusAsyncPlugins::<Identity, OFFSET>,
            GetStylusAsyncPlugin: GetStylusAsyncPlugin::<Identity, OFFSET>,
            GetStylusAsyncPluginCount: GetStylusAsyncPluginCount::<Identity, OFFSET>,
            ChildRealTimeStylusPlugin: ChildRealTimeStylusPlugin::<Identity, OFFSET>,
            putref_ChildRealTimeStylusPlugin: putref_ChildRealTimeStylusPlugin::<Identity, OFFSET>,
            AddCustomStylusDataToQueue: AddCustomStylusDataToQueue::<Identity, OFFSET>,
            ClearStylusQueues: ClearStylusQueues::<Identity, OFFSET>,
            SetAllTabletsMode: SetAllTabletsMode::<Identity, OFFSET>,
            SetSingleTabletMode: SetSingleTabletMode::<Identity, OFFSET>,
            GetTablet: GetTablet::<Identity, OFFSET>,
            GetTabletContextIdFromTablet: GetTabletContextIdFromTablet::<Identity, OFFSET>,
            GetTabletFromTabletContextId: GetTabletFromTabletContextId::<Identity, OFFSET>,
            GetAllTabletContextIds: GetAllTabletContextIds::<Identity, OFFSET>,
            GetStyluses: GetStyluses::<Identity, OFFSET>,
            GetStylusForId: GetStylusForId::<Identity, OFFSET>,
            SetDesiredPacketDescription: SetDesiredPacketDescription::<Identity, OFFSET>,
            GetDesiredPacketDescription: GetDesiredPacketDescription::<Identity, OFFSET>,
            GetPacketDescriptionData: GetPacketDescriptionData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRealTimeStylus as windows_core::Interface>::IID
    }
}
pub trait IRealTimeStylus2_Impl: Sized {
    fn FlicksEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetFlicksEnabled(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRealTimeStylus2 {}
impl IRealTimeStylus2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRealTimeStylus2_Vtbl
    where
        Identity: IRealTimeStylus2_Impl,
    {
        unsafe extern "system" fn FlicksEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus2_Impl::FlicksEnabled(this) {
                Ok(ok__) => {
                    pfenable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlicksEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus2_Impl::SetFlicksEnabled(this, core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FlicksEnabled: FlicksEnabled::<Identity, OFFSET>,
            SetFlicksEnabled: SetFlicksEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRealTimeStylus2 as windows_core::Interface>::IID
    }
}
pub trait IRealTimeStylus3_Impl: Sized {
    fn MultiTouchEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetMultiTouchEnabled(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRealTimeStylus3 {}
impl IRealTimeStylus3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRealTimeStylus3_Vtbl
    where
        Identity: IRealTimeStylus3_Impl,
    {
        unsafe extern "system" fn MultiTouchEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRealTimeStylus3_Impl::MultiTouchEnabled(this) {
                Ok(ok__) => {
                    pfenable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiTouchEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylus3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylus3_Impl::SetMultiTouchEnabled(this, core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MultiTouchEnabled: MultiTouchEnabled::<Identity, OFFSET>,
            SetMultiTouchEnabled: SetMultiTouchEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRealTimeStylus3 as windows_core::Interface>::IID
    }
}
pub trait IRealTimeStylusSynchronization_Impl: Sized {
    fn AcquireLock(&self, lock: RealTimeStylusLockType) -> windows_core::Result<()>;
    fn ReleaseLock(&self, lock: RealTimeStylusLockType) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRealTimeStylusSynchronization {}
impl IRealTimeStylusSynchronization_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRealTimeStylusSynchronization_Vtbl
    where
        Identity: IRealTimeStylusSynchronization_Impl,
    {
        unsafe extern "system" fn AcquireLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lock: RealTimeStylusLockType) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylusSynchronization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylusSynchronization_Impl::AcquireLock(this, core::mem::transmute_copy(&lock)).into()
        }
        unsafe extern "system" fn ReleaseLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lock: RealTimeStylusLockType) -> windows_core::HRESULT
        where
            Identity: IRealTimeStylusSynchronization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRealTimeStylusSynchronization_Impl::ReleaseLock(this, core::mem::transmute_copy(&lock)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireLock: AcquireLock::<Identity, OFFSET>,
            ReleaseLock: ReleaseLock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRealTimeStylusSynchronization as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISketchInk_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISketchInk {}
#[cfg(feature = "Win32_System_Com")]
impl ISketchInk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISketchInk_Vtbl
    where
        Identity: ISketchInk_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISketchInk as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStrokeBuilder_Impl: Sized {
    fn CreateStroke(&self, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut Option<IInkStrokeDisp>) -> windows_core::Result<()>;
    fn BeginStroke(&self, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut Option<IInkStrokeDisp>) -> windows_core::Result<()>;
    fn AppendPackets(&self, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> windows_core::Result<()>;
    fn EndStroke(&self, tcid: u32, sid: u32, ppiinkstroke: *mut Option<IInkStrokeDisp>, pdirtyrect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Ink(&self) -> windows_core::Result<IInkDisp>;
    fn putref_Ink(&self, piinkobj: Option<&IInkDisp>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStrokeBuilder {}
#[cfg(feature = "Win32_System_Com")]
impl IStrokeBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStrokeBuilder_Vtbl
    where
        Identity: IStrokeBuilder_Impl,
    {
        unsafe extern "system" fn CreateStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStrokeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStrokeBuilder_Impl::CreateStroke(this, core::mem::transmute_copy(&cpktbufflength), core::mem::transmute_copy(&ppackets), core::mem::transmute_copy(&cpacketproperties), core::mem::transmute_copy(&ppacketproperties), core::mem::transmute_copy(&finktodevicescalex), core::mem::transmute_copy(&finktodevicescaley), core::mem::transmute_copy(&ppiinkstroke)).into()
        }
        unsafe extern "system" fn BeginStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStrokeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStrokeBuilder_Impl::BeginStroke(this, core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&sid), core::mem::transmute_copy(&ppacket), core::mem::transmute_copy(&cpacketproperties), core::mem::transmute_copy(&ppacketproperties), core::mem::transmute_copy(&finktodevicescalex), core::mem::transmute_copy(&finktodevicescaley), core::mem::transmute_copy(&ppiinkstroke)).into()
        }
        unsafe extern "system" fn AppendPackets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> windows_core::HRESULT
        where
            Identity: IStrokeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStrokeBuilder_Impl::AppendPackets(this, core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&sid), core::mem::transmute_copy(&cpktbufflength), core::mem::transmute_copy(&ppackets)).into()
        }
        unsafe extern "system" fn EndStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tcid: u32, sid: u32, ppiinkstroke: *mut *mut core::ffi::c_void, pdirtyrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IStrokeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStrokeBuilder_Impl::EndStroke(this, core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&sid), core::mem::transmute_copy(&ppiinkstroke), core::mem::transmute_copy(&pdirtyrect)).into()
        }
        unsafe extern "system" fn Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiinkobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStrokeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStrokeBuilder_Impl::Ink(this) {
                Ok(ok__) => {
                    ppiinkobj.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Ink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piinkobj: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStrokeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStrokeBuilder_Impl::putref_Ink(this, windows_core::from_raw_borrowed(&piinkobj)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStroke: CreateStroke::<Identity, OFFSET>,
            BeginStroke: BeginStroke::<Identity, OFFSET>,
            AppendPackets: AppendPackets::<Identity, OFFSET>,
            EndStroke: EndStroke::<Identity, OFFSET>,
            Ink: Ink::<Identity, OFFSET>,
            putref_Ink: putref_Ink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStrokeBuilder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStylusAsyncPlugin_Impl: Sized + IStylusPlugin_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStylusAsyncPlugin {}
#[cfg(feature = "Win32_System_Com")]
impl IStylusAsyncPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStylusAsyncPlugin_Vtbl
    where
        Identity: IStylusAsyncPlugin_Impl,
    {
        Self { base__: IStylusPlugin_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStylusAsyncPlugin as windows_core::Interface>::IID || iid == &<IStylusPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStylusPlugin_Impl: Sized {
    fn RealTimeStylusEnabled(&self, pirtssrc: Option<&IRealTimeStylus>, ctcidcount: u32, ptcids: *const u32) -> windows_core::Result<()>;
    fn RealTimeStylusDisabled(&self, pirtssrc: Option<&IRealTimeStylus>, ctcidcount: u32, ptcids: *const u32) -> windows_core::Result<()>;
    fn StylusInRange(&self, pirtssrc: Option<&IRealTimeStylus>, tcid: u32, sid: u32) -> windows_core::Result<()>;
    fn StylusOutOfRange(&self, pirtssrc: Option<&IRealTimeStylus>, tcid: u32, sid: u32) -> windows_core::Result<()>;
    fn StylusDown(&self, pirtssrc: Option<&IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> windows_core::Result<()>;
    fn StylusUp(&self, pirtssrc: Option<&IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> windows_core::Result<()>;
    fn StylusButtonDown(&self, pirtssrc: Option<&IRealTimeStylus>, sid: u32, pguidstylusbutton: *const windows_core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn StylusButtonUp(&self, pirtssrc: Option<&IRealTimeStylus>, sid: u32, pguidstylusbutton: *const windows_core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn InAirPackets(&self, pirtssrc: Option<&IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> windows_core::Result<()>;
    fn Packets(&self, pirtssrc: Option<&IRealTimeStylus>, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> windows_core::Result<()>;
    fn CustomStylusDataAdded(&self, pirtssrc: Option<&IRealTimeStylus>, pguidid: *const windows_core::GUID, cbdata: u32, pbdata: *const u8) -> windows_core::Result<()>;
    fn SystemEvent(&self, pirtssrc: Option<&IRealTimeStylus>, tcid: u32, sid: u32, event: u16, eventdata: &SYSTEM_EVENT_DATA) -> windows_core::Result<()>;
    fn TabletAdded(&self, pirtssrc: Option<&IRealTimeStylus>, pitablet: Option<&IInkTablet>) -> windows_core::Result<()>;
    fn TabletRemoved(&self, pirtssrc: Option<&IRealTimeStylus>, itabletindex: i32) -> windows_core::Result<()>;
    fn Error(&self, pirtssrc: Option<&IRealTimeStylus>, piplugin: Option<&IStylusPlugin>, datainterest: RealTimeStylusDataInterest, hrerrorcode: windows_core::HRESULT, lptrkey: *mut isize) -> windows_core::Result<()>;
    fn UpdateMapping(&self, pirtssrc: Option<&IRealTimeStylus>) -> windows_core::Result<()>;
    fn DataInterest(&self) -> windows_core::Result<RealTimeStylusDataInterest>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStylusPlugin {}
#[cfg(feature = "Win32_System_Com")]
impl IStylusPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStylusPlugin_Vtbl
    where
        Identity: IStylusPlugin_Impl,
    {
        unsafe extern "system" fn RealTimeStylusEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, ctcidcount: u32, ptcids: *const u32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::RealTimeStylusEnabled(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&ctcidcount), core::mem::transmute_copy(&ptcids)).into()
        }
        unsafe extern "system" fn RealTimeStylusDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, ctcidcount: u32, ptcids: *const u32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::RealTimeStylusDisabled(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&ctcidcount), core::mem::transmute_copy(&ptcids)).into()
        }
        unsafe extern "system" fn StylusInRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, tcid: u32, sid: u32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::StylusInRange(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&sid)).into()
        }
        unsafe extern "system" fn StylusOutOfRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, tcid: u32, sid: u32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::StylusOutOfRange(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&sid)).into()
        }
        unsafe extern "system" fn StylusDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::StylusDown(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&pstylusinfo), core::mem::transmute_copy(&cpropcountperpkt), core::mem::transmute_copy(&ppacket), core::mem::transmute_copy(&ppinoutpkt)).into()
        }
        unsafe extern "system" fn StylusUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::StylusUp(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&pstylusinfo), core::mem::transmute_copy(&cpropcountperpkt), core::mem::transmute_copy(&ppacket), core::mem::transmute_copy(&ppinoutpkt)).into()
        }
        unsafe extern "system" fn StylusButtonDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, sid: u32, pguidstylusbutton: *const windows_core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::StylusButtonDown(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&sid), core::mem::transmute_copy(&pguidstylusbutton), core::mem::transmute_copy(&pstyluspos)).into()
        }
        unsafe extern "system" fn StylusButtonUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, sid: u32, pguidstylusbutton: *const windows_core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::StylusButtonUp(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&sid), core::mem::transmute_copy(&pguidstylusbutton), core::mem::transmute_copy(&pstyluspos)).into()
        }
        unsafe extern "system" fn InAirPackets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::InAirPackets(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&pstylusinfo), core::mem::transmute_copy(&cpktcount), core::mem::transmute_copy(&cpktbufflength), core::mem::transmute_copy(&ppackets), core::mem::transmute_copy(&pcinoutpkts), core::mem::transmute_copy(&ppinoutpkts)).into()
        }
        unsafe extern "system" fn Packets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::Packets(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&pstylusinfo), core::mem::transmute_copy(&cpktcount), core::mem::transmute_copy(&cpktbufflength), core::mem::transmute_copy(&ppackets), core::mem::transmute_copy(&pcinoutpkts), core::mem::transmute_copy(&ppinoutpkts)).into()
        }
        unsafe extern "system" fn CustomStylusDataAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, pguidid: *const windows_core::GUID, cbdata: u32, pbdata: *const u8) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::CustomStylusDataAdded(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&pguidid), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn SystemEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::SystemEvent(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&tcid), core::mem::transmute_copy(&sid), core::mem::transmute_copy(&event), core::mem::transmute(&eventdata)).into()
        }
        unsafe extern "system" fn TabletAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, pitablet: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::TabletAdded(this, windows_core::from_raw_borrowed(&pirtssrc), windows_core::from_raw_borrowed(&pitablet)).into()
        }
        unsafe extern "system" fn TabletRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, itabletindex: i32) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::TabletRemoved(this, windows_core::from_raw_borrowed(&pirtssrc), core::mem::transmute_copy(&itabletindex)).into()
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void, piplugin: *mut core::ffi::c_void, datainterest: RealTimeStylusDataInterest, hrerrorcode: windows_core::HRESULT, lptrkey: *mut isize) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::Error(this, windows_core::from_raw_borrowed(&pirtssrc), windows_core::from_raw_borrowed(&piplugin), core::mem::transmute_copy(&datainterest), core::mem::transmute_copy(&hrerrorcode), core::mem::transmute_copy(&lptrkey)).into()
        }
        unsafe extern "system" fn UpdateMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirtssrc: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStylusPlugin_Impl::UpdateMapping(this, windows_core::from_raw_borrowed(&pirtssrc)).into()
        }
        unsafe extern "system" fn DataInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatainterest: *mut RealTimeStylusDataInterest) -> windows_core::HRESULT
        where
            Identity: IStylusPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStylusPlugin_Impl::DataInterest(this) {
                Ok(ok__) => {
                    pdatainterest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RealTimeStylusEnabled: RealTimeStylusEnabled::<Identity, OFFSET>,
            RealTimeStylusDisabled: RealTimeStylusDisabled::<Identity, OFFSET>,
            StylusInRange: StylusInRange::<Identity, OFFSET>,
            StylusOutOfRange: StylusOutOfRange::<Identity, OFFSET>,
            StylusDown: StylusDown::<Identity, OFFSET>,
            StylusUp: StylusUp::<Identity, OFFSET>,
            StylusButtonDown: StylusButtonDown::<Identity, OFFSET>,
            StylusButtonUp: StylusButtonUp::<Identity, OFFSET>,
            InAirPackets: InAirPackets::<Identity, OFFSET>,
            Packets: Packets::<Identity, OFFSET>,
            CustomStylusDataAdded: CustomStylusDataAdded::<Identity, OFFSET>,
            SystemEvent: SystemEvent::<Identity, OFFSET>,
            TabletAdded: TabletAdded::<Identity, OFFSET>,
            TabletRemoved: TabletRemoved::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            UpdateMapping: UpdateMapping::<Identity, OFFSET>,
            DataInterest: DataInterest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStylusPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStylusSyncPlugin_Impl: Sized + IStylusPlugin_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStylusSyncPlugin {}
#[cfg(feature = "Win32_System_Com")]
impl IStylusSyncPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStylusSyncPlugin_Vtbl
    where
        Identity: IStylusSyncPlugin_Impl,
    {
        Self { base__: IStylusPlugin_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStylusSyncPlugin as windows_core::Interface>::IID || iid == &<IStylusPlugin as windows_core::Interface>::IID
    }
}
pub trait ITextInputPanel_Impl: Sized {
    fn AttachedEditWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn SetAttachedEditWindow(&self, attachededitwindow: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn CurrentInteractionMode(&self) -> windows_core::Result<InteractionMode>;
    fn DefaultInPlaceState(&self) -> windows_core::Result<InPlaceState>;
    fn SetDefaultInPlaceState(&self, state: InPlaceState) -> windows_core::Result<()>;
    fn CurrentInPlaceState(&self) -> windows_core::Result<InPlaceState>;
    fn DefaultInputArea(&self) -> windows_core::Result<PanelInputArea>;
    fn SetDefaultInputArea(&self, area: PanelInputArea) -> windows_core::Result<()>;
    fn CurrentInputArea(&self) -> windows_core::Result<PanelInputArea>;
    fn CurrentCorrectionMode(&self) -> windows_core::Result<CorrectionMode>;
    fn PreferredInPlaceDirection(&self) -> windows_core::Result<InPlaceDirection>;
    fn SetPreferredInPlaceDirection(&self, direction: InPlaceDirection) -> windows_core::Result<()>;
    fn ExpandPostInsertionCorrection(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetExpandPostInsertionCorrection(&self, expand: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InPlaceVisibleOnFocus(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetInPlaceVisibleOnFocus(&self, visible: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InPlaceBoundingRectangle(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn PopUpCorrectionHeight(&self) -> windows_core::Result<i32>;
    fn PopDownCorrectionHeight(&self) -> windows_core::Result<i32>;
    fn CommitPendingInput(&self) -> windows_core::Result<()>;
    fn SetInPlaceVisibility(&self, visible: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::Result<()>;
    fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> windows_core::Result<()>;
    fn Advise(&self, eventsink: Option<&ITextInputPanelEventSink>, eventmask: u32) -> windows_core::Result<()>;
    fn Unadvise(&self, eventsink: Option<&ITextInputPanelEventSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextInputPanel {}
impl ITextInputPanel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextInputPanel_Vtbl
    where
        Identity: ITextInputPanel_Impl,
    {
        unsafe extern "system" fn AttachedEditWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::AttachedEditWindow(this) {
                Ok(ok__) => {
                    attachededitwindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetAttachedEditWindow(this, core::mem::transmute_copy(&attachededitwindow)).into()
        }
        unsafe extern "system" fn CurrentInteractionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::CurrentInteractionMode(this) {
                Ok(ok__) => {
                    currentinteractionmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInPlaceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut InPlaceState) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::DefaultInPlaceState(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInPlaceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: InPlaceState) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetDefaultInPlaceState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CurrentInPlaceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut InPlaceState) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::CurrentInPlaceState(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInputArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: *mut PanelInputArea) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::DefaultInputArea(this) {
                Ok(ok__) => {
                    area.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInputArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: PanelInputArea) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetDefaultInputArea(this, core::mem::transmute_copy(&area)).into()
        }
        unsafe extern "system" fn CurrentInputArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: *mut PanelInputArea) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::CurrentInputArea(this) {
                Ok(ok__) => {
                    area.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCorrectionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut CorrectionMode) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::CurrentCorrectionMode(this) {
                Ok(ok__) => {
                    mode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredInPlaceDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: *mut InPlaceDirection) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::PreferredInPlaceDirection(this) {
                Ok(ok__) => {
                    direction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredInPlaceDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: InPlaceDirection) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetPreferredInPlaceDirection(this, core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn ExpandPostInsertionCorrection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expand: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::ExpandPostInsertionCorrection(this) {
                Ok(ok__) => {
                    expand.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpandPostInsertionCorrection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expand: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetExpandPostInsertionCorrection(this, core::mem::transmute_copy(&expand)).into()
        }
        unsafe extern "system" fn InPlaceVisibleOnFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::InPlaceVisibleOnFocus(this) {
                Ok(ok__) => {
                    visible.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPlaceVisibleOnFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetInPlaceVisibleOnFocus(this, core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn InPlaceBoundingRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingrectangle: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::InPlaceBoundingRectangle(this) {
                Ok(ok__) => {
                    boundingrectangle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopUpCorrectionHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::PopUpCorrectionHeight(this) {
                Ok(ok__) => {
                    height.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopDownCorrectionHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanel_Impl::PopDownCorrectionHeight(this) {
                Ok(ok__) => {
                    height.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitPendingInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::CommitPendingInput(this).into()
        }
        unsafe extern "system" fn SetInPlaceVisibility<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetInPlaceVisibility(this, core::mem::transmute_copy(&visible)).into()
        }
        unsafe extern "system" fn SetInPlacePosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetInPlacePosition(this, core::mem::transmute_copy(&xposition), core::mem::transmute_copy(&yposition), core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetInPlaceHoverTargetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xposition: i32, yposition: i32) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::SetInPlaceHoverTargetPosition(this, core::mem::transmute_copy(&xposition), core::mem::transmute_copy(&yposition)).into()
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsink: *mut core::ffi::c_void, eventmask: u32) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::Advise(this, windows_core::from_raw_borrowed(&eventsink), core::mem::transmute_copy(&eventmask)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITextInputPanel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanel_Impl::Unadvise(this, windows_core::from_raw_borrowed(&eventsink)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachedEditWindow: AttachedEditWindow::<Identity, OFFSET>,
            SetAttachedEditWindow: SetAttachedEditWindow::<Identity, OFFSET>,
            CurrentInteractionMode: CurrentInteractionMode::<Identity, OFFSET>,
            DefaultInPlaceState: DefaultInPlaceState::<Identity, OFFSET>,
            SetDefaultInPlaceState: SetDefaultInPlaceState::<Identity, OFFSET>,
            CurrentInPlaceState: CurrentInPlaceState::<Identity, OFFSET>,
            DefaultInputArea: DefaultInputArea::<Identity, OFFSET>,
            SetDefaultInputArea: SetDefaultInputArea::<Identity, OFFSET>,
            CurrentInputArea: CurrentInputArea::<Identity, OFFSET>,
            CurrentCorrectionMode: CurrentCorrectionMode::<Identity, OFFSET>,
            PreferredInPlaceDirection: PreferredInPlaceDirection::<Identity, OFFSET>,
            SetPreferredInPlaceDirection: SetPreferredInPlaceDirection::<Identity, OFFSET>,
            ExpandPostInsertionCorrection: ExpandPostInsertionCorrection::<Identity, OFFSET>,
            SetExpandPostInsertionCorrection: SetExpandPostInsertionCorrection::<Identity, OFFSET>,
            InPlaceVisibleOnFocus: InPlaceVisibleOnFocus::<Identity, OFFSET>,
            SetInPlaceVisibleOnFocus: SetInPlaceVisibleOnFocus::<Identity, OFFSET>,
            InPlaceBoundingRectangle: InPlaceBoundingRectangle::<Identity, OFFSET>,
            PopUpCorrectionHeight: PopUpCorrectionHeight::<Identity, OFFSET>,
            PopDownCorrectionHeight: PopDownCorrectionHeight::<Identity, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, OFFSET>,
            SetInPlaceVisibility: SetInPlaceVisibility::<Identity, OFFSET>,
            SetInPlacePosition: SetInPlacePosition::<Identity, OFFSET>,
            SetInPlaceHoverTargetPosition: SetInPlaceHoverTargetPosition::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextInputPanel as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextInputPanelEventSink_Impl: Sized {
    fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::Result<()>;
    fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::Result<()>;
    fn InPlaceSizeChanging(&self, oldboundingrectangle: &super::super::Foundation::RECT, newboundingrectangle: &super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn InPlaceSizeChanged(&self, oldboundingrectangle: &super::super::Foundation::RECT, newboundingrectangle: &super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::Result<()>;
    fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::Result<()>;
    fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::Result<()>;
    fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::Result<()>;
    fn InPlaceVisibilityChanging(&self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InPlaceVisibilityChanged(&self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn TextInserting(&self, ink: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn TextInserted(&self, ink: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITextInputPanelEventSink {}
#[cfg(feature = "Win32_System_Com")]
impl ITextInputPanelEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextInputPanelEventSink_Vtbl
    where
        Identity: ITextInputPanelEventSink_Impl,
    {
        unsafe extern "system" fn InPlaceStateChanging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InPlaceStateChanging(this, core::mem::transmute_copy(&oldinplacestate), core::mem::transmute_copy(&newinplacestate)).into()
        }
        unsafe extern "system" fn InPlaceStateChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InPlaceStateChanged(this, core::mem::transmute_copy(&oldinplacestate), core::mem::transmute_copy(&newinplacestate)).into()
        }
        unsafe extern "system" fn InPlaceSizeChanging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InPlaceSizeChanging(this, core::mem::transmute(&oldboundingrectangle), core::mem::transmute(&newboundingrectangle)).into()
        }
        unsafe extern "system" fn InPlaceSizeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InPlaceSizeChanged(this, core::mem::transmute(&oldboundingrectangle), core::mem::transmute(&newboundingrectangle)).into()
        }
        unsafe extern "system" fn InputAreaChanging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InputAreaChanging(this, core::mem::transmute_copy(&oldinputarea), core::mem::transmute_copy(&newinputarea)).into()
        }
        unsafe extern "system" fn InputAreaChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InputAreaChanged(this, core::mem::transmute_copy(&oldinputarea), core::mem::transmute_copy(&newinputarea)).into()
        }
        unsafe extern "system" fn CorrectionModeChanging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::CorrectionModeChanging(this, core::mem::transmute_copy(&oldcorrectionmode), core::mem::transmute_copy(&newcorrectionmode)).into()
        }
        unsafe extern "system" fn CorrectionModeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::CorrectionModeChanged(this, core::mem::transmute_copy(&oldcorrectionmode), core::mem::transmute_copy(&newcorrectionmode)).into()
        }
        unsafe extern "system" fn InPlaceVisibilityChanging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InPlaceVisibilityChanging(this, core::mem::transmute_copy(&oldvisible), core::mem::transmute_copy(&newvisible)).into()
        }
        unsafe extern "system" fn InPlaceVisibilityChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::InPlaceVisibilityChanged(this, core::mem::transmute_copy(&oldvisible), core::mem::transmute_copy(&newvisible)).into()
        }
        unsafe extern "system" fn TextInserting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::TextInserting(this, core::mem::transmute_copy(&ink)).into()
        }
        unsafe extern "system" fn TextInserted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextInputPanelEventSink_Impl::TextInserted(this, core::mem::transmute_copy(&ink)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InPlaceStateChanging: InPlaceStateChanging::<Identity, OFFSET>,
            InPlaceStateChanged: InPlaceStateChanged::<Identity, OFFSET>,
            InPlaceSizeChanging: InPlaceSizeChanging::<Identity, OFFSET>,
            InPlaceSizeChanged: InPlaceSizeChanged::<Identity, OFFSET>,
            InputAreaChanging: InputAreaChanging::<Identity, OFFSET>,
            InputAreaChanged: InputAreaChanged::<Identity, OFFSET>,
            CorrectionModeChanging: CorrectionModeChanging::<Identity, OFFSET>,
            CorrectionModeChanged: CorrectionModeChanged::<Identity, OFFSET>,
            InPlaceVisibilityChanging: InPlaceVisibilityChanging::<Identity, OFFSET>,
            InPlaceVisibilityChanged: InPlaceVisibilityChanged::<Identity, OFFSET>,
            TextInserting: TextInserting::<Identity, OFFSET>,
            TextInserted: TextInserted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextInputPanelEventSink as windows_core::Interface>::IID
    }
}
pub trait ITextInputPanelRunInfo_Impl: Sized {
    fn IsTipRunning(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITextInputPanelRunInfo {}
impl ITextInputPanelRunInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITextInputPanelRunInfo_Vtbl
    where
        Identity: ITextInputPanelRunInfo_Impl,
    {
        unsafe extern "system" fn IsTipRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrunning: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITextInputPanelRunInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextInputPanelRunInfo_Impl::IsTipRunning(this) {
                Ok(ok__) => {
                    pfrunning.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsTipRunning: IsTipRunning::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextInputPanelRunInfo as windows_core::Interface>::IID
    }
}
pub trait ITipAutoCompleteClient_Impl: Sized {
    fn AdviseProvider(&self, hwndfield: super::super::Foundation::HWND, piprovider: Option<&ITipAutoCompleteProvider>) -> windows_core::Result<()>;
    fn UnadviseProvider(&self, hwndfield: super::super::Foundation::HWND, piprovider: Option<&ITipAutoCompleteProvider>) -> windows_core::Result<()>;
    fn UserSelection(&self) -> windows_core::Result<()>;
    fn PreferredRects(&self, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RequestShowUI(&self, hwndlist: super::super::Foundation::HWND) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ITipAutoCompleteClient {}
impl ITipAutoCompleteClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITipAutoCompleteClient_Vtbl
    where
        Identity: ITipAutoCompleteClient_Impl,
    {
        unsafe extern "system" fn AdviseProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipAutoCompleteClient_Impl::AdviseProvider(this, core::mem::transmute_copy(&hwndfield), windows_core::from_raw_borrowed(&piprovider)).into()
        }
        unsafe extern "system" fn UnadviseProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndfield: super::super::Foundation::HWND, piprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipAutoCompleteClient_Impl::UnadviseProvider(this, core::mem::transmute_copy(&hwndfield), windows_core::from_raw_borrowed(&piprovider)).into()
        }
        unsafe extern "system" fn UserSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipAutoCompleteClient_Impl::UserSelection(this).into()
        }
        unsafe extern "system" fn PreferredRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipAutoCompleteClient_Impl::PreferredRects(this, core::mem::transmute_copy(&prcaclist), core::mem::transmute_copy(&prcfield), core::mem::transmute_copy(&prcmodifiedaclist), core::mem::transmute_copy(&pfshownabovetip)).into()
        }
        unsafe extern "system" fn RequestShowUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITipAutoCompleteClient_Impl::RequestShowUI(this, core::mem::transmute_copy(&hwndlist)) {
                Ok(ok__) => {
                    pfallowshowing.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseProvider: AdviseProvider::<Identity, OFFSET>,
            UnadviseProvider: UnadviseProvider::<Identity, OFFSET>,
            UserSelection: UserSelection::<Identity, OFFSET>,
            PreferredRects: PreferredRects::<Identity, OFFSET>,
            RequestShowUI: RequestShowUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipAutoCompleteClient as windows_core::Interface>::IID
    }
}
pub trait ITipAutoCompleteProvider_Impl: Sized {
    fn UpdatePendingText(&self, bstrpendingtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITipAutoCompleteProvider {}
impl ITipAutoCompleteProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITipAutoCompleteProvider_Vtbl
    where
        Identity: ITipAutoCompleteProvider_Impl,
    {
        unsafe extern "system" fn UpdatePendingText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpendingtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipAutoCompleteProvider_Impl::UpdatePendingText(this, core::mem::transmute(&bstrpendingtext)).into()
        }
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITipAutoCompleteProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipAutoCompleteProvider_Impl::Show(this, core::mem::transmute_copy(&fshow)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UpdatePendingText: UpdatePendingText::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipAutoCompleteProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkCollectorEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkCollectorEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkCollectorEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkCollectorEvents_Vtbl
    where
        Identity: _IInkCollectorEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkCollectorEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkEditEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkEditEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkEditEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkEditEvents_Vtbl
    where
        Identity: _IInkEditEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkEditEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkEvents_Vtbl
    where
        Identity: _IInkEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkOverlayEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkOverlayEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkOverlayEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkOverlayEvents_Vtbl
    where
        Identity: _IInkOverlayEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkOverlayEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkPictureEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkPictureEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkPictureEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkPictureEvents_Vtbl
    where
        Identity: _IInkPictureEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkPictureEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkRecognitionEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkRecognitionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkRecognitionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkRecognitionEvents_Vtbl
    where
        Identity: _IInkRecognitionEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkRecognitionEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkStrokesEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IInkStrokesEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IInkStrokesEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IInkStrokesEvents_Vtbl
    where
        Identity: _IInkStrokesEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IInkStrokesEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IMathInputControlEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IMathInputControlEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IMathInputControlEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IMathInputControlEvents_Vtbl
    where
        Identity: _IMathInputControlEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IMathInputControlEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IPenInputPanelEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _IPenInputPanelEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _IPenInputPanelEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IPenInputPanelEvents_Vtbl
    where
        Identity: _IPenInputPanelEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IPenInputPanelEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}

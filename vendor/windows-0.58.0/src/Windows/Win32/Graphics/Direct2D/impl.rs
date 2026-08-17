pub trait ID2D1AnalysisTransform_Impl: Sized {
    fn ProcessAnalysisResults(&self, analysisdata: *const u8, analysisdatacount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1AnalysisTransform {}
impl ID2D1AnalysisTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1AnalysisTransform_Vtbl
    where
        Identity: ID2D1AnalysisTransform_Impl,
    {
        unsafe extern "system" fn ProcessAnalysisResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysisdata: *const u8, analysisdatacount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1AnalysisTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1AnalysisTransform_Impl::ProcessAnalysisResults(this, core::mem::transmute_copy(&analysisdata), core::mem::transmute_copy(&analysisdatacount)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProcessAnalysisResults: ProcessAnalysisResults::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1AnalysisTransform as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1Bitmap_Impl: Sized + ID2D1Image_Impl {
    fn GetSize(&self) -> Common::D2D_SIZE_F;
    fn GetPixelSize(&self) -> Common::D2D_SIZE_U;
    fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT;
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CopyFromBitmap(&self, destpoint: *const Common::D2D_POINT_2U, bitmap: Option<&ID2D1Bitmap>, srcrect: *const Common::D2D_RECT_U) -> windows_core::Result<()>;
    fn CopyFromRenderTarget(&self, destpoint: *const Common::D2D_POINT_2U, rendertarget: Option<&ID2D1RenderTarget>, srcrect: *const Common::D2D_RECT_U) -> windows_core::Result<()>;
    fn CopyFromMemory(&self, dstrect: *const Common::D2D_RECT_U, srcdata: *const core::ffi::c_void, pitch: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1Bitmap {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID2D1Bitmap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Bitmap_Vtbl
    where
        Identity: ID2D1Bitmap_Impl,
    {
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_SIZE_F)
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1Bitmap_Impl::GetSize(this)
        }
        unsafe extern "system" fn GetPixelSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_SIZE_U)
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1Bitmap_Impl::GetPixelSize(this)
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT)
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1Bitmap_Impl::GetPixelFormat(this)
        }
        unsafe extern "system" fn GetDpi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32)
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy))
        }
        unsafe extern "system" fn CopyFromBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, bitmap: *mut core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> windows_core::HRESULT
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap_Impl::CopyFromBitmap(this, core::mem::transmute_copy(&destpoint), windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&srcrect)).into()
        }
        unsafe extern "system" fn CopyFromRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, rendertarget: *mut core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> windows_core::HRESULT
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap_Impl::CopyFromRenderTarget(this, core::mem::transmute_copy(&destpoint), windows_core::from_raw_borrowed(&rendertarget), core::mem::transmute_copy(&srcrect)).into()
        }
        unsafe extern "system" fn CopyFromMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstrect: *const Common::D2D_RECT_U, srcdata: *const core::ffi::c_void, pitch: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Bitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap_Impl::CopyFromMemory(this, core::mem::transmute_copy(&dstrect), core::mem::transmute_copy(&srcdata), core::mem::transmute_copy(&pitch)).into()
        }
        Self {
            base__: ID2D1Image_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetDpi: GetDpi::<Identity, OFFSET>,
            CopyFromBitmap: CopyFromBitmap::<Identity, OFFSET>,
            CopyFromRenderTarget: CopyFromRenderTarget::<Identity, OFFSET>,
            CopyFromMemory: CopyFromMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Bitmap as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1Bitmap1_Impl: Sized + ID2D1Bitmap_Impl {
    fn GetColorContext(&self, colorcontext: *mut Option<ID2D1ColorContext>);
    fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS;
    fn GetSurface(&self) -> windows_core::Result<super::Dxgi::IDXGISurface>;
    fn Map(&self, options: D2D1_MAP_OPTIONS) -> windows_core::Result<D2D1_MAPPED_RECT>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1Bitmap1 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID2D1Bitmap1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Bitmap1_Vtbl
    where
        Identity: ID2D1Bitmap1_Impl,
    {
        unsafe extern "system" fn GetColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorcontext: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1Bitmap1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap1_Impl::GetColorContext(this, core::mem::transmute_copy(&colorcontext))
        }
        unsafe extern "system" fn GetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_BITMAP_OPTIONS
        where
            Identity: ID2D1Bitmap1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap1_Impl::GetOptions(this)
        }
        unsafe extern "system" fn GetSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgisurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Bitmap1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Bitmap1_Impl::GetSurface(this) {
                Ok(ok__) => {
                    dxgisurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> windows_core::HRESULT
        where
            Identity: ID2D1Bitmap1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Bitmap1_Impl::Map(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    mappedrect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Bitmap1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Bitmap1_Impl::Unmap(this).into()
        }
        Self {
            base__: ID2D1Bitmap_Vtbl::new::<Identity, OFFSET>(),
            GetColorContext: GetColorContext::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            GetSurface: GetSurface::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Bitmap1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID || iid == &<ID2D1Bitmap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1BitmapBrush_Impl: Sized + ID2D1Brush_Impl {
    fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE);
    fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE);
    fn SetInterpolationMode(&self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE);
    fn SetBitmap(&self, bitmap: Option<&ID2D1Bitmap>);
    fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE;
    fn GetInterpolationMode(&self) -> D2D1_BITMAP_INTERPOLATION_MODE;
    fn GetBitmap(&self, bitmap: *mut Option<ID2D1Bitmap>);
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeName for ID2D1BitmapBrush {}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1BitmapBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1BitmapBrush_Vtbl
    where
        Identity: ID2D1BitmapBrush_Impl,
    {
        unsafe extern "system" fn SetExtendModeX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE)
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::SetExtendModeX(this, core::mem::transmute_copy(&extendmodex))
        }
        unsafe extern "system" fn SetExtendModeY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE)
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::SetExtendModeY(this, core::mem::transmute_copy(&extendmodey))
        }
        unsafe extern "system" fn SetInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE)
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::SetInterpolationMode(this, core::mem::transmute_copy(&interpolationmode))
        }
        unsafe extern "system" fn SetBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void)
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::SetBitmap(this, windows_core::from_raw_borrowed(&bitmap))
        }
        unsafe extern "system" fn GetExtendModeX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::GetExtendModeX(this)
        }
        unsafe extern "system" fn GetExtendModeY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::GetExtendModeY(this)
        }
        unsafe extern "system" fn GetInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::GetInterpolationMode(this)
        }
        unsafe extern "system" fn GetBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1BitmapBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush_Impl::GetBitmap(this, core::mem::transmute_copy(&bitmap))
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetExtendModeX: SetExtendModeX::<Identity, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Identity, OFFSET>,
            SetBitmap: SetBitmap::<Identity, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, OFFSET>,
            GetInterpolationMode: GetInterpolationMode::<Identity, OFFSET>,
            GetBitmap: GetBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BitmapBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1BitmapBrush1_Impl: Sized + ID2D1BitmapBrush_Impl {
    fn SetInterpolationMode1(&self, interpolationmode: D2D1_INTERPOLATION_MODE);
    fn GetInterpolationMode1(&self) -> D2D1_INTERPOLATION_MODE;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeName for ID2D1BitmapBrush1 {}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1BitmapBrush1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1BitmapBrush1_Vtbl
    where
        Identity: ID2D1BitmapBrush1_Impl,
    {
        unsafe extern "system" fn SetInterpolationMode1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE)
        where
            Identity: ID2D1BitmapBrush1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush1_Impl::SetInterpolationMode1(this, core::mem::transmute_copy(&interpolationmode))
        }
        unsafe extern "system" fn GetInterpolationMode1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE
        where
            Identity: ID2D1BitmapBrush1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BitmapBrush1_Impl::GetInterpolationMode1(this)
        }
        Self {
            base__: ID2D1BitmapBrush_Vtbl::new::<Identity, OFFSET>(),
            SetInterpolationMode1: SetInterpolationMode1::<Identity, OFFSET>,
            GetInterpolationMode1: GetInterpolationMode1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BitmapBrush1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID || iid == &<ID2D1BitmapBrush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1BitmapRenderTarget_Impl: Sized + ID2D1RenderTarget_Impl {
    fn GetBitmap(&self) -> windows_core::Result<ID2D1Bitmap>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1BitmapRenderTarget {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1BitmapRenderTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1BitmapRenderTarget_Vtbl
    where
        Identity: ID2D1BitmapRenderTarget_Impl,
    {
        unsafe extern "system" fn GetBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1BitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1BitmapRenderTarget_Impl::GetBitmap(this) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(), GetBitmap: GetBitmap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BitmapRenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
pub trait ID2D1BlendTransform_Impl: Sized + ID2D1ConcreteTransform_Impl {
    fn SetDescription(&self, description: *const D2D1_BLEND_DESCRIPTION);
    fn GetDescription(&self, description: *mut D2D1_BLEND_DESCRIPTION);
}
impl windows_core::RuntimeName for ID2D1BlendTransform {}
impl ID2D1BlendTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1BlendTransform_Vtbl
    where
        Identity: ID2D1BlendTransform_Impl,
    {
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *const D2D1_BLEND_DESCRIPTION)
        where
            Identity: ID2D1BlendTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BlendTransform_Impl::SetDescription(this, core::mem::transmute_copy(&description))
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut D2D1_BLEND_DESCRIPTION)
        where
            Identity: ID2D1BlendTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BlendTransform_Impl::GetDescription(this, core::mem::transmute_copy(&description))
        }
        Self {
            base__: ID2D1ConcreteTransform_Vtbl::new::<Identity, OFFSET>(),
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BlendTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID || iid == &<ID2D1ConcreteTransform as windows_core::Interface>::IID
    }
}
pub trait ID2D1BorderTransform_Impl: Sized + ID2D1ConcreteTransform_Impl {
    fn SetExtendModeX(&self, extendmode: D2D1_EXTEND_MODE);
    fn SetExtendModeY(&self, extendmode: D2D1_EXTEND_MODE);
    fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE;
}
impl windows_core::RuntimeName for ID2D1BorderTransform {}
impl ID2D1BorderTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1BorderTransform_Vtbl
    where
        Identity: ID2D1BorderTransform_Impl,
    {
        unsafe extern "system" fn SetExtendModeX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmode: D2D1_EXTEND_MODE)
        where
            Identity: ID2D1BorderTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BorderTransform_Impl::SetExtendModeX(this, core::mem::transmute_copy(&extendmode))
        }
        unsafe extern "system" fn SetExtendModeY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmode: D2D1_EXTEND_MODE)
        where
            Identity: ID2D1BorderTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BorderTransform_Impl::SetExtendModeY(this, core::mem::transmute_copy(&extendmode))
        }
        unsafe extern "system" fn GetExtendModeX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1BorderTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BorderTransform_Impl::GetExtendModeX(this)
        }
        unsafe extern "system" fn GetExtendModeY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1BorderTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BorderTransform_Impl::GetExtendModeY(this)
        }
        Self {
            base__: ID2D1ConcreteTransform_Vtbl::new::<Identity, OFFSET>(),
            SetExtendModeX: SetExtendModeX::<Identity, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BorderTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID || iid == &<ID2D1ConcreteTransform as windows_core::Interface>::IID
    }
}
pub trait ID2D1BoundsAdjustmentTransform_Impl: Sized + ID2D1TransformNode_Impl {
    fn SetOutputBounds(&self, outputbounds: *const super::super::Foundation::RECT);
    fn GetOutputBounds(&self, outputbounds: *mut super::super::Foundation::RECT);
}
impl windows_core::RuntimeName for ID2D1BoundsAdjustmentTransform {}
impl ID2D1BoundsAdjustmentTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1BoundsAdjustmentTransform_Vtbl
    where
        Identity: ID2D1BoundsAdjustmentTransform_Impl,
    {
        unsafe extern "system" fn SetOutputBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputbounds: *const super::super::Foundation::RECT)
        where
            Identity: ID2D1BoundsAdjustmentTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BoundsAdjustmentTransform_Impl::SetOutputBounds(this, core::mem::transmute_copy(&outputbounds))
        }
        unsafe extern "system" fn GetOutputBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputbounds: *mut super::super::Foundation::RECT)
        where
            Identity: ID2D1BoundsAdjustmentTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1BoundsAdjustmentTransform_Impl::GetOutputBounds(this, core::mem::transmute_copy(&outputbounds))
        }
        Self {
            base__: ID2D1TransformNode_Vtbl::new::<Identity, OFFSET>(),
            SetOutputBounds: SetOutputBounds::<Identity, OFFSET>,
            GetOutputBounds: GetOutputBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1BoundsAdjustmentTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1Brush_Impl: Sized + ID2D1Resource_Impl {
    fn SetOpacity(&self, opacity: f32);
    fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2);
    fn GetOpacity(&self) -> f32;
    fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeName for ID2D1Brush {}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1Brush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Brush_Vtbl
    where
        Identity: ID2D1Brush_Impl,
    {
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32)
        where
            Identity: ID2D1Brush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Brush_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity))
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1Brush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Brush_Impl::SetTransform(this, core::mem::transmute_copy(&transform))
        }
        unsafe extern "system" fn GetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: ID2D1Brush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Brush_Impl::GetOpacity(this)
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1Brush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Brush_Impl::GetTransform(this, core::mem::transmute_copy(&transform))
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetOpacity: GetOpacity::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Brush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1ColorContext_Impl: Sized + ID2D1Resource_Impl {
    fn GetColorSpace(&self) -> D2D1_COLOR_SPACE;
    fn GetProfileSize(&self) -> u32;
    fn GetProfile(&self, profile: *mut u8, profilesize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1ColorContext {}
impl ID2D1ColorContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ColorContext_Vtbl
    where
        Identity: ID2D1ColorContext_Impl,
    {
        unsafe extern "system" fn GetColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_SPACE
        where
            Identity: ID2D1ColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ColorContext_Impl::GetColorSpace(this)
        }
        unsafe extern "system" fn GetProfileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1ColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ColorContext_Impl::GetProfileSize(this)
        }
        unsafe extern "system" fn GetProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut u8, profilesize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1ColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ColorContext_Impl::GetProfile(this, core::mem::transmute_copy(&profile), core::mem::transmute_copy(&profilesize)).into()
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetColorSpace: GetColorSpace::<Identity, OFFSET>,
            GetProfileSize: GetProfileSize::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ColorContext as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1ColorContext1_Impl: Sized + ID2D1ColorContext_Impl {
    fn GetColorContextType(&self) -> D2D1_COLOR_CONTEXT_TYPE;
    fn GetDXGIColorSpace(&self) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE;
    fn GetSimpleColorProfile(&self, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1ColorContext1 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID2D1ColorContext1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ColorContext1_Vtbl
    where
        Identity: ID2D1ColorContext1_Impl,
    {
        unsafe extern "system" fn GetColorContextType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE
        where
            Identity: ID2D1ColorContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ColorContext1_Impl::GetColorContextType(this)
        }
        unsafe extern "system" fn GetDXGIColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
        where
            Identity: ID2D1ColorContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ColorContext1_Impl::GetDXGIColorSpace(this)
        }
        unsafe extern "system" fn GetSimpleColorProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::HRESULT
        where
            Identity: ID2D1ColorContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ColorContext1_Impl::GetSimpleColorProfile(this, core::mem::transmute_copy(&simpleprofile)).into()
        }
        Self {
            base__: ID2D1ColorContext_Vtbl::new::<Identity, OFFSET>(),
            GetColorContextType: GetColorContextType::<Identity, OFFSET>,
            GetDXGIColorSpace: GetDXGIColorSpace::<Identity, OFFSET>,
            GetSimpleColorProfile: GetSimpleColorProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ColorContext1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1ColorContext as windows_core::Interface>::IID
    }
}
pub trait ID2D1CommandList_Impl: Sized + ID2D1Image_Impl {
    fn Stream(&self, sink: Option<&ID2D1CommandSink>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1CommandList {}
impl ID2D1CommandList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandList_Vtbl
    where
        Identity: ID2D1CommandList_Impl,
    {
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandList_Impl::Stream(this, windows_core::from_raw_borrowed(&sink)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandList_Impl::Close(this).into()
        }
        Self { base__: ID2D1Image_Vtbl::new::<Identity, OFFSET>(), Stream: Stream::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandList as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink_Impl: Sized {
    fn BeginDraw(&self) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> windows_core::Result<()>;
    fn SetTags(&self, tag1: u64, tag2: u64) -> windows_core::Result<()>;
    fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::Result<()>;
    fn SetTextRenderingParams(&self, textrenderingparams: Option<&super::DirectWrite::IDWriteRenderingParams>) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()>;
    fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> windows_core::Result<()>;
    fn Clear(&self, color: *const Common::D2D1_COLOR_F) -> windows_core::Result<()>;
    fn DrawGlyphRun(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: Option<&ID2D1Brush>, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> windows_core::Result<()>;
    fn DrawLine(&self, point0: &Common::D2D_POINT_2F, point1: &Common::D2D_POINT_2F, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>) -> windows_core::Result<()>;
    fn DrawGeometry(&self, geometry: Option<&ID2D1Geometry>, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>) -> windows_core::Result<()>;
    fn DrawRectangle(&self, rect: *const Common::D2D_RECT_F, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>) -> windows_core::Result<()>;
    fn DrawBitmap(&self, bitmap: Option<&ID2D1Bitmap>, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> windows_core::Result<()>;
    fn DrawImage(&self, image: Option<&ID2D1Image>, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> windows_core::Result<()>;
    fn DrawGdiMetafile(&self, gdimetafile: Option<&ID2D1GdiMetafile>, targetoffset: *const Common::D2D_POINT_2F) -> windows_core::Result<()>;
    fn FillMesh(&self, mesh: Option<&ID2D1Mesh>, brush: Option<&ID2D1Brush>) -> windows_core::Result<()>;
    fn FillOpacityMask(&self, opacitymask: Option<&ID2D1Bitmap>, brush: Option<&ID2D1Brush>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> windows_core::Result<()>;
    fn FillGeometry(&self, geometry: Option<&ID2D1Geometry>, brush: Option<&ID2D1Brush>, opacitybrush: Option<&ID2D1Brush>) -> windows_core::Result<()>;
    fn FillRectangle(&self, rect: *const Common::D2D_RECT_F, brush: Option<&ID2D1Brush>) -> windows_core::Result<()>;
    fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> windows_core::Result<()>;
    fn PushLayer(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: Option<&ID2D1Layer>) -> windows_core::Result<()>;
    fn PopAxisAlignedClip(&self) -> windows_core::Result<()>;
    fn PopLayer(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1CommandSink {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandSink_Vtbl
    where
        Identity: ID2D1CommandSink_Impl,
    {
        unsafe extern "system" fn BeginDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::BeginDraw(this).into()
        }
        unsafe extern "system" fn EndDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::EndDraw(this).into()
        }
        unsafe extern "system" fn SetAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetAntialiasMode(this, core::mem::transmute_copy(&antialiasmode)).into()
        }
        unsafe extern "system" fn SetTags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: u64, tag2: u64) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetTags(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetTextAntialiasMode(this, core::mem::transmute_copy(&textantialiasmode)).into()
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetTextRenderingParams(this, windows_core::from_raw_borrowed(&textrenderingparams)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetTransform(this, core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn SetPrimitiveBlend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetPrimitiveBlend(this, core::mem::transmute_copy(&primitiveblend)).into()
        }
        unsafe extern "system" fn SetUnitMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unitmode: D2D1_UNIT_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::SetUnitMode(this, core::mem::transmute_copy(&unitmode)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::Clear(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), windows_core::from_raw_borrowed(&foregroundbrush), core::mem::transmute_copy(&measuringmode)).into()
        }
        unsafe extern "system" fn DrawLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawLine(this, core::mem::transmute(&point0), core::mem::transmute(&point1), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle)).into()
        }
        unsafe extern "system" fn DrawGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawGeometry(this, windows_core::from_raw_borrowed(&geometry), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle)).into()
        }
        unsafe extern "system" fn DrawRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawRectangle(this, core::mem::transmute_copy(&rect), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle)).into()
        }
        unsafe extern "system" fn DrawBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawBitmap(this, windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&opacity), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&sourcerectangle), core::mem::transmute_copy(&perspectivetransform)).into()
        }
        unsafe extern "system" fn DrawImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawImage(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&compositemode)).into()
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::DrawGdiMetafile(this, windows_core::from_raw_borrowed(&gdimetafile), core::mem::transmute_copy(&targetoffset)).into()
        }
        unsafe extern "system" fn FillMesh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mesh: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::FillMesh(this, windows_core::from_raw_borrowed(&mesh), windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn FillOpacityMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymask: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::FillOpacityMask(this, windows_core::from_raw_borrowed(&opacitymask), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle)).into()
        }
        unsafe extern "system" fn FillGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, opacitybrush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::FillGeometry(this, windows_core::from_raw_borrowed(&geometry), windows_core::from_raw_borrowed(&brush), windows_core::from_raw_borrowed(&opacitybrush)).into()
        }
        unsafe extern "system" fn FillRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::FillRectangle(this, core::mem::transmute_copy(&rect), windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn PushAxisAlignedClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::PushAxisAlignedClip(this, core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&antialiasmode)).into()
        }
        unsafe extern "system" fn PushLayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::PushLayer(this, core::mem::transmute_copy(&layerparameters1), windows_core::from_raw_borrowed(&layer)).into()
        }
        unsafe extern "system" fn PopAxisAlignedClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::PopAxisAlignedClip(this).into()
        }
        unsafe extern "system" fn PopLayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink_Impl::PopLayer(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, OFFSET>,
            SetTags: SetTags::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetPrimitiveBlend: SetPrimitiveBlend::<Identity, OFFSET>,
            SetUnitMode: SetUnitMode::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawLine: DrawLine::<Identity, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            DrawImage: DrawImage::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
            FillMesh: FillMesh::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
            FillGeometry: FillGeometry::<Identity, OFFSET>,
            FillRectangle: FillRectangle::<Identity, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, OFFSET>,
            PopLayer: PopLayer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink1_Impl: Sized + ID2D1CommandSink_Impl {
    fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1CommandSink1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandSink1_Vtbl
    where
        Identity: ID2D1CommandSink1_Impl,
    {
        unsafe extern "system" fn SetPrimitiveBlend1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink1_Impl::SetPrimitiveBlend1(this, core::mem::transmute_copy(&primitiveblend)).into()
        }
        Self { base__: ID2D1CommandSink_Vtbl::new::<Identity, OFFSET>(), SetPrimitiveBlend1: SetPrimitiveBlend1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink2_Impl: Sized + ID2D1CommandSink1_Impl {
    fn DrawInk(&self, ink: Option<&ID2D1Ink>, brush: Option<&ID2D1Brush>, inkstyle: Option<&ID2D1InkStyle>) -> windows_core::Result<()>;
    fn DrawGradientMesh(&self, gradientmesh: Option<&ID2D1GradientMesh>) -> windows_core::Result<()>;
    fn DrawGdiMetafile(&self, gdimetafile: Option<&ID2D1GdiMetafile>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1CommandSink2 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandSink2_Vtbl
    where
        Identity: ID2D1CommandSink2_Impl,
    {
        unsafe extern "system" fn DrawInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink2_Impl::DrawInk(this, windows_core::from_raw_borrowed(&ink), windows_core::from_raw_borrowed(&brush), windows_core::from_raw_borrowed(&inkstyle)).into()
        }
        unsafe extern "system" fn DrawGradientMesh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientmesh: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink2_Impl::DrawGradientMesh(this, windows_core::from_raw_borrowed(&gradientmesh)).into()
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink2_Impl::DrawGdiMetafile(this, windows_core::from_raw_borrowed(&gdimetafile), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle)).into()
        }
        Self {
            base__: ID2D1CommandSink1_Vtbl::new::<Identity, OFFSET>(),
            DrawInk: DrawInk::<Identity, OFFSET>,
            DrawGradientMesh: DrawGradientMesh::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink as windows_core::Interface>::IID || iid == &<ID2D1CommandSink1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink3_Impl: Sized + ID2D1CommandSink2_Impl {
    fn DrawSpriteBatch(&self, spritebatch: Option<&ID2D1SpriteBatch>, startindex: u32, spritecount: u32, bitmap: Option<&ID2D1Bitmap>, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1CommandSink3 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandSink3_Vtbl
    where
        Identity: ID2D1CommandSink3_Impl,
    {
        unsafe extern "system" fn DrawSpriteBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritebatch: *mut core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink3_Impl::DrawSpriteBatch(this, windows_core::from_raw_borrowed(&spritebatch), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&spriteoptions)).into()
        }
        Self { base__: ID2D1CommandSink2_Vtbl::new::<Identity, OFFSET>(), DrawSpriteBatch: DrawSpriteBatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink3 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink as windows_core::Interface>::IID || iid == &<ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink4_Impl: Sized + ID2D1CommandSink3_Impl {
    fn SetPrimitiveBlend2(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1CommandSink4 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandSink4_Vtbl
    where
        Identity: ID2D1CommandSink4_Impl,
    {
        unsafe extern "system" fn SetPrimitiveBlend2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink4_Impl::SetPrimitiveBlend2(this, core::mem::transmute_copy(&primitiveblend)).into()
        }
        Self { base__: ID2D1CommandSink3_Vtbl::new::<Identity, OFFSET>(), SetPrimitiveBlend2: SetPrimitiveBlend2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink4 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink as windows_core::Interface>::IID || iid == &<ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink5_Impl: Sized + ID2D1CommandSink4_Impl {
    fn BlendImage(&self, image: Option<&ID2D1Image>, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1CommandSink5 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1CommandSink5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1CommandSink5_Vtbl
    where
        Identity: ID2D1CommandSink5_Impl,
    {
        unsafe extern "system" fn BlendImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> windows_core::HRESULT
        where
            Identity: ID2D1CommandSink5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1CommandSink5_Impl::BlendImage(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&blendmode), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode)).into()
        }
        Self { base__: ID2D1CommandSink4_Vtbl::new::<Identity, OFFSET>(), BlendImage: BlendImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink5 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink as windows_core::Interface>::IID || iid == &<ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink3 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink4 as windows_core::Interface>::IID
    }
}
pub trait ID2D1ComputeInfo_Impl: Sized + ID2D1RenderInfo_Impl {
    fn SetComputeShaderConstantBuffer(&self, buffer: *const u8, buffercount: u32) -> windows_core::Result<()>;
    fn SetComputeShader(&self, shaderid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetResourceTexture(&self, textureindex: u32, resourcetexture: Option<&ID2D1ResourceTexture>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1ComputeInfo {}
impl ID2D1ComputeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ComputeInfo_Vtbl
    where
        Identity: ID2D1ComputeInfo_Impl,
    {
        unsafe extern "system" fn SetComputeShaderConstantBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *const u8, buffercount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1ComputeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ComputeInfo_Impl::SetComputeShaderConstantBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffercount)).into()
        }
        unsafe extern "system" fn SetComputeShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shaderid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ID2D1ComputeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ComputeInfo_Impl::SetComputeShader(this, core::mem::transmute_copy(&shaderid)).into()
        }
        unsafe extern "system" fn SetResourceTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textureindex: u32, resourcetexture: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1ComputeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ComputeInfo_Impl::SetResourceTexture(this, core::mem::transmute_copy(&textureindex), windows_core::from_raw_borrowed(&resourcetexture)).into()
        }
        Self {
            base__: ID2D1RenderInfo_Vtbl::new::<Identity, OFFSET>(),
            SetComputeShaderConstantBuffer: SetComputeShaderConstantBuffer::<Identity, OFFSET>,
            SetComputeShader: SetComputeShader::<Identity, OFFSET>,
            SetResourceTexture: SetResourceTexture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ComputeInfo as windows_core::Interface>::IID || iid == &<ID2D1RenderInfo as windows_core::Interface>::IID
    }
}
pub trait ID2D1ComputeTransform_Impl: Sized + ID2D1Transform_Impl {
    fn SetComputeInfo(&self, computeinfo: Option<&ID2D1ComputeInfo>) -> windows_core::Result<()>;
    fn CalculateThreadgroups(&self, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1ComputeTransform {}
impl ID2D1ComputeTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ComputeTransform_Vtbl
    where
        Identity: ID2D1ComputeTransform_Impl,
    {
        unsafe extern "system" fn SetComputeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, computeinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1ComputeTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ComputeTransform_Impl::SetComputeInfo(this, windows_core::from_raw_borrowed(&computeinfo)).into()
        }
        unsafe extern "system" fn CalculateThreadgroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID2D1ComputeTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ComputeTransform_Impl::CalculateThreadgroups(this, core::mem::transmute_copy(&outputrect), core::mem::transmute_copy(&dimensionx), core::mem::transmute_copy(&dimensiony), core::mem::transmute_copy(&dimensionz)).into()
        }
        Self {
            base__: ID2D1Transform_Vtbl::new::<Identity, OFFSET>(),
            SetComputeInfo: SetComputeInfo::<Identity, OFFSET>,
            CalculateThreadgroups: CalculateThreadgroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ComputeTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID || iid == &<ID2D1Transform as windows_core::Interface>::IID
    }
}
pub trait ID2D1ConcreteTransform_Impl: Sized + ID2D1TransformNode_Impl {
    fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> windows_core::Result<()>;
    fn SetCached(&self, iscached: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for ID2D1ConcreteTransform {}
impl ID2D1ConcreteTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ConcreteTransform_Vtbl
    where
        Identity: ID2D1ConcreteTransform_Impl,
    {
        unsafe extern "system" fn SetOutputBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> windows_core::HRESULT
        where
            Identity: ID2D1ConcreteTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ConcreteTransform_Impl::SetOutputBuffer(this, core::mem::transmute_copy(&bufferprecision), core::mem::transmute_copy(&channeldepth)).into()
        }
        unsafe extern "system" fn SetCached<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscached: super::super::Foundation::BOOL)
        where
            Identity: ID2D1ConcreteTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ConcreteTransform_Impl::SetCached(this, core::mem::transmute_copy(&iscached))
        }
        Self {
            base__: ID2D1TransformNode_Vtbl::new::<Identity, OFFSET>(),
            SetOutputBuffer: SetOutputBuffer::<Identity, OFFSET>,
            SetCached: SetCached::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ConcreteTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DCRenderTarget_Impl: Sized + ID2D1RenderTarget_Impl {
    fn BindDC(&self, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1DCRenderTarget {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DCRenderTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DCRenderTarget_Vtbl
    where
        Identity: ID2D1DCRenderTarget_Impl,
    {
        unsafe extern "system" fn BindDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ID2D1DCRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DCRenderTarget_Impl::BindDC(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&psubrect)).into()
        }
        Self { base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(), BindDC: BindDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DCRenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device_Impl: Sized + ID2D1Resource_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext>;
    fn CreatePrintControl(&self, wicfactory: Option<&super::Imaging::IWICImagingFactory>, documenttarget: Option<&super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES) -> windows_core::Result<ID2D1PrintControl>;
    fn SetMaximumTextureMemory(&self, maximuminbytes: u64);
    fn GetMaximumTextureMemory(&self) -> u64;
    fn ClearResources(&self, millisecondssinceuse: u32);
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device {}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device_Vtbl
    where
        Identity: ID2D1Device_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicfactory: *mut core::ffi::c_void, documenttarget: *mut core::ffi::c_void, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device_Impl::CreatePrintControl(this, windows_core::from_raw_borrowed(&wicfactory), windows_core::from_raw_borrowed(&documenttarget), core::mem::transmute_copy(&printcontrolproperties)) {
                Ok(ok__) => {
                    printcontrol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumTextureMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximuminbytes: u64)
        where
            Identity: ID2D1Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device_Impl::SetMaximumTextureMemory(this, core::mem::transmute_copy(&maximuminbytes))
        }
        unsafe extern "system" fn GetMaximumTextureMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID2D1Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device_Impl::GetMaximumTextureMemory(this)
        }
        unsafe extern "system" fn ClearResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, millisecondssinceuse: u32)
        where
            Identity: ID2D1Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device_Impl::ClearResources(this, core::mem::transmute_copy(&millisecondssinceuse))
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
            CreatePrintControl: CreatePrintControl::<Identity, OFFSET>,
            SetMaximumTextureMemory: SetMaximumTextureMemory::<Identity, OFFSET>,
            GetMaximumTextureMemory: GetMaximumTextureMemory::<Identity, OFFSET>,
            ClearResources: ClearResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device1_Impl: Sized + ID2D1Device_Impl {
    fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY;
    fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY);
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext1>;
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device1 {}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device1_Vtbl
    where
        Identity: ID2D1Device1_Impl,
    {
        unsafe extern "system" fn GetRenderingPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_RENDERING_PRIORITY
        where
            Identity: ID2D1Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device1_Impl::GetRenderingPriority(this)
        }
        unsafe extern "system" fn SetRenderingPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingpriority: D2D1_RENDERING_PRIORITY)
        where
            Identity: ID2D1Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device1_Impl::SetRenderingPriority(this, core::mem::transmute_copy(&renderingpriority))
        }
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device1_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Device_Vtbl::new::<Identity, OFFSET>(),
            GetRenderingPriority: GetRenderingPriority::<Identity, OFFSET>,
            SetRenderingPriority: SetRenderingPriority::<Identity, OFFSET>,
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device2_Impl: Sized + ID2D1Device1_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext2>;
    fn FlushDeviceContexts(&self, bitmap: Option<&ID2D1Bitmap>);
    fn GetDxgiDevice(&self) -> windows_core::Result<super::Dxgi::IDXGIDevice>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device2 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device2_Vtbl
    where
        Identity: ID2D1Device2_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device2_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushDeviceContexts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void)
        where
            Identity: ID2D1Device2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device2_Impl::FlushDeviceContexts(this, windows_core::from_raw_borrowed(&bitmap))
        }
        unsafe extern "system" fn GetDxgiDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device2_Impl::GetDxgiDevice(this) {
                Ok(ok__) => {
                    dxgidevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Device1_Vtbl::new::<Identity, OFFSET>(),
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
            FlushDeviceContexts: FlushDeviceContexts::<Identity, OFFSET>,
            GetDxgiDevice: GetDxgiDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Device1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device3_Impl: Sized + ID2D1Device2_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext3>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device3 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device3_Vtbl
    where
        Identity: ID2D1Device3_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device3_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext3.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Device2_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device4_Impl: Sized + ID2D1Device3_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext4>;
    fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64);
    fn GetMaximumColorGlyphCacheMemory(&self) -> u64;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device4 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device4_Vtbl
    where
        Identity: ID2D1Device4_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device4_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext4.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumColorGlyphCacheMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximuminbytes: u64)
        where
            Identity: ID2D1Device4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device4_Impl::SetMaximumColorGlyphCacheMemory(this, core::mem::transmute_copy(&maximuminbytes))
        }
        unsafe extern "system" fn GetMaximumColorGlyphCacheMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID2D1Device4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Device4_Impl::GetMaximumColorGlyphCacheMemory(this)
        }
        Self {
            base__: ID2D1Device3_Vtbl::new::<Identity, OFFSET>(),
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
            SetMaximumColorGlyphCacheMemory: SetMaximumColorGlyphCacheMemory::<Identity, OFFSET>,
            GetMaximumColorGlyphCacheMemory: GetMaximumColorGlyphCacheMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device4 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device5_Impl: Sized + ID2D1Device4_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext5>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device5 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device5_Vtbl
    where
        Identity: ID2D1Device5_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device5_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext5.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Device4_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device5 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Device4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device6_Impl: Sized + ID2D1Device5_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext6>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device6 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device6_Vtbl
    where
        Identity: ID2D1Device6_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device6_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext6.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Device5_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device6 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Device4 as windows_core::Interface>::IID || iid == &<ID2D1Device5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device7_Impl: Sized + ID2D1Device6_Impl {
    fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext7>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl windows_core::RuntimeName for ID2D1Device7 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ID2D1Device7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Device7_Vtbl
    where
        Identity: ID2D1Device7_Impl,
    {
        unsafe extern "system" fn CreateDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Device7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Device7_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    devicecontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Device6_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device7 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Device as windows_core::Interface>::IID || iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Device4 as windows_core::Interface>::IID || iid == &<ID2D1Device5 as windows_core::Interface>::IID || iid == &<ID2D1Device6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext_Impl: Sized + ID2D1RenderTarget_Impl {
    fn CreateBitmap(&self, size: &Common::D2D_SIZE_U, sourcedata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1>;
    fn CreateBitmapFromWicBitmap(&self, wicbitmapsource: Option<&super::Imaging::IWICBitmapSource>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1>;
    fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromFilename(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromWicColorContext(&self, wiccolorcontext: Option<&super::Imaging::IWICColorContext>) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateBitmapFromDxgiSurface(&self, surface: Option<&super::Dxgi::IDXGISurface>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1>;
    fn CreateEffect(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Effect>;
    fn CreateGradientStopCollection(&self, straightalphagradientstops: *const Common::D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> windows_core::Result<ID2D1GradientStopCollection1>;
    fn CreateImageBrush(&self, image: Option<&ID2D1Image>, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1ImageBrush>;
    fn CreateBitmapBrush(&self, bitmap: Option<&ID2D1Bitmap>, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1BitmapBrush1>;
    fn CreateCommandList(&self) -> windows_core::Result<ID2D1CommandList>;
    fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL;
    fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL;
    fn GetImageLocalBounds(&self, image: Option<&ID2D1Image>) -> windows_core::Result<Common::D2D_RECT_F>;
    fn GetImageWorldBounds(&self, image: Option<&ID2D1Image>) -> windows_core::Result<Common::D2D_RECT_F>;
    fn GetGlyphRunWorldBounds(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> windows_core::Result<Common::D2D_RECT_F>;
    fn GetDevice(&self, device: *mut Option<ID2D1Device>);
    fn SetTarget(&self, image: Option<&ID2D1Image>);
    fn GetTarget(&self, image: *mut Option<ID2D1Image>);
    fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS);
    fn GetRenderingControls(&self, renderingcontrols: *mut D2D1_RENDERING_CONTROLS);
    fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND);
    fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND;
    fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE);
    fn GetUnitMode(&self) -> D2D1_UNIT_MODE;
    fn DrawGlyphRun(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: Option<&ID2D1Brush>, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawImage(&self, image: Option<&ID2D1Image>, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE);
    fn DrawGdiMetafile(&self, gdimetafile: Option<&ID2D1GdiMetafile>, targetoffset: *const Common::D2D_POINT_2F);
    fn DrawBitmap(&self, bitmap: Option<&ID2D1Bitmap>, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F);
    fn PushLayer(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: Option<&ID2D1Layer>);
    fn InvalidateEffectInputRectangle(&self, effect: Option<&ID2D1Effect>, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> windows_core::Result<()>;
    fn GetEffectInvalidRectangleCount(&self, effect: Option<&ID2D1Effect>) -> windows_core::Result<u32>;
    fn GetEffectInvalidRectangles(&self, effect: Option<&ID2D1Effect>, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> windows_core::Result<()>;
    fn GetEffectRequiredInputRectangles(&self, rendereffect: Option<&ID2D1Effect>, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> windows_core::Result<()>;
    fn FillOpacityMask(&self, opacitymask: Option<&ID2D1Bitmap>, brush: Option<&ID2D1Brush>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1DeviceContext {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext_Vtbl
    where
        Identity: ID2D1DeviceContext_Impl,
    {
        unsafe extern "system" fn CreateBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: Common::D2D_SIZE_U, sourcedata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateBitmap(this, core::mem::transmute(&size), core::mem::transmute_copy(&sourcedata), core::mem::transmute_copy(&pitch), core::mem::transmute_copy(&bitmapproperties)) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateBitmapFromWicBitmap(this, windows_core::from_raw_borrowed(&wicbitmapsource), core::mem::transmute_copy(&bitmapproperties)) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateColorContext(this, core::mem::transmute_copy(&space), core::mem::transmute_copy(&profile), core::mem::transmute_copy(&profilesize)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateColorContextFromFilename(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wiccolorcontext: *mut core::ffi::c_void, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateColorContextFromWicColorContext(this, windows_core::from_raw_borrowed(&wiccolorcontext)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromDxgiSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, surface: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateBitmapFromDxgiSurface(this, windows_core::from_raw_borrowed(&surface), core::mem::transmute_copy(&bitmapproperties)) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: *const windows_core::GUID, effect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateEffect(this, core::mem::transmute_copy(&effectid)) {
                Ok(ok__) => {
                    effect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, straightalphagradientstops: *const Common::D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateGradientStopCollection(this, core::mem::transmute_copy(&straightalphagradientstops), core::mem::transmute_copy(&straightalphagradientstopscount), core::mem::transmute_copy(&preinterpolationspace), core::mem::transmute_copy(&postinterpolationspace), core::mem::transmute_copy(&bufferprecision), core::mem::transmute_copy(&extendmode), core::mem::transmute_copy(&colorinterpolationmode)) {
                Ok(ok__) => {
                    gradientstopcollection1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, imagebrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateImageBrush(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&imagebrushproperties), core::mem::transmute_copy(&brushproperties)) {
                Ok(ok__) => {
                    imagebrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateBitmapBrush(this, windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&bitmapbrushproperties), core::mem::transmute_copy(&brushproperties)) {
                Ok(ok__) => {
                    bitmapbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCommandList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::CreateCommandList(this) {
                Ok(ok__) => {
                    commandlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDxgiFormatSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::IsDxgiFormatSupported(this, core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::IsBufferPrecisionSupported(this, core::mem::transmute_copy(&bufferprecision))
        }
        unsafe extern "system" fn GetImageLocalBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, localbounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::GetImageLocalBounds(this, windows_core::from_raw_borrowed(&image)) {
                Ok(ok__) => {
                    localbounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageWorldBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, worldbounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::GetImageWorldBounds(this, windows_core::from_raw_borrowed(&image)) {
                Ok(ok__) => {
                    worldbounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphRunWorldBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::GetGlyphRunWorldBounds(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&measuringmode)) {
                Ok(ok__) => {
                    bounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetDevice(this, core::mem::transmute_copy(&device))
        }
        unsafe extern "system" fn SetTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::SetTarget(this, windows_core::from_raw_borrowed(&image))
        }
        unsafe extern "system" fn GetTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetTarget(this, core::mem::transmute_copy(&image))
        }
        unsafe extern "system" fn SetRenderingControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingcontrols: *const D2D1_RENDERING_CONTROLS)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::SetRenderingControls(this, core::mem::transmute_copy(&renderingcontrols))
        }
        unsafe extern "system" fn GetRenderingControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingcontrols: *mut D2D1_RENDERING_CONTROLS)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetRenderingControls(this, core::mem::transmute_copy(&renderingcontrols))
        }
        unsafe extern "system" fn SetPrimitiveBlend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::SetPrimitiveBlend(this, core::mem::transmute_copy(&primitiveblend))
        }
        unsafe extern "system" fn GetPrimitiveBlend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetPrimitiveBlend(this)
        }
        unsafe extern "system" fn SetUnitMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unitmode: D2D1_UNIT_MODE)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::SetUnitMode(this, core::mem::transmute_copy(&unitmode))
        }
        unsafe extern "system" fn GetUnitMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_UNIT_MODE
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetUnitMode(this)
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::DrawGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), windows_core::from_raw_borrowed(&foregroundbrush), core::mem::transmute_copy(&measuringmode))
        }
        unsafe extern "system" fn DrawImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::DrawImage(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&compositemode))
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::DrawGdiMetafile(this, windows_core::from_raw_borrowed(&gdimetafile), core::mem::transmute_copy(&targetoffset))
        }
        unsafe extern "system" fn DrawBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::DrawBitmap(this, windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&opacity), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&sourcerectangle), core::mem::transmute_copy(&perspectivetransform))
        }
        unsafe extern "system" fn PushLayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::PushLayer(this, core::mem::transmute_copy(&layerparameters), windows_core::from_raw_borrowed(&layer))
        }
        unsafe extern "system" fn InvalidateEffectInputRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::InvalidateEffectInputRectangle(this, windows_core::from_raw_borrowed(&effect), core::mem::transmute_copy(&input), core::mem::transmute_copy(&inputrectangle)).into()
        }
        unsafe extern "system" fn GetEffectInvalidRectangleCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, rectanglecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext_Impl::GetEffectInvalidRectangleCount(this, windows_core::from_raw_borrowed(&effect)) {
                Ok(ok__) => {
                    rectanglecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInvalidRectangles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetEffectInvalidRectangles(this, windows_core::from_raw_borrowed(&effect), core::mem::transmute_copy(&rectangles), core::mem::transmute_copy(&rectanglescount)).into()
        }
        unsafe extern "system" fn GetEffectRequiredInputRectangles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendereffect: *mut core::ffi::c_void, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::GetEffectRequiredInputRectangles(this, windows_core::from_raw_borrowed(&rendereffect), core::mem::transmute_copy(&renderimagerectangle), core::mem::transmute_copy(&inputdescriptions), core::mem::transmute_copy(&requiredinputrects), core::mem::transmute_copy(&inputcount)).into()
        }
        unsafe extern "system" fn FillOpacityMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymask: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F)
        where
            Identity: ID2D1DeviceContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext_Impl::FillOpacityMask(this, windows_core::from_raw_borrowed(&opacitymask), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle))
        }
        Self {
            base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(),
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromWicBitmap: CreateBitmapFromWicBitmap::<Identity, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, OFFSET>,
            CreateColorContextFromFilename: CreateColorContextFromFilename::<Identity, OFFSET>,
            CreateColorContextFromWicColorContext: CreateColorContextFromWicColorContext::<Identity, OFFSET>,
            CreateBitmapFromDxgiSurface: CreateBitmapFromDxgiSurface::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            CreateGradientStopCollection: CreateGradientStopCollection::<Identity, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, OFFSET>,
            CreateBitmapBrush: CreateBitmapBrush::<Identity, OFFSET>,
            CreateCommandList: CreateCommandList::<Identity, OFFSET>,
            IsDxgiFormatSupported: IsDxgiFormatSupported::<Identity, OFFSET>,
            IsBufferPrecisionSupported: IsBufferPrecisionSupported::<Identity, OFFSET>,
            GetImageLocalBounds: GetImageLocalBounds::<Identity, OFFSET>,
            GetImageWorldBounds: GetImageWorldBounds::<Identity, OFFSET>,
            GetGlyphRunWorldBounds: GetGlyphRunWorldBounds::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetTarget: SetTarget::<Identity, OFFSET>,
            GetTarget: GetTarget::<Identity, OFFSET>,
            SetRenderingControls: SetRenderingControls::<Identity, OFFSET>,
            GetRenderingControls: GetRenderingControls::<Identity, OFFSET>,
            SetPrimitiveBlend: SetPrimitiveBlend::<Identity, OFFSET>,
            GetPrimitiveBlend: GetPrimitiveBlend::<Identity, OFFSET>,
            SetUnitMode: SetUnitMode::<Identity, OFFSET>,
            GetUnitMode: GetUnitMode::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawImage: DrawImage::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            InvalidateEffectInputRectangle: InvalidateEffectInputRectangle::<Identity, OFFSET>,
            GetEffectInvalidRectangleCount: GetEffectInvalidRectangleCount::<Identity, OFFSET>,
            GetEffectInvalidRectangles: GetEffectInvalidRectangles::<Identity, OFFSET>,
            GetEffectRequiredInputRectangles: GetEffectRequiredInputRectangles::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext1_Impl: Sized + ID2D1DeviceContext_Impl {
    fn CreateFilledGeometryRealization(&self, geometry: Option<&ID2D1Geometry>, flatteningtolerance: f32) -> windows_core::Result<ID2D1GeometryRealization>;
    fn CreateStrokedGeometryRealization(&self, geometry: Option<&ID2D1Geometry>, flatteningtolerance: f32, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>) -> windows_core::Result<ID2D1GeometryRealization>;
    fn DrawGeometryRealization(&self, geometryrealization: Option<&ID2D1GeometryRealization>, brush: Option<&ID2D1Brush>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1DeviceContext1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext1_Vtbl
    where
        Identity: ID2D1DeviceContext1_Impl,
    {
        unsafe extern "system" fn CreateFilledGeometryRealization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, flatteningtolerance: f32, geometryrealization: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext1_Impl::CreateFilledGeometryRealization(this, windows_core::from_raw_borrowed(&geometry), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    geometryrealization.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokedGeometryRealization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, flatteningtolerance: f32, strokewidth: f32, strokestyle: *mut core::ffi::c_void, geometryrealization: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext1_Impl::CreateStrokedGeometryRealization(this, windows_core::from_raw_borrowed(&geometry), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle)) {
                Ok(ok__) => {
                    geometryrealization.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawGeometryRealization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryrealization: *mut core::ffi::c_void, brush: *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext1_Impl::DrawGeometryRealization(this, windows_core::from_raw_borrowed(&geometryrealization), windows_core::from_raw_borrowed(&brush))
        }
        Self {
            base__: ID2D1DeviceContext_Vtbl::new::<Identity, OFFSET>(),
            CreateFilledGeometryRealization: CreateFilledGeometryRealization::<Identity, OFFSET>,
            CreateStrokedGeometryRealization: CreateStrokedGeometryRealization::<Identity, OFFSET>,
            DrawGeometryRealization: DrawGeometryRealization::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext2_Impl: Sized + ID2D1DeviceContext1_Impl {
    fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> windows_core::Result<ID2D1Ink>;
    fn CreateInkStyle(&self, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES) -> windows_core::Result<ID2D1InkStyle>;
    fn CreateGradientMesh(&self, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> windows_core::Result<ID2D1GradientMesh>;
    fn CreateImageSourceFromWic(&self, wicbitmapsource: Option<&super::Imaging::IWICBitmapSource>, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> windows_core::Result<ID2D1ImageSourceFromWic>;
    fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32) -> windows_core::Result<ID2D1LookupTable3D>;
    fn CreateImageSourceFromDxgi(&self, surfaces: *const Option<super::Dxgi::IDXGISurface>, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> windows_core::Result<ID2D1ImageSource>;
    fn GetGradientMeshWorldBounds(&self, gradientmesh: Option<&ID2D1GradientMesh>) -> windows_core::Result<Common::D2D_RECT_F>;
    fn DrawInk(&self, ink: Option<&ID2D1Ink>, brush: Option<&ID2D1Brush>, inkstyle: Option<&ID2D1InkStyle>);
    fn DrawGradientMesh(&self, gradientmesh: Option<&ID2D1GradientMesh>);
    fn DrawGdiMetafile(&self, gdimetafile: Option<&ID2D1GdiMetafile>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F);
    fn CreateTransformedImageSource(&self, imagesource: Option<&ID2D1ImageSource>, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> windows_core::Result<ID2D1TransformedImageSource>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1DeviceContext2 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext2_Vtbl
    where
        Identity: ID2D1DeviceContext2_Impl,
    {
        unsafe extern "system" fn CreateInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const D2D1_INK_POINT, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateInk(this, core::mem::transmute_copy(&startpoint)) {
                Ok(ok__) => {
                    ink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInkStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateInkStyle(this, core::mem::transmute_copy(&inkstyleproperties)) {
                Ok(ok__) => {
                    inkstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientMesh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateGradientMesh(this, core::mem::transmute_copy(&patches), core::mem::transmute_copy(&patchescount)) {
                Ok(ok__) => {
                    gradientmesh.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageSourceFromWic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut core::ffi::c_void, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE, imagesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateImageSourceFromWic(this, windows_core::from_raw_borrowed(&wicbitmapsource), core::mem::transmute_copy(&loadingoptions), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    imagesource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLookupTable3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateLookupTable3D(this, core::mem::transmute_copy(&precision), core::mem::transmute_copy(&extents), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&strides)) {
                Ok(ok__) => {
                    lookuptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateImageSourceFromDxgi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, surfaces: *const *mut core::ffi::c_void, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateImageSourceFromDxgi(this, core::mem::transmute_copy(&surfaces), core::mem::transmute_copy(&surfacecount), core::mem::transmute_copy(&colorspace), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    imagesource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGradientMeshWorldBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientmesh: *mut core::ffi::c_void, pbounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::GetGradientMeshWorldBounds(this, windows_core::from_raw_borrowed(&gradientmesh)) {
                Ok(ok__) => {
                    pbounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawInk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext2_Impl::DrawInk(this, windows_core::from_raw_borrowed(&ink), windows_core::from_raw_borrowed(&brush), windows_core::from_raw_borrowed(&inkstyle))
        }
        unsafe extern "system" fn DrawGradientMesh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientmesh: *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext2_Impl::DrawGradientMesh(this, windows_core::from_raw_borrowed(&gradientmesh))
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F)
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext2_Impl::DrawGdiMetafile(this, windows_core::from_raw_borrowed(&gdimetafile), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle))
        }
        unsafe extern "system" fn CreateTransformedImageSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagesource: *mut core::ffi::c_void, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext2_Impl::CreateTransformedImageSource(this, windows_core::from_raw_borrowed(&imagesource), core::mem::transmute_copy(&properties)) {
                Ok(ok__) => {
                    transformedimagesource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1DeviceContext1_Vtbl::new::<Identity, OFFSET>(),
            CreateInk: CreateInk::<Identity, OFFSET>,
            CreateInkStyle: CreateInkStyle::<Identity, OFFSET>,
            CreateGradientMesh: CreateGradientMesh::<Identity, OFFSET>,
            CreateImageSourceFromWic: CreateImageSourceFromWic::<Identity, OFFSET>,
            CreateLookupTable3D: CreateLookupTable3D::<Identity, OFFSET>,
            CreateImageSourceFromDxgi: CreateImageSourceFromDxgi::<Identity, OFFSET>,
            GetGradientMeshWorldBounds: GetGradientMeshWorldBounds::<Identity, OFFSET>,
            DrawInk: DrawInk::<Identity, OFFSET>,
            DrawGradientMesh: DrawGradientMesh::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
            CreateTransformedImageSource: CreateTransformedImageSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext3_Impl: Sized + ID2D1DeviceContext2_Impl {
    fn CreateSpriteBatch(&self) -> windows_core::Result<ID2D1SpriteBatch>;
    fn DrawSpriteBatch(&self, spritebatch: Option<&ID2D1SpriteBatch>, startindex: u32, spritecount: u32, bitmap: Option<&ID2D1Bitmap>, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1DeviceContext3 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext3_Vtbl
    where
        Identity: ID2D1DeviceContext3_Impl,
    {
        unsafe extern "system" fn CreateSpriteBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritebatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext3_Impl::CreateSpriteBatch(this) {
                Ok(ok__) => {
                    spritebatch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawSpriteBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritebatch: *mut core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
        where
            Identity: ID2D1DeviceContext3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext3_Impl::DrawSpriteBatch(this, windows_core::from_raw_borrowed(&spritebatch), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&spriteoptions))
        }
        Self {
            base__: ID2D1DeviceContext2_Vtbl::new::<Identity, OFFSET>(),
            CreateSpriteBatch: CreateSpriteBatch::<Identity, OFFSET>,
            DrawSpriteBatch: DrawSpriteBatch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext4_Impl: Sized + ID2D1DeviceContext3_Impl {
    fn CreateSvgGlyphStyle(&self) -> windows_core::Result<ID2D1SvgGlyphStyle>;
    fn DrawText(&self, string: &windows_core::PCWSTR, stringlength: u32, textformat: Option<&super::DirectWrite::IDWriteTextFormat>, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: Option<&ID2D1Brush>, svgglyphstyle: Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawTextLayout(&self, origin: &Common::D2D_POINT_2F, textlayout: Option<&super::DirectWrite::IDWriteTextLayout>, defaultfillbrush: Option<&ID2D1Brush>, svgglyphstyle: Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS);
    fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION);
    fn DrawSvgGlyphRun(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: Option<&ID2D1Brush>, svgglyphstyle: Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn GetColorBitmapGlyphImage(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: &Common::D2D_POINT_2F, fontface: Option<&super::DirectWrite::IDWriteFontFace>, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut Option<ID2D1Image>) -> windows_core::Result<()>;
    fn GetSvgGlyphImage(&self, glyphorigin: &Common::D2D_POINT_2F, fontface: Option<&super::DirectWrite::IDWriteFontFace>, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: Option<&ID2D1Brush>, svgglyphstyle: Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut Option<ID2D1CommandList>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1DeviceContext4 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1DeviceContext4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext4_Vtbl
    where
        Identity: ID2D1DeviceContext4_Impl,
    {
        unsafe extern "system" fn CreateSvgGlyphStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, svgglyphstyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext4_Impl::CreateSvgGlyphStyle(this) {
                Ok(ok__) => {
                    svgglyphstyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: windows_core::PCWSTR, stringlength: u32, textformat: *mut core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext4_Impl::DrawText(this, core::mem::transmute(&string), core::mem::transmute_copy(&stringlength), windows_core::from_raw_borrowed(&textformat), core::mem::transmute_copy(&layoutrect), windows_core::from_raw_borrowed(&defaultfillbrush), windows_core::from_raw_borrowed(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&options), core::mem::transmute_copy(&measuringmode))
        }
        unsafe extern "system" fn DrawTextLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: *mut core::ffi::c_void, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS)
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext4_Impl::DrawTextLayout(this, core::mem::transmute(&origin), windows_core::from_raw_borrowed(&textlayout), windows_core::from_raw_borrowed(&defaultfillbrush), windows_core::from_raw_borrowed(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&options))
        }
        unsafe extern "system" fn DrawColorBitmapGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION)
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext4_Impl::DrawColorBitmapGlyphRun(this, core::mem::transmute_copy(&glyphimageformat), core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&bitmapsnapoption))
        }
        unsafe extern "system" fn DrawSvgGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext4_Impl::DrawSvgGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), windows_core::from_raw_borrowed(&defaultfillbrush), windows_core::from_raw_borrowed(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&measuringmode))
        }
        unsafe extern "system" fn GetColorBitmapGlyphImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: *mut core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext4_Impl::GetColorBitmapGlyphImage(this, core::mem::transmute_copy(&glyphimageformat), core::mem::transmute(&glyphorigin), windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&glyphindex), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&glyphtransform), core::mem::transmute_copy(&glyphimage)).into()
        }
        unsafe extern "system" fn GetSvgGlyphImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorigin: Common::D2D_POINT_2F, fontface: *mut core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext4_Impl::GetSvgGlyphImage(this, core::mem::transmute(&glyphorigin), windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&glyphindex), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&worldtransform), windows_core::from_raw_borrowed(&defaultfillbrush), windows_core::from_raw_borrowed(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&glyphtransform), core::mem::transmute_copy(&glyphimage)).into()
        }
        Self {
            base__: ID2D1DeviceContext3_Vtbl::new::<Identity, OFFSET>(),
            CreateSvgGlyphStyle: CreateSvgGlyphStyle::<Identity, OFFSET>,
            DrawText: DrawText::<Identity, OFFSET>,
            DrawTextLayout: DrawTextLayout::<Identity, OFFSET>,
            DrawColorBitmapGlyphRun: DrawColorBitmapGlyphRun::<Identity, OFFSET>,
            DrawSvgGlyphRun: DrawSvgGlyphRun::<Identity, OFFSET>,
            GetColorBitmapGlyphImage: GetColorBitmapGlyphImage::<Identity, OFFSET>,
            GetSvgGlyphImage: GetSvgGlyphImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext5_Impl: Sized + ID2D1DeviceContext4_Impl {
    fn CreateSvgDocument(&self, inputxmlstream: Option<&super::super::System::Com::IStream>, viewportsize: &Common::D2D_SIZE_F) -> windows_core::Result<ID2D1SvgDocument>;
    fn DrawSvgDocument(&self, svgdocument: Option<&ID2D1SvgDocument>);
    fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<ID2D1ColorContext1>;
    fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<ID2D1ColorContext1>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1DeviceContext5 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1DeviceContext5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext5_Vtbl
    where
        Identity: ID2D1DeviceContext5_Impl,
    {
        unsafe extern "system" fn CreateSvgDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputxmlstream: *mut core::ffi::c_void, viewportsize: Common::D2D_SIZE_F, svgdocument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext5_Impl::CreateSvgDocument(this, windows_core::from_raw_borrowed(&inputxmlstream), core::mem::transmute(&viewportsize)) {
                Ok(ok__) => {
                    svgdocument.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawSvgDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, svgdocument: *mut core::ffi::c_void)
        where
            Identity: ID2D1DeviceContext5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext5_Impl::DrawSvgDocument(this, windows_core::from_raw_borrowed(&svgdocument))
        }
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext5_Impl::CreateColorContextFromDxgiColorSpace(this, core::mem::transmute_copy(&colorspace)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DeviceContext5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1DeviceContext5_Impl::CreateColorContextFromSimpleColorProfile(this, core::mem::transmute_copy(&simpleprofile)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1DeviceContext4_Vtbl::new::<Identity, OFFSET>(),
            CreateSvgDocument: CreateSvgDocument::<Identity, OFFSET>,
            DrawSvgDocument: DrawSvgDocument::<Identity, OFFSET>,
            CreateColorContextFromDxgiColorSpace: CreateColorContextFromDxgiColorSpace::<Identity, OFFSET>,
            CreateColorContextFromSimpleColorProfile: CreateColorContextFromSimpleColorProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext5 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext6_Impl: Sized + ID2D1DeviceContext5_Impl {
    fn BlendImage(&self, image: Option<&ID2D1Image>, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1DeviceContext6 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1DeviceContext6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext6_Vtbl
    where
        Identity: ID2D1DeviceContext6_Impl,
    {
        unsafe extern "system" fn BlendImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE)
        where
            Identity: ID2D1DeviceContext6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext6_Impl::BlendImage(this, windows_core::from_raw_borrowed(&image), core::mem::transmute_copy(&blendmode), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode))
        }
        Self { base__: ID2D1DeviceContext5_Vtbl::new::<Identity, OFFSET>(), BlendImage: BlendImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext6 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext7_Impl: Sized + ID2D1DeviceContext6_Impl {
    fn GetPaintFeatureLevel(&self) -> super::DirectWrite::DWRITE_PAINT_FEATURE_LEVEL;
    fn DrawPaintGlyphRun(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: Option<&ID2D1Brush>, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawGlyphRunWithColorSupport(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: Option<&ID2D1Brush>, svgglyphstyle: Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1DeviceContext7 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1DeviceContext7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DeviceContext7_Vtbl
    where
        Identity: ID2D1DeviceContext7_Impl,
    {
        unsafe extern "system" fn GetPaintFeatureLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DirectWrite::DWRITE_PAINT_FEATURE_LEVEL
        where
            Identity: ID2D1DeviceContext7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext7_Impl::GetPaintFeatureLevel(this)
        }
        unsafe extern "system" fn DrawPaintGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
        where
            Identity: ID2D1DeviceContext7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext7_Impl::DrawPaintGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), windows_core::from_raw_borrowed(&defaultfillbrush), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&measuringmode))
        }
        unsafe extern "system" fn DrawGlyphRunWithColorSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION)
        where
            Identity: ID2D1DeviceContext7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DeviceContext7_Impl::DrawGlyphRunWithColorSupport(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), windows_core::from_raw_borrowed(&foregroundbrush), windows_core::from_raw_borrowed(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&bitmapsnapoption))
        }
        Self {
            base__: ID2D1DeviceContext6_Vtbl::new::<Identity, OFFSET>(),
            GetPaintFeatureLevel: GetPaintFeatureLevel::<Identity, OFFSET>,
            DrawPaintGlyphRun: DrawPaintGlyphRun::<Identity, OFFSET>,
            DrawGlyphRunWithColorSupport: DrawGlyphRunWithColorSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext7 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext5 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext6 as windows_core::Interface>::IID
    }
}
pub trait ID2D1DrawInfo_Impl: Sized + ID2D1RenderInfo_Impl {
    fn SetPixelShaderConstantBuffer(&self, buffer: *const u8, buffercount: u32) -> windows_core::Result<()>;
    fn SetResourceTexture(&self, textureindex: u32, resourcetexture: Option<&ID2D1ResourceTexture>) -> windows_core::Result<()>;
    fn SetVertexShaderConstantBuffer(&self, buffer: *const u8, buffercount: u32) -> windows_core::Result<()>;
    fn SetPixelShader(&self, shaderid: *const windows_core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> windows_core::Result<()>;
    fn SetVertexProcessing(&self, vertexbuffer: Option<&ID2D1VertexBuffer>, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1DrawInfo {}
impl ID2D1DrawInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DrawInfo_Vtbl
    where
        Identity: ID2D1DrawInfo_Impl,
    {
        unsafe extern "system" fn SetPixelShaderConstantBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *const u8, buffercount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1DrawInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawInfo_Impl::SetPixelShaderConstantBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffercount)).into()
        }
        unsafe extern "system" fn SetResourceTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textureindex: u32, resourcetexture: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DrawInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawInfo_Impl::SetResourceTexture(this, core::mem::transmute_copy(&textureindex), windows_core::from_raw_borrowed(&resourcetexture)).into()
        }
        unsafe extern "system" fn SetVertexShaderConstantBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *const u8, buffercount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1DrawInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawInfo_Impl::SetVertexShaderConstantBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffercount)).into()
        }
        unsafe extern "system" fn SetPixelShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shaderid: *const windows_core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> windows_core::HRESULT
        where
            Identity: ID2D1DrawInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawInfo_Impl::SetPixelShader(this, core::mem::transmute_copy(&shaderid), core::mem::transmute_copy(&pixeloptions)).into()
        }
        unsafe extern "system" fn SetVertexProcessing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexbuffer: *mut core::ffi::c_void, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ID2D1DrawInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawInfo_Impl::SetVertexProcessing(this, windows_core::from_raw_borrowed(&vertexbuffer), core::mem::transmute_copy(&vertexoptions), core::mem::transmute_copy(&blenddescription), core::mem::transmute_copy(&vertexrange), core::mem::transmute_copy(&vertexshader)).into()
        }
        Self {
            base__: ID2D1RenderInfo_Vtbl::new::<Identity, OFFSET>(),
            SetPixelShaderConstantBuffer: SetPixelShaderConstantBuffer::<Identity, OFFSET>,
            SetResourceTexture: SetResourceTexture::<Identity, OFFSET>,
            SetVertexShaderConstantBuffer: SetVertexShaderConstantBuffer::<Identity, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, OFFSET>,
            SetVertexProcessing: SetVertexProcessing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DrawInfo as windows_core::Interface>::IID || iid == &<ID2D1RenderInfo as windows_core::Interface>::IID
    }
}
pub trait ID2D1DrawTransform_Impl: Sized + ID2D1Transform_Impl {
    fn SetDrawInfo(&self, drawinfo: Option<&ID2D1DrawInfo>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1DrawTransform {}
impl ID2D1DrawTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DrawTransform_Vtbl
    where
        Identity: ID2D1DrawTransform_Impl,
    {
        unsafe extern "system" fn SetDrawInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1DrawTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawTransform_Impl::SetDrawInfo(this, windows_core::from_raw_borrowed(&drawinfo)).into()
        }
        Self { base__: ID2D1Transform_Vtbl::new::<Identity, OFFSET>(), SetDrawInfo: SetDrawInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DrawTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID || iid == &<ID2D1Transform as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1DrawingStateBlock_Impl: Sized + ID2D1Resource_Impl {
    fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION);
    fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION);
    fn SetTextRenderingParams(&self, textrenderingparams: Option<&super::DirectWrite::IDWriteRenderingParams>);
    fn GetTextRenderingParams(&self, textrenderingparams: *mut Option<super::DirectWrite::IDWriteRenderingParams>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1DrawingStateBlock {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1DrawingStateBlock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DrawingStateBlock_Vtbl
    where
        Identity: ID2D1DrawingStateBlock_Impl,
    {
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION)
        where
            Identity: ID2D1DrawingStateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawingStateBlock_Impl::GetDescription(this, core::mem::transmute_copy(&statedescription))
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION)
        where
            Identity: ID2D1DrawingStateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawingStateBlock_Impl::SetDescription(this, core::mem::transmute_copy(&statedescription))
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut core::ffi::c_void)
        where
            Identity: ID2D1DrawingStateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawingStateBlock_Impl::SetTextRenderingParams(this, windows_core::from_raw_borrowed(&textrenderingparams))
        }
        unsafe extern "system" fn GetTextRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1DrawingStateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawingStateBlock_Impl::GetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams))
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DrawingStateBlock as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1DrawingStateBlock1_Impl: Sized + ID2D1DrawingStateBlock_Impl {
    fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1);
    fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl windows_core::RuntimeName for ID2D1DrawingStateBlock1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl ID2D1DrawingStateBlock1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1DrawingStateBlock1_Vtbl
    where
        Identity: ID2D1DrawingStateBlock1_Impl,
    {
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1)
        where
            Identity: ID2D1DrawingStateBlock1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawingStateBlock1_Impl::GetDescription(this, core::mem::transmute_copy(&statedescription))
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1)
        where
            Identity: ID2D1DrawingStateBlock1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1DrawingStateBlock1_Impl::SetDescription(this, core::mem::transmute_copy(&statedescription))
        }
        Self {
            base__: ID2D1DrawingStateBlock_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DrawingStateBlock1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1DrawingStateBlock as windows_core::Interface>::IID
    }
}
pub trait ID2D1Effect_Impl: Sized + ID2D1Properties_Impl {
    fn SetInput(&self, index: u32, input: Option<&ID2D1Image>, invalidate: super::super::Foundation::BOOL);
    fn SetInputCount(&self, inputcount: u32) -> windows_core::Result<()>;
    fn GetInput(&self, index: u32, input: *mut Option<ID2D1Image>);
    fn GetInputCount(&self) -> u32;
    fn GetOutput(&self, outputimage: *mut Option<ID2D1Image>);
}
impl windows_core::RuntimeName for ID2D1Effect {}
impl ID2D1Effect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Effect_Vtbl
    where
        Identity: ID2D1Effect_Impl,
    {
        unsafe extern "system" fn SetInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, input: *mut core::ffi::c_void, invalidate: super::super::Foundation::BOOL)
        where
            Identity: ID2D1Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Effect_Impl::SetInput(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&input), core::mem::transmute_copy(&invalidate))
        }
        unsafe extern "system" fn SetInputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputcount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Effect_Impl::SetInputCount(this, core::mem::transmute_copy(&inputcount)).into()
        }
        unsafe extern "system" fn GetInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, input: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Effect_Impl::GetInput(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&input))
        }
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Effect_Impl::GetInputCount(this)
        }
        unsafe extern "system" fn GetOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputimage: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Effect_Impl::GetOutput(this, core::mem::transmute_copy(&outputimage))
        }
        Self {
            base__: ID2D1Properties_Vtbl::new::<Identity, OFFSET>(),
            SetInput: SetInput::<Identity, OFFSET>,
            SetInputCount: SetInputCount::<Identity, OFFSET>,
            GetInput: GetInput::<Identity, OFFSET>,
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetOutput: GetOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Effect as windows_core::Interface>::IID || iid == &<ID2D1Properties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext_Impl: Sized {
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CreateEffect(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Effect>;
    fn GetMaximumSupportedFeatureLevel(&self, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32) -> windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn CreateTransformNodeFromEffect(&self, effect: Option<&ID2D1Effect>) -> windows_core::Result<ID2D1TransformNode>;
    fn CreateBlendTransform(&self, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION) -> windows_core::Result<ID2D1BlendTransform>;
    fn CreateBorderTransform(&self, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE) -> windows_core::Result<ID2D1BorderTransform>;
    fn CreateOffsetTransform(&self, offset: &super::super::Foundation::POINT) -> windows_core::Result<ID2D1OffsetTransform>;
    fn CreateBoundsAdjustmentTransform(&self, outputrectangle: *const super::super::Foundation::RECT) -> windows_core::Result<ID2D1BoundsAdjustmentTransform>;
    fn LoadPixelShader(&self, shaderid: *const windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> windows_core::Result<()>;
    fn LoadVertexShader(&self, resourceid: *const windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> windows_core::Result<()>;
    fn LoadComputeShader(&self, resourceid: *const windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> windows_core::Result<()>;
    fn IsShaderLoaded(&self, shaderid: *const windows_core::GUID) -> super::super::Foundation::BOOL;
    fn CreateResourceTexture(&self, resourceid: *const windows_core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32) -> windows_core::Result<ID2D1ResourceTexture>;
    fn FindResourceTexture(&self, resourceid: *const windows_core::GUID) -> windows_core::Result<ID2D1ResourceTexture>;
    fn CreateVertexBuffer(&self, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const windows_core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES) -> windows_core::Result<ID2D1VertexBuffer>;
    fn FindVertexBuffer(&self, resourceid: *const windows_core::GUID) -> windows_core::Result<ID2D1VertexBuffer>;
    fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromFilename(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromWicColorContext(&self, wiccolorcontext: Option<&super::Imaging::IWICColorContext>) -> windows_core::Result<ID2D1ColorContext>;
    fn CheckFeatureSupport(&self, feature: D2D1_FEATURE, featuresupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1EffectContext {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1EffectContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1EffectContext_Vtbl
    where
        Identity: ID2D1EffectContext_Impl,
    {
        unsafe extern "system" fn GetDpi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32)
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy))
        }
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: *const windows_core::GUID, effect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateEffect(this, core::mem::transmute_copy(&effectid)) {
                Ok(ok__) => {
                    effect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumSupportedFeatureLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32, maximumsupportedfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::GetMaximumSupportedFeatureLevel(this, core::mem::transmute_copy(&featurelevels), core::mem::transmute_copy(&featurelevelscount)) {
                Ok(ok__) => {
                    maximumsupportedfeaturelevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformNodeFromEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void, transformnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateTransformNodeFromEffect(this, windows_core::from_raw_borrowed(&effect)) {
                Ok(ok__) => {
                    transformnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateBlendTransform(this, core::mem::transmute_copy(&numinputs), core::mem::transmute_copy(&blenddescription)) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBorderTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateBorderTransform(this, core::mem::transmute_copy(&extendmodex), core::mem::transmute_copy(&extendmodey)) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOffsetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: super::super::Foundation::POINT, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateOffsetTransform(this, core::mem::transmute(&offset)) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBoundsAdjustmentTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputrectangle: *const super::super::Foundation::RECT, transform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateBoundsAdjustmentTransform(this, core::mem::transmute_copy(&outputrectangle)) {
                Ok(ok__) => {
                    transform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadPixelShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shaderid: *const windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::LoadPixelShader(this, core::mem::transmute_copy(&shaderid), core::mem::transmute_copy(&shaderbuffer), core::mem::transmute_copy(&shaderbuffercount)).into()
        }
        unsafe extern "system" fn LoadVertexShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *const windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::LoadVertexShader(this, core::mem::transmute_copy(&resourceid), core::mem::transmute_copy(&shaderbuffer), core::mem::transmute_copy(&shaderbuffercount)).into()
        }
        unsafe extern "system" fn LoadComputeShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *const windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::LoadComputeShader(this, core::mem::transmute_copy(&resourceid), core::mem::transmute_copy(&shaderbuffer), core::mem::transmute_copy(&shaderbuffercount)).into()
        }
        unsafe extern "system" fn IsShaderLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shaderid: *const windows_core::GUID) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::IsShaderLoaded(this, core::mem::transmute_copy(&shaderid))
        }
        unsafe extern "system" fn CreateResourceTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *const windows_core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32, resourcetexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateResourceTexture(this, core::mem::transmute_copy(&resourceid), core::mem::transmute_copy(&resourcetextureproperties), core::mem::transmute_copy(&data), core::mem::transmute_copy(&strides), core::mem::transmute_copy(&datasize)) {
                Ok(ok__) => {
                    resourcetexture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindResourceTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *const windows_core::GUID, resourcetexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::FindResourceTexture(this, core::mem::transmute_copy(&resourceid)) {
                Ok(ok__) => {
                    resourcetexture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const windows_core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, buffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateVertexBuffer(this, core::mem::transmute_copy(&vertexbufferproperties), core::mem::transmute_copy(&resourceid), core::mem::transmute_copy(&customvertexbufferproperties)) {
                Ok(ok__) => {
                    buffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindVertexBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *const windows_core::GUID, buffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::FindVertexBuffer(this, core::mem::transmute_copy(&resourceid)) {
                Ok(ok__) => {
                    buffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateColorContext(this, core::mem::transmute_copy(&space), core::mem::transmute_copy(&profile), core::mem::transmute_copy(&profilesize)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateColorContextFromFilename(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wiccolorcontext: *mut core::ffi::c_void, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext_Impl::CreateColorContextFromWicColorContext(this, windows_core::from_raw_borrowed(&wiccolorcontext)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: D2D1_FEATURE, featuresupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&featuresupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1EffectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectContext_Impl::IsBufferPrecisionSupported(this, core::mem::transmute_copy(&bufferprecision))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDpi: GetDpi::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            GetMaximumSupportedFeatureLevel: GetMaximumSupportedFeatureLevel::<Identity, OFFSET>,
            CreateTransformNodeFromEffect: CreateTransformNodeFromEffect::<Identity, OFFSET>,
            CreateBlendTransform: CreateBlendTransform::<Identity, OFFSET>,
            CreateBorderTransform: CreateBorderTransform::<Identity, OFFSET>,
            CreateOffsetTransform: CreateOffsetTransform::<Identity, OFFSET>,
            CreateBoundsAdjustmentTransform: CreateBoundsAdjustmentTransform::<Identity, OFFSET>,
            LoadPixelShader: LoadPixelShader::<Identity, OFFSET>,
            LoadVertexShader: LoadVertexShader::<Identity, OFFSET>,
            LoadComputeShader: LoadComputeShader::<Identity, OFFSET>,
            IsShaderLoaded: IsShaderLoaded::<Identity, OFFSET>,
            CreateResourceTexture: CreateResourceTexture::<Identity, OFFSET>,
            FindResourceTexture: FindResourceTexture::<Identity, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, OFFSET>,
            FindVertexBuffer: FindVertexBuffer::<Identity, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, OFFSET>,
            CreateColorContextFromFilename: CreateColorContextFromFilename::<Identity, OFFSET>,
            CreateColorContextFromWicColorContext: CreateColorContextFromWicColorContext::<Identity, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            IsBufferPrecisionSupported: IsBufferPrecisionSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1EffectContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext1_Impl: Sized + ID2D1EffectContext_Impl {
    fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32) -> windows_core::Result<ID2D1LookupTable3D>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1EffectContext1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1EffectContext1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1EffectContext1_Vtbl
    where
        Identity: ID2D1EffectContext1_Impl,
    {
        unsafe extern "system" fn CreateLookupTable3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext1_Impl::CreateLookupTable3D(this, core::mem::transmute_copy(&precision), core::mem::transmute_copy(&extents), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&strides)) {
                Ok(ok__) => {
                    lookuptable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1EffectContext_Vtbl::new::<Identity, OFFSET>(), CreateLookupTable3D: CreateLookupTable3D::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1EffectContext1 as windows_core::Interface>::IID || iid == &<ID2D1EffectContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext2_Impl: Sized + ID2D1EffectContext1_Impl {
    fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<ID2D1ColorContext1>;
    fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<ID2D1ColorContext1>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1EffectContext2 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1EffectContext2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1EffectContext2_Vtbl
    where
        Identity: ID2D1EffectContext2_Impl,
    {
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext2_Impl::CreateColorContextFromDxgiColorSpace(this, core::mem::transmute_copy(&colorspace)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectContext2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1EffectContext2_Impl::CreateColorContextFromSimpleColorProfile(this, core::mem::transmute_copy(&simpleprofile)) {
                Ok(ok__) => {
                    colorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1EffectContext1_Vtbl::new::<Identity, OFFSET>(),
            CreateColorContextFromDxgiColorSpace: CreateColorContextFromDxgiColorSpace::<Identity, OFFSET>,
            CreateColorContextFromSimpleColorProfile: CreateColorContextFromSimpleColorProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1EffectContext2 as windows_core::Interface>::IID || iid == &<ID2D1EffectContext as windows_core::Interface>::IID || iid == &<ID2D1EffectContext1 as windows_core::Interface>::IID
    }
}
pub trait ID2D1EffectImpl_Impl: Sized {
    fn Initialize(&self, effectcontext: Option<&ID2D1EffectContext>, transformgraph: Option<&ID2D1TransformGraph>) -> windows_core::Result<()>;
    fn PrepareForRender(&self, changetype: D2D1_CHANGE_TYPE) -> windows_core::Result<()>;
    fn SetGraph(&self, transformgraph: Option<&ID2D1TransformGraph>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1EffectImpl {}
impl ID2D1EffectImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1EffectImpl_Vtbl
    where
        Identity: ID2D1EffectImpl_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectcontext: *mut core::ffi::c_void, transformgraph: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectImpl_Impl::Initialize(this, windows_core::from_raw_borrowed(&effectcontext), windows_core::from_raw_borrowed(&transformgraph)).into()
        }
        unsafe extern "system" fn PrepareForRender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: D2D1_CHANGE_TYPE) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectImpl_Impl::PrepareForRender(this, core::mem::transmute_copy(&changetype)).into()
        }
        unsafe extern "system" fn SetGraph<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformgraph: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1EffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EffectImpl_Impl::SetGraph(this, windows_core::from_raw_borrowed(&transformgraph)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            PrepareForRender: PrepareForRender::<Identity, OFFSET>,
            SetGraph: SetGraph::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1EffectImpl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1EllipseGeometry_Impl: Sized + ID2D1Geometry_Impl {
    fn GetEllipse(&self, ellipse: *mut D2D1_ELLIPSE);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1EllipseGeometry {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1EllipseGeometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1EllipseGeometry_Vtbl
    where
        Identity: ID2D1EllipseGeometry_Impl,
    {
        unsafe extern "system" fn GetEllipse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *mut D2D1_ELLIPSE)
        where
            Identity: ID2D1EllipseGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1EllipseGeometry_Impl::GetEllipse(this, core::mem::transmute_copy(&ellipse))
        }
        Self { base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(), GetEllipse: GetEllipse::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1EllipseGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1Factory_Impl: Sized {
    fn ReloadSystemMetrics(&self) -> windows_core::Result<()>;
    fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> windows_core::Result<ID2D1RectangleGeometry>;
    fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> windows_core::Result<ID2D1RoundedRectangleGeometry>;
    fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> windows_core::Result<ID2D1EllipseGeometry>;
    fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: *const Option<ID2D1Geometry>, geometriescount: u32) -> windows_core::Result<ID2D1GeometryGroup>;
    fn CreateTransformedGeometry(&self, sourcegeometry: Option<&ID2D1Geometry>, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<ID2D1TransformedGeometry>;
    fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry>;
    fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32) -> windows_core::Result<ID2D1StrokeStyle>;
    fn CreateDrawingStateBlock(&self, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: Option<&super::DirectWrite::IDWriteRenderingParams>) -> windows_core::Result<ID2D1DrawingStateBlock>;
    fn CreateWicBitmapRenderTarget(&self, target: Option<&super::Imaging::IWICBitmap>, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1HwndRenderTarget>;
    fn CreateDxgiSurfaceRenderTarget(&self, dxgisurface: Option<&super::Dxgi::IDXGISurface>, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1DCRenderTarget>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1Factory {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1Factory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory_Vtbl
    where
        Identity: ID2D1Factory_Impl,
    {
        unsafe extern "system" fn ReloadSystemMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Factory_Impl::ReloadSystemMetrics(this).into()
        }
        unsafe extern "system" fn GetDesktopDpi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32)
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Factory_Impl::GetDesktopDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy))
        }
        unsafe extern "system" fn CreateRectangleGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangle: *const Common::D2D_RECT_F, rectanglegeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateRectangleGeometry(this, core::mem::transmute_copy(&rectangle)) {
                Ok(ok__) => {
                    rectanglegeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateRoundedRectangleGeometry(this, core::mem::transmute_copy(&roundedrectangle)) {
                Ok(ok__) => {
                    roundedrectanglegeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEllipseGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateEllipseGeometry(this, core::mem::transmute_copy(&ellipse)) {
                Ok(ok__) => {
                    ellipsegeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeometryGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, geometries: *const *mut core::ffi::c_void, geometriescount: u32, geometrygroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateGeometryGroup(this, core::mem::transmute_copy(&fillmode), core::mem::transmute_copy(&geometries), core::mem::transmute_copy(&geometriescount)) {
                Ok(ok__) => {
                    geometrygroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformedGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcegeometry: *mut core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2, transformedgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateTransformedGeometry(this, windows_core::from_raw_borrowed(&sourcegeometry), core::mem::transmute_copy(&transform)) {
                Ok(ok__) => {
                    transformedgeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreatePathGeometry(this) {
                Ok(ok__) => {
                    pathgeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateStrokeStyle(this, core::mem::transmute_copy(&strokestyleproperties), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount)) {
                Ok(ok__) => {
                    strokestyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: *mut core::ffi::c_void, drawingstateblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateDrawingStateBlock(this, core::mem::transmute_copy(&drawingstatedescription), windows_core::from_raw_borrowed(&textrenderingparams)) {
                Ok(ok__) => {
                    drawingstateblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateWicBitmapRenderTarget(this, windows_core::from_raw_borrowed(&target), core::mem::transmute_copy(&rendertargetproperties)) {
                Ok(ok__) => {
                    rendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHwndRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateHwndRenderTarget(this, core::mem::transmute_copy(&rendertargetproperties), core::mem::transmute_copy(&hwndrendertargetproperties)) {
                Ok(ok__) => {
                    hwndrendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgisurface: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateDxgiSurfaceRenderTarget(this, windows_core::from_raw_borrowed(&dxgisurface), core::mem::transmute_copy(&rendertargetproperties)) {
                Ok(ok__) => {
                    rendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDCRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory_Impl::CreateDCRenderTarget(this, core::mem::transmute_copy(&rendertargetproperties)) {
                Ok(ok__) => {
                    dcrendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReloadSystemMetrics: ReloadSystemMetrics::<Identity, OFFSET>,
            GetDesktopDpi: GetDesktopDpi::<Identity, OFFSET>,
            CreateRectangleGeometry: CreateRectangleGeometry::<Identity, OFFSET>,
            CreateRoundedRectangleGeometry: CreateRoundedRectangleGeometry::<Identity, OFFSET>,
            CreateEllipseGeometry: CreateEllipseGeometry::<Identity, OFFSET>,
            CreateGeometryGroup: CreateGeometryGroup::<Identity, OFFSET>,
            CreateTransformedGeometry: CreateTransformedGeometry::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, OFFSET>,
            CreateWicBitmapRenderTarget: CreateWicBitmapRenderTarget::<Identity, OFFSET>,
            CreateHwndRenderTarget: CreateHwndRenderTarget::<Identity, OFFSET>,
            CreateDxgiSurfaceRenderTarget: CreateDxgiSurfaceRenderTarget::<Identity, OFFSET>,
            CreateDCRenderTarget: CreateDCRenderTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory1_Impl: Sized + ID2D1Factory_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device>;
    fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32) -> windows_core::Result<ID2D1StrokeStyle1>;
    fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry1>;
    fn CreateDrawingStateBlock(&self, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: Option<&super::DirectWrite::IDWriteRenderingParams>) -> windows_core::Result<ID2D1DrawingStateBlock1>;
    fn CreateGdiMetafile(&self, metafilestream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<ID2D1GdiMetafile>;
    fn RegisterEffectFromStream(&self, classid: *const windows_core::GUID, propertyxml: Option<&super::super::System::Com::IStream>, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::Result<()>;
    fn RegisterEffectFromString(&self, classid: *const windows_core::GUID, propertyxml: &windows_core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::Result<()>;
    fn UnregisterEffect(&self, classid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetRegisteredEffects(&self, effects: *mut windows_core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> windows_core::Result<()>;
    fn GetEffectProperties(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Properties>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory1_Vtbl
    where
        Identity: ID2D1Factory1_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory1_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory1_Impl::CreateStrokeStyle(this, core::mem::transmute_copy(&strokestyleproperties), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount)) {
                Ok(ok__) => {
                    strokestyle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pathgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory1_Impl::CreatePathGeometry(this) {
                Ok(ok__) => {
                    pathgeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: *mut core::ffi::c_void, drawingstateblock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory1_Impl::CreateDrawingStateBlock(this, core::mem::transmute_copy(&drawingstatedescription), windows_core::from_raw_borrowed(&textrenderingparams)) {
                Ok(ok__) => {
                    drawingstateblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGdiMetafile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metafilestream: *mut core::ffi::c_void, metafile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory1_Impl::CreateGdiMetafile(this, windows_core::from_raw_borrowed(&metafilestream)) {
                Ok(ok__) => {
                    metafile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEffectFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: *const windows_core::GUID, propertyxml: *mut core::ffi::c_void, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Factory1_Impl::RegisterEffectFromStream(this, core::mem::transmute_copy(&classid), windows_core::from_raw_borrowed(&propertyxml), core::mem::transmute_copy(&bindings), core::mem::transmute_copy(&bindingscount), core::mem::transmute_copy(&effectfactory)).into()
        }
        unsafe extern "system" fn RegisterEffectFromString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: *const windows_core::GUID, propertyxml: windows_core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Factory1_Impl::RegisterEffectFromString(this, core::mem::transmute_copy(&classid), core::mem::transmute(&propertyxml), core::mem::transmute_copy(&bindings), core::mem::transmute_copy(&bindingscount), core::mem::transmute_copy(&effectfactory)).into()
        }
        unsafe extern "system" fn UnregisterEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Factory1_Impl::UnregisterEffect(this, core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn GetRegisteredEffects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effects: *mut windows_core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Factory1_Impl::GetRegisteredEffects(this, core::mem::transmute_copy(&effects), core::mem::transmute_copy(&effectscount), core::mem::transmute_copy(&effectsreturned), core::mem::transmute_copy(&effectsregistered)).into()
        }
        unsafe extern "system" fn GetEffectProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: *const windows_core::GUID, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory1_Impl::GetEffectProperties(this, core::mem::transmute_copy(&effectid)) {
                Ok(ok__) => {
                    properties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Factory_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, OFFSET>,
            CreateGdiMetafile: CreateGdiMetafile::<Identity, OFFSET>,
            RegisterEffectFromStream: RegisterEffectFromStream::<Identity, OFFSET>,
            RegisterEffectFromString: RegisterEffectFromString::<Identity, OFFSET>,
            UnregisterEffect: UnregisterEffect::<Identity, OFFSET>,
            GetRegisteredEffects: GetRegisteredEffects::<Identity, OFFSET>,
            GetEffectProperties: GetEffectProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory2_Impl: Sized + ID2D1Factory1_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device1>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory2 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory2_Vtbl
    where
        Identity: ID2D1Factory2_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory2_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice1.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory1_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory3_Impl: Sized + ID2D1Factory2_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device2>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory3 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory3_Vtbl
    where
        Identity: ID2D1Factory3_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory3_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory2_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory4_Impl: Sized + ID2D1Factory3_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory4 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory4_Vtbl
    where
        Identity: ID2D1Factory4_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice3: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory4_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice3.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory3_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory5_Impl: Sized + ID2D1Factory4_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device4>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory5 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory5_Vtbl
    where
        Identity: ID2D1Factory5_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice4: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory5_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice4.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory4_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory5 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory6_Impl: Sized + ID2D1Factory5_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device5>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory6 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory6_Vtbl
    where
        Identity: ID2D1Factory6_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice5: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory6_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice5.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory5_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory6 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory7_Impl: Sized + ID2D1Factory6_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device6>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory7 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory7_Vtbl
    where
        Identity: ID2D1Factory7_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice6: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory7_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice6.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory6_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory7 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory5 as windows_core::Interface>::IID || iid == &<ID2D1Factory6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory8_Impl: Sized + ID2D1Factory7_Impl {
    fn CreateDevice(&self, dxgidevice: Option<&super::Dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device7>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1Factory8 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ID2D1Factory8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Factory8_Vtbl
    where
        Identity: ID2D1Factory8_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice6: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Factory8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Factory8_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&dxgidevice)) {
                Ok(ok__) => {
                    d2ddevice6.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Factory7_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory8 as windows_core::Interface>::IID || iid == &<ID2D1Factory as windows_core::Interface>::IID || iid == &<ID2D1Factory1 as windows_core::Interface>::IID || iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory5 as windows_core::Interface>::IID || iid == &<ID2D1Factory6 as windows_core::Interface>::IID || iid == &<ID2D1Factory7 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ID2D1GdiInteropRenderTarget_Impl: Sized {
    fn GetDC(&self, mode: D2D1_DC_INITIALIZE_MODE) -> windows_core::Result<super::Gdi::HDC>;
    fn ReleaseDC(&self, update: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for ID2D1GdiInteropRenderTarget {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ID2D1GdiInteropRenderTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GdiInteropRenderTarget_Vtbl
    where
        Identity: ID2D1GdiInteropRenderTarget_Impl,
    {
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiInteropRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1GdiInteropRenderTarget_Impl::GetDC(this, core::mem::transmute_copy(&mode)) {
                Ok(ok__) => {
                    hdc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, update: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiInteropRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GdiInteropRenderTarget_Impl::ReleaseDC(this, core::mem::transmute_copy(&update)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDC: GetDC::<Identity, OFFSET>, ReleaseDC: ReleaseDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiInteropRenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GdiMetafile_Impl: Sized + ID2D1Resource_Impl {
    fn Stream(&self, sink: Option<&ID2D1GdiMetafileSink>) -> windows_core::Result<()>;
    fn GetBounds(&self) -> windows_core::Result<Common::D2D_RECT_F>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GdiMetafile {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GdiMetafile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GdiMetafile_Vtbl
    where
        Identity: ID2D1GdiMetafile_Impl,
    {
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiMetafile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GdiMetafile_Impl::Stream(this, windows_core::from_raw_borrowed(&sink)).into()
        }
        unsafe extern "system" fn GetBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiMetafile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1GdiMetafile_Impl::GetBounds(this) {
                Ok(ok__) => {
                    bounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), Stream: Stream::<Identity, OFFSET>, GetBounds: GetBounds::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafile as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GdiMetafile1_Impl: Sized + ID2D1GdiMetafile_Impl {
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) -> windows_core::Result<()>;
    fn GetSourceBounds(&self) -> windows_core::Result<Common::D2D_RECT_F>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GdiMetafile1 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GdiMetafile1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GdiMetafile1_Vtbl
    where
        Identity: ID2D1GdiMetafile1_Impl,
    {
        unsafe extern "system" fn GetDpi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiMetafile1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GdiMetafile1_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn GetSourceBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiMetafile1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1GdiMetafile1_Impl::GetSourceBounds(this) {
                Ok(ok__) => {
                    bounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1GdiMetafile_Vtbl::new::<Identity, OFFSET>(),
            GetDpi: GetDpi::<Identity, OFFSET>,
            GetSourceBounds: GetSourceBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafile1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1GdiMetafile as windows_core::Interface>::IID
    }
}
pub trait ID2D1GdiMetafileSink_Impl: Sized {
    fn ProcessRecord(&self, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1GdiMetafileSink {}
impl ID2D1GdiMetafileSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GdiMetafileSink_Vtbl
    where
        Identity: ID2D1GdiMetafileSink_Impl,
    {
        unsafe extern "system" fn ProcessRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiMetafileSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GdiMetafileSink_Impl::ProcessRecord(this, core::mem::transmute_copy(&recordtype), core::mem::transmute_copy(&recorddata), core::mem::transmute_copy(&recorddatasize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProcessRecord: ProcessRecord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafileSink as windows_core::Interface>::IID
    }
}
pub trait ID2D1GdiMetafileSink1_Impl: Sized + ID2D1GdiMetafileSink_Impl {
    fn ProcessRecord(&self, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32, flags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1GdiMetafileSink1 {}
impl ID2D1GdiMetafileSink1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GdiMetafileSink1_Vtbl
    where
        Identity: ID2D1GdiMetafileSink1_Impl,
    {
        unsafe extern "system" fn ProcessRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32, flags: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1GdiMetafileSink1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GdiMetafileSink1_Impl::ProcessRecord(this, core::mem::transmute_copy(&recordtype), core::mem::transmute_copy(&recorddata), core::mem::transmute_copy(&recorddatasize), core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: ID2D1GdiMetafileSink_Vtbl::new::<Identity, OFFSET>(), ProcessRecord: ProcessRecord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafileSink1 as windows_core::Interface>::IID || iid == &<ID2D1GdiMetafileSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1Geometry_Impl: Sized + ID2D1Resource_Impl {
    fn GetBounds(&self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<Common::D2D_RECT_F>;
    fn GetWidenedBounds(&self, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<Common::D2D_RECT_F>;
    fn StrokeContainsPoint(&self, point: &Common::D2D_POINT_2F, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn FillContainsPoint(&self, point: &Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareWithGeometry(&self, inputgeometry: Option<&ID2D1Geometry>, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<D2D1_GEOMETRY_RELATION>;
    fn Simplify(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: Option<&Common::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn Tessellate(&self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: Option<&ID2D1TessellationSink>) -> windows_core::Result<()>;
    fn CombineWithGeometry(&self, inputgeometry: Option<&ID2D1Geometry>, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: Option<&Common::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn Outline(&self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: Option<&Common::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn ComputeArea(&self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<f32>;
    fn ComputeLength(&self, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> windows_core::Result<f32>;
    fn ComputePointAtLength(&self, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> windows_core::Result<()>;
    fn Widen(&self, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: Option<&Common::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1Geometry {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1Geometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Geometry_Vtbl
    where
        Identity: ID2D1Geometry_Impl,
    {
        unsafe extern "system" fn GetBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::GetBounds(this, core::mem::transmute_copy(&worldtransform)) {
                Ok(ok__) => {
                    bounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidenedBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::GetWidenedBounds(this, core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    bounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrokeContainsPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::StrokeContainsPoint(this, core::mem::transmute(&point), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    contains.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillContainsPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::FillContainsPoint(this, core::mem::transmute(&point), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    contains.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareWithGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputgeometry: *mut core::ffi::c_void, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::CompareWithGeometry(this, windows_core::from_raw_borrowed(&inputgeometry), core::mem::transmute_copy(&inputgeometrytransform), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    relation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Simplify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Geometry_Impl::Simplify(this, core::mem::transmute_copy(&simplificationoption), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn Tessellate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Geometry_Impl::Tessellate(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), windows_core::from_raw_borrowed(&tessellationsink)).into()
        }
        unsafe extern "system" fn CombineWithGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputgeometry: *mut core::ffi::c_void, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Geometry_Impl::CombineWithGeometry(this, windows_core::from_raw_borrowed(&inputgeometry), core::mem::transmute_copy(&combinemode), core::mem::transmute_copy(&inputgeometrytransform), core::mem::transmute_copy(&flatteningtolerance), windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn Outline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Geometry_Impl::Outline(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn ComputeArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::ComputeArea(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    area.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Geometry_Impl::ComputeLength(this, core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance)) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputePointAtLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Geometry_Impl::ComputePointAtLength(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&point), core::mem::transmute_copy(&unittangentvector)).into()
        }
        unsafe extern "system" fn Widen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Geometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Geometry_Impl::Widen(this, core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetBounds: GetBounds::<Identity, OFFSET>,
            GetWidenedBounds: GetWidenedBounds::<Identity, OFFSET>,
            StrokeContainsPoint: StrokeContainsPoint::<Identity, OFFSET>,
            FillContainsPoint: FillContainsPoint::<Identity, OFFSET>,
            CompareWithGeometry: CompareWithGeometry::<Identity, OFFSET>,
            Simplify: Simplify::<Identity, OFFSET>,
            Tessellate: Tessellate::<Identity, OFFSET>,
            CombineWithGeometry: CombineWithGeometry::<Identity, OFFSET>,
            Outline: Outline::<Identity, OFFSET>,
            ComputeArea: ComputeArea::<Identity, OFFSET>,
            ComputeLength: ComputeLength::<Identity, OFFSET>,
            ComputePointAtLength: ComputePointAtLength::<Identity, OFFSET>,
            Widen: Widen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Geometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1GeometryGroup_Impl: Sized + ID2D1Geometry_Impl {
    fn GetFillMode(&self) -> Common::D2D1_FILL_MODE;
    fn GetSourceGeometryCount(&self) -> u32;
    fn GetSourceGeometries(&self, geometries: *mut Option<ID2D1Geometry>, geometriescount: u32);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1GeometryGroup {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1GeometryGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GeometryGroup_Vtbl
    where
        Identity: ID2D1GeometryGroup_Impl,
    {
        unsafe extern "system" fn GetFillMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Common::D2D1_FILL_MODE
        where
            Identity: ID2D1GeometryGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometryGroup_Impl::GetFillMode(this)
        }
        unsafe extern "system" fn GetSourceGeometryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1GeometryGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometryGroup_Impl::GetSourceGeometryCount(this)
        }
        unsafe extern "system" fn GetSourceGeometries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometries: *mut *mut core::ffi::c_void, geometriescount: u32)
        where
            Identity: ID2D1GeometryGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometryGroup_Impl::GetSourceGeometries(this, core::mem::transmute_copy(&geometries), core::mem::transmute_copy(&geometriescount))
        }
        Self {
            base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(),
            GetFillMode: GetFillMode::<Identity, OFFSET>,
            GetSourceGeometryCount: GetSourceGeometryCount::<Identity, OFFSET>,
            GetSourceGeometries: GetSourceGeometries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometryGroup as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
pub trait ID2D1GeometryRealization_Impl: Sized + ID2D1Resource_Impl {}
impl windows_core::RuntimeName for ID2D1GeometryRealization {}
impl ID2D1GeometryRealization_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GeometryRealization_Vtbl
    where
        Identity: ID2D1GeometryRealization_Impl,
    {
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometryRealization as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GeometrySink_Impl: Sized + Common::ID2D1SimplifiedGeometrySink_Impl {
    fn AddLine(&self, point: &Common::D2D_POINT_2F);
    fn AddBezier(&self, bezier: *const Common::D2D1_BEZIER_SEGMENT);
    fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT);
    fn AddQuadraticBeziers(&self, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32);
    fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GeometrySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GeometrySink_Vtbl
    where
        Identity: ID2D1GeometrySink_Impl,
    {
        unsafe extern "system" fn AddLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: Common::D2D_POINT_2F)
        where
            Identity: ID2D1GeometrySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometrySink_Impl::AddLine(this, core::mem::transmute(&point))
        }
        unsafe extern "system" fn AddBezier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bezier: *const Common::D2D1_BEZIER_SEGMENT)
        where
            Identity: ID2D1GeometrySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometrySink_Impl::AddBezier(this, core::mem::transmute_copy(&bezier))
        }
        unsafe extern "system" fn AddQuadraticBezier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT)
        where
            Identity: ID2D1GeometrySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometrySink_Impl::AddQuadraticBezier(this, core::mem::transmute_copy(&bezier))
        }
        unsafe extern "system" fn AddQuadraticBeziers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32)
        where
            Identity: ID2D1GeometrySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometrySink_Impl::AddQuadraticBeziers(this, core::mem::transmute_copy(&beziers), core::mem::transmute_copy(&bezierscount))
        }
        unsafe extern "system" fn AddArc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, arc: *const D2D1_ARC_SEGMENT)
        where
            Identity: ID2D1GeometrySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GeometrySink_Impl::AddArc(this, core::mem::transmute_copy(&arc))
        }
        Self {
            base__: Common::ID2D1SimplifiedGeometrySink_Vtbl::new::<Identity, OFFSET>(),
            AddLine: AddLine::<Identity, OFFSET>,
            AddBezier: AddBezier::<Identity, OFFSET>,
            AddQuadraticBezier: AddQuadraticBezier::<Identity, OFFSET>,
            AddQuadraticBeziers: AddQuadraticBeziers::<Identity, OFFSET>,
            AddArc: AddArc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometrySink as windows_core::Interface>::IID || iid == &<Common::ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientMesh_Impl: Sized + ID2D1Resource_Impl {
    fn GetPatchCount(&self) -> u32;
    fn GetPatches(&self, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GradientMesh {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GradientMesh_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GradientMesh_Vtbl
    where
        Identity: ID2D1GradientMesh_Impl,
    {
        unsafe extern "system" fn GetPatchCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1GradientMesh_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientMesh_Impl::GetPatchCount(this)
        }
        unsafe extern "system" fn GetPatches<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1GradientMesh_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientMesh_Impl::GetPatches(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&patches), core::mem::transmute_copy(&patchescount)).into()
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetPatchCount: GetPatchCount::<Identity, OFFSET>,
            GetPatches: GetPatches::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GradientMesh as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientStopCollection_Impl: Sized + ID2D1Resource_Impl {
    fn GetGradientStopCount(&self) -> u32;
    fn GetGradientStops(&self, gradientstops: *mut Common::D2D1_GRADIENT_STOP, gradientstopscount: u32);
    fn GetColorInterpolationGamma(&self) -> D2D1_GAMMA;
    fn GetExtendMode(&self) -> D2D1_EXTEND_MODE;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GradientStopCollection {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GradientStopCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GradientStopCollection_Vtbl
    where
        Identity: ID2D1GradientStopCollection_Impl,
    {
        unsafe extern "system" fn GetGradientStopCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1GradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection_Impl::GetGradientStopCount(this)
        }
        unsafe extern "system" fn GetGradientStops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut Common::D2D1_GRADIENT_STOP, gradientstopscount: u32)
        where
            Identity: ID2D1GradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection_Impl::GetGradientStops(this, core::mem::transmute_copy(&gradientstops), core::mem::transmute_copy(&gradientstopscount))
        }
        unsafe extern "system" fn GetColorInterpolationGamma<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_GAMMA
        where
            Identity: ID2D1GradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection_Impl::GetColorInterpolationGamma(this)
        }
        unsafe extern "system" fn GetExtendMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1GradientStopCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection_Impl::GetExtendMode(this)
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetGradientStopCount: GetGradientStopCount::<Identity, OFFSET>,
            GetGradientStops: GetGradientStops::<Identity, OFFSET>,
            GetColorInterpolationGamma: GetColorInterpolationGamma::<Identity, OFFSET>,
            GetExtendMode: GetExtendMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GradientStopCollection as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientStopCollection1_Impl: Sized + ID2D1GradientStopCollection_Impl {
    fn GetGradientStops1(&self, gradientstops: *mut Common::D2D1_GRADIENT_STOP, gradientstopscount: u32);
    fn GetPreInterpolationSpace(&self) -> D2D1_COLOR_SPACE;
    fn GetPostInterpolationSpace(&self) -> D2D1_COLOR_SPACE;
    fn GetBufferPrecision(&self) -> D2D1_BUFFER_PRECISION;
    fn GetColorInterpolationMode(&self) -> D2D1_COLOR_INTERPOLATION_MODE;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GradientStopCollection1 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GradientStopCollection1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1GradientStopCollection1_Vtbl
    where
        Identity: ID2D1GradientStopCollection1_Impl,
    {
        unsafe extern "system" fn GetGradientStops1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *mut Common::D2D1_GRADIENT_STOP, gradientstopscount: u32)
        where
            Identity: ID2D1GradientStopCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection1_Impl::GetGradientStops1(this, core::mem::transmute_copy(&gradientstops), core::mem::transmute_copy(&gradientstopscount))
        }
        unsafe extern "system" fn GetPreInterpolationSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_SPACE
        where
            Identity: ID2D1GradientStopCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection1_Impl::GetPreInterpolationSpace(this)
        }
        unsafe extern "system" fn GetPostInterpolationSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_SPACE
        where
            Identity: ID2D1GradientStopCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection1_Impl::GetPostInterpolationSpace(this)
        }
        unsafe extern "system" fn GetBufferPrecision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_BUFFER_PRECISION
        where
            Identity: ID2D1GradientStopCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection1_Impl::GetBufferPrecision(this)
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE
        where
            Identity: ID2D1GradientStopCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1GradientStopCollection1_Impl::GetColorInterpolationMode(this)
        }
        Self {
            base__: ID2D1GradientStopCollection_Vtbl::new::<Identity, OFFSET>(),
            GetGradientStops1: GetGradientStops1::<Identity, OFFSET>,
            GetPreInterpolationSpace: GetPreInterpolationSpace::<Identity, OFFSET>,
            GetPostInterpolationSpace: GetPostInterpolationSpace::<Identity, OFFSET>,
            GetBufferPrecision: GetBufferPrecision::<Identity, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GradientStopCollection1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1GradientStopCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1HwndRenderTarget_Impl: Sized + ID2D1RenderTarget_Impl {
    fn CheckWindowState(&self) -> D2D1_WINDOW_STATE;
    fn Resize(&self, pixelsize: *const Common::D2D_SIZE_U) -> windows_core::Result<()>;
    fn GetHwnd(&self) -> super::super::Foundation::HWND;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1HwndRenderTarget {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1HwndRenderTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1HwndRenderTarget_Vtbl
    where
        Identity: ID2D1HwndRenderTarget_Impl,
    {
        unsafe extern "system" fn CheckWindowState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_WINDOW_STATE
        where
            Identity: ID2D1HwndRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1HwndRenderTarget_Impl::CheckWindowState(this)
        }
        unsafe extern "system" fn Resize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelsize: *const Common::D2D_SIZE_U) -> windows_core::HRESULT
        where
            Identity: ID2D1HwndRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1HwndRenderTarget_Impl::Resize(this, core::mem::transmute_copy(&pixelsize)).into()
        }
        unsafe extern "system" fn GetHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HWND
        where
            Identity: ID2D1HwndRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1HwndRenderTarget_Impl::GetHwnd(this)
        }
        Self {
            base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(),
            CheckWindowState: CheckWindowState::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1HwndRenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
pub trait ID2D1Image_Impl: Sized + ID2D1Resource_Impl {}
impl windows_core::RuntimeName for ID2D1Image {}
impl ID2D1Image_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Image_Vtbl
    where
        Identity: ID2D1Image_Impl,
    {
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Image as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1ImageBrush_Impl: Sized + ID2D1Brush_Impl {
    fn SetImage(&self, image: Option<&ID2D1Image>);
    fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE);
    fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE);
    fn SetInterpolationMode(&self, interpolationmode: D2D1_INTERPOLATION_MODE);
    fn SetSourceRectangle(&self, sourcerectangle: *const Common::D2D_RECT_F);
    fn GetImage(&self, image: *mut Option<ID2D1Image>);
    fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE;
    fn GetInterpolationMode(&self) -> D2D1_INTERPOLATION_MODE;
    fn GetSourceRectangle(&self, sourcerectangle: *mut Common::D2D_RECT_F);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1ImageBrush {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1ImageBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ImageBrush_Vtbl
    where
        Identity: ID2D1ImageBrush_Impl,
    {
        unsafe extern "system" fn SetImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::SetImage(this, windows_core::from_raw_borrowed(&image))
        }
        unsafe extern "system" fn SetExtendModeX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::SetExtendModeX(this, core::mem::transmute_copy(&extendmodex))
        }
        unsafe extern "system" fn SetExtendModeY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::SetExtendModeY(this, core::mem::transmute_copy(&extendmodey))
        }
        unsafe extern "system" fn SetInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::SetInterpolationMode(this, core::mem::transmute_copy(&interpolationmode))
        }
        unsafe extern "system" fn SetSourceRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerectangle: *const Common::D2D_RECT_F)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::SetSourceRectangle(this, core::mem::transmute_copy(&sourcerectangle))
        }
        unsafe extern "system" fn GetImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::GetImage(this, core::mem::transmute_copy(&image))
        }
        unsafe extern "system" fn GetExtendModeX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::GetExtendModeX(this)
        }
        unsafe extern "system" fn GetExtendModeY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_EXTEND_MODE
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::GetExtendModeY(this)
        }
        unsafe extern "system" fn GetInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::GetInterpolationMode(this)
        }
        unsafe extern "system" fn GetSourceRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerectangle: *mut Common::D2D_RECT_F)
        where
            Identity: ID2D1ImageBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageBrush_Impl::GetSourceRectangle(this, core::mem::transmute_copy(&sourcerectangle))
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetImage: SetImage::<Identity, OFFSET>,
            SetExtendModeX: SetExtendModeX::<Identity, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Identity, OFFSET>,
            SetSourceRectangle: SetSourceRectangle::<Identity, OFFSET>,
            GetImage: GetImage::<Identity, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, OFFSET>,
            GetInterpolationMode: GetInterpolationMode::<Identity, OFFSET>,
            GetSourceRectangle: GetSourceRectangle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ImageBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
pub trait ID2D1ImageSource_Impl: Sized + ID2D1Image_Impl {
    fn OfferResources(&self) -> windows_core::Result<()>;
    fn TryReclaimResources(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ID2D1ImageSource {}
impl ID2D1ImageSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ImageSource_Vtbl
    where
        Identity: ID2D1ImageSource_Impl,
    {
        unsafe extern "system" fn OfferResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1ImageSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageSource_Impl::OfferResources(this).into()
        }
        unsafe extern "system" fn TryReclaimResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcesdiscarded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID2D1ImageSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1ImageSource_Impl::TryReclaimResources(this) {
                Ok(ok__) => {
                    resourcesdiscarded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Image_Vtbl::new::<Identity, OFFSET>(),
            OfferResources: OfferResources::<Identity, OFFSET>,
            TryReclaimResources: TryReclaimResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ImageSource as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1ImageSourceFromWic_Impl: Sized + ID2D1ImageSource_Impl {
    fn EnsureCached(&self, rectangletofill: *const Common::D2D_RECT_U) -> windows_core::Result<()>;
    fn TrimCache(&self, rectangletopreserve: *const Common::D2D_RECT_U) -> windows_core::Result<()>;
    fn GetSource(&self, wicbitmapsource: *mut Option<super::Imaging::IWICBitmapSource>);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1ImageSourceFromWic {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1ImageSourceFromWic_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ImageSourceFromWic_Vtbl
    where
        Identity: ID2D1ImageSourceFromWic_Impl,
    {
        unsafe extern "system" fn EnsureCached<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangletofill: *const Common::D2D_RECT_U) -> windows_core::HRESULT
        where
            Identity: ID2D1ImageSourceFromWic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageSourceFromWic_Impl::EnsureCached(this, core::mem::transmute_copy(&rectangletofill)).into()
        }
        unsafe extern "system" fn TrimCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangletopreserve: *const Common::D2D_RECT_U) -> windows_core::HRESULT
        where
            Identity: ID2D1ImageSourceFromWic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageSourceFromWic_Impl::TrimCache(this, core::mem::transmute_copy(&rectangletopreserve)).into()
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1ImageSourceFromWic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ImageSourceFromWic_Impl::GetSource(this, core::mem::transmute_copy(&wicbitmapsource))
        }
        Self {
            base__: ID2D1ImageSource_Vtbl::new::<Identity, OFFSET>(),
            EnsureCached: EnsureCached::<Identity, OFFSET>,
            TrimCache: TrimCache::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ImageSourceFromWic as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID || iid == &<ID2D1ImageSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1Ink_Impl: Sized + ID2D1Resource_Impl {
    fn SetStartPoint(&self, startpoint: *const D2D1_INK_POINT);
    fn GetStartPoint(&self) -> D2D1_INK_POINT;
    fn AddSegments(&self, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::Result<()>;
    fn RemoveSegmentsAtEnd(&self, segmentscount: u32) -> windows_core::Result<()>;
    fn SetSegments(&self, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::Result<()>;
    fn SetSegmentAtEnd(&self, segment: *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::Result<()>;
    fn GetSegmentCount(&self) -> u32;
    fn GetSegments(&self, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::Result<()>;
    fn StreamAsGeometry(&self, inkstyle: Option<&ID2D1InkStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: Option<&Common::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn GetBounds(&self, inkstyle: Option<&ID2D1InkStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<Common::D2D_RECT_F>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1Ink {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1Ink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Ink_Vtbl
    where
        Identity: ID2D1Ink_Impl,
    {
        unsafe extern "system" fn SetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const D2D1_INK_POINT)
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint))
        }
        unsafe extern "system" fn GetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D2D1_INK_POINT)
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1Ink_Impl::GetStartPoint(this)
        }
        unsafe extern "system" fn AddSegments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::AddSegments(this, core::mem::transmute_copy(&segments), core::mem::transmute_copy(&segmentscount)).into()
        }
        unsafe extern "system" fn RemoveSegmentsAtEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::RemoveSegmentsAtEnd(this, core::mem::transmute_copy(&segmentscount)).into()
        }
        unsafe extern "system" fn SetSegments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::SetSegments(this, core::mem::transmute_copy(&startsegment), core::mem::transmute_copy(&segments), core::mem::transmute_copy(&segmentscount)).into()
        }
        unsafe extern "system" fn SetSegmentAtEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::SetSegmentAtEnd(this, core::mem::transmute_copy(&segment)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::GetSegmentCount(this)
        }
        unsafe extern "system" fn GetSegments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::GetSegments(this, core::mem::transmute_copy(&startsegment), core::mem::transmute_copy(&segments), core::mem::transmute_copy(&segmentscount)).into()
        }
        unsafe extern "system" fn StreamAsGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Ink_Impl::StreamAsGeometry(this, windows_core::from_raw_borrowed(&inkstyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn GetBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: ID2D1Ink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Ink_Impl::GetBounds(this, windows_core::from_raw_borrowed(&inkstyle), core::mem::transmute_copy(&worldtransform)) {
                Ok(ok__) => {
                    bounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            AddSegments: AddSegments::<Identity, OFFSET>,
            RemoveSegmentsAtEnd: RemoveSegmentsAtEnd::<Identity, OFFSET>,
            SetSegments: SetSegments::<Identity, OFFSET>,
            SetSegmentAtEnd: SetSegmentAtEnd::<Identity, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, OFFSET>,
            GetSegments: GetSegments::<Identity, OFFSET>,
            StreamAsGeometry: StreamAsGeometry::<Identity, OFFSET>,
            GetBounds: GetBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Ink as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1InkStyle_Impl: Sized + ID2D1Resource_Impl {
    fn SetNibTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2);
    fn GetNibTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    fn SetNibShape(&self, nibshape: D2D1_INK_NIB_SHAPE);
    fn GetNibShape(&self) -> D2D1_INK_NIB_SHAPE;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeName for ID2D1InkStyle {}
#[cfg(feature = "Foundation_Numerics")]
impl ID2D1InkStyle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1InkStyle_Vtbl
    where
        Identity: ID2D1InkStyle_Impl,
    {
        unsafe extern "system" fn SetNibTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1InkStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1InkStyle_Impl::SetNibTransform(this, core::mem::transmute_copy(&transform))
        }
        unsafe extern "system" fn GetNibTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1InkStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1InkStyle_Impl::GetNibTransform(this, core::mem::transmute_copy(&transform))
        }
        unsafe extern "system" fn SetNibShape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nibshape: D2D1_INK_NIB_SHAPE)
        where
            Identity: ID2D1InkStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1InkStyle_Impl::SetNibShape(this, core::mem::transmute_copy(&nibshape))
        }
        unsafe extern "system" fn GetNibShape<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_INK_NIB_SHAPE
        where
            Identity: ID2D1InkStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1InkStyle_Impl::GetNibShape(this)
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetNibTransform: SetNibTransform::<Identity, OFFSET>,
            GetNibTransform: GetNibTransform::<Identity, OFFSET>,
            SetNibShape: SetNibShape::<Identity, OFFSET>,
            GetNibShape: GetNibShape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1InkStyle as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1Layer_Impl: Sized + ID2D1Resource_Impl {
    fn GetSize(&self) -> Common::D2D_SIZE_F;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1Layer {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1Layer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Layer_Vtbl
    where
        Identity: ID2D1Layer_Impl,
    {
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_SIZE_F)
        where
            Identity: ID2D1Layer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1Layer_Impl::GetSize(this)
        }
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), GetSize: GetSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Layer as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1LinearGradientBrush_Impl: Sized + ID2D1Brush_Impl {
    fn SetStartPoint(&self, startpoint: &Common::D2D_POINT_2F);
    fn SetEndPoint(&self, endpoint: &Common::D2D_POINT_2F);
    fn GetStartPoint(&self) -> Common::D2D_POINT_2F;
    fn GetEndPoint(&self) -> Common::D2D_POINT_2F;
    fn GetGradientStopCollection(&self, gradientstopcollection: *mut Option<ID2D1GradientStopCollection>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1LinearGradientBrush {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1LinearGradientBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1LinearGradientBrush_Vtbl
    where
        Identity: ID2D1LinearGradientBrush_Impl,
    {
        unsafe extern "system" fn SetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: Common::D2D_POINT_2F)
        where
            Identity: ID2D1LinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1LinearGradientBrush_Impl::SetStartPoint(this, core::mem::transmute(&startpoint))
        }
        unsafe extern "system" fn SetEndPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: Common::D2D_POINT_2F)
        where
            Identity: ID2D1LinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1LinearGradientBrush_Impl::SetEndPoint(this, core::mem::transmute(&endpoint))
        }
        unsafe extern "system" fn GetStartPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_POINT_2F)
        where
            Identity: ID2D1LinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1LinearGradientBrush_Impl::GetStartPoint(this)
        }
        unsafe extern "system" fn GetEndPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_POINT_2F)
        where
            Identity: ID2D1LinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1LinearGradientBrush_Impl::GetEndPoint(this)
        }
        unsafe extern "system" fn GetGradientStopCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstopcollection: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1LinearGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1LinearGradientBrush_Impl::GetGradientStopCollection(this, core::mem::transmute_copy(&gradientstopcollection))
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, OFFSET>,
            GetGradientStopCollection: GetGradientStopCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1LinearGradientBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
pub trait ID2D1LookupTable3D_Impl: Sized + ID2D1Resource_Impl {}
impl windows_core::RuntimeName for ID2D1LookupTable3D {}
impl ID2D1LookupTable3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1LookupTable3D_Vtbl
    where
        Identity: ID2D1LookupTable3D_Impl,
    {
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1LookupTable3D as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1Mesh_Impl: Sized + ID2D1Resource_Impl {
    fn Open(&self) -> windows_core::Result<ID2D1TessellationSink>;
}
impl windows_core::RuntimeName for ID2D1Mesh {}
impl ID2D1Mesh_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Mesh_Vtbl
    where
        Identity: ID2D1Mesh_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tessellationsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Mesh_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Mesh_Impl::Open(this) {
                Ok(ok__) => {
                    tessellationsink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), Open: Open::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Mesh as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1Multithread_Impl: Sized {
    fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL;
    fn Enter(&self);
    fn Leave(&self);
}
impl windows_core::RuntimeName for ID2D1Multithread {}
impl ID2D1Multithread_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Multithread_Vtbl
    where
        Identity: ID2D1Multithread_Impl,
    {
        unsafe extern "system" fn GetMultithreadProtected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Multithread_Impl::GetMultithreadProtected(this)
        }
        unsafe extern "system" fn Enter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Multithread_Impl::Enter(this)
        }
        unsafe extern "system" fn Leave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Multithread_Impl::Leave(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMultithreadProtected: GetMultithreadProtected::<Identity, OFFSET>,
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Multithread as windows_core::Interface>::IID
    }
}
pub trait ID2D1OffsetTransform_Impl: Sized + ID2D1TransformNode_Impl {
    fn SetOffset(&self, offset: &super::super::Foundation::POINT);
    fn GetOffset(&self) -> super::super::Foundation::POINT;
}
impl windows_core::RuntimeName for ID2D1OffsetTransform {}
impl ID2D1OffsetTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1OffsetTransform_Vtbl
    where
        Identity: ID2D1OffsetTransform_Impl,
    {
        unsafe extern "system" fn SetOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: super::super::Foundation::POINT)
        where
            Identity: ID2D1OffsetTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1OffsetTransform_Impl::SetOffset(this, core::mem::transmute(&offset))
        }
        unsafe extern "system" fn GetOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::POINT)
        where
            Identity: ID2D1OffsetTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1OffsetTransform_Impl::GetOffset(this)
        }
        Self { base__: ID2D1TransformNode_Vtbl::new::<Identity, OFFSET>(), SetOffset: SetOffset::<Identity, OFFSET>, GetOffset: GetOffset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1OffsetTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1PathGeometry_Impl: Sized + ID2D1Geometry_Impl {
    fn Open(&self) -> windows_core::Result<ID2D1GeometrySink>;
    fn Stream(&self, geometrysink: Option<&ID2D1GeometrySink>) -> windows_core::Result<()>;
    fn GetSegmentCount(&self) -> windows_core::Result<u32>;
    fn GetFigureCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1PathGeometry {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1PathGeometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1PathGeometry_Vtbl
    where
        Identity: ID2D1PathGeometry_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometrysink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1PathGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1PathGeometry_Impl::Open(this) {
                Ok(ok__) => {
                    geometrysink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1PathGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1PathGeometry_Impl::Stream(this, windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn GetSegmentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID2D1PathGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1PathGeometry_Impl::GetSegmentCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFigureCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID2D1PathGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1PathGeometry_Impl::GetFigureCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, OFFSET>,
            GetFigureCount: GetFigureCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1PathGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1PathGeometry1_Impl: Sized + ID2D1PathGeometry_Impl {
    fn ComputePointAndSegmentAtLength(&self, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1PathGeometry1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1PathGeometry1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1PathGeometry1_Vtbl
    where
        Identity: ID2D1PathGeometry1_Impl,
    {
        unsafe extern "system" fn ComputePointAndSegmentAtLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> windows_core::HRESULT
        where
            Identity: ID2D1PathGeometry1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1PathGeometry1_Impl::ComputePointAndSegmentAtLength(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&startsegment), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&pointdescription)).into()
        }
        Self { base__: ID2D1PathGeometry_Vtbl::new::<Identity, OFFSET>(), ComputePointAndSegmentAtLength: ComputePointAndSegmentAtLength::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1PathGeometry1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID || iid == &<ID2D1PathGeometry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
pub trait ID2D1PrintControl_Impl: Sized {
    fn AddPage(&self, commandlist: Option<&ID2D1CommandList>, pagesize: &Common::D2D_SIZE_F, pageprintticketstream: Option<&super::super::System::Com::IStream>, tag1: *mut u64, tag2: *mut u64) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1PrintControl {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl ID2D1PrintControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1PrintControl_Vtbl
    where
        Identity: ID2D1PrintControl_Impl,
    {
        unsafe extern "system" fn AddPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandlist: *mut core::ffi::c_void, pagesize: Common::D2D_SIZE_F, pageprintticketstream: *mut core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> windows_core::HRESULT
        where
            Identity: ID2D1PrintControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1PrintControl_Impl::AddPage(this, windows_core::from_raw_borrowed(&commandlist), core::mem::transmute(&pagesize), windows_core::from_raw_borrowed(&pageprintticketstream), core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1PrintControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1PrintControl_Impl::Close(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPage: AddPage::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1PrintControl as windows_core::Interface>::IID
    }
}
pub trait ID2D1Properties_Impl: Sized {
    fn GetPropertyCount(&self) -> u32;
    fn GetPropertyName(&self, index: u32, name: windows_core::PWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetPropertyNameLength(&self, index: u32) -> u32;
    fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE;
    fn GetPropertyIndex(&self, name: &windows_core::PCWSTR) -> u32;
    fn SetValueByName(&self, name: &windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::Result<()>;
    fn SetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::Result<()>;
    fn GetValueByName(&self, name: &windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::Result<()>;
    fn GetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::Result<()>;
    fn GetValueSize(&self, index: u32) -> u32;
    fn GetSubProperties(&self, index: u32) -> windows_core::Result<ID2D1Properties>;
}
impl windows_core::RuntimeName for ID2D1Properties {}
impl ID2D1Properties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Properties_Vtbl
    where
        Identity: ID2D1Properties_Impl,
    {
        unsafe extern "system" fn GetPropertyCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetPropertyCount(this)
        }
        unsafe extern "system" fn GetPropertyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: windows_core::PWSTR, namecount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetPropertyName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn GetPropertyNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> u32
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetPropertyNameLength(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> D2D1_PROPERTY_TYPE
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetType(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetPropertyIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> u32
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetPropertyIndex(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn SetValueByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::SetValueByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetValueByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetValueByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetValueSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> u32
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Properties_Impl::GetValueSize(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetSubProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, subproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1Properties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Properties_Impl::GetSubProperties(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    subproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, OFFSET>,
            GetPropertyNameLength: GetPropertyNameLength::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetPropertyIndex: GetPropertyIndex::<Identity, OFFSET>,
            SetValueByName: SetValueByName::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValueByName: GetValueByName::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetValueSize: GetValueSize::<Identity, OFFSET>,
            GetSubProperties: GetSubProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Properties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RadialGradientBrush_Impl: Sized + ID2D1Brush_Impl {
    fn SetCenter(&self, center: &Common::D2D_POINT_2F);
    fn SetGradientOriginOffset(&self, gradientoriginoffset: &Common::D2D_POINT_2F);
    fn SetRadiusX(&self, radiusx: f32);
    fn SetRadiusY(&self, radiusy: f32);
    fn GetCenter(&self) -> Common::D2D_POINT_2F;
    fn GetGradientOriginOffset(&self) -> Common::D2D_POINT_2F;
    fn GetRadiusX(&self) -> f32;
    fn GetRadiusY(&self) -> f32;
    fn GetGradientStopCollection(&self, gradientstopcollection: *mut Option<ID2D1GradientStopCollection>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1RadialGradientBrush {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1RadialGradientBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1RadialGradientBrush_Vtbl
    where
        Identity: ID2D1RadialGradientBrush_Impl,
    {
        unsafe extern "system" fn SetCenter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: Common::D2D_POINT_2F)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::SetCenter(this, core::mem::transmute(&center))
        }
        unsafe extern "system" fn SetGradientOriginOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientoriginoffset: Common::D2D_POINT_2F)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::SetGradientOriginOffset(this, core::mem::transmute(&gradientoriginoffset))
        }
        unsafe extern "system" fn SetRadiusX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiusx: f32)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::SetRadiusX(this, core::mem::transmute_copy(&radiusx))
        }
        unsafe extern "system" fn SetRadiusY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiusy: f32)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::SetRadiusY(this, core::mem::transmute_copy(&radiusy))
        }
        unsafe extern "system" fn GetCenter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_POINT_2F)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1RadialGradientBrush_Impl::GetCenter(this)
        }
        unsafe extern "system" fn GetGradientOriginOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_POINT_2F)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1RadialGradientBrush_Impl::GetGradientOriginOffset(this)
        }
        unsafe extern "system" fn GetRadiusX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::GetRadiusX(this)
        }
        unsafe extern "system" fn GetRadiusY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::GetRadiusY(this)
        }
        unsafe extern "system" fn GetGradientStopCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstopcollection: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1RadialGradientBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RadialGradientBrush_Impl::GetGradientStopCollection(this, core::mem::transmute_copy(&gradientstopcollection))
        }
        Self {
            base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(),
            SetCenter: SetCenter::<Identity, OFFSET>,
            SetGradientOriginOffset: SetGradientOriginOffset::<Identity, OFFSET>,
            SetRadiusX: SetRadiusX::<Identity, OFFSET>,
            SetRadiusY: SetRadiusY::<Identity, OFFSET>,
            GetCenter: GetCenter::<Identity, OFFSET>,
            GetGradientOriginOffset: GetGradientOriginOffset::<Identity, OFFSET>,
            GetRadiusX: GetRadiusX::<Identity, OFFSET>,
            GetRadiusY: GetRadiusY::<Identity, OFFSET>,
            GetGradientStopCollection: GetGradientStopCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RadialGradientBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RectangleGeometry_Impl: Sized + ID2D1Geometry_Impl {
    fn GetRect(&self, rect: *mut Common::D2D_RECT_F);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1RectangleGeometry {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1RectangleGeometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1RectangleGeometry_Vtbl
    where
        Identity: ID2D1RectangleGeometry_Impl,
    {
        unsafe extern "system" fn GetRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *mut Common::D2D_RECT_F)
        where
            Identity: ID2D1RectangleGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RectangleGeometry_Impl::GetRect(this, core::mem::transmute_copy(&rect))
        }
        Self { base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(), GetRect: GetRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RectangleGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
pub trait ID2D1RenderInfo_Impl: Sized {
    fn SetInputDescription(&self, inputindex: u32, inputdescription: &D2D1_INPUT_DESCRIPTION) -> windows_core::Result<()>;
    fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> windows_core::Result<()>;
    fn SetCached(&self, iscached: super::super::Foundation::BOOL);
    fn SetInstructionCountHint(&self, instructioncount: u32);
}
impl windows_core::RuntimeName for ID2D1RenderInfo {}
impl ID2D1RenderInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1RenderInfo_Vtbl
    where
        Identity: ID2D1RenderInfo_Impl,
    {
        unsafe extern "system" fn SetInputDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderInfo_Impl::SetInputDescription(this, core::mem::transmute_copy(&inputindex), core::mem::transmute(&inputdescription)).into()
        }
        unsafe extern "system" fn SetOutputBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderInfo_Impl::SetOutputBuffer(this, core::mem::transmute_copy(&bufferprecision), core::mem::transmute_copy(&channeldepth)).into()
        }
        unsafe extern "system" fn SetCached<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscached: super::super::Foundation::BOOL)
        where
            Identity: ID2D1RenderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderInfo_Impl::SetCached(this, core::mem::transmute_copy(&iscached))
        }
        unsafe extern "system" fn SetInstructionCountHint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, instructioncount: u32)
        where
            Identity: ID2D1RenderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderInfo_Impl::SetInstructionCountHint(this, core::mem::transmute_copy(&instructioncount))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInputDescription: SetInputDescription::<Identity, OFFSET>,
            SetOutputBuffer: SetOutputBuffer::<Identity, OFFSET>,
            SetCached: SetCached::<Identity, OFFSET>,
            SetInstructionCountHint: SetInstructionCountHint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RenderInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1RenderTarget_Impl: Sized + ID2D1Resource_Impl {
    fn CreateBitmap(&self, size: &Common::D2D_SIZE_U, srcdata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap>;
    fn CreateBitmapFromWicBitmap(&self, wicbitmapsource: Option<&super::Imaging::IWICBitmapSource>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap>;
    fn CreateSharedBitmap(&self, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut Option<ID2D1Bitmap>) -> windows_core::Result<()>;
    fn CreateBitmapBrush(&self, bitmap: Option<&ID2D1Bitmap>, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1BitmapBrush>;
    fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> windows_core::Result<ID2D1SolidColorBrush>;
    fn CreateGradientStopCollection(&self, gradientstops: *const Common::D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> windows_core::Result<ID2D1GradientStopCollection>;
    fn CreateLinearGradientBrush(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: Option<&ID2D1GradientStopCollection>) -> windows_core::Result<ID2D1LinearGradientBrush>;
    fn CreateRadialGradientBrush(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: Option<&ID2D1GradientStopCollection>) -> windows_core::Result<ID2D1RadialGradientBrush>;
    fn CreateCompatibleRenderTarget(&self, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> windows_core::Result<ID2D1BitmapRenderTarget>;
    fn CreateLayer(&self, size: *const Common::D2D_SIZE_F) -> windows_core::Result<ID2D1Layer>;
    fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh>;
    fn DrawLine(&self, point0: &Common::D2D_POINT_2F, point1: &Common::D2D_POINT_2F, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>);
    fn DrawRectangle(&self, rect: *const Common::D2D_RECT_F, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>);
    fn FillRectangle(&self, rect: *const Common::D2D_RECT_F, brush: Option<&ID2D1Brush>);
    fn DrawRoundedRectangle(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>);
    fn FillRoundedRectangle(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: Option<&ID2D1Brush>);
    fn DrawEllipse(&self, ellipse: *const D2D1_ELLIPSE, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>);
    fn FillEllipse(&self, ellipse: *const D2D1_ELLIPSE, brush: Option<&ID2D1Brush>);
    fn DrawGeometry(&self, geometry: Option<&ID2D1Geometry>, brush: Option<&ID2D1Brush>, strokewidth: f32, strokestyle: Option<&ID2D1StrokeStyle>);
    fn FillGeometry(&self, geometry: Option<&ID2D1Geometry>, brush: Option<&ID2D1Brush>, opacitybrush: Option<&ID2D1Brush>);
    fn FillMesh(&self, mesh: Option<&ID2D1Mesh>, brush: Option<&ID2D1Brush>);
    fn FillOpacityMask(&self, opacitymask: Option<&ID2D1Bitmap>, brush: Option<&ID2D1Brush>, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F);
    fn DrawBitmap(&self, bitmap: Option<&ID2D1Bitmap>, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F);
    fn DrawText(&self, string: &windows_core::PCWSTR, stringlength: u32, textformat: Option<&super::DirectWrite::IDWriteTextFormat>, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: Option<&ID2D1Brush>, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawTextLayout(&self, origin: &Common::D2D_POINT_2F, textlayout: Option<&super::DirectWrite::IDWriteTextLayout>, defaultfillbrush: Option<&ID2D1Brush>, options: D2D1_DRAW_TEXT_OPTIONS);
    fn DrawGlyphRun(&self, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: Option<&ID2D1Brush>, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2);
    fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE);
    fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE;
    fn SetTextRenderingParams(&self, textrenderingparams: Option<&super::DirectWrite::IDWriteRenderingParams>);
    fn GetTextRenderingParams(&self, textrenderingparams: *mut Option<super::DirectWrite::IDWriteRenderingParams>);
    fn SetTags(&self, tag1: u64, tag2: u64);
    fn GetTags(&self, tag1: *mut u64, tag2: *mut u64);
    fn PushLayer(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: Option<&ID2D1Layer>);
    fn PopLayer(&self);
    fn Flush(&self, tag1: *mut u64, tag2: *mut u64) -> windows_core::Result<()>;
    fn SaveDrawingState(&self, drawingstateblock: Option<&ID2D1DrawingStateBlock>);
    fn RestoreDrawingState(&self, drawingstateblock: Option<&ID2D1DrawingStateBlock>);
    fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn PopAxisAlignedClip(&self);
    fn Clear(&self, clearcolor: *const Common::D2D1_COLOR_F);
    fn BeginDraw(&self);
    fn EndDraw(&self, tag1: *mut u64, tag2: *mut u64) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT;
    fn SetDpi(&self, dpix: f32, dpiy: f32);
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn GetSize(&self) -> Common::D2D_SIZE_F;
    fn GetPixelSize(&self) -> Common::D2D_SIZE_U;
    fn GetMaximumBitmapSize(&self) -> u32;
    fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for ID2D1RenderTarget {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ID2D1RenderTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1RenderTarget_Vtbl
    where
        Identity: ID2D1RenderTarget_Impl,
    {
        unsafe extern "system" fn CreateBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: Common::D2D_SIZE_U, srcdata: *const core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateBitmap(this, core::mem::transmute(&size), core::mem::transmute_copy(&srcdata), core::mem::transmute_copy(&pitch), core::mem::transmute_copy(&bitmapproperties)) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateBitmapFromWicBitmap(this, windows_core::from_raw_borrowed(&wicbitmapsource), core::mem::transmute_copy(&bitmapproperties)) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::CreateSharedBitmap(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&data), core::mem::transmute_copy(&bitmapproperties), core::mem::transmute_copy(&bitmap)).into()
        }
        unsafe extern "system" fn CreateBitmapBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateBitmapBrush(this, windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&bitmapbrushproperties), core::mem::transmute_copy(&brushproperties)) {
                Ok(ok__) => {
                    bitmapbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateSolidColorBrush(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&brushproperties)) {
                Ok(ok__) => {
                    solidcolorbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientstops: *const Common::D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateGradientStopCollection(this, core::mem::transmute_copy(&gradientstops), core::mem::transmute_copy(&gradientstopscount), core::mem::transmute_copy(&colorinterpolationgamma), core::mem::transmute_copy(&extendmode)) {
                Ok(ok__) => {
                    gradientstopcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut core::ffi::c_void, lineargradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateLinearGradientBrush(this, core::mem::transmute_copy(&lineargradientbrushproperties), core::mem::transmute_copy(&brushproperties), windows_core::from_raw_borrowed(&gradientstopcollection)) {
                Ok(ok__) => {
                    lineargradientbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut core::ffi::c_void, radialgradientbrush: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateRadialGradientBrush(this, core::mem::transmute_copy(&radialgradientbrushproperties), core::mem::transmute_copy(&brushproperties), windows_core::from_raw_borrowed(&gradientstopcollection)) {
                Ok(ok__) => {
                    radialgradientbrush.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompatibleRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateCompatibleRenderTarget(this, core::mem::transmute_copy(&desiredsize), core::mem::transmute_copy(&desiredpixelsize), core::mem::transmute_copy(&desiredformat), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    bitmaprendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *const Common::D2D_SIZE_F, layer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateLayer(this, core::mem::transmute_copy(&size)) {
                Ok(ok__) => {
                    layer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMesh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mesh: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1RenderTarget_Impl::CreateMesh(this) {
                Ok(ok__) => {
                    mesh.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawLine(this, core::mem::transmute(&point0), core::mem::transmute(&point1), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle))
        }
        unsafe extern "system" fn DrawRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawRectangle(this, core::mem::transmute_copy(&rect), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle))
        }
        unsafe extern "system" fn FillRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::FillRectangle(this, core::mem::transmute_copy(&rect), windows_core::from_raw_borrowed(&brush))
        }
        unsafe extern "system" fn DrawRoundedRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawRoundedRectangle(this, core::mem::transmute_copy(&roundedrect), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle))
        }
        unsafe extern "system" fn FillRoundedRectangle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::FillRoundedRectangle(this, core::mem::transmute_copy(&roundedrect), windows_core::from_raw_borrowed(&brush))
        }
        unsafe extern "system" fn DrawEllipse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawEllipse(this, core::mem::transmute_copy(&ellipse), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle))
        }
        unsafe extern "system" fn FillEllipse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::FillEllipse(this, core::mem::transmute_copy(&ellipse), windows_core::from_raw_borrowed(&brush))
        }
        unsafe extern "system" fn DrawGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, strokewidth: f32, strokestyle: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawGeometry(this, windows_core::from_raw_borrowed(&geometry), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), windows_core::from_raw_borrowed(&strokestyle))
        }
        unsafe extern "system" fn FillGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, opacitybrush: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::FillGeometry(this, windows_core::from_raw_borrowed(&geometry), windows_core::from_raw_borrowed(&brush), windows_core::from_raw_borrowed(&opacitybrush))
        }
        unsafe extern "system" fn FillMesh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mesh: *mut core::ffi::c_void, brush: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::FillMesh(this, windows_core::from_raw_borrowed(&mesh), windows_core::from_raw_borrowed(&brush))
        }
        unsafe extern "system" fn FillOpacityMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacitymask: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::FillOpacityMask(this, windows_core::from_raw_borrowed(&opacitymask), windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&content), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle))
        }
        unsafe extern "system" fn DrawBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawBitmap(this, windows_core::from_raw_borrowed(&bitmap), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&opacity), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&sourcerectangle))
        }
        unsafe extern "system" fn DrawText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: windows_core::PCWSTR, stringlength: u32, textformat: *mut core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawText(this, core::mem::transmute(&string), core::mem::transmute_copy(&stringlength), windows_core::from_raw_borrowed(&textformat), core::mem::transmute_copy(&layoutrect), windows_core::from_raw_borrowed(&defaultfillbrush), core::mem::transmute_copy(&options), core::mem::transmute_copy(&measuringmode))
        }
        unsafe extern "system" fn DrawTextLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: *mut core::ffi::c_void, defaultfillbrush: *mut core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawTextLayout(this, core::mem::transmute(&origin), windows_core::from_raw_borrowed(&textlayout), windows_core::from_raw_borrowed(&defaultfillbrush), core::mem::transmute_copy(&options))
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: *mut core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::DrawGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), windows_core::from_raw_borrowed(&foregroundbrush), core::mem::transmute_copy(&measuringmode))
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SetTransform(this, core::mem::transmute_copy(&transform))
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetTransform(this, core::mem::transmute_copy(&transform))
        }
        unsafe extern "system" fn SetAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SetAntialiasMode(this, core::mem::transmute_copy(&antialiasmode))
        }
        unsafe extern "system" fn GetAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetAntialiasMode(this)
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SetTextAntialiasMode(this, core::mem::transmute_copy(&textantialiasmode))
        }
        unsafe extern "system" fn GetTextAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetTextAntialiasMode(this)
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SetTextRenderingParams(this, windows_core::from_raw_borrowed(&textrenderingparams))
        }
        unsafe extern "system" fn GetTextRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textrenderingparams: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetTextRenderingParams(this, core::mem::transmute_copy(&textrenderingparams))
        }
        unsafe extern "system" fn SetTags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: u64, tag2: u64)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SetTags(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2))
        }
        unsafe extern "system" fn GetTags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: *mut u64, tag2: *mut u64)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetTags(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2))
        }
        unsafe extern "system" fn PushLayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::PushLayer(this, core::mem::transmute_copy(&layerparameters), windows_core::from_raw_borrowed(&layer))
        }
        unsafe extern "system" fn PopLayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::PopLayer(this)
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::Flush(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
        }
        unsafe extern "system" fn SaveDrawingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstateblock: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SaveDrawingState(this, windows_core::from_raw_borrowed(&drawingstateblock))
        }
        unsafe extern "system" fn RestoreDrawingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingstateblock: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::RestoreDrawingState(this, windows_core::from_raw_borrowed(&drawingstateblock))
        }
        unsafe extern "system" fn PushAxisAlignedClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::PushAxisAlignedClip(this, core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&antialiasmode))
        }
        unsafe extern "system" fn PopAxisAlignedClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::PopAxisAlignedClip(this)
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clearcolor: *const Common::D2D1_COLOR_F)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::Clear(this, core::mem::transmute_copy(&clearcolor))
        }
        unsafe extern "system" fn BeginDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::BeginDraw(this)
        }
        unsafe extern "system" fn EndDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> windows_core::HRESULT
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::EndDraw(this, core::mem::transmute_copy(&tag1), core::mem::transmute_copy(&tag2)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1RenderTarget_Impl::GetPixelFormat(this)
        }
        unsafe extern "system" fn SetDpi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: f32, dpiy: f32)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::SetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy))
        }
        unsafe extern "system" fn GetDpi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy))
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_SIZE_F)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1RenderTarget_Impl::GetSize(this)
        }
        unsafe extern "system" fn GetPixelSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_SIZE_U)
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1RenderTarget_Impl::GetPixelSize(this)
        }
        unsafe extern "system" fn GetMaximumBitmapSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::GetMaximumBitmapSize(this)
        }
        unsafe extern "system" fn IsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1RenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RenderTarget_Impl::IsSupported(this, core::mem::transmute_copy(&rendertargetproperties))
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromWicBitmap: CreateBitmapFromWicBitmap::<Identity, OFFSET>,
            CreateSharedBitmap: CreateSharedBitmap::<Identity, OFFSET>,
            CreateBitmapBrush: CreateBitmapBrush::<Identity, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, OFFSET>,
            CreateGradientStopCollection: CreateGradientStopCollection::<Identity, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, OFFSET>,
            CreateCompatibleRenderTarget: CreateCompatibleRenderTarget::<Identity, OFFSET>,
            CreateLayer: CreateLayer::<Identity, OFFSET>,
            CreateMesh: CreateMesh::<Identity, OFFSET>,
            DrawLine: DrawLine::<Identity, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, OFFSET>,
            FillRectangle: FillRectangle::<Identity, OFFSET>,
            DrawRoundedRectangle: DrawRoundedRectangle::<Identity, OFFSET>,
            FillRoundedRectangle: FillRoundedRectangle::<Identity, OFFSET>,
            DrawEllipse: DrawEllipse::<Identity, OFFSET>,
            FillEllipse: FillEllipse::<Identity, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, OFFSET>,
            FillGeometry: FillGeometry::<Identity, OFFSET>,
            FillMesh: FillMesh::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            DrawText: DrawText::<Identity, OFFSET>,
            DrawTextLayout: DrawTextLayout::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, OFFSET>,
            GetAntialiasMode: GetAntialiasMode::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, OFFSET>,
            SetTags: SetTags::<Identity, OFFSET>,
            GetTags: GetTags::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            PopLayer: PopLayer::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            SaveDrawingState: SaveDrawingState::<Identity, OFFSET>,
            RestoreDrawingState: RestoreDrawingState::<Identity, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            SetDpi: SetDpi::<Identity, OFFSET>,
            GetDpi: GetDpi::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, OFFSET>,
            GetMaximumBitmapSize: GetMaximumBitmapSize::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1Resource_Impl: Sized {
    fn GetFactory(&self, factory: *mut Option<ID2D1Factory>);
}
impl windows_core::RuntimeName for ID2D1Resource {}
impl ID2D1Resource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Resource_Vtbl
    where
        Identity: ID2D1Resource_Impl,
    {
        unsafe extern "system" fn GetFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1Resource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Resource_Impl::GetFactory(this, core::mem::transmute_copy(&factory))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFactory: GetFactory::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1ResourceTexture_Impl: Sized {
    fn Update(&self, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1ResourceTexture {}
impl ID2D1ResourceTexture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1ResourceTexture_Vtbl
    where
        Identity: ID2D1ResourceTexture_Impl,
    {
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1ResourceTexture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1ResourceTexture_Impl::Update(this, core::mem::transmute_copy(&minimumextents), core::mem::transmute_copy(&maximimumextents), core::mem::transmute_copy(&strides), core::mem::transmute_copy(&dimensions), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ResourceTexture as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RoundedRectangleGeometry_Impl: Sized + ID2D1Geometry_Impl {
    fn GetRoundedRect(&self, roundedrect: *mut D2D1_ROUNDED_RECT);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1RoundedRectangleGeometry {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1RoundedRectangleGeometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1RoundedRectangleGeometry_Vtbl
    where
        Identity: ID2D1RoundedRectangleGeometry_Impl,
    {
        unsafe extern "system" fn GetRoundedRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, roundedrect: *mut D2D1_ROUNDED_RECT)
        where
            Identity: ID2D1RoundedRectangleGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1RoundedRectangleGeometry_Impl::GetRoundedRect(this, core::mem::transmute_copy(&roundedrect))
        }
        Self { base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(), GetRoundedRect: GetRoundedRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RoundedRectangleGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SolidColorBrush_Impl: Sized + ID2D1Brush_Impl {
    fn SetColor(&self, color: *const Common::D2D1_COLOR_F);
    fn GetColor(&self) -> Common::D2D1_COLOR_F;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1SolidColorBrush {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1SolidColorBrush_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SolidColorBrush_Vtbl
    where
        Identity: ID2D1SolidColorBrush_Impl,
    {
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const Common::D2D1_COLOR_F)
        where
            Identity: ID2D1SolidColorBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SolidColorBrush_Impl::SetColor(this, core::mem::transmute_copy(&color))
        }
        unsafe extern "system" fn GetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D1_COLOR_F)
        where
            Identity: ID2D1SolidColorBrush_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1SolidColorBrush_Impl::GetColor(this)
        }
        Self { base__: ID2D1Brush_Vtbl::new::<Identity, OFFSET>(), SetColor: SetColor::<Identity, OFFSET>, GetColor: GetColor::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SolidColorBrush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Brush as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SourceTransform_Impl: Sized + ID2D1Transform_Impl {
    fn SetRenderInfo(&self, renderinfo: Option<&ID2D1RenderInfo>) -> windows_core::Result<()>;
    fn Draw(&self, target: Option<&ID2D1Bitmap1>, drawrect: *const super::super::Foundation::RECT, targetorigin: &Common::D2D_POINT_2U) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1SourceTransform {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1SourceTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SourceTransform_Vtbl
    where
        Identity: ID2D1SourceTransform_Impl,
    {
        unsafe extern "system" fn SetRenderInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SourceTransform_Impl::SetRenderInfo(this, windows_core::from_raw_borrowed(&renderinfo)).into()
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> windows_core::HRESULT
        where
            Identity: ID2D1SourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SourceTransform_Impl::Draw(this, windows_core::from_raw_borrowed(&target), core::mem::transmute_copy(&drawrect), core::mem::transmute(&targetorigin)).into()
        }
        Self { base__: ID2D1Transform_Vtbl::new::<Identity, OFFSET>(), SetRenderInfo: SetRenderInfo::<Identity, OFFSET>, Draw: Draw::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SourceTransform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID || iid == &<ID2D1Transform as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SpriteBatch_Impl: Sized + ID2D1Resource_Impl {
    fn AddSprites(&self, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::Result<()>;
    fn SetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::Result<()>;
    fn GetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()>;
    fn GetSpriteCount(&self) -> u32;
    fn Clear(&self);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1SpriteBatch {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1SpriteBatch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SpriteBatch_Vtbl
    where
        Identity: ID2D1SpriteBatch_Impl,
    {
        unsafe extern "system" fn AddSprites<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SpriteBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SpriteBatch_Impl::AddSprites(this, core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&destinationrectangles), core::mem::transmute_copy(&sourcerectangles), core::mem::transmute_copy(&colors), core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&destinationrectanglesstride), core::mem::transmute_copy(&sourcerectanglesstride), core::mem::transmute_copy(&colorsstride), core::mem::transmute_copy(&transformsstride)).into()
        }
        unsafe extern "system" fn SetSprites<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SpriteBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SpriteBatch_Impl::SetSprites(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&destinationrectangles), core::mem::transmute_copy(&sourcerectangles), core::mem::transmute_copy(&colors), core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&destinationrectanglesstride), core::mem::transmute_copy(&sourcerectanglesstride), core::mem::transmute_copy(&colorsstride), core::mem::transmute_copy(&transformsstride)).into()
        }
        unsafe extern "system" fn GetSprites<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT
        where
            Identity: ID2D1SpriteBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SpriteBatch_Impl::GetSprites(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&destinationrectangles), core::mem::transmute_copy(&sourcerectangles), core::mem::transmute_copy(&colors), core::mem::transmute_copy(&transforms)).into()
        }
        unsafe extern "system" fn GetSpriteCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SpriteBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SpriteBatch_Impl::GetSpriteCount(this)
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1SpriteBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SpriteBatch_Impl::Clear(this)
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            AddSprites: AddSprites::<Identity, OFFSET>,
            SetSprites: SetSprites::<Identity, OFFSET>,
            GetSprites: GetSprites::<Identity, OFFSET>,
            GetSpriteCount: GetSpriteCount::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SpriteBatch as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1StrokeStyle_Impl: Sized + ID2D1Resource_Impl {
    fn GetStartCap(&self) -> D2D1_CAP_STYLE;
    fn GetEndCap(&self) -> D2D1_CAP_STYLE;
    fn GetDashCap(&self) -> D2D1_CAP_STYLE;
    fn GetMiterLimit(&self) -> f32;
    fn GetLineJoin(&self) -> D2D1_LINE_JOIN;
    fn GetDashOffset(&self) -> f32;
    fn GetDashStyle(&self) -> D2D1_DASH_STYLE;
    fn GetDashesCount(&self) -> u32;
    fn GetDashes(&self, dashes: *mut f32, dashescount: u32);
}
impl windows_core::RuntimeName for ID2D1StrokeStyle {}
impl ID2D1StrokeStyle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1StrokeStyle_Vtbl
    where
        Identity: ID2D1StrokeStyle_Impl,
    {
        unsafe extern "system" fn GetStartCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_CAP_STYLE
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetStartCap(this)
        }
        unsafe extern "system" fn GetEndCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_CAP_STYLE
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetEndCap(this)
        }
        unsafe extern "system" fn GetDashCap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_CAP_STYLE
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetDashCap(this)
        }
        unsafe extern "system" fn GetMiterLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetMiterLimit(this)
        }
        unsafe extern "system" fn GetLineJoin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_LINE_JOIN
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetLineJoin(this)
        }
        unsafe extern "system" fn GetDashOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetDashOffset(this)
        }
        unsafe extern "system" fn GetDashStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_DASH_STYLE
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetDashStyle(this)
        }
        unsafe extern "system" fn GetDashesCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetDashesCount(this)
        }
        unsafe extern "system" fn GetDashes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *mut f32, dashescount: u32)
        where
            Identity: ID2D1StrokeStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle_Impl::GetDashes(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount))
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetStartCap: GetStartCap::<Identity, OFFSET>,
            GetEndCap: GetEndCap::<Identity, OFFSET>,
            GetDashCap: GetDashCap::<Identity, OFFSET>,
            GetMiterLimit: GetMiterLimit::<Identity, OFFSET>,
            GetLineJoin: GetLineJoin::<Identity, OFFSET>,
            GetDashOffset: GetDashOffset::<Identity, OFFSET>,
            GetDashStyle: GetDashStyle::<Identity, OFFSET>,
            GetDashesCount: GetDashesCount::<Identity, OFFSET>,
            GetDashes: GetDashes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1StrokeStyle as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1StrokeStyle1_Impl: Sized + ID2D1StrokeStyle_Impl {
    fn GetStrokeTransformType(&self) -> D2D1_STROKE_TRANSFORM_TYPE;
}
impl windows_core::RuntimeName for ID2D1StrokeStyle1 {}
impl ID2D1StrokeStyle1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1StrokeStyle1_Vtbl
    where
        Identity: ID2D1StrokeStyle1_Impl,
    {
        unsafe extern "system" fn GetStrokeTransformType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE
        where
            Identity: ID2D1StrokeStyle1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1StrokeStyle1_Impl::GetStrokeTransformType(this)
        }
        Self { base__: ID2D1StrokeStyle_Vtbl::new::<Identity, OFFSET>(), GetStrokeTransformType: GetStrokeTransformType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1StrokeStyle1 as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1StrokeStyle as windows_core::Interface>::IID
    }
}
pub trait ID2D1SvgAttribute_Impl: Sized + ID2D1Resource_Impl {
    fn GetElement(&self, element: *mut Option<ID2D1SvgElement>);
    fn Clone(&self) -> windows_core::Result<ID2D1SvgAttribute>;
}
impl windows_core::RuntimeName for ID2D1SvgAttribute {}
impl ID2D1SvgAttribute_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgAttribute_Vtbl
    where
        Identity: ID2D1SvgAttribute_Impl,
    {
        unsafe extern "system" fn GetElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgAttribute_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgAttribute_Impl::GetElement(this, core::mem::transmute_copy(&element))
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgAttribute_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgAttribute_Impl::Clone(this) {
                Ok(ok__) => {
                    attribute.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), GetElement: GetElement::<Identity, OFFSET>, Clone: Clone::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
pub trait ID2D1SvgDocument_Impl: Sized + ID2D1Resource_Impl {
    fn SetViewportSize(&self, viewportsize: &Common::D2D_SIZE_F) -> windows_core::Result<()>;
    fn GetViewportSize(&self) -> Common::D2D_SIZE_F;
    fn SetRoot(&self, root: Option<&ID2D1SvgElement>) -> windows_core::Result<()>;
    fn GetRoot(&self, root: *mut Option<ID2D1SvgElement>);
    fn FindElementById(&self, id: &windows_core::PCWSTR) -> windows_core::Result<ID2D1SvgElement>;
    fn Serialize(&self, outputxmlstream: Option<&super::super::System::Com::IStream>, subtree: Option<&ID2D1SvgElement>) -> windows_core::Result<()>;
    fn Deserialize(&self, inputxmlstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<ID2D1SvgElement>;
    fn CreatePaint(&self, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: &windows_core::PCWSTR) -> windows_core::Result<ID2D1SvgPaint>;
    fn CreateStrokeDashArray(&self, dashes: *const D2D1_SVG_LENGTH, dashescount: u32) -> windows_core::Result<ID2D1SvgStrokeDashArray>;
    fn CreatePointCollection(&self, points: *const Common::D2D_POINT_2F, pointscount: u32) -> windows_core::Result<ID2D1SvgPointCollection>;
    fn CreatePathData(&self, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32) -> windows_core::Result<ID2D1SvgPathData>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ID2D1SvgDocument {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl ID2D1SvgDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgDocument_Vtbl
    where
        Identity: ID2D1SvgDocument_Impl,
    {
        unsafe extern "system" fn SetViewportSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewportsize: Common::D2D_SIZE_F) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgDocument_Impl::SetViewportSize(this, core::mem::transmute(&viewportsize)).into()
        }
        unsafe extern "system" fn GetViewportSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Common::D2D_SIZE_F)
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            *result__ = ID2D1SvgDocument_Impl::GetViewportSize(this)
        }
        unsafe extern "system" fn SetRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, root: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgDocument_Impl::SetRoot(this, windows_core::from_raw_borrowed(&root)).into()
        }
        unsafe extern "system" fn GetRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, root: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgDocument_Impl::GetRoot(this, core::mem::transmute_copy(&root))
        }
        unsafe extern "system" fn FindElementById<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PCWSTR, svgelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgDocument_Impl::FindElementById(this, core::mem::transmute(&id)) {
                Ok(ok__) => {
                    svgelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputxmlstream: *mut core::ffi::c_void, subtree: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgDocument_Impl::Serialize(this, windows_core::from_raw_borrowed(&outputxmlstream), windows_core::from_raw_borrowed(&subtree)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputxmlstream: *mut core::ffi::c_void, subtree: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgDocument_Impl::Deserialize(this, windows_core::from_raw_borrowed(&inputxmlstream)) {
                Ok(ok__) => {
                    subtree.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePaint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: windows_core::PCWSTR, paint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgDocument_Impl::CreatePaint(this, core::mem::transmute_copy(&painttype), core::mem::transmute_copy(&color), core::mem::transmute(&id)) {
                Ok(ok__) => {
                    paint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStrokeDashArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgDocument_Impl::CreateStrokeDashArray(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount)) {
                Ok(ok__) => {
                    strokedasharray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePointCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, pointcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgDocument_Impl::CreatePointCollection(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount)) {
                Ok(ok__) => {
                    pointcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePathData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgDocument_Impl::CreatePathData(this, core::mem::transmute_copy(&segmentdata), core::mem::transmute_copy(&segmentdatacount), core::mem::transmute_copy(&commands), core::mem::transmute_copy(&commandscount)) {
                Ok(ok__) => {
                    pathdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetViewportSize: SetViewportSize::<Identity, OFFSET>,
            GetViewportSize: GetViewportSize::<Identity, OFFSET>,
            SetRoot: SetRoot::<Identity, OFFSET>,
            GetRoot: GetRoot::<Identity, OFFSET>,
            FindElementById: FindElementById::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
            CreatePaint: CreatePaint::<Identity, OFFSET>,
            CreateStrokeDashArray: CreateStrokeDashArray::<Identity, OFFSET>,
            CreatePointCollection: CreatePointCollection::<Identity, OFFSET>,
            CreatePathData: CreatePathData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgDocument as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1SvgElement_Impl: Sized + ID2D1Resource_Impl {
    fn GetDocument(&self, document: *mut Option<ID2D1SvgDocument>);
    fn GetTagName(&self, name: windows_core::PWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetTagNameLength(&self) -> u32;
    fn IsTextContent(&self) -> super::super::Foundation::BOOL;
    fn GetParent(&self, parent: *mut Option<ID2D1SvgElement>);
    fn HasChildren(&self) -> super::super::Foundation::BOOL;
    fn GetFirstChild(&self, child: *mut Option<ID2D1SvgElement>);
    fn GetLastChild(&self, child: *mut Option<ID2D1SvgElement>);
    fn GetPreviousChild(&self, referencechild: Option<&ID2D1SvgElement>) -> windows_core::Result<ID2D1SvgElement>;
    fn GetNextChild(&self, referencechild: Option<&ID2D1SvgElement>) -> windows_core::Result<ID2D1SvgElement>;
    fn InsertChildBefore(&self, newchild: Option<&ID2D1SvgElement>, referencechild: Option<&ID2D1SvgElement>) -> windows_core::Result<()>;
    fn AppendChild(&self, newchild: Option<&ID2D1SvgElement>) -> windows_core::Result<()>;
    fn ReplaceChild(&self, newchild: Option<&ID2D1SvgElement>, oldchild: Option<&ID2D1SvgElement>) -> windows_core::Result<()>;
    fn RemoveChild(&self, oldchild: Option<&ID2D1SvgElement>) -> windows_core::Result<()>;
    fn CreateChild(&self, tagname: &windows_core::PCWSTR) -> windows_core::Result<ID2D1SvgElement>;
    fn IsAttributeSpecified(&self, name: &windows_core::PCWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetSpecifiedAttributeCount(&self) -> u32;
    fn GetSpecifiedAttributeName(&self, index: u32, name: windows_core::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSpecifiedAttributeNameLength(&self, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RemoveAttribute(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetTextValue(&self, name: &windows_core::PCWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetTextValue(&self, name: windows_core::PWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetTextValueLength(&self) -> u32;
    fn SetAttributeValue(&self, name: &windows_core::PCWSTR, value: Option<&ID2D1SvgAttribute>) -> windows_core::Result<()>;
    fn SetAttributeValue2(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::Result<()>;
    fn SetAttributeValue3(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAttributeValue(&self, name: &windows_core::PCWSTR, riid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAttributeValue2(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::Result<()>;
    fn GetAttributeValue3(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: windows_core::PWSTR, valuecount: u32) -> windows_core::Result<()>;
    fn GetAttributeValueLength(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ID2D1SvgElement {}
impl ID2D1SvgElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgElement_Vtbl
    where
        Identity: ID2D1SvgElement_Impl,
    {
        unsafe extern "system" fn GetDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetDocument(this, core::mem::transmute_copy(&document))
        }
        unsafe extern "system" fn GetTagName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PWSTR, namecount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetTagName(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn GetTagNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetTagNameLength(this)
        }
        unsafe extern "system" fn IsTextContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::IsTextContent(this)
        }
        unsafe extern "system" fn GetParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetParent(this, core::mem::transmute_copy(&parent))
        }
        unsafe extern "system" fn HasChildren<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::HasChildren(this)
        }
        unsafe extern "system" fn GetFirstChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, child: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetFirstChild(this, core::mem::transmute_copy(&child))
        }
        unsafe extern "system" fn GetLastChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, child: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetLastChild(this, core::mem::transmute_copy(&child))
        }
        unsafe extern "system" fn GetPreviousChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void, previouschild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgElement_Impl::GetPreviousChild(this, windows_core::from_raw_borrowed(&referencechild)) {
                Ok(ok__) => {
                    previouschild.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void, nextchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgElement_Impl::GetNextChild(this, windows_core::from_raw_borrowed(&referencechild)) {
                Ok(ok__) => {
                    nextchild.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChildBefore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::InsertChildBefore(this, windows_core::from_raw_borrowed(&newchild), windows_core::from_raw_borrowed(&referencechild)).into()
        }
        unsafe extern "system" fn AppendChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::AppendChild(this, windows_core::from_raw_borrowed(&newchild)).into()
        }
        unsafe extern "system" fn ReplaceChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, oldchild: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::ReplaceChild(this, windows_core::from_raw_borrowed(&newchild), windows_core::from_raw_borrowed(&oldchild)).into()
        }
        unsafe extern "system" fn RemoveChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldchild: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::RemoveChild(this, windows_core::from_raw_borrowed(&oldchild)).into()
        }
        unsafe extern "system" fn CreateChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: windows_core::PCWSTR, newchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgElement_Impl::CreateChild(this, core::mem::transmute(&tagname)) {
                Ok(ok__) => {
                    newchild.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAttributeSpecified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::IsAttributeSpecified(this, core::mem::transmute(&name), core::mem::transmute_copy(&inherited))
        }
        unsafe extern "system" fn GetSpecifiedAttributeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetSpecifiedAttributeCount(this)
        }
        unsafe extern "system" fn GetSpecifiedAttributeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: windows_core::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetSpecifiedAttributeName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount), core::mem::transmute_copy(&inherited)).into()
        }
        unsafe extern "system" fn GetSpecifiedAttributeNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetSpecifiedAttributeNameLength(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&namelength), core::mem::transmute_copy(&inherited)).into()
        }
        unsafe extern "system" fn RemoveAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::RemoveAttribute(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn SetTextValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, namecount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::SetTextValue(this, core::mem::transmute(&name), core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn GetTextValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PWSTR, namecount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetTextValue(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn GetTextValueLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetTextValueLength(this)
        }
        unsafe extern "system" fn SetAttributeValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::SetAttributeValue(this, core::mem::transmute(&name), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetAttributeValue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::SetAttributeValue2(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value), core::mem::transmute_copy(&valuesizeinbytes)).into()
        }
        unsafe extern "system" fn SetAttributeValue3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::SetAttributeValue3(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetAttributeValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, riid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetAttributeValue(this, core::mem::transmute(&name), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAttributeValue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetAttributeValue2(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value), core::mem::transmute_copy(&valuesizeinbytes)).into()
        }
        unsafe extern "system" fn GetAttributeValue3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: windows_core::PWSTR, valuecount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgElement_Impl::GetAttributeValue3(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value), core::mem::transmute_copy(&valuecount)).into()
        }
        unsafe extern "system" fn GetAttributeValueLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgElement_Impl::GetAttributeValueLength(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    valuelength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetDocument: GetDocument::<Identity, OFFSET>,
            GetTagName: GetTagName::<Identity, OFFSET>,
            GetTagNameLength: GetTagNameLength::<Identity, OFFSET>,
            IsTextContent: IsTextContent::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
            HasChildren: HasChildren::<Identity, OFFSET>,
            GetFirstChild: GetFirstChild::<Identity, OFFSET>,
            GetLastChild: GetLastChild::<Identity, OFFSET>,
            GetPreviousChild: GetPreviousChild::<Identity, OFFSET>,
            GetNextChild: GetNextChild::<Identity, OFFSET>,
            InsertChildBefore: InsertChildBefore::<Identity, OFFSET>,
            AppendChild: AppendChild::<Identity, OFFSET>,
            ReplaceChild: ReplaceChild::<Identity, OFFSET>,
            RemoveChild: RemoveChild::<Identity, OFFSET>,
            CreateChild: CreateChild::<Identity, OFFSET>,
            IsAttributeSpecified: IsAttributeSpecified::<Identity, OFFSET>,
            GetSpecifiedAttributeCount: GetSpecifiedAttributeCount::<Identity, OFFSET>,
            GetSpecifiedAttributeName: GetSpecifiedAttributeName::<Identity, OFFSET>,
            GetSpecifiedAttributeNameLength: GetSpecifiedAttributeNameLength::<Identity, OFFSET>,
            RemoveAttribute: RemoveAttribute::<Identity, OFFSET>,
            SetTextValue: SetTextValue::<Identity, OFFSET>,
            GetTextValue: GetTextValue::<Identity, OFFSET>,
            GetTextValueLength: GetTextValueLength::<Identity, OFFSET>,
            SetAttributeValue: SetAttributeValue::<Identity, OFFSET>,
            SetAttributeValue2: SetAttributeValue2::<Identity, OFFSET>,
            SetAttributeValue3: SetAttributeValue3::<Identity, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, OFFSET>,
            GetAttributeValue2: GetAttributeValue2::<Identity, OFFSET>,
            GetAttributeValue3: GetAttributeValue3::<Identity, OFFSET>,
            GetAttributeValueLength: GetAttributeValueLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgElement as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
pub trait ID2D1SvgGlyphStyle_Impl: Sized + ID2D1Resource_Impl {
    fn SetFill(&self, brush: Option<&ID2D1Brush>) -> windows_core::Result<()>;
    fn GetFill(&self, brush: *mut Option<ID2D1Brush>);
    fn SetStroke(&self, brush: Option<&ID2D1Brush>, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> windows_core::Result<()>;
    fn GetStrokeDashesCount(&self) -> u32;
    fn GetStroke(&self, brush: *mut Option<ID2D1Brush>, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32);
}
impl windows_core::RuntimeName for ID2D1SvgGlyphStyle {}
impl ID2D1SvgGlyphStyle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgGlyphStyle_Vtbl
    where
        Identity: ID2D1SvgGlyphStyle_Impl,
    {
        unsafe extern "system" fn SetFill<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgGlyphStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgGlyphStyle_Impl::SetFill(this, windows_core::from_raw_borrowed(&brush)).into()
        }
        unsafe extern "system" fn GetFill<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1SvgGlyphStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgGlyphStyle_Impl::GetFill(this, core::mem::transmute_copy(&brush))
        }
        unsafe extern "system" fn SetStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgGlyphStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgGlyphStyle_Impl::SetStroke(this, windows_core::from_raw_borrowed(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&dashoffset)).into()
        }
        unsafe extern "system" fn GetStrokeDashesCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgGlyphStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgGlyphStyle_Impl::GetStrokeDashesCount(this)
        }
        unsafe extern "system" fn GetStroke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32)
        where
            Identity: ID2D1SvgGlyphStyle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgGlyphStyle_Impl::GetStroke(this, core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&dashoffset))
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetFill: SetFill::<Identity, OFFSET>,
            GetFill: GetFill::<Identity, OFFSET>,
            SetStroke: SetStroke::<Identity, OFFSET>,
            GetStrokeDashesCount: GetStrokeDashesCount::<Identity, OFFSET>,
            GetStroke: GetStroke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgGlyphStyle as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPaint_Impl: Sized + ID2D1SvgAttribute_Impl {
    fn SetPaintType(&self, painttype: D2D1_SVG_PAINT_TYPE) -> windows_core::Result<()>;
    fn GetPaintType(&self) -> D2D1_SVG_PAINT_TYPE;
    fn SetColor(&self, color: *const Common::D2D1_COLOR_F) -> windows_core::Result<()>;
    fn GetColor(&self, color: *mut Common::D2D1_COLOR_F);
    fn SetId(&self, id: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetId(&self, id: windows_core::PWSTR, idcount: u32) -> windows_core::Result<()>;
    fn GetIdLength(&self) -> u32;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1SvgPaint {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1SvgPaint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgPaint_Vtbl
    where
        Identity: ID2D1SvgPaint_Impl,
    {
        unsafe extern "system" fn SetPaintType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::SetPaintType(this, core::mem::transmute_copy(&painttype)).into()
        }
        unsafe extern "system" fn GetPaintType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::GetPaintType(this)
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn GetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut Common::D2D1_COLOR_F)
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::GetColor(this, core::mem::transmute_copy(&color))
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::SetId(this, core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PWSTR, idcount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::GetId(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&idcount)).into()
        }
        unsafe extern "system" fn GetIdLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgPaint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPaint_Impl::GetIdLength(this)
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            SetPaintType: SetPaintType::<Identity, OFFSET>,
            GetPaintType: GetPaintType::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            GetColor: GetColor::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetIdLength: GetIdLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgPaint as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPathData_Impl: Sized + ID2D1SvgAttribute_Impl {
    fn RemoveSegmentDataAtEnd(&self, datacount: u32) -> windows_core::Result<()>;
    fn UpdateSegmentData(&self, data: *const f32, datacount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetSegmentData(&self, data: *mut f32, datacount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetSegmentDataCount(&self) -> u32;
    fn RemoveCommandsAtEnd(&self, commandscount: u32) -> windows_core::Result<()>;
    fn UpdateCommands(&self, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetCommands(&self, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetCommandsCount(&self) -> u32;
    fn CreatePathGeometry(&self, fillmode: Common::D2D1_FILL_MODE) -> windows_core::Result<ID2D1PathGeometry1>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1SvgPathData {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1SvgPathData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgPathData_Vtbl
    where
        Identity: ID2D1SvgPathData_Impl,
    {
        unsafe extern "system" fn RemoveSegmentDataAtEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, datacount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::RemoveSegmentDataAtEnd(this, core::mem::transmute_copy(&datacount)).into()
        }
        unsafe extern "system" fn UpdateSegmentData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *const f32, datacount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::UpdateSegmentData(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetSegmentData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut f32, datacount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::GetSegmentData(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::GetSegmentDataCount(this)
        }
        unsafe extern "system" fn RemoveCommandsAtEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::RemoveCommandsAtEnd(this, core::mem::transmute_copy(&commandscount)).into()
        }
        unsafe extern "system" fn UpdateCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::UpdateCommands(this, core::mem::transmute_copy(&commands), core::mem::transmute_copy(&commandscount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::GetCommands(this, core::mem::transmute_copy(&commands), core::mem::transmute_copy(&commandscount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetCommandsCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPathData_Impl::GetCommandsCount(this)
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, pathgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPathData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1SvgPathData_Impl::CreatePathGeometry(this, core::mem::transmute_copy(&fillmode)) {
                Ok(ok__) => {
                    pathgeometry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            RemoveSegmentDataAtEnd: RemoveSegmentDataAtEnd::<Identity, OFFSET>,
            UpdateSegmentData: UpdateSegmentData::<Identity, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, OFFSET>,
            RemoveCommandsAtEnd: RemoveCommandsAtEnd::<Identity, OFFSET>,
            UpdateCommands: UpdateCommands::<Identity, OFFSET>,
            GetCommands: GetCommands::<Identity, OFFSET>,
            GetCommandsCount: GetCommandsCount::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgPathData as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPointCollection_Impl: Sized + ID2D1SvgAttribute_Impl {
    fn RemovePointsAtEnd(&self, pointscount: u32) -> windows_core::Result<()>;
    fn UpdatePoints(&self, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetPoints(&self, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetPointsCount(&self) -> u32;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1SvgPointCollection {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1SvgPointCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgPointCollection_Vtbl
    where
        Identity: ID2D1SvgPointCollection_Impl,
    {
        unsafe extern "system" fn RemovePointsAtEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPointCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPointCollection_Impl::RemovePointsAtEnd(this, core::mem::transmute_copy(&pointscount)).into()
        }
        unsafe extern "system" fn UpdatePoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPointCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPointCollection_Impl::UpdatePoints(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgPointCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPointCollection_Impl::GetPoints(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetPointsCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgPointCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgPointCollection_Impl::GetPointsCount(this)
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            RemovePointsAtEnd: RemovePointsAtEnd::<Identity, OFFSET>,
            UpdatePoints: UpdatePoints::<Identity, OFFSET>,
            GetPoints: GetPoints::<Identity, OFFSET>,
            GetPointsCount: GetPointsCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgPointCollection as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
pub trait ID2D1SvgStrokeDashArray_Impl: Sized + ID2D1SvgAttribute_Impl {
    fn RemoveDashesAtEnd(&self, dashescount: u32) -> windows_core::Result<()>;
    fn UpdateDashes(&self, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn UpdateDashes2(&self, dashes: *const f32, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetDashes(&self, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetDashes2(&self, dashes: *mut f32, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetDashesCount(&self) -> u32;
}
impl windows_core::RuntimeName for ID2D1SvgStrokeDashArray {}
impl ID2D1SvgStrokeDashArray_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1SvgStrokeDashArray_Vtbl
    where
        Identity: ID2D1SvgStrokeDashArray_Impl,
    {
        unsafe extern "system" fn RemoveDashesAtEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashescount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgStrokeDashArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgStrokeDashArray_Impl::RemoveDashesAtEnd(this, core::mem::transmute_copy(&dashescount)).into()
        }
        unsafe extern "system" fn UpdateDashes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgStrokeDashArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgStrokeDashArray_Impl::UpdateDashes(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn UpdateDashes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *const f32, dashescount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgStrokeDashArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgStrokeDashArray_Impl::UpdateDashes2(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetDashes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgStrokeDashArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgStrokeDashArray_Impl::GetDashes(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetDashes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *mut f32, dashescount: u32, startindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1SvgStrokeDashArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgStrokeDashArray_Impl::GetDashes2(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
        }
        unsafe extern "system" fn GetDashesCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1SvgStrokeDashArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1SvgStrokeDashArray_Impl::GetDashesCount(this)
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            RemoveDashesAtEnd: RemoveDashesAtEnd::<Identity, OFFSET>,
            UpdateDashes: UpdateDashes::<Identity, OFFSET>,
            UpdateDashes2: UpdateDashes2::<Identity, OFFSET>,
            GetDashes: GetDashes::<Identity, OFFSET>,
            GetDashes2: GetDashes2::<Identity, OFFSET>,
            GetDashesCount: GetDashesCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgStrokeDashArray as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1TessellationSink_Impl: Sized {
    fn AddTriangles(&self, triangles: *const D2D1_TRIANGLE, trianglescount: u32);
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1TessellationSink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1TessellationSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1TessellationSink_Vtbl
    where
        Identity: ID2D1TessellationSink_Impl,
    {
        unsafe extern "system" fn AddTriangles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, triangles: *const D2D1_TRIANGLE, trianglescount: u32)
        where
            Identity: ID2D1TessellationSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TessellationSink_Impl::AddTriangles(this, core::mem::transmute_copy(&triangles), core::mem::transmute_copy(&trianglescount))
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1TessellationSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TessellationSink_Impl::Close(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddTriangles: AddTriangles::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TessellationSink as windows_core::Interface>::IID
    }
}
pub trait ID2D1Transform_Impl: Sized + ID2D1TransformNode_Impl {
    fn MapOutputRectToInputRects(&self, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> windows_core::Result<()>;
    fn MapInputRectsToOutputRect(&self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn MapInvalidRect(&self, inputindex: u32, invalidinputrect: &super::super::Foundation::RECT) -> windows_core::Result<super::super::Foundation::RECT>;
}
impl windows_core::RuntimeName for ID2D1Transform {}
impl ID2D1Transform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1Transform_Vtbl
    where
        Identity: ID2D1Transform_Impl,
    {
        unsafe extern "system" fn MapOutputRectToInputRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1Transform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Transform_Impl::MapOutputRectToInputRects(this, core::mem::transmute_copy(&outputrect), core::mem::transmute_copy(&inputrects), core::mem::transmute_copy(&inputrectscount)).into()
        }
        unsafe extern "system" fn MapInputRectsToOutputRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ID2D1Transform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1Transform_Impl::MapInputRectsToOutputRect(this, core::mem::transmute_copy(&inputrects), core::mem::transmute_copy(&inputopaquesubrects), core::mem::transmute_copy(&inputrectcount), core::mem::transmute_copy(&outputrect), core::mem::transmute_copy(&outputopaquesubrect)).into()
        }
        unsafe extern "system" fn MapInvalidRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputindex: u32, invalidinputrect: super::super::Foundation::RECT, invalidoutputrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: ID2D1Transform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID2D1Transform_Impl::MapInvalidRect(this, core::mem::transmute_copy(&inputindex), core::mem::transmute(&invalidinputrect)) {
                Ok(ok__) => {
                    invalidoutputrect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID2D1TransformNode_Vtbl::new::<Identity, OFFSET>(),
            MapOutputRectToInputRects: MapOutputRectToInputRects::<Identity, OFFSET>,
            MapInputRectsToOutputRect: MapInputRectsToOutputRect::<Identity, OFFSET>,
            MapInvalidRect: MapInvalidRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Transform as windows_core::Interface>::IID || iid == &<ID2D1TransformNode as windows_core::Interface>::IID
    }
}
pub trait ID2D1TransformGraph_Impl: Sized {
    fn GetInputCount(&self) -> u32;
    fn SetSingleTransformNode(&self, node: Option<&ID2D1TransformNode>) -> windows_core::Result<()>;
    fn AddNode(&self, node: Option<&ID2D1TransformNode>) -> windows_core::Result<()>;
    fn RemoveNode(&self, node: Option<&ID2D1TransformNode>) -> windows_core::Result<()>;
    fn SetOutputNode(&self, node: Option<&ID2D1TransformNode>) -> windows_core::Result<()>;
    fn ConnectNode(&self, fromnode: Option<&ID2D1TransformNode>, tonode: Option<&ID2D1TransformNode>, tonodeinputindex: u32) -> windows_core::Result<()>;
    fn ConnectToEffectInput(&self, toeffectinputindex: u32, node: Option<&ID2D1TransformNode>, tonodeinputindex: u32) -> windows_core::Result<()>;
    fn Clear(&self);
    fn SetPassthroughGraph(&self, effectinputindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1TransformGraph {}
impl ID2D1TransformGraph_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1TransformGraph_Vtbl
    where
        Identity: ID2D1TransformGraph_Impl,
    {
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::GetInputCount(this)
        }
        unsafe extern "system" fn SetSingleTransformNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::SetSingleTransformNode(this, windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn AddNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::AddNode(this, windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn RemoveNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::RemoveNode(this, windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn SetOutputNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::SetOutputNode(this, windows_core::from_raw_borrowed(&node)).into()
        }
        unsafe extern "system" fn ConnectNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fromnode: *mut core::ffi::c_void, tonode: *mut core::ffi::c_void, tonodeinputindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::ConnectNode(this, windows_core::from_raw_borrowed(&fromnode), windows_core::from_raw_borrowed(&tonode), core::mem::transmute_copy(&tonodeinputindex)).into()
        }
        unsafe extern "system" fn ConnectToEffectInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, toeffectinputindex: u32, node: *mut core::ffi::c_void, tonodeinputindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::ConnectToEffectInput(this, core::mem::transmute_copy(&toeffectinputindex), windows_core::from_raw_borrowed(&node), core::mem::transmute_copy(&tonodeinputindex)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::Clear(this)
        }
        unsafe extern "system" fn SetPassthroughGraph<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectinputindex: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1TransformGraph_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformGraph_Impl::SetPassthroughGraph(this, core::mem::transmute_copy(&effectinputindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            SetSingleTransformNode: SetSingleTransformNode::<Identity, OFFSET>,
            AddNode: AddNode::<Identity, OFFSET>,
            RemoveNode: RemoveNode::<Identity, OFFSET>,
            SetOutputNode: SetOutputNode::<Identity, OFFSET>,
            ConnectNode: ConnectNode::<Identity, OFFSET>,
            ConnectToEffectInput: ConnectToEffectInput::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetPassthroughGraph: SetPassthroughGraph::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TransformGraph as windows_core::Interface>::IID
    }
}
pub trait ID2D1TransformNode_Impl: Sized {
    fn GetInputCount(&self) -> u32;
}
impl windows_core::RuntimeName for ID2D1TransformNode {}
impl ID2D1TransformNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1TransformNode_Vtbl
    where
        Identity: ID2D1TransformNode_Impl,
    {
        unsafe extern "system" fn GetInputCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID2D1TransformNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformNode_Impl::GetInputCount(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInputCount: GetInputCount::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TransformNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1TransformedGeometry_Impl: Sized + ID2D1Geometry_Impl {
    fn GetSourceGeometry(&self, sourcegeometry: *mut Option<ID2D1Geometry>);
    fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for ID2D1TransformedGeometry {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ID2D1TransformedGeometry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1TransformedGeometry_Vtbl
    where
        Identity: ID2D1TransformedGeometry_Impl,
    {
        unsafe extern "system" fn GetSourceGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcegeometry: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1TransformedGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformedGeometry_Impl::GetSourceGeometry(this, core::mem::transmute_copy(&sourcegeometry))
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2)
        where
            Identity: ID2D1TransformedGeometry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformedGeometry_Impl::GetTransform(this, core::mem::transmute_copy(&transform))
        }
        Self {
            base__: ID2D1Geometry_Vtbl::new::<Identity, OFFSET>(),
            GetSourceGeometry: GetSourceGeometry::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TransformedGeometry as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Geometry as windows_core::Interface>::IID
    }
}
pub trait ID2D1TransformedImageSource_Impl: Sized + ID2D1Image_Impl {
    fn GetSource(&self, imagesource: *mut Option<ID2D1ImageSource>);
    fn GetProperties(&self, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES);
}
impl windows_core::RuntimeName for ID2D1TransformedImageSource {}
impl ID2D1TransformedImageSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1TransformedImageSource_Vtbl
    where
        Identity: ID2D1TransformedImageSource_Impl,
    {
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagesource: *mut *mut core::ffi::c_void)
        where
            Identity: ID2D1TransformedImageSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformedImageSource_Impl::GetSource(this, core::mem::transmute_copy(&imagesource))
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES)
        where
            Identity: ID2D1TransformedImageSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1TransformedImageSource_Impl::GetProperties(this, core::mem::transmute_copy(&properties))
        }
        Self { base__: ID2D1Image_Vtbl::new::<Identity, OFFSET>(), GetSource: GetSource::<Identity, OFFSET>, GetProperties: GetProperties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TransformedImageSource as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1Image as windows_core::Interface>::IID
    }
}
pub trait ID2D1VertexBuffer_Impl: Sized {
    fn Map(&self, data: *mut *mut u8, buffersize: u32) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID2D1VertexBuffer {}
impl ID2D1VertexBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID2D1VertexBuffer_Vtbl
    where
        Identity: ID2D1VertexBuffer_Impl,
    {
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, buffersize: u32) -> windows_core::HRESULT
        where
            Identity: ID2D1VertexBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1VertexBuffer_Impl::Map(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID2D1VertexBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID2D1VertexBuffer_Impl::Unmap(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Map: Map::<Identity, OFFSET>, Unmap: Unmap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1VertexBuffer as windows_core::Interface>::IID
    }
}

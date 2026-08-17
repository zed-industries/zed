#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[cfg(feature = "Foundation_Numerics")]
#[inline]
pub unsafe fn D2D1ComputeMaximumScaleFactor(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> f32 {
    windows_targets::link!("d2d1.dll" "system" fn D2D1ComputeMaximumScaleFactor(matrix : *const super::super::super::Foundation::Numerics:: Matrix3x2) -> f32);
    D2D1ComputeMaximumScaleFactor(matrix)
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[inline]
pub unsafe fn D2D1ConvertColorSpace(sourcecolorspace: D2D1_COLOR_SPACE, destinationcolorspace: D2D1_COLOR_SPACE, color: *const Common::D2D1_COLOR_F) -> Common::D2D1_COLOR_F {
    windows_targets::link!("d2d1.dll" "system" fn D2D1ConvertColorSpace(sourcecolorspace : D2D1_COLOR_SPACE, destinationcolorspace : D2D1_COLOR_SPACE, color : *const Common:: D2D1_COLOR_F) -> Common:: D2D1_COLOR_F);
    D2D1ConvertColorSpace(sourcecolorspace, destinationcolorspace, color)
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D2D1CreateDevice<P0>(dxgidevice: P0, creationproperties: Option<*const D2D1_CREATION_PROPERTIES>) -> windows_core::Result<ID2D1Device>
where
    P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
{
    windows_targets::link!("d2d1.dll" "system" fn D2D1CreateDevice(dxgidevice : * mut core::ffi::c_void, creationproperties : *const D2D1_CREATION_PROPERTIES, d2ddevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D2D1CreateDevice(dxgidevice.param().abi(), core::mem::transmute(creationproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn D2D1CreateDeviceContext<P0>(dxgisurface: P0, creationproperties: Option<*const D2D1_CREATION_PROPERTIES>) -> windows_core::Result<ID2D1DeviceContext>
where
    P0: windows_core::Param<super::Dxgi::IDXGISurface>,
{
    windows_targets::link!("d2d1.dll" "system" fn D2D1CreateDeviceContext(dxgisurface : * mut core::ffi::c_void, creationproperties : *const D2D1_CREATION_PROPERTIES, d2ddevicecontext : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    D2D1CreateDeviceContext(dxgisurface.param().abi(), core::mem::transmute(creationproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn D2D1CreateFactory<T>(factorytype: D2D1_FACTORY_TYPE, pfactoryoptions: Option<*const D2D1_FACTORY_OPTIONS>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    D2D1CreateFactory(factorytype, &T::IID, core::mem::transmute(pfactoryoptions.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[inline]
pub unsafe fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0: *const Common::D2D_POINT_2F, ppoint1: *const Common::D2D_POINT_2F, ppoint2: *const Common::D2D_POINT_2F, ppoint3: *const Common::D2D_POINT_2F, ppoint4: *const Common::D2D_POINT_2F, ppoint5: *const Common::D2D_POINT_2F, ppoint6: *const Common::D2D_POINT_2F, ppoint7: *const Common::D2D_POINT_2F, ppoint8: *const Common::D2D_POINT_2F, ppoint9: *const Common::D2D_POINT_2F, ppoint10: *const Common::D2D_POINT_2F, ppoint11: *const Common::D2D_POINT_2F, ptensorpoint11: *mut Common::D2D_POINT_2F, ptensorpoint12: *mut Common::D2D_POINT_2F, ptensorpoint21: *mut Common::D2D_POINT_2F, ptensorpoint22: *mut Common::D2D_POINT_2F) {
    windows_targets::link!("d2d1.dll" "system" fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0 : *const Common:: D2D_POINT_2F, ppoint1 : *const Common:: D2D_POINT_2F, ppoint2 : *const Common:: D2D_POINT_2F, ppoint3 : *const Common:: D2D_POINT_2F, ppoint4 : *const Common:: D2D_POINT_2F, ppoint5 : *const Common:: D2D_POINT_2F, ppoint6 : *const Common:: D2D_POINT_2F, ppoint7 : *const Common:: D2D_POINT_2F, ppoint8 : *const Common:: D2D_POINT_2F, ppoint9 : *const Common:: D2D_POINT_2F, ppoint10 : *const Common:: D2D_POINT_2F, ppoint11 : *const Common:: D2D_POINT_2F, ptensorpoint11 : *mut Common:: D2D_POINT_2F, ptensorpoint12 : *mut Common:: D2D_POINT_2F, ptensorpoint21 : *mut Common:: D2D_POINT_2F, ptensorpoint22 : *mut Common:: D2D_POINT_2F));
    D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0, ppoint1, ppoint2, ppoint3, ppoint4, ppoint5, ppoint6, ppoint7, ppoint8, ppoint9, ppoint10, ppoint11, ptensorpoint11, ptensorpoint12, ptensorpoint21, ptensorpoint22)
}
#[cfg(feature = "Foundation_Numerics")]
#[inline]
pub unsafe fn D2D1InvertMatrix(matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL {
    windows_targets::link!("d2d1.dll" "system" fn D2D1InvertMatrix(matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2) -> super::super::Foundation:: BOOL);
    D2D1InvertMatrix(matrix)
}
#[cfg(feature = "Foundation_Numerics")]
#[inline]
pub unsafe fn D2D1IsMatrixInvertible(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL {
    windows_targets::link!("d2d1.dll" "system" fn D2D1IsMatrixInvertible(matrix : *const super::super::super::Foundation::Numerics:: Matrix3x2) -> super::super::Foundation:: BOOL);
    D2D1IsMatrixInvertible(matrix)
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
#[inline]
pub unsafe fn D2D1MakeRotateMatrix(angle: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
    windows_targets::link!("d2d1.dll" "system" fn D2D1MakeRotateMatrix(angle : f32, center : Common:: D2D_POINT_2F, matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2));
    D2D1MakeRotateMatrix(angle, core::mem::transmute(center), matrix)
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
#[inline]
pub unsafe fn D2D1MakeSkewMatrix(anglex: f32, angley: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
    windows_targets::link!("d2d1.dll" "system" fn D2D1MakeSkewMatrix(anglex : f32, angley : f32, center : Common:: D2D_POINT_2F, matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2));
    D2D1MakeSkewMatrix(anglex, angley, core::mem::transmute(center), matrix)
}
#[inline]
pub unsafe fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32) {
    windows_targets::link!("d2d1.dll" "system" fn D2D1SinCos(angle : f32, s : *mut f32, c : *mut f32));
    D2D1SinCos(angle, s, c)
}
#[inline]
pub unsafe fn D2D1Tan(angle: f32) -> f32 {
    windows_targets::link!("d2d1.dll" "system" fn D2D1Tan(angle : f32) -> f32);
    D2D1Tan(angle)
}
#[inline]
pub unsafe fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32 {
    windows_targets::link!("d2d1.dll" "system" fn D2D1Vec3Length(x : f32, y : f32, z : f32) -> f32);
    D2D1Vec3Length(x, y, z)
}
windows_core::imp::define_interface!(ID2D1AnalysisTransform, ID2D1AnalysisTransform_Vtbl, 0x0359dc30_95e6_4568_9055_27720d130e93);
impl core::ops::Deref for ID2D1AnalysisTransform {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1AnalysisTransform, windows_core::IUnknown);
impl ID2D1AnalysisTransform {
    pub unsafe fn ProcessAnalysisResults(&self, analysisdata: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessAnalysisResults)(windows_core::Interface::as_raw(self), core::mem::transmute(analysisdata.as_ptr()), analysisdata.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID2D1AnalysisTransform {}
unsafe impl Sync for ID2D1AnalysisTransform {}
#[repr(C)]
pub struct ID2D1AnalysisTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProcessAnalysisResults: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Bitmap, ID2D1Bitmap_Vtbl, 0xa2296057_ea42_4099_983b_539fb6505426);
impl core::ops::Deref for ID2D1Bitmap {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Bitmap, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
impl ID2D1Bitmap {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromBitmap<P0>(&self, destpoint: Option<*const Common::D2D_POINT_2U>, bitmap: P0, srcrect: Option<*const Common::D2D_RECT_U>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).CopyFromBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(destpoint.unwrap_or(std::ptr::null())), bitmap.param().abi(), core::mem::transmute(srcrect.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromRenderTarget<P0>(&self, destpoint: Option<*const Common::D2D_POINT_2U>, rendertarget: P0, srcrect: Option<*const Common::D2D_RECT_U>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1RenderTarget>,
    {
        (windows_core::Interface::vtable(self).CopyFromRenderTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(destpoint.unwrap_or(std::ptr::null())), rendertarget.param().abi(), core::mem::transmute(srcrect.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromMemory(&self, dstrect: Option<*const Common::D2D_RECT_U>, srcdata: *const core::ffi::c_void, pitch: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyFromMemory)(windows_core::Interface::as_raw(self), core::mem::transmute(dstrect.unwrap_or(std::ptr::null())), srcdata, pitch).ok()
    }
}
unsafe impl Send for ID2D1Bitmap {}
unsafe impl Sync for ID2D1Bitmap {}
#[repr(C)]
pub struct ID2D1Bitmap_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_POINT_2U, *mut core::ffi::c_void, *const Common::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromBitmap: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_POINT_2U, *mut core::ffi::c_void, *const Common::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromRenderTarget: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CopyFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_U, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CopyFromMemory: usize,
}
windows_core::imp::define_interface!(ID2D1Bitmap1, ID2D1Bitmap1_Vtbl, 0xa898a84c_3873_4588_b08b_ebbf978df041);
impl core::ops::Deref for ID2D1Bitmap1 {
    type Target = ID2D1Bitmap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Bitmap1, windows_core::IUnknown, ID2D1Resource, ID2D1Image, ID2D1Bitmap);
impl ID2D1Bitmap1 {
    pub unsafe fn GetColorContext(&self) -> windows_core::Result<ID2D1ColorContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColorContext)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS {
        (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSurface(&self) -> windows_core::Result<super::Dxgi::IDXGISurface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Map(&self, options: D2D1_MAP_OPTIONS) -> windows_core::Result<D2D1_MAPPED_RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), options, &mut result__).map(|| result__)
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1Bitmap1 {}
unsafe impl Sync for ID2D1Bitmap1 {}
#[repr(C)]
pub struct ID2D1Bitmap1_Vtbl {
    pub base__: ID2D1Bitmap_Vtbl,
    pub GetColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BITMAP_OPTIONS,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetSurface: usize,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_MAP_OPTIONS, *mut D2D1_MAPPED_RECT) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1BitmapBrush, ID2D1BitmapBrush_Vtbl, 0x2cd906aa_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1BitmapBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BitmapBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1BitmapBrush {
    pub unsafe fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE) {
        (windows_core::Interface::vtable(self).SetExtendModeX)(windows_core::Interface::as_raw(self), extendmodex)
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE) {
        (windows_core::Interface::vtable(self).SetExtendModeY)(windows_core::Interface::as_raw(self), extendmodey)
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
        (windows_core::Interface::vtable(self).SetInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode)
    }
    pub unsafe fn SetBitmap<P0>(&self, bitmap: P0)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).SetBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi())
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendModeX)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendModeY)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_BITMAP_INTERPOLATION_MODE {
        (windows_core::Interface::vtable(self).GetInterpolationMode)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBitmap(&self) -> windows_core::Result<ID2D1Bitmap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1BitmapBrush {}
unsafe impl Sync for ID2D1BitmapBrush {}
#[repr(C)]
pub struct ID2D1BitmapBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BITMAP_INTERPOLATION_MODE),
    pub SetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE,
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1BitmapBrush1, ID2D1BitmapBrush1_Vtbl, 0x41343a53_e41a_49a2_91cd_21793bbb62e5);
impl core::ops::Deref for ID2D1BitmapBrush1 {
    type Target = ID2D1BitmapBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BitmapBrush1, windows_core::IUnknown, ID2D1Resource, ID2D1Brush, ID2D1BitmapBrush);
impl ID2D1BitmapBrush1 {
    pub unsafe fn SetInterpolationMode1(&self, interpolationmode: D2D1_INTERPOLATION_MODE) {
        (windows_core::Interface::vtable(self).SetInterpolationMode1)(windows_core::Interface::as_raw(self), interpolationmode)
    }
    pub unsafe fn GetInterpolationMode1(&self) -> D2D1_INTERPOLATION_MODE {
        (windows_core::Interface::vtable(self).GetInterpolationMode1)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1BitmapBrush1 {}
unsafe impl Sync for ID2D1BitmapBrush1 {}
#[repr(C)]
pub struct ID2D1BitmapBrush1_Vtbl {
    pub base__: ID2D1BitmapBrush_Vtbl,
    pub SetInterpolationMode1: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_INTERPOLATION_MODE),
    pub GetInterpolationMode1: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE,
}
windows_core::imp::define_interface!(ID2D1BitmapRenderTarget, ID2D1BitmapRenderTarget_Vtbl, 0x2cd90695_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1BitmapRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BitmapRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1BitmapRenderTarget {
    pub unsafe fn GetBitmap(&self) -> windows_core::Result<ID2D1Bitmap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1BitmapRenderTarget {}
unsafe impl Sync for ID2D1BitmapRenderTarget {}
#[repr(C)]
pub struct ID2D1BitmapRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1BlendTransform, ID2D1BlendTransform_Vtbl, 0x63ac0b32_ba44_450f_8806_7f4ca1ff2f1b);
impl core::ops::Deref for ID2D1BlendTransform {
    type Target = ID2D1ConcreteTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BlendTransform, windows_core::IUnknown, ID2D1TransformNode, ID2D1ConcreteTransform);
impl ID2D1BlendTransform {
    pub unsafe fn SetDescription(&self, description: *const D2D1_BLEND_DESCRIPTION) {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description)
    }
    pub unsafe fn GetDescription(&self, description: *mut D2D1_BLEND_DESCRIPTION) {
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), description)
    }
}
unsafe impl Send for ID2D1BlendTransform {}
unsafe impl Sync for ID2D1BlendTransform {}
#[repr(C)]
pub struct ID2D1BlendTransform_Vtbl {
    pub base__: ID2D1ConcreteTransform_Vtbl,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BLEND_DESCRIPTION),
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_BLEND_DESCRIPTION),
}
windows_core::imp::define_interface!(ID2D1BorderTransform, ID2D1BorderTransform_Vtbl, 0x4998735c_3a19_473c_9781_656847e3a347);
impl core::ops::Deref for ID2D1BorderTransform {
    type Target = ID2D1ConcreteTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BorderTransform, windows_core::IUnknown, ID2D1TransformNode, ID2D1ConcreteTransform);
impl ID2D1BorderTransform {
    pub unsafe fn SetExtendModeX(&self, extendmode: D2D1_EXTEND_MODE) {
        (windows_core::Interface::vtable(self).SetExtendModeX)(windows_core::Interface::as_raw(self), extendmode)
    }
    pub unsafe fn SetExtendModeY(&self, extendmode: D2D1_EXTEND_MODE) {
        (windows_core::Interface::vtable(self).SetExtendModeY)(windows_core::Interface::as_raw(self), extendmode)
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendModeX)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendModeY)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1BorderTransform {}
unsafe impl Sync for ID2D1BorderTransform {}
#[repr(C)]
pub struct ID2D1BorderTransform_Vtbl {
    pub base__: ID2D1ConcreteTransform_Vtbl,
    pub SetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub GetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
}
windows_core::imp::define_interface!(ID2D1BoundsAdjustmentTransform, ID2D1BoundsAdjustmentTransform_Vtbl, 0x90f732e2_5092_4606_a819_8651970baccd);
impl core::ops::Deref for ID2D1BoundsAdjustmentTransform {
    type Target = ID2D1TransformNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1BoundsAdjustmentTransform, windows_core::IUnknown, ID2D1TransformNode);
impl ID2D1BoundsAdjustmentTransform {
    pub unsafe fn SetOutputBounds(&self, outputbounds: *const super::super::Foundation::RECT) {
        (windows_core::Interface::vtable(self).SetOutputBounds)(windows_core::Interface::as_raw(self), outputbounds)
    }
    pub unsafe fn GetOutputBounds(&self) -> super::super::Foundation::RECT {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputBounds)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1BoundsAdjustmentTransform {}
unsafe impl Sync for ID2D1BoundsAdjustmentTransform {}
#[repr(C)]
pub struct ID2D1BoundsAdjustmentTransform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    pub SetOutputBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT),
    pub GetOutputBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT),
}
windows_core::imp::define_interface!(ID2D1Brush, ID2D1Brush_Vtbl, 0x2cd906a8_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Brush {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Brush, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Brush {
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform)
    }
}
unsafe impl Send for ID2D1Brush {}
unsafe impl Sync for ID2D1Brush {}
#[repr(C)]
pub struct ID2D1Brush_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
}
windows_core::imp::define_interface!(ID2D1ColorContext, ID2D1ColorContext_Vtbl, 0x1c4820bb_5771_4518_a581_2fe4dd0ec657);
impl core::ops::Deref for ID2D1ColorContext {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ColorContext, windows_core::IUnknown, ID2D1Resource);
impl ID2D1ColorContext {
    pub unsafe fn GetColorSpace(&self) -> D2D1_COLOR_SPACE {
        (windows_core::Interface::vtable(self).GetColorSpace)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetProfileSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetProfileSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetProfile(&self, profile: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(profile.as_ptr()), profile.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID2D1ColorContext {}
unsafe impl Sync for ID2D1ColorContext {}
#[repr(C)]
pub struct ID2D1ColorContext_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetProfileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1ColorContext1, ID2D1ColorContext1_Vtbl, 0x1ab42875_c57f_4be9_bd85_9cd78d6f55ee);
impl core::ops::Deref for ID2D1ColorContext1 {
    type Target = ID2D1ColorContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ColorContext1, windows_core::IUnknown, ID2D1Resource, ID2D1ColorContext);
impl ID2D1ColorContext1 {
    pub unsafe fn GetColorContextType(&self) -> D2D1_COLOR_CONTEXT_TYPE {
        (windows_core::Interface::vtable(self).GetColorContextType)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDXGIColorSpace(&self) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE {
        (windows_core::Interface::vtable(self).GetDXGIColorSpace)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSimpleColorProfile(&self, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSimpleColorProfile)(windows_core::Interface::as_raw(self), simpleprofile).ok()
    }
}
unsafe impl Send for ID2D1ColorContext1 {}
unsafe impl Sync for ID2D1ColorContext1 {}
#[repr(C)]
pub struct ID2D1ColorContext1_Vtbl {
    pub base__: ID2D1ColorContext_Vtbl,
    pub GetColorContextType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDXGIColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDXGIColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSimpleColorProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSimpleColorProfile: usize,
}
windows_core::imp::define_interface!(ID2D1CommandList, ID2D1CommandList_Vtbl, 0xb4f34a19_2383_4d76_94f6_ec343657c3dc);
impl core::ops::Deref for ID2D1CommandList {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandList, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
impl ID2D1CommandList {
    pub unsafe fn Stream<P0>(&self, sink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1CommandSink>,
    {
        (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), sink.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1CommandList {}
unsafe impl Sync for ID2D1CommandList {}
#[repr(C)]
pub struct ID2D1CommandList_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1CommandSink, ID2D1CommandSink_Vtbl, 0x54d7898a_a061_40a7_bec7_e465bcba2c4f);
impl core::ops::Deref for ID2D1CommandSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandSink, windows_core::IUnknown);
impl ID2D1CommandSink {
    pub unsafe fn BeginDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAntialiasMode)(windows_core::Interface::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTags)(windows_core::Interface::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextAntialiasMode)(windows_core::Interface::as_raw(self), textantialiasmode).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteRenderingParams>,
    {
        (windows_core::Interface::vtable(self).SetTextRenderingParams)(windows_core::Interface::as_raw(self), textrenderingparams.param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrimitiveBlend)(windows_core::Interface::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUnitMode)(windows_core::Interface::as_raw(self), unitmode).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: Option<*const Common::D2D1_COLOR_F>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), core::mem::transmute(color.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, core::mem::transmute(glyphrundescription.unwrap_or(std::ptr::null())), foregroundbrush.param().abi(), measuringmode).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawLine)(windows_core::Interface::as_raw(self), core::mem::transmute(point0), core::mem::transmute(point1), brush.param().abi(), strokewidth, strokestyle.param().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), strokewidth, strokestyle.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi(), strokewidth, strokestyle.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: Option<*const Common::D2D_RECT_F>, perspectivetransform: Option<*const Common::D2D_MATRIX_4X4_F>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), opacity, interpolationmode, core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null())), core::mem::transmute(perspectivetransform.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: Option<*const Common::D2D_POINT_2F>, imagerectangle: Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        (windows_core::Interface::vtable(self).DrawImage)(windows_core::Interface::as_raw(self), image.param().abi(), core::mem::transmute(targetoffset.unwrap_or(std::ptr::null())), core::mem::transmute(imagerectangle.unwrap_or(std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: Option<*const Common::D2D_POINT_2F>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), core::mem::transmute(targetoffset.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Mesh>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillMesh)(windows_core::Interface::as_raw(self), mesh.param().abi(), brush.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: Option<*const Common::D2D_RECT_F>, sourcerectangle: Option<*const Common::D2D_RECT_F>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), opacitybrush.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PushAxisAlignedClip)(windows_core::Interface::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Layer>,
    {
        (windows_core::Interface::vtable(self).PushLayer)(windows_core::Interface::as_raw(self), layerparameters1, layer.param().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PopAxisAlignedClip)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1CommandSink {}
unsafe impl Sync for ID2D1CommandSink {}
#[repr(C)]
pub struct ID2D1CommandSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT,
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT,
    pub SetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_UNIT_MODE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void, super::DirectWrite::DWRITE_MEASURING_MODE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawLine: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, Common::D2D_POINT_2F, *mut core::ffi::c_void, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawLine: usize,
    pub DrawGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, f32, D2D1_INTERPOLATION_MODE, *const Common::D2D_RECT_F, *const Common::D2D_MATRIX_4X4_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_POINT_2F, *const Common::D2D_RECT_F, D2D1_INTERPOLATION_MODE, Common::D2D1_COMPOSITE_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawImage: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_POINT_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    pub FillMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *const Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
    pub FillGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, D2D1_ANTIALIAS_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    PushAxisAlignedClip: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS1, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1CommandSink1, ID2D1CommandSink1_Vtbl, 0x9eb767fd_4269_4467_b8c2_eb30cb305743);
impl core::ops::Deref for ID2D1CommandSink1 {
    type Target = ID2D1CommandSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandSink1, windows_core::IUnknown, ID2D1CommandSink);
impl ID2D1CommandSink1 {
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrimitiveBlend1)(windows_core::Interface::as_raw(self), primitiveblend).ok()
    }
}
unsafe impl Send for ID2D1CommandSink1 {}
unsafe impl Sync for ID2D1CommandSink1 {}
#[repr(C)]
pub struct ID2D1CommandSink1_Vtbl {
    pub base__: ID2D1CommandSink_Vtbl,
    pub SetPrimitiveBlend1: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1CommandSink2, ID2D1CommandSink2_Vtbl, 0x3bab440e_417e_47df_a2e2_bc0be6a00916);
impl core::ops::Deref for ID2D1CommandSink2 {
    type Target = ID2D1CommandSink1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandSink2, windows_core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1);
impl ID2D1CommandSink2 {
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Ink>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1InkStyle>,
    {
        (windows_core::Interface::vtable(self).DrawInk)(windows_core::Interface::as_raw(self), ink.param().abi(), brush.param().abi(), inkstyle.param().abi()).ok()
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GradientMesh>,
    {
        (windows_core::Interface::vtable(self).DrawGradientMesh)(windows_core::Interface::as_raw(self), gradientmesh.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, destinationrectangle: Option<*const Common::D2D_RECT_F>, sourcerectangle: Option<*const Common::D2D_RECT_F>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null()))).ok()
    }
}
unsafe impl Send for ID2D1CommandSink2 {}
unsafe impl Sync for ID2D1CommandSink2 {}
#[repr(C)]
pub struct ID2D1CommandSink2_Vtbl {
    pub base__: ID2D1CommandSink1_Vtbl,
    pub DrawInk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawGradientMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *const Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
}
windows_core::imp::define_interface!(ID2D1CommandSink3, ID2D1CommandSink3_Vtbl, 0x18079135_4cf3_4868_bc8e_06067e6d242d);
impl core::ops::Deref for ID2D1CommandSink3 {
    type Target = ID2D1CommandSink2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandSink3, windows_core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1, ID2D1CommandSink2);
impl ID2D1CommandSink3 {
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1SpriteBatch>,
        P1: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).DrawSpriteBatch)(windows_core::Interface::as_raw(self), spritebatch.param().abi(), startindex, spritecount, bitmap.param().abi(), interpolationmode, spriteoptions).ok()
    }
}
unsafe impl Send for ID2D1CommandSink3 {}
unsafe impl Sync for ID2D1CommandSink3 {}
#[repr(C)]
pub struct ID2D1CommandSink3_Vtbl {
    pub base__: ID2D1CommandSink2_Vtbl,
    pub DrawSpriteBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, D2D1_BITMAP_INTERPOLATION_MODE, D2D1_SPRITE_OPTIONS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1CommandSink4, ID2D1CommandSink4_Vtbl, 0xc78a6519_40d6_4218_b2de_beeeb744bb3e);
impl core::ops::Deref for ID2D1CommandSink4 {
    type Target = ID2D1CommandSink3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandSink4, windows_core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1, ID2D1CommandSink2, ID2D1CommandSink3);
impl ID2D1CommandSink4 {
    pub unsafe fn SetPrimitiveBlend2(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrimitiveBlend2)(windows_core::Interface::as_raw(self), primitiveblend).ok()
    }
}
unsafe impl Send for ID2D1CommandSink4 {}
unsafe impl Sync for ID2D1CommandSink4 {}
#[repr(C)]
pub struct ID2D1CommandSink4_Vtbl {
    pub base__: ID2D1CommandSink3_Vtbl,
    pub SetPrimitiveBlend2: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1CommandSink5, ID2D1CommandSink5_Vtbl, 0x7047dd26_b1e7_44a7_959a_8349e2144fa8);
impl core::ops::Deref for ID2D1CommandSink5 {
    type Target = ID2D1CommandSink4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1CommandSink5, windows_core::IUnknown, ID2D1CommandSink, ID2D1CommandSink1, ID2D1CommandSink2, ID2D1CommandSink3, ID2D1CommandSink4);
impl ID2D1CommandSink5 {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn BlendImage<P0>(&self, image: P0, blendmode: Common::D2D1_BLEND_MODE, targetoffset: Option<*const Common::D2D_POINT_2F>, imagerectangle: Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        (windows_core::Interface::vtable(self).BlendImage)(windows_core::Interface::as_raw(self), image.param().abi(), blendmode, core::mem::transmute(targetoffset.unwrap_or(std::ptr::null())), core::mem::transmute(imagerectangle.unwrap_or(std::ptr::null())), interpolationmode).ok()
    }
}
unsafe impl Send for ID2D1CommandSink5 {}
unsafe impl Sync for ID2D1CommandSink5 {}
#[repr(C)]
pub struct ID2D1CommandSink5_Vtbl {
    pub base__: ID2D1CommandSink4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub BlendImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, Common::D2D1_BLEND_MODE, *const Common::D2D_POINT_2F, *const Common::D2D_RECT_F, D2D1_INTERPOLATION_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    BlendImage: usize,
}
windows_core::imp::define_interface!(ID2D1ComputeInfo, ID2D1ComputeInfo_Vtbl, 0x5598b14b_9fd7_48b7_9bdb_8f0964eb38bc);
impl core::ops::Deref for ID2D1ComputeInfo {
    type Target = ID2D1RenderInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ComputeInfo, windows_core::IUnknown, ID2D1RenderInfo);
impl ID2D1ComputeInfo {
    pub unsafe fn SetComputeShaderConstantBuffer(&self, buffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetComputeShaderConstantBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetComputeShader(&self, shaderid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetComputeShader)(windows_core::Interface::as_raw(self), shaderid).ok()
    }
    pub unsafe fn SetResourceTexture<P0>(&self, textureindex: u32, resourcetexture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1ResourceTexture>,
    {
        (windows_core::Interface::vtable(self).SetResourceTexture)(windows_core::Interface::as_raw(self), textureindex, resourcetexture.param().abi()).ok()
    }
}
unsafe impl Send for ID2D1ComputeInfo {}
unsafe impl Sync for ID2D1ComputeInfo {}
#[repr(C)]
pub struct ID2D1ComputeInfo_Vtbl {
    pub base__: ID2D1RenderInfo_Vtbl,
    pub SetComputeShaderConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub SetComputeShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetResourceTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1ComputeTransform, ID2D1ComputeTransform_Vtbl, 0x0d85573c_01e3_4f7d_bfd9_0d60608bf3c3);
impl core::ops::Deref for ID2D1ComputeTransform {
    type Target = ID2D1Transform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ComputeTransform, windows_core::IUnknown, ID2D1TransformNode, ID2D1Transform);
impl ID2D1ComputeTransform {
    pub unsafe fn SetComputeInfo<P0>(&self, computeinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1ComputeInfo>,
    {
        (windows_core::Interface::vtable(self).SetComputeInfo)(windows_core::Interface::as_raw(self), computeinfo.param().abi()).ok()
    }
    pub unsafe fn CalculateThreadgroups(&self, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CalculateThreadgroups)(windows_core::Interface::as_raw(self), outputrect, dimensionx, dimensiony, dimensionz).ok()
    }
}
unsafe impl Send for ID2D1ComputeTransform {}
unsafe impl Sync for ID2D1ComputeTransform {}
#[repr(C)]
pub struct ID2D1ComputeTransform_Vtbl {
    pub base__: ID2D1Transform_Vtbl,
    pub SetComputeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CalculateThreadgroups: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1ConcreteTransform, ID2D1ConcreteTransform_Vtbl, 0x1a799d8a_69f7_4e4c_9fed_437ccc6684cc);
impl core::ops::Deref for ID2D1ConcreteTransform {
    type Target = ID2D1TransformNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ConcreteTransform, windows_core::IUnknown, ID2D1TransformNode);
impl ID2D1ConcreteTransform {
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOutputBuffer)(windows_core::Interface::as_raw(self), bufferprecision, channeldepth).ok()
    }
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCached)(windows_core::Interface::as_raw(self), iscached.param().abi())
    }
}
unsafe impl Send for ID2D1ConcreteTransform {}
unsafe impl Sync for ID2D1ConcreteTransform {}
#[repr(C)]
pub struct ID2D1ConcreteTransform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    pub SetOutputBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION, D2D1_CHANNEL_DEPTH) -> windows_core::HRESULT,
    pub SetCached: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
}
windows_core::imp::define_interface!(ID2D1DCRenderTarget, ID2D1DCRenderTarget_Vtbl, 0x1c51bc64_de61_46fd_9899_63a5d8f03950);
impl core::ops::Deref for ID2D1DCRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DCRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1DCRenderTarget {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn BindDC<P0>(&self, hdc: P0, psubrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).BindDC)(windows_core::Interface::as_raw(self), hdc.param().abi(), psubrect).ok()
    }
}
unsafe impl Send for ID2D1DCRenderTarget {}
unsafe impl Sync for ID2D1DCRenderTarget {}
#[repr(C)]
pub struct ID2D1DCRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub BindDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::Gdi::HDC, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    BindDC: usize,
}
windows_core::imp::define_interface!(ID2D1Device, ID2D1Device_Vtbl, 0x47dd575d_ac05_4cdd_8049_9b02cd16f44c);
impl core::ops::Deref for ID2D1Device {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Device {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> windows_core::Result<ID2D1PrintControl>
    where
        P0: windows_core::Param<super::Imaging::IWICImagingFactory>,
        P1: windows_core::Param<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePrintControl)(windows_core::Interface::as_raw(self), wicfactory.param().abi(), documenttarget.param().abi(), core::mem::transmute(printcontrolproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (windows_core::Interface::vtable(self).SetMaximumTextureMemory)(windows_core::Interface::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetMaximumTextureMemory)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (windows_core::Interface::vtable(self).ClearResources)(windows_core::Interface::as_raw(self), millisecondssinceuse)
    }
}
unsafe impl Send for ID2D1Device {}
unsafe impl Sync for ID2D1Device {}
#[repr(C)]
pub struct ID2D1Device_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub CreatePrintControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_PRINT_CONTROL_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing")))]
    CreatePrintControl: usize,
    pub SetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub GetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub ClearResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
windows_core::imp::define_interface!(ID2D1Device1, ID2D1Device1_Vtbl, 0xd21768e1_23a4_4823_a14b_7c3eba85d658);
impl core::ops::Deref for ID2D1Device1 {
    type Target = ID2D1Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device1, windows_core::IUnknown, ID2D1Resource, ID2D1Device);
impl ID2D1Device1 {
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (windows_core::Interface::vtable(self).GetRenderingPriority)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (windows_core::Interface::vtable(self).SetRenderingPriority)(windows_core::Interface::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Device1 {}
unsafe impl Sync for ID2D1Device1 {}
#[repr(C)]
pub struct ID2D1Device1_Vtbl {
    pub base__: ID2D1Device_Vtbl,
    pub GetRenderingPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_RENDERING_PRIORITY,
    pub SetRenderingPriority: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_RENDERING_PRIORITY),
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Device2, ID2D1Device2_Vtbl, 0xa44472e1_8dfb_4e60_8492_6e2861c9ca8b);
impl core::ops::Deref for ID2D1Device2 {
    type Target = ID2D1Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device2, windows_core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1);
impl ID2D1Device2 {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).FlushDeviceContexts)(windows_core::Interface::as_raw(self), bitmap.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> windows_core::Result<super::Dxgi::IDXGIDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDxgiDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Device2 {}
unsafe impl Sync for ID2D1Device2 {}
#[repr(C)]
pub struct ID2D1Device2_Vtbl {
    pub base__: ID2D1Device1_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushDeviceContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetDxgiDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetDxgiDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Device3, ID2D1Device3_Vtbl, 0x852f2087_802c_4037_ab60_ff2e7ee6fc01);
impl core::ops::Deref for ID2D1Device3 {
    type Target = ID2D1Device2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device3, windows_core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2);
impl ID2D1Device3 {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Device3 {}
unsafe impl Sync for ID2D1Device3 {}
#[repr(C)]
pub struct ID2D1Device3_Vtbl {
    pub base__: ID2D1Device2_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Device4, ID2D1Device4_Vtbl, 0xd7bdb159_5683_4a46_bc9c_72dc720b858b);
impl core::ops::Deref for ID2D1Device4 {
    type Target = ID2D1Device3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device4, windows_core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3);
impl ID2D1Device4 {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64) {
        (windows_core::Interface::vtable(self).SetMaximumColorGlyphCacheMemory)(windows_core::Interface::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumColorGlyphCacheMemory(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetMaximumColorGlyphCacheMemory)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1Device4 {}
unsafe impl Sync for ID2D1Device4 {}
#[repr(C)]
pub struct ID2D1Device4_Vtbl {
    pub base__: ID2D1Device3_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub GetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
}
windows_core::imp::define_interface!(ID2D1Device5, ID2D1Device5_Vtbl, 0xd55ba0a4_6405_4694_aef5_08ee1a4358b4);
impl core::ops::Deref for ID2D1Device5 {
    type Target = ID2D1Device4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device5, windows_core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4);
impl ID2D1Device5 {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext5> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Device5 {}
unsafe impl Sync for ID2D1Device5 {}
#[repr(C)]
pub struct ID2D1Device5_Vtbl {
    pub base__: ID2D1Device4_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Device6, ID2D1Device6_Vtbl, 0x7bfef914_2d75_4bad_be87_e18ddb077b6d);
impl core::ops::Deref for ID2D1Device6 {
    type Target = ID2D1Device5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device6, windows_core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4, ID2D1Device5);
impl ID2D1Device6 {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext6> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Device6 {}
unsafe impl Sync for ID2D1Device6 {}
#[repr(C)]
pub struct ID2D1Device6_Vtbl {
    pub base__: ID2D1Device5_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Device7, ID2D1Device7_Vtbl, 0xf07c8968_dd4e_4ba6_9cbd_eb6d3752dcbb);
impl core::ops::Deref for ID2D1Device7 {
    type Target = ID2D1Device6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device7, windows_core::IUnknown, ID2D1Resource, ID2D1Device, ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4, ID2D1Device5, ID2D1Device6);
impl ID2D1Device7 {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext7> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Device7 {}
unsafe impl Sync for ID2D1Device7 {}
#[repr(C)]
pub struct ID2D1Device7_Vtbl {
    pub base__: ID2D1Device6_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1DeviceContext, ID2D1DeviceContext_Vtbl, 0xe8f7fe7a_191c_466d_ad95_975678bda998);
impl core::ops::Deref for ID2D1DeviceContext {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1DeviceContext {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, sourcedata: Option<*const core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(size), core::mem::transmute(sourcedata.unwrap_or(std::ptr::null())), pitch, bitmapproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<super::Imaging::IWICBitmapSource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(windows_core::Interface::as_raw(self), wicbitmapsource.param().abi(), core::mem::transmute(bitmapproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: Option<&[u8]>) -> windows_core::Result<ID2D1ColorContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContext)(windows_core::Interface::as_raw(self), space, core::mem::transmute(profile.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromFilename)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<super::Imaging::IWICColorContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromWicColorContext)(windows_core::Interface::as_raw(self), wiccolorcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<super::Dxgi::IDXGISurface>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromDxgiSurface)(windows_core::Interface::as_raw(self), surface.param().abi(), core::mem::transmute(bitmapproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEffect(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Effect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), effectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, straightalphagradientstops: &[Common::D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> windows_core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGradientStopCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len().try_into().unwrap(), preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1ImageBrush>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageBrush)(windows_core::Interface::as_raw(self), image.param().abi(), imagebrushproperties, core::mem::transmute(brushproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1BitmapBrush1>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapBrush)(windows_core::Interface::as_raw(self), bitmap.param().abi(), core::mem::transmute(bitmapbrushproperties.unwrap_or(std::ptr::null())), core::mem::transmute(brushproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCommandList(&self) -> windows_core::Result<ID2D1CommandList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCommandList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsDxgiFormatSupported)(windows_core::Interface::as_raw(self), format)
    }
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsBufferPrecisionSupported)(windows_core::Interface::as_raw(self), bufferprecision)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageLocalBounds)(windows_core::Interface::as_raw(self), image.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImageWorldBounds)(windows_core::Interface::as_raw(self), image.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> windows_core::Result<Common::D2D_RECT_F> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphRunWorldBounds)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, measuringmode, &mut result__).map(|| result__)
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID2D1Device> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        (windows_core::Interface::vtable(self).SetTarget)(windows_core::Interface::as_raw(self), image.param().abi())
    }
    pub unsafe fn GetTarget(&self) -> windows_core::Result<ID2D1Image> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTarget)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (windows_core::Interface::vtable(self).SetRenderingControls)(windows_core::Interface::as_raw(self), renderingcontrols)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRenderingControls)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (windows_core::Interface::vtable(self).SetPrimitiveBlend)(windows_core::Interface::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (windows_core::Interface::vtable(self).GetPrimitiveBlend)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (windows_core::Interface::vtable(self).SetUnitMode)(windows_core::Interface::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (windows_core::Interface::vtable(self).GetUnitMode)(windows_core::Interface::as_raw(self))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, core::mem::transmute(glyphrundescription.unwrap_or(std::ptr::null())), foregroundbrush.param().abi(), measuringmode)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: Option<*const Common::D2D_POINT_2F>, imagerectangle: Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        (windows_core::Interface::vtable(self).DrawImage)(windows_core::Interface::as_raw(self), image.param().abi(), core::mem::transmute(targetoffset.unwrap_or(std::ptr::null())), core::mem::transmute(imagerectangle.unwrap_or(std::ptr::null())), interpolationmode, compositemode)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: Option<*const Common::D2D_POINT_2F>)
    where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), core::mem::transmute(targetoffset.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: Option<*const Common::D2D_RECT_F>, perspectivetransform: Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), opacity, interpolationmode, core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null())), core::mem::transmute(perspectivetransform.unwrap_or(std::ptr::null())))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: windows_core::Param<ID2D1Layer>,
    {
        (windows_core::Interface::vtable(self).PushLayer)(windows_core::Interface::as_raw(self), layerparameters, layer.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        (windows_core::Interface::vtable(self).InvalidateEffectInputRectangle)(windows_core::Interface::as_raw(self), effect.param().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEffectInvalidRectangleCount)(windows_core::Interface::as_raw(self), effect.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        (windows_core::Interface::vtable(self).GetEffectInvalidRectangles)(windows_core::Interface::as_raw(self), effect.param().abi(), core::mem::transmute(rectangles.as_ptr()), rectangles.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        (windows_core::Interface::vtable(self).GetEffectRequiredInputRectangles)(windows_core::Interface::as_raw(self), rendereffect.param().abi(), core::mem::transmute(renderimagerectangle.unwrap_or(std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: Option<*const Common::D2D_RECT_F>, sourcerectangle: Option<*const Common::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null())))
    }
}
unsafe impl Send for ID2D1DeviceContext {}
unsafe impl Sync for ID2D1DeviceContext {}
#[repr(C)]
pub struct ID2D1DeviceContext_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_SIZE_U, *const core::ffi::c_void, u32, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateBitmapFromWicBitmap: usize,
    pub CreateColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_COLOR_SPACE, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateColorContextFromWicColorContext: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmapFromDxgiSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmapFromDxgiSurface: usize,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_GRADIENT_STOP, u32, D2D1_COLOR_SPACE, D2D1_COLOR_SPACE, D2D1_BUFFER_PRECISION, D2D1_EXTEND_MODE, D2D1_COLOR_INTERPOLATION_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientStopCollection: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateImageBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_IMAGE_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateImageBrush: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateBitmapBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_BRUSH_PROPERTIES1, *const D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateBitmapBrush: usize,
    pub CreateCommandList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IsDxgiFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IsDxgiFormatSupported: usize,
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageLocalBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageLocalBounds: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageWorldBounds: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetGlyphRunWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, super::DirectWrite::DWRITE_MEASURING_MODE, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetGlyphRunWorldBounds: usize,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetRenderingControls: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetRenderingControls: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRenderingControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_RENDERING_CONTROLS),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRenderingControls: usize,
    pub SetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND),
    pub GetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND,
    pub SetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_UNIT_MODE),
    pub GetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_UNIT_MODE,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void, super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_POINT_2F, *const Common::D2D_RECT_F, D2D1_INTERPOLATION_MODE, Common::D2D1_COMPOSITE_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawImage: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, f32, D2D1_INTERPOLATION_MODE, *const Common::D2D_RECT_F, *const Common::D2D_MATRIX_4X4_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS1, *mut core::ffi::c_void),
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub InvalidateEffectInputRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    InvalidateEffectInputRectangle: usize,
    pub GetEffectInvalidRectangleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEffectInvalidRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut Common::D2D_RECT_F, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEffectInvalidRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEffectRequiredInputRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *const D2D1_EFFECT_INPUT_DESCRIPTION, *mut Common::D2D_RECT_F, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEffectRequiredInputRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
}
windows_core::imp::define_interface!(ID2D1DeviceContext1, ID2D1DeviceContext1_Vtbl, 0xd37f57e4_6908_459f_a199_e72f24f79987);
impl core::ops::Deref for ID2D1DeviceContext1 {
    type Target = ID2D1DeviceContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext1, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext);
impl ID2D1DeviceContext1 {
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> windows_core::Result<ID2D1GeometryRealization>
    where
        P0: windows_core::Param<ID2D1Geometry>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFilledGeometryRealization)(windows_core::Interface::as_raw(self), geometry.param().abi(), flatteningtolerance, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> windows_core::Result<ID2D1GeometryRealization>
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStrokedGeometryRealization)(windows_core::Interface::as_raw(self), geometry.param().abi(), flatteningtolerance, strokewidth, strokestyle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: windows_core::Param<ID2D1GeometryRealization>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawGeometryRealization)(windows_core::Interface::as_raw(self), geometryrealization.param().abi(), brush.param().abi())
    }
}
unsafe impl Send for ID2D1DeviceContext1 {}
unsafe impl Sync for ID2D1DeviceContext1 {}
#[repr(C)]
pub struct ID2D1DeviceContext1_Vtbl {
    pub base__: ID2D1DeviceContext_Vtbl,
    pub CreateFilledGeometryRealization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStrokedGeometryRealization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, f32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawGeometryRealization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1DeviceContext2, ID2D1DeviceContext2_Vtbl, 0x394ea6a3_0c34_4321_950b_6ca20f0be6c7);
impl core::ops::Deref for ID2D1DeviceContext2 {
    type Target = ID2D1DeviceContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext2, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1);
impl ID2D1DeviceContext2 {
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> windows_core::Result<ID2D1Ink> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInk)(windows_core::Interface::as_raw(self), startpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: Option<*const D2D1_INK_STYLE_PROPERTIES>) -> windows_core::Result<ID2D1InkStyle> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInkStyle)(windows_core::Interface::as_raw(self), core::mem::transmute(inkstyleproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> windows_core::Result<ID2D1GradientMesh> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGradientMesh)(windows_core::Interface::as_raw(self), core::mem::transmute(patches.as_ptr()), patches.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> windows_core::Result<ID2D1ImageSourceFromWic>
    where
        P0: windows_core::Param<super::Imaging::IWICBitmapSource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageSourceFromWic)(windows_core::Interface::as_raw(self), wicbitmapsource.param().abi(), loadingoptions, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> windows_core::Result<ID2D1LookupTable3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLookupTable3D)(windows_core::Interface::as_raw(self), precision, core::mem::transmute(extents.as_ptr()), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), core::mem::transmute(strides.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[Option<super::Dxgi::IDXGISurface>], colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> windows_core::Result<ID2D1ImageSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageSourceFromDxgi)(windows_core::Interface::as_raw(self), core::mem::transmute(surfaces.as_ptr()), surfaces.len().try_into().unwrap(), colorspace, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1GradientMesh>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGradientMeshWorldBounds)(windows_core::Interface::as_raw(self), gradientmesh.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: windows_core::Param<ID2D1Ink>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1InkStyle>,
    {
        (windows_core::Interface::vtable(self).DrawInk)(windows_core::Interface::as_raw(self), ink.param().abi(), brush.param().abi(), inkstyle.param().abi())
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: windows_core::Param<ID2D1GradientMesh>,
    {
        (windows_core::Interface::vtable(self).DrawGradientMesh)(windows_core::Interface::as_raw(self), gradientmesh.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, destinationrectangle: Option<*const Common::D2D_RECT_F>, sourcerectangle: Option<*const Common::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> windows_core::Result<ID2D1TransformedImageSource>
    where
        P0: windows_core::Param<ID2D1ImageSource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransformedImageSource)(windows_core::Interface::as_raw(self), imagesource.param().abi(), properties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1DeviceContext2 {}
unsafe impl Sync for ID2D1DeviceContext2 {}
#[repr(C)]
pub struct ID2D1DeviceContext2_Vtbl {
    pub base__: ID2D1DeviceContext1_Vtbl,
    pub CreateInk: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateInkStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_STYLE_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateInkStyle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_GRADIENT_MESH_PATCH, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientMesh: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateImageSourceFromWic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_IMAGE_SOURCE_LOADING_OPTIONS, Common::D2D1_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging")))]
    CreateImageSourceFromWic: usize,
    pub CreateLookupTable3D: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION, *const u32, *const u8, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateImageSourceFromDxgi: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateImageSourceFromDxgi: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientMeshWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientMeshWorldBounds: usize,
    pub DrawInk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DrawGradientMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawGdiMetafile: usize,
    pub CreateTransformedImageSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1DeviceContext3, ID2D1DeviceContext3_Vtbl, 0x235a7496_8351_414c_bcd4_6672ab2d8e00);
impl core::ops::Deref for ID2D1DeviceContext3 {
    type Target = ID2D1DeviceContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext3, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2);
impl ID2D1DeviceContext3 {
    pub unsafe fn CreateSpriteBatch(&self) -> windows_core::Result<ID2D1SpriteBatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSpriteBatch)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
    where
        P0: windows_core::Param<ID2D1SpriteBatch>,
        P1: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).DrawSpriteBatch)(windows_core::Interface::as_raw(self), spritebatch.param().abi(), startindex, spritecount, bitmap.param().abi(), interpolationmode, spriteoptions)
    }
}
unsafe impl Send for ID2D1DeviceContext3 {}
unsafe impl Sync for ID2D1DeviceContext3 {}
#[repr(C)]
pub struct ID2D1DeviceContext3_Vtbl {
    pub base__: ID2D1DeviceContext2_Vtbl,
    pub CreateSpriteBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawSpriteBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, D2D1_BITMAP_INTERPOLATION_MODE, D2D1_SPRITE_OPTIONS),
}
windows_core::imp::define_interface!(ID2D1DeviceContext4, ID2D1DeviceContext4_Vtbl, 0x8c427831_3d90_4476_b647_c4fae349e4db);
impl core::ops::Deref for ID2D1DeviceContext4 {
    type Target = ID2D1DeviceContext3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext4, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3);
impl ID2D1DeviceContext4 {
    pub unsafe fn CreateSvgGlyphStyle(&self) -> windows_core::Result<ID2D1SvgGlyphStyle> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSvgGlyphStyle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1, P2>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteTextFormat>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        (windows_core::Interface::vtable(self).DrawText)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), layoutrect, defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, options, measuringmode)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1, P2>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteTextLayout>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        (windows_core::Interface::vtable(self).DrawTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(origin), textlayout.param().abi(), defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, options)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
        (windows_core::Interface::vtable(self).DrawColorBitmapGlyphRun)(windows_core::Interface::as_raw(self), glyphimageformat, core::mem::transmute(baselineorigin), glyphrun, measuringmode, bitmapsnapoption)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawSvgGlyphRun<P0, P1>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: P0, svgglyphstyle: P1, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        (windows_core::Interface::vtable(self).DrawSvgGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, measuringmode)
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetColorBitmapGlyphImage<P0, P1>(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut Option<ID2D1Image>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteFontFace>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetColorBitmapGlyphImage)(windows_core::Interface::as_raw(self), glyphimageformat, core::mem::transmute(glyphorigin), fontface.param().abi(), fontemsize, glyphindex, issideways.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), dpix, dpiy, glyphtransform, core::mem::transmute(glyphimage)).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetSvgGlyphImage<P0, P1, P2, P3>(&self, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, defaultfillbrush: P2, svgglyphstyle: P3, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut Option<ID2D1CommandList>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteFontFace>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        (windows_core::Interface::vtable(self).GetSvgGlyphImage)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphorigin), fontface.param().abi(), fontemsize, glyphindex, issideways.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, glyphtransform, core::mem::transmute(glyphimage)).ok()
    }
}
unsafe impl Send for ID2D1DeviceContext4 {}
unsafe impl Sync for ID2D1DeviceContext4 {}
#[repr(C)]
pub struct ID2D1DeviceContext4_Vtbl {
    pub base__: ID2D1DeviceContext3_Vtbl,
    pub CreateSvgGlyphStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, D2D1_DRAW_TEXT_OPTIONS, super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawText: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawColorBitmapGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, super::DirectWrite::DWRITE_MEASURING_MODE, D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawColorBitmapGlyphRun: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawSvgGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawSvgGlyphRun: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetColorBitmapGlyphImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, Common::D2D_POINT_2F, *mut core::ffi::c_void, f32, u16, super::super::Foundation::BOOL, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, f32, *mut super::super::super::Foundation::Numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetColorBitmapGlyphImage: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub GetSvgGlyphImage: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *mut core::ffi::c_void, f32, u16, super::super::Foundation::BOOL, *const super::super::super::Foundation::Numerics::Matrix3x2, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::super::Foundation::Numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    GetSvgGlyphImage: usize,
}
windows_core::imp::define_interface!(ID2D1DeviceContext5, ID2D1DeviceContext5_Vtbl, 0x7836d248_68cc_4df6_b9e8_de991bf62eb7);
impl core::ops::Deref for ID2D1DeviceContext5 {
    type Target = ID2D1DeviceContext4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext5, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4);
impl ID2D1DeviceContext5 {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSvgDocument<P0>(&self, inputxmlstream: P0, viewportsize: Common::D2D_SIZE_F) -> windows_core::Result<ID2D1SvgDocument>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSvgDocument)(windows_core::Interface::as_raw(self), inputxmlstream.param().abi(), core::mem::transmute(viewportsize), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DrawSvgDocument<P0>(&self, svgdocument: P0)
    where
        P0: windows_core::Param<ID2D1SvgDocument>,
    {
        (windows_core::Interface::vtable(self).DrawSvgDocument)(windows_core::Interface::as_raw(self), svgdocument.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<ID2D1ColorContext1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromDxgiColorSpace)(windows_core::Interface::as_raw(self), colorspace, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<ID2D1ColorContext1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromSimpleColorProfile)(windows_core::Interface::as_raw(self), simpleprofile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1DeviceContext5 {}
unsafe impl Sync for ID2D1DeviceContext5 {}
#[repr(C)]
pub struct ID2D1DeviceContext5_Vtbl {
    pub base__: ID2D1DeviceContext4_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub CreateSvgDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, Common::D2D_SIZE_F, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com")))]
    CreateSvgDocument: usize,
    pub DrawSvgDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateColorContextFromDxgiColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SIMPLE_COLOR_PROFILE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateColorContextFromSimpleColorProfile: usize,
}
windows_core::imp::define_interface!(ID2D1DeviceContext6, ID2D1DeviceContext6_Vtbl, 0x985f7e37_4ed0_4a19_98a3_15b0edfde306);
impl core::ops::Deref for ID2D1DeviceContext6 {
    type Target = ID2D1DeviceContext5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext6, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4, ID2D1DeviceContext5);
impl ID2D1DeviceContext6 {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn BlendImage<P0>(&self, image: P0, blendmode: Common::D2D1_BLEND_MODE, targetoffset: Option<*const Common::D2D_POINT_2F>, imagerectangle: Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE)
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        (windows_core::Interface::vtable(self).BlendImage)(windows_core::Interface::as_raw(self), image.param().abi(), blendmode, core::mem::transmute(targetoffset.unwrap_or(std::ptr::null())), core::mem::transmute(imagerectangle.unwrap_or(std::ptr::null())), interpolationmode)
    }
}
unsafe impl Send for ID2D1DeviceContext6 {}
unsafe impl Sync for ID2D1DeviceContext6 {}
#[repr(C)]
pub struct ID2D1DeviceContext6_Vtbl {
    pub base__: ID2D1DeviceContext5_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub BlendImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, Common::D2D1_BLEND_MODE, *const Common::D2D_POINT_2F, *const Common::D2D_RECT_F, D2D1_INTERPOLATION_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    BlendImage: usize,
}
windows_core::imp::define_interface!(ID2D1DeviceContext7, ID2D1DeviceContext7_Vtbl, 0xec891cf7_9b69_4851_9def_4e0915771e62);
impl core::ops::Deref for ID2D1DeviceContext7 {
    type Target = ID2D1DeviceContext6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext7, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget, ID2D1DeviceContext, ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4, ID2D1DeviceContext5, ID2D1DeviceContext6);
impl ID2D1DeviceContext7 {
    pub unsafe fn GetPaintFeatureLevel(&self) -> DWRITE_PAINT_FEATURE_LEVEL {
        (windows_core::Interface::vtable(self).GetPaintFeatureLevel)(windows_core::Interface::as_raw(self))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawPaintGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: P0, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawPaintGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, defaultfillbrush.param().abi(), colorpaletteindex, measuringmode)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRunWithColorSupport<P0, P1>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, svgglyphstyle: P1, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION)
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRunWithColorSupport)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, core::mem::transmute(glyphrundescription.unwrap_or(std::ptr::null())), foregroundbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, measuringmode, bitmapsnapoption)
    }
}
unsafe impl Send for ID2D1DeviceContext7 {}
unsafe impl Sync for ID2D1DeviceContext7 {}
#[repr(C)]
pub struct ID2D1DeviceContext7_Vtbl {
    pub base__: ID2D1DeviceContext6_Vtbl,
    pub GetPaintFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PAINT_FEATURE_LEVEL,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawPaintGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, u32, super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawPaintGlyphRun: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRunWithColorSupport: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::DirectWrite::DWRITE_MEASURING_MODE, D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRunWithColorSupport: usize,
}
windows_core::imp::define_interface!(ID2D1DrawInfo, ID2D1DrawInfo_Vtbl, 0x693ce632_7f2f_45de_93fe_18d88b37aa21);
impl core::ops::Deref for ID2D1DrawInfo {
    type Target = ID2D1RenderInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DrawInfo, windows_core::IUnknown, ID2D1RenderInfo);
impl ID2D1DrawInfo {
    pub unsafe fn SetPixelShaderConstantBuffer(&self, buffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPixelShaderConstantBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetResourceTexture<P0>(&self, textureindex: u32, resourcetexture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1ResourceTexture>,
    {
        (windows_core::Interface::vtable(self).SetResourceTexture)(windows_core::Interface::as_raw(self), textureindex, resourcetexture.param().abi()).ok()
    }
    pub unsafe fn SetVertexShaderConstantBuffer(&self, buffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVertexShaderConstantBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetPixelShader(&self, shaderid: *const windows_core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPixelShader)(windows_core::Interface::as_raw(self), shaderid, pixeloptions).ok()
    }
    pub unsafe fn SetVertexProcessing<P0>(&self, vertexbuffer: P0, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: Option<*const D2D1_BLEND_DESCRIPTION>, vertexrange: Option<*const D2D1_VERTEX_RANGE>, vertexshader: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1VertexBuffer>,
    {
        (windows_core::Interface::vtable(self).SetVertexProcessing)(windows_core::Interface::as_raw(self), vertexbuffer.param().abi(), vertexoptions, core::mem::transmute(blenddescription.unwrap_or(std::ptr::null())), core::mem::transmute(vertexrange.unwrap_or(std::ptr::null())), core::mem::transmute(vertexshader.unwrap_or(std::ptr::null()))).ok()
    }
}
unsafe impl Send for ID2D1DrawInfo {}
unsafe impl Sync for ID2D1DrawInfo {}
#[repr(C)]
pub struct ID2D1DrawInfo_Vtbl {
    pub base__: ID2D1RenderInfo_Vtbl,
    pub SetPixelShaderConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub SetResourceTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShaderConstantBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, D2D1_PIXEL_OPTIONS) -> windows_core::HRESULT,
    pub SetVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_VERTEX_OPTIONS, *const D2D1_BLEND_DESCRIPTION, *const D2D1_VERTEX_RANGE, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1DrawTransform, ID2D1DrawTransform_Vtbl, 0x36bfdcb6_9739_435d_a30d_a653beff6a6f);
impl core::ops::Deref for ID2D1DrawTransform {
    type Target = ID2D1Transform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DrawTransform, windows_core::IUnknown, ID2D1TransformNode, ID2D1Transform);
impl ID2D1DrawTransform {
    pub unsafe fn SetDrawInfo<P0>(&self, drawinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1DrawInfo>,
    {
        (windows_core::Interface::vtable(self).SetDrawInfo)(windows_core::Interface::as_raw(self), drawinfo.param().abi()).ok()
    }
}
unsafe impl Send for ID2D1DrawTransform {}
unsafe impl Sync for ID2D1DrawTransform {}
#[repr(C)]
pub struct ID2D1DrawTransform_Vtbl {
    pub base__: ID2D1Transform_Vtbl,
    pub SetDrawInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1DrawingStateBlock, ID2D1DrawingStateBlock_Vtbl, 0x28506e39_ebf6_46a1_bb47_fd85565ab957);
impl core::ops::Deref for ID2D1DrawingStateBlock {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DrawingStateBlock, windows_core::IUnknown, ID2D1Resource);
impl ID2D1DrawingStateBlock {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), statedescription)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), statedescription)
    }
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteRenderingParams>,
    {
        (windows_core::Interface::vtable(self).SetTextRenderingParams)(windows_core::Interface::as_raw(self), textrenderingparams.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> windows_core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTextRenderingParams)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1DrawingStateBlock {}
unsafe impl Sync for ID2D1DrawingStateBlock {}
#[repr(C)]
pub struct ID2D1DrawingStateBlock_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetDescription: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    GetTextRenderingParams: usize,
}
windows_core::imp::define_interface!(ID2D1DrawingStateBlock1, ID2D1DrawingStateBlock1_Vtbl, 0x689f1f85_c72e_4e33_8f19_85754efd5ace);
impl core::ops::Deref for ID2D1DrawingStateBlock1 {
    type Target = ID2D1DrawingStateBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DrawingStateBlock1, windows_core::IUnknown, ID2D1Resource, ID2D1DrawingStateBlock);
impl ID2D1DrawingStateBlock1 {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), statedescription)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), statedescription)
    }
}
unsafe impl Send for ID2D1DrawingStateBlock1 {}
unsafe impl Sync for ID2D1DrawingStateBlock1 {}
#[repr(C)]
pub struct ID2D1DrawingStateBlock1_Vtbl {
    pub base__: ID2D1DrawingStateBlock_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_DRAWING_STATE_DESCRIPTION1),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetDescription: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION1),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDescription: usize,
}
windows_core::imp::define_interface!(ID2D1Effect, ID2D1Effect_Vtbl, 0x28211a43_7d89_476f_8181_2d6159b220ad);
impl core::ops::Deref for ID2D1Effect {
    type Target = ID2D1Properties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Effect, windows_core::IUnknown, ID2D1Properties);
impl ID2D1Effect {
    pub unsafe fn SetInput<P0, P1>(&self, index: u32, input: P0, invalidate: P1)
    where
        P0: windows_core::Param<ID2D1Image>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetInput)(windows_core::Interface::as_raw(self), index, input.param().abi(), invalidate.param().abi())
    }
    pub unsafe fn SetInputCount(&self, inputcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInputCount)(windows_core::Interface::as_raw(self), inputcount).ok()
    }
    pub unsafe fn GetInput(&self, index: u32) -> windows_core::Result<ID2D1Image> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInput)(windows_core::Interface::as_raw(self), index, &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetInputCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetOutput(&self) -> windows_core::Result<ID2D1Image> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1Effect {}
unsafe impl Sync for ID2D1Effect {}
#[repr(C)]
pub struct ID2D1Effect_Vtbl {
    pub base__: ID2D1Properties_Vtbl,
    pub SetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub SetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void),
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1EffectContext, ID2D1EffectContext_Vtbl, 0x3d9f916b_27dc_4ad7_b4f1_64945340f563);
impl core::ops::Deref for ID2D1EffectContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1EffectContext, windows_core::IUnknown);
impl ID2D1EffectContext {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Effect> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), effectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetMaximumSupportedFeatureLevel(&self, featurelevels: &[super::Direct3D::D3D_FEATURE_LEVEL]) -> windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaximumSupportedFeatureLevel)(windows_core::Interface::as_raw(self), core::mem::transmute(featurelevels.as_ptr()), featurelevels.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateTransformNodeFromEffect<P0>(&self, effect: P0) -> windows_core::Result<ID2D1TransformNode>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransformNodeFromEffect)(windows_core::Interface::as_raw(self), effect.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBlendTransform(&self, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION) -> windows_core::Result<ID2D1BlendTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBlendTransform)(windows_core::Interface::as_raw(self), numinputs, blenddescription, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBorderTransform(&self, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE) -> windows_core::Result<ID2D1BorderTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBorderTransform)(windows_core::Interface::as_raw(self), extendmodex, extendmodey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateOffsetTransform(&self, offset: super::super::Foundation::POINT) -> windows_core::Result<ID2D1OffsetTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateOffsetTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(offset), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBoundsAdjustmentTransform(&self, outputrectangle: *const super::super::Foundation::RECT) -> windows_core::Result<ID2D1BoundsAdjustmentTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBoundsAdjustmentTransform)(windows_core::Interface::as_raw(self), outputrectangle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadPixelShader(&self, shaderid: *const windows_core::GUID, shaderbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadPixelShader)(windows_core::Interface::as_raw(self), shaderid, core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn LoadVertexShader(&self, resourceid: *const windows_core::GUID, shaderbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadVertexShader)(windows_core::Interface::as_raw(self), resourceid, core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn LoadComputeShader(&self, resourceid: *const windows_core::GUID, shaderbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadComputeShader)(windows_core::Interface::as_raw(self), resourceid, core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn IsShaderLoaded(&self, shaderid: *const windows_core::GUID) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsShaderLoaded)(windows_core::Interface::as_raw(self), shaderid)
    }
    pub unsafe fn CreateResourceTexture(&self, resourceid: Option<*const windows_core::GUID>, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: Option<&[u8]>, strides: Option<*const u32>) -> windows_core::Result<ID2D1ResourceTexture> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateResourceTexture)(windows_core::Interface::as_raw(self), core::mem::transmute(resourceid.unwrap_or(std::ptr::null())), resourcetextureproperties, core::mem::transmute(data.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(strides.unwrap_or(std::ptr::null())), data.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindResourceTexture(&self, resourceid: *const windows_core::GUID) -> windows_core::Result<ID2D1ResourceTexture> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindResourceTexture)(windows_core::Interface::as_raw(self), resourceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVertexBuffer(&self, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: Option<*const windows_core::GUID>, customvertexbufferproperties: Option<*const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES>) -> windows_core::Result<ID2D1VertexBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVertexBuffer)(windows_core::Interface::as_raw(self), vertexbufferproperties, core::mem::transmute(resourceid.unwrap_or(std::ptr::null())), core::mem::transmute(customvertexbufferproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindVertexBuffer(&self, resourceid: *const windows_core::GUID) -> windows_core::Result<ID2D1VertexBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindVertexBuffer)(windows_core::Interface::as_raw(self), resourceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: Option<&[u8]>) -> windows_core::Result<ID2D1ColorContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContext)(windows_core::Interface::as_raw(self), space, core::mem::transmute(profile.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromFilename)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<super::Imaging::IWICColorContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromWicColorContext)(windows_core::Interface::as_raw(self), wiccolorcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D2D1_FEATURE, featuresupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, featuresupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsBufferPrecisionSupported)(windows_core::Interface::as_raw(self), bufferprecision)
    }
}
unsafe impl Send for ID2D1EffectContext {}
unsafe impl Sync for ID2D1EffectContext {}
#[repr(C)]
pub struct ID2D1EffectContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetMaximumSupportedFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Direct3D::D3D_FEATURE_LEVEL, u32, *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetMaximumSupportedFeatureLevel: usize,
    pub CreateTransformNodeFromEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlendTransform: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D2D1_BLEND_DESCRIPTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBorderTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE, D2D1_EXTEND_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOffsetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBoundsAdjustmentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const u8, u32) -> windows_core::HRESULT,
    pub LoadVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const u8, u32) -> windows_core::HRESULT,
    pub LoadComputeShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const u8, u32) -> windows_core::HRESULT,
    pub IsShaderLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> super::super::Foundation::BOOL,
    pub CreateResourceTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const D2D1_RESOURCE_TEXTURE_PROPERTIES, *const u8, *const u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindResourceTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVertexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_VERTEX_BUFFER_PROPERTIES, *const windows_core::GUID, *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVertexBuffer: usize,
    pub FindVertexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_COLOR_SPACE, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateColorContextFromWicColorContext: usize,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(ID2D1EffectContext1, ID2D1EffectContext1_Vtbl, 0x84ab595a_fc81_4546_bacd_e8ef4d8abe7a);
impl core::ops::Deref for ID2D1EffectContext1 {
    type Target = ID2D1EffectContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1EffectContext1, windows_core::IUnknown, ID2D1EffectContext);
impl ID2D1EffectContext1 {
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> windows_core::Result<ID2D1LookupTable3D> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLookupTable3D)(windows_core::Interface::as_raw(self), precision, core::mem::transmute(extents.as_ptr()), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), core::mem::transmute(strides.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1EffectContext1 {}
unsafe impl Sync for ID2D1EffectContext1 {}
#[repr(C)]
pub struct ID2D1EffectContext1_Vtbl {
    pub base__: ID2D1EffectContext_Vtbl,
    pub CreateLookupTable3D: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION, *const u32, *const u8, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1EffectContext2, ID2D1EffectContext2_Vtbl, 0x577ad2a0_9fc7_4dda_8b18_dab810140052);
impl core::ops::Deref for ID2D1EffectContext2 {
    type Target = ID2D1EffectContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1EffectContext2, windows_core::IUnknown, ID2D1EffectContext, ID2D1EffectContext1);
impl ID2D1EffectContext2 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<ID2D1ColorContext1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromDxgiColorSpace)(windows_core::Interface::as_raw(self), colorspace, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<ID2D1ColorContext1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContextFromSimpleColorProfile)(windows_core::Interface::as_raw(self), simpleprofile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1EffectContext2 {}
unsafe impl Sync for ID2D1EffectContext2 {}
#[repr(C)]
pub struct ID2D1EffectContext2_Vtbl {
    pub base__: ID2D1EffectContext1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateColorContextFromDxgiColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SIMPLE_COLOR_PROFILE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateColorContextFromSimpleColorProfile: usize,
}
windows_core::imp::define_interface!(ID2D1EffectImpl, ID2D1EffectImpl_Vtbl, 0xa248fd3f_3e6c_4e63_9f03_7f68ecc91db9);
impl core::ops::Deref for ID2D1EffectImpl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1EffectImpl, windows_core::IUnknown);
impl ID2D1EffectImpl {
    pub unsafe fn Initialize<P0, P1>(&self, effectcontext: P0, transformgraph: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1EffectContext>,
        P1: windows_core::Param<ID2D1TransformGraph>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), effectcontext.param().abi(), transformgraph.param().abi()).ok()
    }
    pub unsafe fn PrepareForRender(&self, changetype: D2D1_CHANGE_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrepareForRender)(windows_core::Interface::as_raw(self), changetype).ok()
    }
    pub unsafe fn SetGraph<P0>(&self, transformgraph: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformGraph>,
    {
        (windows_core::Interface::vtable(self).SetGraph)(windows_core::Interface::as_raw(self), transformgraph.param().abi()).ok()
    }
}
unsafe impl Send for ID2D1EffectImpl {}
unsafe impl Sync for ID2D1EffectImpl {}
#[repr(C)]
pub struct ID2D1EffectImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareForRender: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_CHANGE_TYPE) -> windows_core::HRESULT,
    pub SetGraph: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1EllipseGeometry, ID2D1EllipseGeometry_Vtbl, 0x2cd906a4_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1EllipseGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1EllipseGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1EllipseGeometry {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEllipse(&self) -> D2D1_ELLIPSE {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEllipse)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1EllipseGeometry {}
unsafe impl Sync for ID2D1EllipseGeometry {}
#[repr(C)]
pub struct ID2D1EllipseGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_ELLIPSE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEllipse: usize,
}
windows_core::imp::define_interface!(ID2D1Factory, ID2D1Factory_Vtbl, 0x06152247_6f50_465a_9245_118bfd3b6007);
impl core::ops::Deref for ID2D1Factory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn ReloadSystemMetrics(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReloadSystemMetrics)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (windows_core::Interface::vtable(self).GetDesktopDpi)(windows_core::Interface::as_raw(self), dpix, dpiy)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> windows_core::Result<ID2D1RectangleGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRectangleGeometry)(windows_core::Interface::as_raw(self), rectangle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> windows_core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRoundedRectangleGeometry)(windows_core::Interface::as_raw(self), roundedrectangle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> windows_core::Result<ID2D1EllipseGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEllipseGeometry)(windows_core::Interface::as_raw(self), ellipse, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[Option<ID2D1Geometry>]) -> windows_core::Result<ID2D1GeometryGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGeometryGroup)(windows_core::Interface::as_raw(self), fillmode, core::mem::transmute(geometries.as_ptr()), geometries.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<ID2D1TransformedGeometry>
    where
        P0: windows_core::Param<ID2D1Geometry>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransformedGeometry)(windows_core::Interface::as_raw(self), sourcegeometry.param().abi(), transform, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: Option<&[f32]>) -> windows_core::Result<ID2D1StrokeStyle> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStrokeStyle)(windows_core::Interface::as_raw(self), strokestyleproperties, core::mem::transmute(dashes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> windows_core::Result<ID2D1DrawingStateBlock>
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteRenderingParams>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDrawingStateBlock)(windows_core::Interface::as_raw(self), core::mem::transmute(drawingstatedescription.unwrap_or(std::ptr::null())), textrenderingparams.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<super::Imaging::IWICBitmap>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateWicBitmapRenderTarget)(windows_core::Interface::as_raw(self), target.param().abi(), rendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateHwndRenderTarget)(windows_core::Interface::as_raw(self), rendertargetproperties, hwndrendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<super::Dxgi::IDXGISurface>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDxgiSurfaceRenderTarget)(windows_core::Interface::as_raw(self), dxgisurface.param().abi(), rendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::Result<ID2D1DCRenderTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDCRenderTarget)(windows_core::Interface::as_raw(self), rendertargetproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory {}
unsafe impl Sync for ID2D1Factory {}
#[repr(C)]
pub struct ID2D1Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReloadSystemMetrics: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateRectangleGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateRectangleGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateRoundedRectangleGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateEllipseGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateEllipseGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGeometryGroup: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D1_FILL_MODE, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGeometryGroup: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateTransformedGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateTransformedGeometry: usize,
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStrokeStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_STROKE_STYLE_PROPERTIES, *const f32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub CreateDrawingStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite")))]
    CreateDrawingStateBlock: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateWicBitmapRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateHwndRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *const D2D1_HWND_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateHwndRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDxgiSurfaceRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDxgiSurfaceRenderTarget: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDCRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDCRenderTarget: usize,
}
windows_core::imp::define_interface!(ID2D1Factory1, ID2D1Factory1_Vtbl, 0xbb12d362_daee_4b9a_aa1d_14ba401cfa1f);
impl core::ops::Deref for ID2D1Factory1 {
    type Target = ID2D1Factory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory1, windows_core::IUnknown, ID2D1Factory);
impl ID2D1Factory1 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: Option<&[f32]>) -> windows_core::Result<ID2D1StrokeStyle1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStrokeStyle)(windows_core::Interface::as_raw(self), strokestyleproperties, core::mem::transmute(dashes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> windows_core::Result<ID2D1DrawingStateBlock1>
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteRenderingParams>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDrawingStateBlock)(windows_core::Interface::as_raw(self), core::mem::transmute(drawingstatedescription.unwrap_or(std::ptr::null())), textrenderingparams.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> windows_core::Result<ID2D1GdiMetafile>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGdiMetafile)(windows_core::Interface::as_raw(self), metafilestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const windows_core::GUID, propertyxml: P0, bindings: Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).RegisterEffectFromStream)(windows_core::Interface::as_raw(self), classid, propertyxml.param().abi(), core::mem::transmute(bindings.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const windows_core::GUID, propertyxml: P0, bindings: Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterEffectFromString)(windows_core::Interface::as_raw(self), classid, propertyxml.param().abi(), core::mem::transmute(bindings.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterEffect)(windows_core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: Option<&mut [windows_core::GUID]>, effectsreturned: Option<*mut u32>, effectsregistered: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRegisteredEffects)(windows_core::Interface::as_raw(self), core::mem::transmute(effects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(effectsreturned.unwrap_or(std::ptr::null_mut())), core::mem::transmute(effectsregistered.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const windows_core::GUID) -> windows_core::Result<ID2D1Properties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEffectProperties)(windows_core::Interface::as_raw(self), effectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory1 {}
unsafe impl Sync for ID2D1Factory1 {}
#[repr(C)]
pub struct ID2D1Factory1_Vtbl {
    pub base__: ID2D1Factory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
    pub CreateStrokeStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_STROKE_STYLE_PROPERTIES1, *const f32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub CreateDrawingStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_DRAWING_STATE_DESCRIPTION1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite")))]
    CreateDrawingStateBlock: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGdiMetafile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterEffectFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const D2D1_PROPERTY_BINDING, u32, PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterEffectFromStream: usize,
    pub RegisterEffectFromString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const D2D1_PROPERTY_BINDING, u32, PD2D1_EFFECT_FACTORY) -> windows_core::HRESULT,
    pub UnregisterEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetRegisteredEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetEffectProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Factory2, ID2D1Factory2_Vtbl, 0x94f81a73_9212_4376_9c58_b16a3a0d3992);
impl core::ops::Deref for ID2D1Factory2 {
    type Target = ID2D1Factory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory2, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1);
impl ID2D1Factory2 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device1>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory2 {}
unsafe impl Sync for ID2D1Factory2 {}
#[repr(C)]
pub struct ID2D1Factory2_Vtbl {
    pub base__: ID2D1Factory1_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Factory3, ID2D1Factory3_Vtbl, 0x0869759f_4f00_413f_b03e_2bda45404d0f);
impl core::ops::Deref for ID2D1Factory3 {
    type Target = ID2D1Factory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory3, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2);
impl ID2D1Factory3 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device2>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory3 {}
unsafe impl Sync for ID2D1Factory3 {}
#[repr(C)]
pub struct ID2D1Factory3_Vtbl {
    pub base__: ID2D1Factory2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Factory4, ID2D1Factory4_Vtbl, 0xbd4ec2d2_0662_4bee_ba8e_6f29f032e096);
impl core::ops::Deref for ID2D1Factory4 {
    type Target = ID2D1Factory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory4, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3);
impl ID2D1Factory4 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device3>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory4 {}
unsafe impl Sync for ID2D1Factory4 {}
#[repr(C)]
pub struct ID2D1Factory4_Vtbl {
    pub base__: ID2D1Factory3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Factory5, ID2D1Factory5_Vtbl, 0xc4349994_838e_4b0f_8cab_44997d9eeacc);
impl core::ops::Deref for ID2D1Factory5 {
    type Target = ID2D1Factory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory5, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4);
impl ID2D1Factory5 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device4>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory5 {}
unsafe impl Sync for ID2D1Factory5 {}
#[repr(C)]
pub struct ID2D1Factory5_Vtbl {
    pub base__: ID2D1Factory4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Factory6, ID2D1Factory6_Vtbl, 0xf9976f46_f642_44c1_97ca_da32ea2a2635);
impl core::ops::Deref for ID2D1Factory6 {
    type Target = ID2D1Factory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory6, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5);
impl ID2D1Factory6 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device5>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory6 {}
unsafe impl Sync for ID2D1Factory6 {}
#[repr(C)]
pub struct ID2D1Factory6_Vtbl {
    pub base__: ID2D1Factory5_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Factory7, ID2D1Factory7_Vtbl, 0xbdc2bdd3_b96c_4de6_bdf7_99d4745454de);
impl core::ops::Deref for ID2D1Factory7 {
    type Target = ID2D1Factory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory7, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5, ID2D1Factory6);
impl ID2D1Factory7 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device6>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory7 {}
unsafe impl Sync for ID2D1Factory7 {}
#[repr(C)]
pub struct ID2D1Factory7_Vtbl {
    pub base__: ID2D1Factory6_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1Factory8, ID2D1Factory8_Vtbl, 0x677c9311_f36d_4b1f_ae86_86d1223ffd3a);
impl core::ops::Deref for ID2D1Factory8 {
    type Target = ID2D1Factory7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory8, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1, ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5, ID2D1Factory6, ID2D1Factory7);
impl ID2D1Factory8 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device7>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Factory8 {}
unsafe impl Sync for ID2D1Factory8 {}
#[repr(C)]
pub struct ID2D1Factory8_Vtbl {
    pub base__: ID2D1Factory7_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
}
windows_core::imp::define_interface!(ID2D1GdiInteropRenderTarget, ID2D1GdiInteropRenderTarget_Vtbl, 0xe0db51c3_6f77_4bae_b3d5_e47509b35838);
impl core::ops::Deref for ID2D1GdiInteropRenderTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GdiInteropRenderTarget, windows_core::IUnknown);
impl ID2D1GdiInteropRenderTarget {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, mode: D2D1_DC_INITIALIZE_MODE) -> windows_core::Result<super::Gdi::HDC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), mode, &mut result__).map(|| result__)
    }
    pub unsafe fn ReleaseDC(&self, update: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), core::mem::transmute(update.unwrap_or(std::ptr::null()))).ok()
    }
}
unsafe impl Send for ID2D1GdiInteropRenderTarget {}
unsafe impl Sync for ID2D1GdiInteropRenderTarget {}
#[repr(C)]
pub struct ID2D1GdiInteropRenderTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DC_INITIALIZE_MODE, *mut super::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1GdiMetafile, ID2D1GdiMetafile_Vtbl, 0x2f543dc3_cfc1_4211_864f_cfd91c6f3395);
impl core::ops::Deref for ID2D1GdiMetafile {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafile, windows_core::IUnknown, ID2D1Resource);
impl ID2D1GdiMetafile {
    pub unsafe fn Stream<P0>(&self, sink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GdiMetafileSink>,
    {
        (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), sink.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetBounds(&self) -> windows_core::Result<Common::D2D_RECT_F> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1GdiMetafile {}
unsafe impl Sync for ID2D1GdiMetafile {}
#[repr(C)]
pub struct ID2D1GdiMetafile_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetBounds: usize,
}
windows_core::imp::define_interface!(ID2D1GdiMetafile1, ID2D1GdiMetafile1_Vtbl, 0x2e69f9e8_dd3f_4bf9_95ba_c04f49d788df);
impl core::ops::Deref for ID2D1GdiMetafile1 {
    type Target = ID2D1GdiMetafile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafile1, windows_core::IUnknown, ID2D1Resource, ID2D1GdiMetafile);
impl ID2D1GdiMetafile1 {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSourceBounds(&self) -> windows_core::Result<Common::D2D_RECT_F> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceBounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1GdiMetafile1 {}
unsafe impl Sync for ID2D1GdiMetafile1 {}
#[repr(C)]
pub struct ID2D1GdiMetafile1_Vtbl {
    pub base__: ID2D1GdiMetafile_Vtbl,
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSourceBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSourceBounds: usize,
}
windows_core::imp::define_interface!(ID2D1GdiMetafileSink, ID2D1GdiMetafileSink_Vtbl, 0x82237326_8111_4f7c_bcf4_b5c1175564fe);
impl core::ops::Deref for ID2D1GdiMetafileSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafileSink, windows_core::IUnknown);
impl ID2D1GdiMetafileSink {
    pub unsafe fn ProcessRecord(&self, recordtype: u32, recorddata: Option<*const core::ffi::c_void>, recorddatasize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessRecord)(windows_core::Interface::as_raw(self), recordtype, core::mem::transmute(recorddata.unwrap_or(std::ptr::null())), recorddatasize).ok()
    }
}
unsafe impl Send for ID2D1GdiMetafileSink {}
unsafe impl Sync for ID2D1GdiMetafileSink {}
#[repr(C)]
pub struct ID2D1GdiMetafileSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProcessRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1GdiMetafileSink1, ID2D1GdiMetafileSink1_Vtbl, 0xfd0ecb6b_91e6_411e_8655_395e760f91b4);
impl core::ops::Deref for ID2D1GdiMetafileSink1 {
    type Target = ID2D1GdiMetafileSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafileSink1, windows_core::IUnknown, ID2D1GdiMetafileSink);
impl ID2D1GdiMetafileSink1 {
    pub unsafe fn ProcessRecord(&self, recordtype: u32, recorddata: Option<*const core::ffi::c_void>, recorddatasize: u32, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessRecord)(windows_core::Interface::as_raw(self), recordtype, core::mem::transmute(recorddata.unwrap_or(std::ptr::null())), recorddatasize, flags).ok()
    }
}
unsafe impl Send for ID2D1GdiMetafileSink1 {}
unsafe impl Sync for ID2D1GdiMetafileSink1 {}
#[repr(C)]
pub struct ID2D1GdiMetafileSink1_Vtbl {
    pub base__: ID2D1GdiMetafileSink_Vtbl,
    pub ProcessRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Geometry, ID2D1Geometry_Vtbl, 0x2cd906a1_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Geometry {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Geometry, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Geometry {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> windows_core::Result<Common::D2D_RECT_F> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1StrokeStyle>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWidenedBounds)(windows_core::Interface::as_raw(self), strokewidth, strokestyle.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<ID2D1StrokeStyle>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrokeContainsPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(point), strokewidth, strokestyle.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FillContainsPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(point), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: windows_core::Param<ID2D1Geometry>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompareWithGeometry)(windows_core::Interface::as_raw(self), inputgeometry.param().abi(), core::mem::transmute(inputgeometrytransform.unwrap_or(std::ptr::null())), flatteningtolerance, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
    {
        (windows_core::Interface::vtable(self).Simplify)(windows_core::Interface::as_raw(self), simplificationoption, core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, geometrysink.param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TessellationSink>,
    {
        (windows_core::Interface::vtable(self).Tessellate)(windows_core::Interface::as_raw(self), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, tessellationsink.param().abi()).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
    {
        (windows_core::Interface::vtable(self).CombineWithGeometry)(windows_core::Interface::as_raw(self), inputgeometry.param().abi(), combinemode, core::mem::transmute(inputgeometrytransform.unwrap_or(std::ptr::null())), flatteningtolerance, geometrysink.param().abi()).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
    {
        (windows_core::Interface::vtable(self).Outline)(windows_core::Interface::as_raw(self), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, geometrysink.param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputeArea)(windows_core::Interface::as_raw(self), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputeLength)(windows_core::Interface::as_raw(self), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: Option<*mut Common::D2D_POINT_2F>, unittangentvector: Option<*mut Common::D2D_POINT_2F>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComputePointAtLength)(windows_core::Interface::as_raw(self), length, core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, core::mem::transmute(point.unwrap_or(std::ptr::null_mut())), core::mem::transmute(unittangentvector.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1StrokeStyle>,
        P1: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
    {
        (windows_core::Interface::vtable(self).Widen)(windows_core::Interface::as_raw(self), strokewidth, strokestyle.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, geometrysink.param().abi()).ok()
    }
}
unsafe impl Send for ID2D1Geometry {}
unsafe impl Sync for ID2D1Geometry {}
#[repr(C)]
pub struct ID2D1Geometry_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetBounds: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetWidenedBounds: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetWidenedBounds: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub StrokeContainsPoint: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, f32, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    StrokeContainsPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub FillContainsPoint: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    FillContainsPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CompareWithGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut D2D1_GEOMETRY_RELATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CompareWithGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Simplify: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_GEOMETRY_SIMPLIFICATION_OPTION, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Simplify: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Tessellate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Tessellate: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CombineWithGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_COMBINE_MODE, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CombineWithGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Outline: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Outline: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeArea: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeArea: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeLength: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputePointAtLength: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut Common::D2D_POINT_2F, *mut Common::D2D_POINT_2F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputePointAtLength: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub Widen: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    Widen: usize,
}
windows_core::imp::define_interface!(ID2D1GeometryGroup, ID2D1GeometryGroup_Vtbl, 0x2cd906a6_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1GeometryGroup {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GeometryGroup, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1GeometryGroup {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetFillMode(&self) -> Common::D2D1_FILL_MODE {
        (windows_core::Interface::vtable(self).GetFillMode)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetSourceGeometryCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetSourceGeometryCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetSourceGeometries(&self, geometries: &mut [Option<ID2D1Geometry>]) {
        (windows_core::Interface::vtable(self).GetSourceGeometries)(windows_core::Interface::as_raw(self), core::mem::transmute(geometries.as_ptr()), geometries.len().try_into().unwrap())
    }
}
unsafe impl Send for ID2D1GeometryGroup {}
unsafe impl Sync for ID2D1GeometryGroup {}
#[repr(C)]
pub struct ID2D1GeometryGroup_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetFillMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> Common::D2D1_FILL_MODE,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetFillMode: usize,
    pub GetSourceGeometryCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSourceGeometries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32),
}
windows_core::imp::define_interface!(ID2D1GeometryRealization, ID2D1GeometryRealization_Vtbl, 0xa16907d7_bc02_4801_99e8_8cf7f485f774);
impl core::ops::Deref for ID2D1GeometryRealization {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GeometryRealization, windows_core::IUnknown, ID2D1Resource);
impl ID2D1GeometryRealization {}
unsafe impl Send for ID2D1GeometryRealization {}
unsafe impl Sync for ID2D1GeometryRealization {}
#[repr(C)]
pub struct ID2D1GeometryRealization_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
windows_core::imp::define_interface!(ID2D1GeometrySink, ID2D1GeometrySink_Vtbl, 0x2cd9069f_12e2_11dc_9fed_001143a055f9);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl core::ops::Deref for ID2D1GeometrySink {
    type Target = Common::ID2D1SimplifiedGeometrySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
windows_core::imp::interface_hierarchy!(ID2D1GeometrySink, windows_core::IUnknown, Common::ID2D1SimplifiedGeometrySink);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GeometrySink {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddLine(&self, point: Common::D2D_POINT_2F) {
        (windows_core::Interface::vtable(self).AddLine)(windows_core::Interface::as_raw(self), core::mem::transmute(point))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddBezier(&self, bezier: *const Common::D2D1_BEZIER_SEGMENT) {
        (windows_core::Interface::vtable(self).AddBezier)(windows_core::Interface::as_raw(self), bezier)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
        (windows_core::Interface::vtable(self).AddQuadraticBezier)(windows_core::Interface::as_raw(self), bezier)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddQuadraticBeziers(&self, beziers: &[D2D1_QUADRATIC_BEZIER_SEGMENT]) {
        (windows_core::Interface::vtable(self).AddQuadraticBeziers)(windows_core::Interface::as_raw(self), core::mem::transmute(beziers.as_ptr()), beziers.len().try_into().unwrap())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT) {
        (windows_core::Interface::vtable(self).AddArc)(windows_core::Interface::as_raw(self), arc)
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl Send for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl Sync for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[repr(C)]
pub struct ID2D1GeometrySink_Vtbl {
    pub base__: Common::ID2D1SimplifiedGeometrySink_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddLine: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddLine: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddBezier: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_BEZIER_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddBezier: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddQuadraticBezier: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_QUADRATIC_BEZIER_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddQuadraticBezier: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddQuadraticBeziers: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_QUADRATIC_BEZIER_SEGMENT, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddQuadraticBeziers: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddArc: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ARC_SEGMENT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddArc: usize,
}
windows_core::imp::define_interface!(ID2D1GradientMesh, ID2D1GradientMesh_Vtbl, 0xf292e401_c050_4cde_83d7_04962d3b23c2);
impl core::ops::Deref for ID2D1GradientMesh {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GradientMesh, windows_core::IUnknown, ID2D1Resource);
impl ID2D1GradientMesh {
    pub unsafe fn GetPatchCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetPatchCount)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPatches(&self, startindex: u32, patches: &mut [D2D1_GRADIENT_MESH_PATCH]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPatches)(windows_core::Interface::as_raw(self), startindex, core::mem::transmute(patches.as_ptr()), patches.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID2D1GradientMesh {}
unsafe impl Sync for ID2D1GradientMesh {}
#[repr(C)]
pub struct ID2D1GradientMesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetPatchCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPatches: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D2D1_GRADIENT_MESH_PATCH, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPatches: usize,
}
windows_core::imp::define_interface!(ID2D1GradientStopCollection, ID2D1GradientStopCollection_Vtbl, 0x2cd906a7_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1GradientStopCollection {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GradientStopCollection, windows_core::IUnknown, ID2D1Resource);
impl ID2D1GradientStopCollection {
    pub unsafe fn GetGradientStopCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetGradientStopCount)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientStops(&self, gradientstops: &mut [Common::D2D1_GRADIENT_STOP]) {
        (windows_core::Interface::vtable(self).GetGradientStops)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientstops.as_ptr()), gradientstops.len().try_into().unwrap())
    }
    pub unsafe fn GetColorInterpolationGamma(&self) -> D2D1_GAMMA {
        (windows_core::Interface::vtable(self).GetColorInterpolationGamma)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetExtendMode(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendMode)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1GradientStopCollection {}
unsafe impl Sync for ID2D1GradientStopCollection {}
#[repr(C)]
pub struct ID2D1GradientStopCollection_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetGradientStopCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_GRADIENT_STOP, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops: usize,
    pub GetColorInterpolationGamma: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_GAMMA,
    pub GetExtendMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
}
windows_core::imp::define_interface!(ID2D1GradientStopCollection1, ID2D1GradientStopCollection1_Vtbl, 0xae1572f4_5dd0_4777_998b_9279472ae63b);
impl core::ops::Deref for ID2D1GradientStopCollection1 {
    type Target = ID2D1GradientStopCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GradientStopCollection1, windows_core::IUnknown, ID2D1Resource, ID2D1GradientStopCollection);
impl ID2D1GradientStopCollection1 {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientStops1(&self, gradientstops: &mut [Common::D2D1_GRADIENT_STOP]) {
        (windows_core::Interface::vtable(self).GetGradientStops1)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientstops.as_ptr()), gradientstops.len().try_into().unwrap())
    }
    pub unsafe fn GetPreInterpolationSpace(&self) -> D2D1_COLOR_SPACE {
        (windows_core::Interface::vtable(self).GetPreInterpolationSpace)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPostInterpolationSpace(&self) -> D2D1_COLOR_SPACE {
        (windows_core::Interface::vtable(self).GetPostInterpolationSpace)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetBufferPrecision(&self) -> D2D1_BUFFER_PRECISION {
        (windows_core::Interface::vtable(self).GetBufferPrecision)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> D2D1_COLOR_INTERPOLATION_MODE {
        (windows_core::Interface::vtable(self).GetColorInterpolationMode)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1GradientStopCollection1 {}
unsafe impl Sync for ID2D1GradientStopCollection1 {}
#[repr(C)]
pub struct ID2D1GradientStopCollection1_Vtbl {
    pub base__: ID2D1GradientStopCollection_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_GRADIENT_STOP, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops1: usize,
    pub GetPreInterpolationSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetPostInterpolationSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_SPACE,
    pub GetBufferPrecision: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BUFFER_PRECISION,
    pub GetColorInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE,
}
windows_core::imp::define_interface!(ID2D1HwndRenderTarget, ID2D1HwndRenderTarget_Vtbl, 0x2cd90698_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1HwndRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1HwndRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1HwndRenderTarget {
    pub unsafe fn CheckWindowState(&self) -> D2D1_WINDOW_STATE {
        (windows_core::Interface::vtable(self).CheckWindowState)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Resize(&self, pixelsize: *const Common::D2D_SIZE_U) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), pixelsize).ok()
    }
    pub unsafe fn GetHwnd(&self) -> super::super::Foundation::HWND {
        (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1HwndRenderTarget {}
unsafe impl Sync for ID2D1HwndRenderTarget {}
#[repr(C)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub CheckWindowState: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_WINDOW_STATE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_SIZE_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Resize: usize,
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HWND,
}
windows_core::imp::define_interface!(ID2D1Image, ID2D1Image_Vtbl, 0x65019f75_8da2_497c_b32c_dfa34e48ede6);
impl core::ops::Deref for ID2D1Image {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Image, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Image {}
unsafe impl Send for ID2D1Image {}
unsafe impl Sync for ID2D1Image {}
#[repr(C)]
pub struct ID2D1Image_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
windows_core::imp::define_interface!(ID2D1ImageBrush, ID2D1ImageBrush_Vtbl, 0xfe9e984d_3f95_407c_b5db_cb94d4e8f87c);
impl core::ops::Deref for ID2D1ImageBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ImageBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1ImageBrush {
    pub unsafe fn SetImage<P0>(&self, image: P0)
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        (windows_core::Interface::vtable(self).SetImage)(windows_core::Interface::as_raw(self), image.param().abi())
    }
    pub unsafe fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE) {
        (windows_core::Interface::vtable(self).SetExtendModeX)(windows_core::Interface::as_raw(self), extendmodex)
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE) {
        (windows_core::Interface::vtable(self).SetExtendModeY)(windows_core::Interface::as_raw(self), extendmodey)
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_INTERPOLATION_MODE) {
        (windows_core::Interface::vtable(self).SetInterpolationMode)(windows_core::Interface::as_raw(self), interpolationmode)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSourceRectangle(&self, sourcerectangle: *const Common::D2D_RECT_F) {
        (windows_core::Interface::vtable(self).SetSourceRectangle)(windows_core::Interface::as_raw(self), sourcerectangle)
    }
    pub unsafe fn GetImage(&self) -> windows_core::Result<ID2D1Image> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImage)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendModeX)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (windows_core::Interface::vtable(self).GetExtendModeY)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_INTERPOLATION_MODE {
        (windows_core::Interface::vtable(self).GetInterpolationMode)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSourceRectangle(&self) -> Common::D2D_RECT_F {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceRectangle)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1ImageBrush {}
unsafe impl Sync for ID2D1ImageBrush {}
#[repr(C)]
pub struct ID2D1ImageBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_EXTEND_MODE),
    pub SetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_INTERPOLATION_MODE),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSourceRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSourceRectangle: usize,
    pub GetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetExtendModeX: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetExtendModeY: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
    pub GetInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_INTERPOLATION_MODE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSourceRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSourceRectangle: usize,
}
windows_core::imp::define_interface!(ID2D1ImageSource, ID2D1ImageSource_Vtbl, 0xc9b664e5_74a1_4378_9ac2_eefc37a3f4d8);
impl core::ops::Deref for ID2D1ImageSource {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ImageSource, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
impl ID2D1ImageSource {
    pub unsafe fn OfferResources(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OfferResources)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TryReclaimResources(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TryReclaimResources)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1ImageSource {}
unsafe impl Sync for ID2D1ImageSource {}
#[repr(C)]
pub struct ID2D1ImageSource_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub OfferResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryReclaimResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1ImageSourceFromWic, ID2D1ImageSourceFromWic_Vtbl, 0x77395441_1c8f_4555_8683_f50dab0fe792);
impl core::ops::Deref for ID2D1ImageSourceFromWic {
    type Target = ID2D1ImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ImageSourceFromWic, windows_core::IUnknown, ID2D1Resource, ID2D1Image, ID2D1ImageSource);
impl ID2D1ImageSourceFromWic {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnsureCached(&self, rectangletofill: Option<*const Common::D2D_RECT_U>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnsureCached)(windows_core::Interface::as_raw(self), core::mem::transmute(rectangletofill.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn TrimCache(&self, rectangletopreserve: Option<*const Common::D2D_RECT_U>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TrimCache)(windows_core::Interface::as_raw(self), core::mem::transmute(rectangletopreserve.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetSource(&self) -> windows_core::Result<super::Imaging::IWICBitmapSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1ImageSourceFromWic {}
unsafe impl Sync for ID2D1ImageSourceFromWic {}
#[repr(C)]
pub struct ID2D1ImageSourceFromWic_Vtbl {
    pub base__: ID2D1ImageSource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnsureCached: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnsureCached: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub TrimCache: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    TrimCache: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetSource: usize,
}
windows_core::imp::define_interface!(ID2D1Ink, ID2D1Ink_Vtbl, 0xb499923b_7029_478f_a8b3_432c7c5f5312);
impl core::ops::Deref for ID2D1Ink {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Ink, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Ink {
    pub unsafe fn SetStartPoint(&self, startpoint: *const D2D1_INK_POINT) {
        (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), startpoint)
    }
    pub unsafe fn GetStartPoint(&self) -> D2D1_INK_POINT {
        let mut result__: D2D1_INK_POINT = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn AddSegments(&self, segments: &[D2D1_INK_BEZIER_SEGMENT]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSegments)(windows_core::Interface::as_raw(self), core::mem::transmute(segments.as_ptr()), segments.len().try_into().unwrap()).ok()
    }
    pub unsafe fn RemoveSegmentsAtEnd(&self, segmentscount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveSegmentsAtEnd)(windows_core::Interface::as_raw(self), segmentscount).ok()
    }
    pub unsafe fn SetSegments(&self, startsegment: u32, segments: &[D2D1_INK_BEZIER_SEGMENT]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSegments)(windows_core::Interface::as_raw(self), startsegment, core::mem::transmute(segments.as_ptr()), segments.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetSegmentAtEnd(&self, segment: *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSegmentAtEnd)(windows_core::Interface::as_raw(self), segment).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetSegments(&self, startsegment: u32, segments: &mut [D2D1_INK_BEZIER_SEGMENT]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSegments)(windows_core::Interface::as_raw(self), startsegment, core::mem::transmute(segments.as_ptr()), segments.len().try_into().unwrap()).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StreamAsGeometry<P0, P1>(&self, inkstyle: P0, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1InkStyle>,
        P1: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
    {
        (windows_core::Interface::vtable(self).StreamAsGeometry)(windows_core::Interface::as_raw(self), inkstyle.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, geometrysink.param().abi()).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds<P0>(&self, inkstyle: P0, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1InkStyle>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), inkstyle.param().abi(), core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1Ink {}
unsafe impl Sync for ID2D1Ink {}
#[repr(C)]
pub struct ID2D1Ink_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_POINT),
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_INK_POINT),
    pub AddSegments: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_BEZIER_SEGMENT, u32) -> windows_core::HRESULT,
    pub RemoveSegmentsAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D2D1_INK_BEZIER_SEGMENT, u32) -> windows_core::HRESULT,
    pub SetSegmentAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D2D1_INK_BEZIER_SEGMENT, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub StreamAsGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    StreamAsGeometry: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetBounds: usize,
}
windows_core::imp::define_interface!(ID2D1InkStyle, ID2D1InkStyle_Vtbl, 0xbae8b344_23fc_4071_8cb5_d05d6f073848);
impl core::ops::Deref for ID2D1InkStyle {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1InkStyle, windows_core::IUnknown, ID2D1Resource);
impl ID2D1InkStyle {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetNibTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).SetNibTransform)(windows_core::Interface::as_raw(self), transform)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetNibTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).GetNibTransform)(windows_core::Interface::as_raw(self), transform)
    }
    pub unsafe fn SetNibShape(&self, nibshape: D2D1_INK_NIB_SHAPE) {
        (windows_core::Interface::vtable(self).SetNibShape)(windows_core::Interface::as_raw(self), nibshape)
    }
    pub unsafe fn GetNibShape(&self) -> D2D1_INK_NIB_SHAPE {
        (windows_core::Interface::vtable(self).GetNibShape)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1InkStyle {}
unsafe impl Sync for ID2D1InkStyle {}
#[repr(C)]
pub struct ID2D1InkStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetNibTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetNibTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetNibTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetNibTransform: usize,
    pub SetNibShape: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_INK_NIB_SHAPE),
    pub GetNibShape: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_INK_NIB_SHAPE,
}
windows_core::imp::define_interface!(ID2D1Layer, ID2D1Layer_Vtbl, 0x2cd9069b_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Layer {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Layer, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Layer {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1Layer {}
unsafe impl Sync for ID2D1Layer {}
#[repr(C)]
pub struct ID2D1Layer_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
}
windows_core::imp::define_interface!(ID2D1LinearGradientBrush, ID2D1LinearGradientBrush_Vtbl, 0x2cd906ab_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1LinearGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1LinearGradientBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1LinearGradientBrush {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetStartPoint(&self, startpoint: Common::D2D_POINT_2F) {
        (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(startpoint))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetEndPoint(&self, endpoint: Common::D2D_POINT_2F) {
        (windows_core::Interface::vtable(self).SetEndPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(endpoint))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetStartPoint(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEndPoint(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEndPoint)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGradientStopCollection(&self) -> windows_core::Result<ID2D1GradientStopCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGradientStopCollection)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1LinearGradientBrush {}
unsafe impl Sync for ID2D1LinearGradientBrush {}
#[repr(C)]
pub struct ID2D1LinearGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetStartPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetEndPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetStartPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetEndPoint: usize,
    pub GetGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1LookupTable3D, ID2D1LookupTable3D_Vtbl, 0x53dd9855_a3b0_4d5b_82e1_26e25c5e5797);
impl core::ops::Deref for ID2D1LookupTable3D {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1LookupTable3D, windows_core::IUnknown, ID2D1Resource);
impl ID2D1LookupTable3D {}
unsafe impl Send for ID2D1LookupTable3D {}
unsafe impl Sync for ID2D1LookupTable3D {}
#[repr(C)]
pub struct ID2D1LookupTable3D_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
windows_core::imp::define_interface!(ID2D1Mesh, ID2D1Mesh_Vtbl, 0x2cd906c2_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Mesh {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Mesh, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Mesh {
    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1TessellationSink> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Mesh {}
unsafe impl Sync for ID2D1Mesh {}
#[repr(C)]
pub struct ID2D1Mesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Multithread, ID2D1Multithread_Vtbl, 0x31e6e7bc_e0ff_4d46_8c64_a0a8c41c15d3);
impl core::ops::Deref for ID2D1Multithread {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Multithread, windows_core::IUnknown);
impl ID2D1Multithread {
    pub unsafe fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetMultithreadProtected)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Enter(&self) {
        (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Leave(&self) {
        (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1Multithread {}
unsafe impl Sync for ID2D1Multithread {}
#[repr(C)]
pub struct ID2D1Multithread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1OffsetTransform, ID2D1OffsetTransform_Vtbl, 0x3fe6adea_7643_4f53_bd14_a0ce63f24042);
impl core::ops::Deref for ID2D1OffsetTransform {
    type Target = ID2D1TransformNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1OffsetTransform, windows_core::IUnknown, ID2D1TransformNode);
impl ID2D1OffsetTransform {
    pub unsafe fn SetOffset(&self, offset: super::super::Foundation::POINT) {
        (windows_core::Interface::vtable(self).SetOffset)(windows_core::Interface::as_raw(self), core::mem::transmute(offset))
    }
    pub unsafe fn GetOffset(&self) -> super::super::Foundation::POINT {
        let mut result__: super::super::Foundation::POINT = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOffset)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1OffsetTransform {}
unsafe impl Sync for ID2D1OffsetTransform {}
#[repr(C)]
pub struct ID2D1OffsetTransform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    pub SetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::POINT),
    pub GetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT),
}
windows_core::imp::define_interface!(ID2D1PathGeometry, ID2D1PathGeometry_Vtbl, 0x2cd906a5_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1PathGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1PathGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1PathGeometry {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1GeometrySink> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Stream<P0>(&self, geometrysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GeometrySink>,
    {
        (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), geometrysink.param().abi()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFigureCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFigureCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1PathGeometry {}
unsafe impl Sync for ID2D1PathGeometry {}
#[repr(C)]
pub struct ID2D1PathGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Open: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Stream: usize,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFigureCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1PathGeometry1, ID2D1PathGeometry1_Vtbl, 0x62baa2d2_ab54_41b7_b872_787e0106a421);
impl core::ops::Deref for ID2D1PathGeometry1 {
    type Target = ID2D1PathGeometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1PathGeometry1, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry, ID2D1PathGeometry);
impl ID2D1PathGeometry1 {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAndSegmentAtLength(&self, length: f32, startsegment: u32, worldtransform: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComputePointAndSegmentAtLength)(windows_core::Interface::as_raw(self), length, startsegment, core::mem::transmute(worldtransform.unwrap_or(std::ptr::null())), flatteningtolerance, pointdescription).ok()
    }
}
unsafe impl Send for ID2D1PathGeometry1 {}
unsafe impl Sync for ID2D1PathGeometry1 {}
#[repr(C)]
pub struct ID2D1PathGeometry1_Vtbl {
    pub base__: ID2D1PathGeometry_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputePointAndSegmentAtLength: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32, *const super::super::super::Foundation::Numerics::Matrix3x2, f32, *mut D2D1_POINT_DESCRIPTION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputePointAndSegmentAtLength: usize,
}
windows_core::imp::define_interface!(ID2D1PrintControl, ID2D1PrintControl_Vtbl, 0x2c1d867d_c290_41c8_ae7e_34a98702e9a5);
impl core::ops::Deref for ID2D1PrintControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1PrintControl, windows_core::IUnknown);
impl ID2D1PrintControl {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub unsafe fn AddPage<P0, P1>(&self, commandlist: P0, pagesize: Common::D2D_SIZE_F, pageprintticketstream: P1, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1CommandList>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddPage)(windows_core::Interface::as_raw(self), commandlist.param().abi(), core::mem::transmute(pagesize), pageprintticketstream.param().abi(), core::mem::transmute(tag1.unwrap_or(std::ptr::null_mut())), core::mem::transmute(tag2.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1PrintControl {}
unsafe impl Sync for ID2D1PrintControl {}
#[repr(C)]
pub struct ID2D1PrintControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub AddPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, Common::D2D_SIZE_F, *mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com")))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Properties, ID2D1Properties_Vtbl, 0x483473d7_cd46_4f9d_9d3a_3112aa80159d);
impl core::ops::Deref for ID2D1Properties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Properties, windows_core::IUnknown);
impl ID2D1Properties {
    pub unsafe fn GetPropertyCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPropertyName(&self, index: u32, name: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyName)(windows_core::Interface::as_raw(self), index, core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPropertyNameLength(&self, index: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetPropertyNameLength)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetPropertyIndex<P0>(&self, name: P0) -> u32
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyIndex)(windows_core::Interface::as_raw(self), name.param().abi())
    }
    pub unsafe fn SetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetValueByName)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), index, r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetValueByName)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), index, r#type, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetValueSize(&self, index: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetValueSize)(windows_core::Interface::as_raw(self), index)
    }
    pub unsafe fn GetSubProperties(&self, index: u32) -> windows_core::Result<ID2D1Properties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubProperties)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1Properties {}
unsafe impl Sync for ID2D1Properties {}
#[repr(C)]
pub struct ID2D1Properties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetPropertyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> D2D1_PROPERTY_TYPE,
    pub GetPropertyIndex: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> u32,
    pub SetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_PROPERTY_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D2D1_PROPERTY_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub GetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_PROPERTY_TYPE, *mut u8, u32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D2D1_PROPERTY_TYPE, *mut u8, u32) -> windows_core::HRESULT,
    pub GetValueSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetSubProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1RadialGradientBrush, ID2D1RadialGradientBrush_Vtbl, 0x2cd906ac_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RadialGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RadialGradientBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1RadialGradientBrush {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCenter(&self, center: Common::D2D_POINT_2F) {
        (windows_core::Interface::vtable(self).SetCenter)(windows_core::Interface::as_raw(self), core::mem::transmute(center))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetGradientOriginOffset(&self, gradientoriginoffset: Common::D2D_POINT_2F) {
        (windows_core::Interface::vtable(self).SetGradientOriginOffset)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientoriginoffset))
    }
    pub unsafe fn SetRadiusX(&self, radiusx: f32) {
        (windows_core::Interface::vtable(self).SetRadiusX)(windows_core::Interface::as_raw(self), radiusx)
    }
    pub unsafe fn SetRadiusY(&self, radiusy: f32) {
        (windows_core::Interface::vtable(self).SetRadiusY)(windows_core::Interface::as_raw(self), radiusy)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetCenter(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCenter)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientOriginOffset(&self) -> Common::D2D_POINT_2F {
        let mut result__: Common::D2D_POINT_2F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGradientOriginOffset)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetRadiusX(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetRadiusX)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRadiusY(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetRadiusY)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetGradientStopCollection(&self) -> windows_core::Result<ID2D1GradientStopCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGradientStopCollection)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1RadialGradientBrush {}
unsafe impl Sync for ID2D1RadialGradientBrush {}
#[repr(C)]
pub struct ID2D1RadialGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCenter: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCenter: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetGradientOriginOffset: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetGradientOriginOffset: usize,
    pub SetRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    pub SetRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetCenter: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientOriginOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_POINT_2F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientOriginOffset: usize,
    pub GetRadiusX: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetRadiusY: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1RectangleGeometry, ID2D1RectangleGeometry_Vtbl, 0x2cd906a2_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RectangleGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1RectangleGeometry {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRect(&self) -> Common::D2D_RECT_F {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1RectangleGeometry {}
unsafe impl Sync for ID2D1RectangleGeometry {}
#[repr(C)]
pub struct ID2D1RectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRect: usize,
}
windows_core::imp::define_interface!(ID2D1RenderInfo, ID2D1RenderInfo_Vtbl, 0x519ae1bd_d19a_420d_b849_364f594776b7);
impl core::ops::Deref for ID2D1RenderInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RenderInfo, windows_core::IUnknown);
impl ID2D1RenderInfo {
    pub unsafe fn SetInputDescription(&self, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInputDescription)(windows_core::Interface::as_raw(self), inputindex, core::mem::transmute(inputdescription)).ok()
    }
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOutputBuffer)(windows_core::Interface::as_raw(self), bufferprecision, channeldepth).ok()
    }
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCached)(windows_core::Interface::as_raw(self), iscached.param().abi())
    }
    pub unsafe fn SetInstructionCountHint(&self, instructioncount: u32) {
        (windows_core::Interface::vtable(self).SetInstructionCountHint)(windows_core::Interface::as_raw(self), instructioncount)
    }
}
unsafe impl Send for ID2D1RenderInfo {}
unsafe impl Sync for ID2D1RenderInfo {}
#[repr(C)]
pub struct ID2D1RenderInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInputDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D2D1_INPUT_DESCRIPTION) -> windows_core::HRESULT,
    pub SetOutputBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_BUFFER_PRECISION, D2D1_CHANNEL_DEPTH) -> windows_core::HRESULT,
    pub SetCached: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub SetInstructionCountHint: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
windows_core::imp::define_interface!(ID2D1RenderTarget, ID2D1RenderTarget_Vtbl, 0x2cd90694_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RenderTarget {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RenderTarget, windows_core::IUnknown, ID2D1Resource);
impl ID2D1RenderTarget {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: Option<*const core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(size), core::mem::transmute(srcdata.unwrap_or(std::ptr::null())), pitch, bitmapproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>) -> windows_core::Result<ID2D1Bitmap>
    where
        P0: windows_core::Param<super::Imaging::IWICBitmapSource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(windows_core::Interface::as_raw(self), wicbitmapsource.param().abi(), core::mem::transmute(bitmapproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut Option<ID2D1Bitmap>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateSharedBitmap)(windows_core::Interface::as_raw(self), riid, data, core::mem::transmute(bitmapproperties.unwrap_or(std::ptr::null())), core::mem::transmute(bitmap)).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1BitmapBrush>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapBrush)(windows_core::Interface::as_raw(self), bitmap.param().abi(), core::mem::transmute(bitmapbrushproperties.unwrap_or(std::ptr::null())), core::mem::transmute(brushproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>) -> windows_core::Result<ID2D1SolidColorBrush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSolidColorBrush)(windows_core::Interface::as_raw(self), color, core::mem::transmute(brushproperties.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[Common::D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> windows_core::Result<ID2D1GradientStopCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGradientStopCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(gradientstops.as_ptr()), gradientstops.len().try_into().unwrap(), colorinterpolationgamma, extendmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> windows_core::Result<ID2D1LinearGradientBrush>
    where
        P0: windows_core::Param<ID2D1GradientStopCollection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(windows_core::Interface::as_raw(self), lineargradientbrushproperties, core::mem::transmute(brushproperties.unwrap_or(std::ptr::null())), gradientstopcollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> windows_core::Result<ID2D1RadialGradientBrush>
    where
        P0: windows_core::Param<ID2D1GradientStopCollection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRadialGradientBrush)(windows_core::Interface::as_raw(self), radialgradientbrushproperties, core::mem::transmute(brushproperties.unwrap_or(std::ptr::null())), gradientstopcollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: Option<*const Common::D2D_SIZE_F>, desiredpixelsize: Option<*const Common::D2D_SIZE_U>, desiredformat: Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> windows_core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCompatibleRenderTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(desiredsize.unwrap_or(std::ptr::null())), core::mem::transmute(desiredpixelsize.unwrap_or(std::ptr::null())), core::mem::transmute(desiredformat.unwrap_or(std::ptr::null())), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: Option<*const Common::D2D_SIZE_F>) -> windows_core::Result<ID2D1Layer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLayer)(windows_core::Interface::as_raw(self), core::mem::transmute(size.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMesh)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawLine)(windows_core::Interface::as_raw(self), core::mem::transmute(point0), core::mem::transmute(point1), brush.param().abi(), strokewidth, strokestyle.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi(), strokewidth, strokestyle.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawRoundedRectangle)(windows_core::Interface::as_raw(self), roundedrect, brush.param().abi(), strokewidth, strokestyle.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillRoundedRectangle)(windows_core::Interface::as_raw(self), roundedrect, brush.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: windows_core::Param<ID2D1Brush>,
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawEllipse)(windows_core::Interface::as_raw(self), ellipse, brush.param().abi(), strokewidth, strokestyle.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillEllipse)(windows_core::Interface::as_raw(self), ellipse, brush.param().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1StrokeStyle>,
    {
        (windows_core::Interface::vtable(self).DrawGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), strokewidth, strokestyle.param().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), opacitybrush.param().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: windows_core::Param<ID2D1Mesh>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillMesh)(windows_core::Interface::as_raw(self), mesh.param().abi(), brush.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: Option<*const Common::D2D_RECT_F>, sourcerectangle: Option<*const Common::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), content, core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null())))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: Option<*const Common::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), core::mem::transmute(destinationrectangle.unwrap_or(std::ptr::null())), opacity, interpolationmode, core::mem::transmute(sourcerectangle.unwrap_or(std::ptr::null())))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteTextFormat>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawText)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), layoutrect, defaultfillbrush.param().abi(), options, measuringmode)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteTextLayout>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(origin), textlayout.param().abi(), defaultfillbrush.param().abi(), options)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.param().abi(), measuringmode)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (windows_core::Interface::vtable(self).SetAntialiasMode)(windows_core::Interface::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (windows_core::Interface::vtable(self).GetAntialiasMode)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (windows_core::Interface::vtable(self).SetTextAntialiasMode)(windows_core::Interface::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (windows_core::Interface::vtable(self).GetTextAntialiasMode)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: windows_core::Param<super::DirectWrite::IDWriteRenderingParams>,
    {
        (windows_core::Interface::vtable(self).SetTextRenderingParams)(windows_core::Interface::as_raw(self), textrenderingparams.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> windows_core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTextRenderingParams)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (windows_core::Interface::vtable(self).SetTags)(windows_core::Interface::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) {
        (windows_core::Interface::vtable(self).GetTags)(windows_core::Interface::as_raw(self), core::mem::transmute(tag1.unwrap_or(std::ptr::null_mut())), core::mem::transmute(tag2.unwrap_or(std::ptr::null_mut())))
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: windows_core::Param<ID2D1Layer>,
    {
        (windows_core::Interface::vtable(self).PushLayer)(windows_core::Interface::as_raw(self), layerparameters, layer.param().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), core::mem::transmute(tag1.unwrap_or(std::ptr::null_mut())), core::mem::transmute(tag2.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: windows_core::Param<ID2D1DrawingStateBlock>,
    {
        (windows_core::Interface::vtable(self).SaveDrawingState)(windows_core::Interface::as_raw(self), drawingstateblock.param().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: windows_core::Param<ID2D1DrawingStateBlock>,
    {
        (windows_core::Interface::vtable(self).RestoreDrawingState)(windows_core::Interface::as_raw(self), drawingstateblock.param().abi())
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (windows_core::Interface::vtable(self).PushAxisAlignedClip)(windows_core::Interface::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (windows_core::Interface::vtable(self).PopAxisAlignedClip)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: Option<*const Common::D2D1_COLOR_F>) {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), core::mem::transmute(clearcolor.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self), core::mem::transmute(tag1.unwrap_or(std::ptr::null_mut())), core::mem::transmute(tag2.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (windows_core::Interface::vtable(self).SetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetMaximumBitmapSize)(windows_core::Interface::as_raw(self))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), rendertargetproperties)
    }
}
unsafe impl Send for ID2D1RenderTarget {}
unsafe impl Sync for ID2D1RenderTarget {}
#[repr(C)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_SIZE_U, *const core::ffi::c_void, u32, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging")))]
    CreateBitmapFromWicBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSharedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSharedBitmap: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateBitmapBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateBitmapBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateSolidColorBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F, *const D2D1_BRUSH_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateSolidColorBrush: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_GRADIENT_STOP, u32, D2D1_GAMMA, D2D1_EXTEND_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateGradientStopCollection: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateLinearGradientBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateLinearGradientBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub CreateRadialGradientBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, *const D2D1_BRUSH_PROPERTIES, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    CreateRadialGradientBrush: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateCompatibleRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_SIZE_F, *const Common::D2D_SIZE_U, *const Common::D2D1_PIXEL_FORMAT, D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateCompatibleRenderTarget: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreateLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_SIZE_F, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreateLayer: usize,
    pub CreateMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawLine: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, Common::D2D_POINT_2F, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawLine: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawRoundedRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRoundedRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawEllipse: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillEllipse: usize,
    pub DrawGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f32, *mut core::ffi::c_void),
    pub FillGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub FillMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_OPACITY_MASK_CONTENT, *const Common::D2D_RECT_F, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, f32, D2D1_BITMAP_INTERPOLATION_MODE, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void, D2D1_DRAW_TEXT_OPTIONS, super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawText: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const super::DirectWrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, super::DirectWrite::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_ANTIALIAS_MODE),
    pub GetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE),
    pub GetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub SetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    SetTextRenderingParams: usize,
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub GetTextRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_DirectWrite"))]
    GetTextRenderingParams: usize,
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64),
    pub GetTags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64),
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS, *mut core::ffi::c_void),
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    PushLayer: usize,
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub SaveDrawingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RestoreDrawingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub PushAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, D2D1_ANTIALIAS_MODE),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    PushAxisAlignedClip: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub SetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    pub GetMaximumBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    IsSupported: usize,
}
windows_core::imp::define_interface!(ID2D1Resource, ID2D1Resource_Vtbl, 0x2cd90691_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Resource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Resource, windows_core::IUnknown);
impl ID2D1Resource {
    pub unsafe fn GetFactory(&self) -> windows_core::Result<ID2D1Factory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
}
unsafe impl Send for ID2D1Resource {}
unsafe impl Sync for ID2D1Resource {}
#[repr(C)]
pub struct ID2D1Resource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1ResourceTexture, ID2D1ResourceTexture_Vtbl, 0x688d15c3_02b0_438d_b13a_d1b44c32c39a);
impl core::ops::Deref for ID2D1ResourceTexture {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ResourceTexture, windows_core::IUnknown);
impl ID2D1ResourceTexture {
    pub unsafe fn Update(&self, minimumextents: Option<*const u32>, maximimumextents: Option<*const u32>, strides: Option<*const u32>, dimensions: u32, data: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), core::mem::transmute(minimumextents.unwrap_or(std::ptr::null())), core::mem::transmute(maximimumextents.unwrap_or(std::ptr::null())), core::mem::transmute(strides.unwrap_or(std::ptr::null())), dimensions, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for ID2D1ResourceTexture {}
unsafe impl Sync for ID2D1ResourceTexture {}
#[repr(C)]
pub struct ID2D1ResourceTexture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *const u32, *const u32, u32, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1RoundedRectangleGeometry, ID2D1RoundedRectangleGeometry_Vtbl, 0x2cd906a3_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RoundedRectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RoundedRectangleGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1RoundedRectangleGeometry {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRoundedRect(&self, roundedrect: *mut D2D1_ROUNDED_RECT) {
        (windows_core::Interface::vtable(self).GetRoundedRect)(windows_core::Interface::as_raw(self), roundedrect)
    }
}
unsafe impl Send for ID2D1RoundedRectangleGeometry {}
unsafe impl Sync for ID2D1RoundedRectangleGeometry {}
#[repr(C)]
pub struct ID2D1RoundedRectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetRoundedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_ROUNDED_RECT),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetRoundedRect: usize,
}
windows_core::imp::define_interface!(ID2D1SolidColorBrush, ID2D1SolidColorBrush_Vtbl, 0x2cd906a9_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1SolidColorBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SolidColorBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
impl ID2D1SolidColorBrush {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const Common::D2D1_COLOR_F) {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetColor(&self) -> Common::D2D1_COLOR_F {
        let mut result__: Common::D2D1_COLOR_F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
unsafe impl Send for ID2D1SolidColorBrush {}
unsafe impl Sync for ID2D1SolidColorBrush {}
#[repr(C)]
pub struct ID2D1SolidColorBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetColor: usize,
}
windows_core::imp::define_interface!(ID2D1SourceTransform, ID2D1SourceTransform_Vtbl, 0xdb1800dd_0c34_4cf9_be90_31cc0a5653e1);
impl core::ops::Deref for ID2D1SourceTransform {
    type Target = ID2D1Transform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SourceTransform, windows_core::IUnknown, ID2D1TransformNode, ID2D1Transform);
impl ID2D1SourceTransform {
    pub unsafe fn SetRenderInfo<P0>(&self, renderinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1RenderInfo>,
    {
        (windows_core::Interface::vtable(self).SetRenderInfo)(windows_core::Interface::as_raw(self), renderinfo.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Draw<P0>(&self, target: P0, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Bitmap1>,
    {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), target.param().abi(), drawrect, core::mem::transmute(targetorigin)).ok()
    }
}
unsafe impl Send for ID2D1SourceTransform {}
unsafe impl Sync for ID2D1SourceTransform {}
#[repr(C)]
pub struct ID2D1SourceTransform_Vtbl {
    pub base__: ID2D1Transform_Vtbl,
    pub SetRenderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Foundation::RECT, Common::D2D_POINT_2U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Draw: usize,
}
windows_core::imp::define_interface!(ID2D1SpriteBatch, ID2D1SpriteBatch_Vtbl, 0x4dc583bf_3a10_438a_8722_e9765224f1f1);
impl core::ops::Deref for ID2D1SpriteBatch {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SpriteBatch, windows_core::IUnknown, ID2D1Resource);
impl ID2D1SpriteBatch {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn AddSprites(&self, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: Option<*const Common::D2D_RECT_U>, colors: Option<*const Common::D2D1_COLOR_F>, transforms: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSprites)(windows_core::Interface::as_raw(self), spritecount, destinationrectangles, core::mem::transmute(sourcerectangles.unwrap_or(std::ptr::null())), core::mem::transmute(colors.unwrap_or(std::ptr::null())), core::mem::transmute(transforms.unwrap_or(std::ptr::null())), destinationrectanglesstride, sourcerectanglesstride, colorsstride, transformsstride).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn SetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: Option<*const Common::D2D_RECT_F>, sourcerectangles: Option<*const Common::D2D_RECT_U>, colors: Option<*const Common::D2D1_COLOR_F>, transforms: Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSprites)(windows_core::Interface::as_raw(self), startindex, spritecount, core::mem::transmute(destinationrectangles.unwrap_or(std::ptr::null())), core::mem::transmute(sourcerectangles.unwrap_or(std::ptr::null())), core::mem::transmute(colors.unwrap_or(std::ptr::null())), core::mem::transmute(transforms.unwrap_or(std::ptr::null())), destinationrectanglesstride, sourcerectanglesstride, colorsstride, transformsstride).ok()
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: Option<*mut Common::D2D_RECT_F>, sourcerectangles: Option<*mut Common::D2D_RECT_U>, colors: Option<*mut Common::D2D1_COLOR_F>, transforms: Option<*mut super::super::super::Foundation::Numerics::Matrix3x2>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSprites)(windows_core::Interface::as_raw(self), startindex, spritecount, core::mem::transmute(destinationrectangles.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sourcerectangles.unwrap_or(std::ptr::null_mut())), core::mem::transmute(colors.unwrap_or(std::ptr::null_mut())), core::mem::transmute(transforms.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetSpriteCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetSpriteCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Clear(&self) {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1SpriteBatch {}
unsafe impl Sync for ID2D1SpriteBatch {}
#[repr(C)]
pub struct ID2D1SpriteBatch_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub AddSprites: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Common::D2D_RECT_F, *const Common::D2D_RECT_U, *const Common::D2D1_COLOR_F, *const super::super::super::Foundation::Numerics::Matrix3x2, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    AddSprites: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub SetSprites: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const Common::D2D_RECT_F, *const Common::D2D_RECT_U, *const Common::D2D1_COLOR_F, *const super::super::super::Foundation::Numerics::Matrix3x2, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    SetSprites: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetSprites: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut Common::D2D_RECT_F, *mut Common::D2D_RECT_U, *mut Common::D2D1_COLOR_F, *mut super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common")))]
    GetSprites: usize,
    pub GetSpriteCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(ID2D1StrokeStyle, ID2D1StrokeStyle_Vtbl, 0x2cd9069d_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1StrokeStyle {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle, windows_core::IUnknown, ID2D1Resource);
impl ID2D1StrokeStyle {
    pub unsafe fn GetStartCap(&self) -> D2D1_CAP_STYLE {
        (windows_core::Interface::vtable(self).GetStartCap)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetEndCap(&self) -> D2D1_CAP_STYLE {
        (windows_core::Interface::vtable(self).GetEndCap)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDashCap(&self) -> D2D1_CAP_STYLE {
        (windows_core::Interface::vtable(self).GetDashCap)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMiterLimit(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetMiterLimit)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetLineJoin(&self) -> D2D1_LINE_JOIN {
        (windows_core::Interface::vtable(self).GetLineJoin)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDashOffset(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetDashOffset)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDashStyle(&self) -> D2D1_DASH_STYLE {
        (windows_core::Interface::vtable(self).GetDashStyle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetDashesCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [f32]) {
        (windows_core::Interface::vtable(self).GetDashes)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap())
    }
}
unsafe impl Send for ID2D1StrokeStyle {}
unsafe impl Sync for ID2D1StrokeStyle {}
#[repr(C)]
pub struct ID2D1StrokeStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetStartCap: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetEndCap: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetDashCap: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_CAP_STYLE,
    pub GetMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetLineJoin: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_LINE_JOIN,
    pub GetDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetDashStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_DASH_STYLE,
    pub GetDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32),
}
windows_core::imp::define_interface!(ID2D1StrokeStyle1, ID2D1StrokeStyle1_Vtbl, 0x10a72a66_e91c_43f4_993f_ddf4b82b0b4a);
impl core::ops::Deref for ID2D1StrokeStyle1 {
    type Target = ID2D1StrokeStyle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle1, windows_core::IUnknown, ID2D1Resource, ID2D1StrokeStyle);
impl ID2D1StrokeStyle1 {
    pub unsafe fn GetStrokeTransformType(&self) -> D2D1_STROKE_TRANSFORM_TYPE {
        (windows_core::Interface::vtable(self).GetStrokeTransformType)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1StrokeStyle1 {}
unsafe impl Sync for ID2D1StrokeStyle1 {}
#[repr(C)]
pub struct ID2D1StrokeStyle1_Vtbl {
    pub base__: ID2D1StrokeStyle_Vtbl,
    pub GetStrokeTransformType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE,
}
windows_core::imp::define_interface!(ID2D1SvgAttribute, ID2D1SvgAttribute_Vtbl, 0xc9cdb0dd_f8c9_4e70_b7c2_301c80292c5e);
impl core::ops::Deref for ID2D1SvgAttribute {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgAttribute, windows_core::IUnknown, ID2D1Resource);
impl ID2D1SvgAttribute {
    pub unsafe fn GetElement(&self) -> windows_core::Result<ID2D1SvgElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ID2D1SvgAttribute> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1SvgAttribute {}
unsafe impl Sync for ID2D1SvgAttribute {}
#[repr(C)]
pub struct ID2D1SvgAttribute_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1SvgDocument, ID2D1SvgDocument_Vtbl, 0x86b88e4d_afa4_4d7b_88e4_68a51c4a0aec);
impl core::ops::Deref for ID2D1SvgDocument {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgDocument, windows_core::IUnknown, ID2D1Resource);
impl ID2D1SvgDocument {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetViewportSize(&self, viewportsize: Common::D2D_SIZE_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewportSize)(windows_core::Interface::as_raw(self), core::mem::transmute(viewportsize)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetViewportSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewportSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetRoot<P0>(&self, root: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
    {
        (windows_core::Interface::vtable(self).SetRoot)(windows_core::Interface::as_raw(self), root.param().abi()).ok()
    }
    pub unsafe fn GetRoot(&self) -> windows_core::Result<ID2D1SvgElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRoot)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn FindElementById<P0>(&self, id: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindElementById)(windows_core::Interface::as_raw(self), id.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1>(&self, outputxmlstream: P0, subtree: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<ID2D1SvgElement>,
    {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), outputxmlstream.param().abi(), subtree.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<P0>(&self, inputxmlstream: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Deserialize)(windows_core::Interface::as_raw(self), inputxmlstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreatePaint<P0>(&self, painttype: D2D1_SVG_PAINT_TYPE, color: Option<*const Common::D2D1_COLOR_F>, id: P0) -> windows_core::Result<ID2D1SvgPaint>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePaint)(windows_core::Interface::as_raw(self), painttype, core::mem::transmute(color.unwrap_or(std::ptr::null())), id.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateStrokeDashArray(&self, dashes: Option<&[D2D1_SVG_LENGTH]>) -> windows_core::Result<ID2D1SvgStrokeDashArray> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStrokeDashArray)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreatePointCollection(&self, points: Option<&[Common::D2D_POINT_2F]>) -> windows_core::Result<ID2D1SvgPointCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePointCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), points.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePathData(&self, segmentdata: Option<&[f32]>, commands: Option<&[D2D1_SVG_PATH_COMMAND]>) -> windows_core::Result<ID2D1SvgPathData> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePathData)(windows_core::Interface::as_raw(self), core::mem::transmute(segmentdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), segmentdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(commands.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), commands.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1SvgDocument {}
unsafe impl Sync for ID2D1SvgDocument {}
#[repr(C)]
pub struct ID2D1SvgDocument_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_SIZE_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetViewportSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetViewportSize: usize,
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub FindElementById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePaint: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_SVG_PAINT_TYPE, *const Common::D2D1_COLOR_F, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePaint: usize,
    pub CreateStrokeDashArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SVG_LENGTH, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePointCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_POINT_2F, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePointCollection: usize,
    pub CreatePathData: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, *const D2D1_SVG_PATH_COMMAND, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1SvgElement, ID2D1SvgElement_Vtbl, 0xac7b67a6_183e_49c1_a823_0ebe40b0db29);
impl core::ops::Deref for ID2D1SvgElement {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgElement, windows_core::IUnknown, ID2D1Resource);
impl ID2D1SvgElement {
    pub unsafe fn GetDocument(&self) -> windows_core::Result<ID2D1SvgDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetTagName(&self, name: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTagName)(windows_core::Interface::as_raw(self), core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetTagNameLength(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetTagNameLength)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsTextContent(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsTextContent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<ID2D1SvgElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn HasChildren(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasChildren)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFirstChild(&self) -> windows_core::Result<ID2D1SvgElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstChild)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetLastChild(&self) -> windows_core::Result<ID2D1SvgElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastChild)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetPreviousChild<P0>(&self, referencechild: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousChild)(windows_core::Interface::as_raw(self), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextChild<P0>(&self, referencechild: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextChild)(windows_core::Interface::as_raw(self), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InsertChildBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
        P1: windows_core::Param<ID2D1SvgElement>,
    {
        (windows_core::Interface::vtable(self).InsertChildBefore)(windows_core::Interface::as_raw(self), newchild.param().abi(), referencechild.param().abi()).ok()
    }
    pub unsafe fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
    {
        (windows_core::Interface::vtable(self).AppendChild)(windows_core::Interface::as_raw(self), newchild.param().abi()).ok()
    }
    pub unsafe fn ReplaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
        P1: windows_core::Param<ID2D1SvgElement>,
    {
        (windows_core::Interface::vtable(self).ReplaceChild)(windows_core::Interface::as_raw(self), newchild.param().abi(), oldchild.param().abi()).ok()
    }
    pub unsafe fn RemoveChild<P0>(&self, oldchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1SvgElement>,
    {
        (windows_core::Interface::vtable(self).RemoveChild)(windows_core::Interface::as_raw(self), oldchild.param().abi()).ok()
    }
    pub unsafe fn CreateChild<P0>(&self, tagname: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateChild)(windows_core::Interface::as_raw(self), tagname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsAttributeSpecified<P0>(&self, name: P0, inherited: Option<*mut super::super::Foundation::BOOL>) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsAttributeSpecified)(windows_core::Interface::as_raw(self), name.param().abi(), core::mem::transmute(inherited.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GetSpecifiedAttributeCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetSpecifiedAttributeCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetSpecifiedAttributeName(&self, index: u32, name: &mut [u16], inherited: Option<*mut super::super::Foundation::BOOL>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSpecifiedAttributeName)(windows_core::Interface::as_raw(self), index, core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap(), core::mem::transmute(inherited.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetSpecifiedAttributeNameLength(&self, index: u32, namelength: *mut u32, inherited: Option<*mut super::super::Foundation::BOOL>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSpecifiedAttributeNameLength)(windows_core::Interface::as_raw(self), index, namelength, core::mem::transmute(inherited.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn RemoveAttribute<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveAttribute)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn SetTextValue(&self, name: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextValue)(windows_core::Interface::as_raw(self), core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetTextValue(&self, name: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTextValue)(windows_core::Interface::as_raw(self), core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetTextValueLength(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetTextValueLength)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetAttributeValue<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ID2D1SvgAttribute>,
    {
        (windows_core::Interface::vtable(self).SetAttributeValue)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn SetAttributeValue2<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAttributeValue2)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, value, valuesizeinbytes).ok()
    }
    pub unsafe fn SetAttributeValue3<P0, P1>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAttributeValue3)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, value.param().abi()).ok()
    }
    pub unsafe fn GetAttributeValue<P0, T>(&self, name: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), name.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAttributeValue2<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAttributeValue2)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, value, valuesizeinbytes).ok()
    }
    pub unsafe fn GetAttributeValue3<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: &mut [u16]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAttributeValue3)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetAttributeValueLength<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttributeValueLength)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1SvgElement {}
unsafe impl Sync for ID2D1SvgElement {}
#[repr(C)]
pub struct ID2D1SvgElement_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetTagName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetTagNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsTextContent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub HasChildren: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetFirstChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetLastChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPreviousChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertChildBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsAttributeSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL,
    pub GetSpecifiedAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSpecifiedAttributeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSpecifiedAttributeNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetTextValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetTextValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetTextValueLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub SetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttributeValue2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_POD_TYPE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAttributeValue3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_STRING_TYPE, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeValue2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_POD_TYPE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttributeValue3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_STRING_TYPE, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetAttributeValueLength: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_STRING_TYPE, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1SvgGlyphStyle, ID2D1SvgGlyphStyle_Vtbl, 0xaf671749_d241_4db8_8e41_dcc2e5c1a438);
impl core::ops::Deref for ID2D1SvgGlyphStyle {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgGlyphStyle, windows_core::IUnknown, ID2D1Resource);
impl ID2D1SvgGlyphStyle {
    pub unsafe fn SetFill<P0>(&self, brush: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).SetFill)(windows_core::Interface::as_raw(self), brush.param().abi()).ok()
    }
    pub unsafe fn GetFill(&self) -> windows_core::Result<ID2D1Brush> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFill)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn SetStroke<P0>(&self, brush: P0, strokewidth: f32, dashes: Option<&[f32]>, dashoffset: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Brush>,
    {
        (windows_core::Interface::vtable(self).SetStroke)(windows_core::Interface::as_raw(self), brush.param().abi(), strokewidth, core::mem::transmute(dashes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dashoffset).ok()
    }
    pub unsafe fn GetStrokeDashesCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetStrokeDashesCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStroke(&self, brush: Option<*mut Option<ID2D1Brush>>, strokewidth: Option<*mut f32>, dashes: Option<&mut [f32]>, dashoffset: Option<*mut f32>) {
        (windows_core::Interface::vtable(self).GetStroke)(windows_core::Interface::as_raw(self), core::mem::transmute(brush.unwrap_or(std::ptr::null_mut())), core::mem::transmute(strokewidth.unwrap_or(std::ptr::null_mut())), core::mem::transmute(dashes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(dashoffset.unwrap_or(std::ptr::null_mut())))
    }
}
unsafe impl Send for ID2D1SvgGlyphStyle {}
unsafe impl Sync for ID2D1SvgGlyphStyle {}
#[repr(C)]
pub struct ID2D1SvgGlyphStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, *const f32, u32, f32) -> windows_core::HRESULT,
    pub GetStrokeDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut f32, *mut f32, u32, *mut f32),
}
windows_core::imp::define_interface!(ID2D1SvgPaint, ID2D1SvgPaint_Vtbl, 0xd59bab0a_68a2_455b_a5dc_9eb2854e2490);
impl core::ops::Deref for ID2D1SvgPaint {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgPaint, windows_core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ID2D1SvgPaint {
    pub unsafe fn SetPaintType(&self, painttype: D2D1_SVG_PAINT_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPaintType)(windows_core::Interface::as_raw(self), painttype).ok()
    }
    pub unsafe fn GetPaintType(&self) -> D2D1_SVG_PAINT_TYPE {
        (windows_core::Interface::vtable(self).GetPaintType)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const Common::D2D1_COLOR_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetColor(&self) -> Common::D2D1_COLOR_F {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi()).ok()
    }
    pub unsafe fn GetId(&self, id: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), core::mem::transmute(id.as_ptr()), id.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetIdLength(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetIdLength)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1SvgPaint {}
unsafe impl Sync for ID2D1SvgPaint {}
#[repr(C)]
pub struct ID2D1SvgPaint_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub SetPaintType: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_SVG_PAINT_TYPE) -> windows_core::HRESULT,
    pub GetPaintType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetColor: usize,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetIdLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID2D1SvgPathData, ID2D1SvgPathData_Vtbl, 0xc095e4f4_bb98_43d6_9745_4d1b84ec9888);
impl core::ops::Deref for ID2D1SvgPathData {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgPathData, windows_core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ID2D1SvgPathData {
    pub unsafe fn RemoveSegmentDataAtEnd(&self, datacount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveSegmentDataAtEnd)(windows_core::Interface::as_raw(self), datacount).ok()
    }
    pub unsafe fn UpdateSegmentData(&self, data: &[f32], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateSegmentData)(windows_core::Interface::as_raw(self), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetSegmentData(&self, data: &mut [f32], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSegmentData)(windows_core::Interface::as_raw(self), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetSegmentDataCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetSegmentDataCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn RemoveCommandsAtEnd(&self, commandscount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveCommandsAtEnd)(windows_core::Interface::as_raw(self), commandscount).ok()
    }
    pub unsafe fn UpdateCommands(&self, commands: &[D2D1_SVG_PATH_COMMAND], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateCommands)(windows_core::Interface::as_raw(self), core::mem::transmute(commands.as_ptr()), commands.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetCommands(&self, commands: &mut [D2D1_SVG_PATH_COMMAND], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCommands)(windows_core::Interface::as_raw(self), core::mem::transmute(commands.as_ptr()), commands.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetCommandsCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetCommandsCount)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreatePathGeometry(&self, fillmode: Common::D2D1_FILL_MODE) -> windows_core::Result<ID2D1PathGeometry1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), fillmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for ID2D1SvgPathData {}
unsafe impl Sync for ID2D1SvgPathData {}
#[repr(C)]
pub struct ID2D1SvgPathData_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemoveSegmentDataAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateSegmentData: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, u32) -> windows_core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub RemoveCommandsAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SVG_PATH_COMMAND, u32, u32) -> windows_core::HRESULT,
    pub GetCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_SVG_PATH_COMMAND, u32, u32) -> windows_core::HRESULT,
    pub GetCommandsCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D1_FILL_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    CreatePathGeometry: usize,
}
windows_core::imp::define_interface!(ID2D1SvgPointCollection, ID2D1SvgPointCollection_Vtbl, 0x9dbe4c0d_3572_4dd9_9825_5530813bb712);
impl core::ops::Deref for ID2D1SvgPointCollection {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgPointCollection, windows_core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ID2D1SvgPointCollection {
    pub unsafe fn RemovePointsAtEnd(&self, pointscount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemovePointsAtEnd)(windows_core::Interface::as_raw(self), pointscount).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn UpdatePoints(&self, points: &[Common::D2D_POINT_2F], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdatePoints)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap(), startindex).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPoints(&self, points: &mut [Common::D2D_POINT_2F], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPoints)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetPointsCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetPointsCount)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1SvgPointCollection {}
unsafe impl Sync for ID2D1SvgPointCollection {}
#[repr(C)]
pub struct ID2D1SvgPointCollection_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemovePointsAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub UpdatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_POINT_2F, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    UpdatePoints: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_POINT_2F, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPoints: usize,
    pub GetPointsCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID2D1SvgStrokeDashArray, ID2D1SvgStrokeDashArray_Vtbl, 0xf1c0ca52_92a3_4f00_b4ce_f35691efd9d9);
impl core::ops::Deref for ID2D1SvgStrokeDashArray {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SvgStrokeDashArray, windows_core::IUnknown, ID2D1Resource, ID2D1SvgAttribute);
impl ID2D1SvgStrokeDashArray {
    pub unsafe fn RemoveDashesAtEnd(&self, dashescount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveDashesAtEnd)(windows_core::Interface::as_raw(self), dashescount).ok()
    }
    pub unsafe fn UpdateDashes(&self, dashes: &[D2D1_SVG_LENGTH], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateDashes)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn UpdateDashes2(&self, dashes: &[f32], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateDashes2)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [D2D1_SVG_LENGTH], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDashes)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetDashes2(&self, dashes: &mut [f32], startindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDashes2)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex).ok()
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetDashesCount)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1SvgStrokeDashArray {}
unsafe impl Sync for ID2D1SvgStrokeDashArray {}
#[repr(C)]
pub struct ID2D1SvgStrokeDashArray_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemoveDashesAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SVG_LENGTH, u32, u32) -> windows_core::HRESULT,
    pub UpdateDashes2: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, u32) -> windows_core::HRESULT,
    pub GetDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_SVG_LENGTH, u32, u32) -> windows_core::HRESULT,
    pub GetDashes2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID2D1TessellationSink, ID2D1TessellationSink_Vtbl, 0x2cd906c1_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1TessellationSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1TessellationSink, windows_core::IUnknown);
impl ID2D1TessellationSink {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddTriangles(&self, triangles: &[D2D1_TRIANGLE]) {
        (windows_core::Interface::vtable(self).AddTriangles)(windows_core::Interface::as_raw(self), core::mem::transmute(triangles.as_ptr()), triangles.len().try_into().unwrap())
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1TessellationSink {}
unsafe impl Sync for ID2D1TessellationSink {}
#[repr(C)]
pub struct ID2D1TessellationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub AddTriangles: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_TRIANGLE, u32),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    AddTriangles: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1Transform, ID2D1Transform_Vtbl, 0xef1a287d_342a_4f76_8fdb_da0d6ea9f92b);
impl core::ops::Deref for ID2D1Transform {
    type Target = ID2D1TransformNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Transform, windows_core::IUnknown, ID2D1TransformNode);
impl ID2D1Transform {
    pub unsafe fn MapOutputRectToInputRects(&self, outputrect: *const super::super::Foundation::RECT, inputrects: &mut [super::super::Foundation::RECT]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MapOutputRectToInputRects)(windows_core::Interface::as_raw(self), outputrect, core::mem::transmute(inputrects.as_ptr()), inputrects.len().try_into().unwrap()).ok()
    }
    pub unsafe fn MapInputRectsToOutputRect(&self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MapInputRectsToOutputRect)(windows_core::Interface::as_raw(self), inputrects, inputopaquesubrects, inputrectcount, outputrect, outputopaquesubrect).ok()
    }
    pub unsafe fn MapInvalidRect(&self, inputindex: u32, invalidinputrect: super::super::Foundation::RECT) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MapInvalidRect)(windows_core::Interface::as_raw(self), inputindex, core::mem::transmute(invalidinputrect), &mut result__).map(|| result__)
    }
}
unsafe impl Send for ID2D1Transform {}
unsafe impl Sync for ID2D1Transform {}
#[repr(C)]
pub struct ID2D1Transform_Vtbl {
    pub base__: ID2D1TransformNode_Vtbl,
    pub MapOutputRectToInputRects: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *mut super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
    pub MapInputRectsToOutputRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, u32, *mut super::super::Foundation::RECT, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub MapInvalidRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::RECT, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1TransformGraph, ID2D1TransformGraph_Vtbl, 0x13d29038_c3e6_4034_9081_13b53a417992);
impl core::ops::Deref for ID2D1TransformGraph {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1TransformGraph, windows_core::IUnknown);
impl ID2D1TransformGraph {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetSingleTransformNode<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformNode>,
    {
        (windows_core::Interface::vtable(self).SetSingleTransformNode)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    pub unsafe fn AddNode<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformNode>,
    {
        (windows_core::Interface::vtable(self).AddNode)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    pub unsafe fn RemoveNode<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformNode>,
    {
        (windows_core::Interface::vtable(self).RemoveNode)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    pub unsafe fn SetOutputNode<P0>(&self, node: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformNode>,
    {
        (windows_core::Interface::vtable(self).SetOutputNode)(windows_core::Interface::as_raw(self), node.param().abi()).ok()
    }
    pub unsafe fn ConnectNode<P0, P1>(&self, fromnode: P0, tonode: P1, tonodeinputindex: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformNode>,
        P1: windows_core::Param<ID2D1TransformNode>,
    {
        (windows_core::Interface::vtable(self).ConnectNode)(windows_core::Interface::as_raw(self), fromnode.param().abi(), tonode.param().abi(), tonodeinputindex).ok()
    }
    pub unsafe fn ConnectToEffectInput<P0>(&self, toeffectinputindex: u32, node: P0, tonodeinputindex: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1TransformNode>,
    {
        (windows_core::Interface::vtable(self).ConnectToEffectInput)(windows_core::Interface::as_raw(self), toeffectinputindex, node.param().abi(), tonodeinputindex).ok()
    }
    pub unsafe fn Clear(&self) {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetPassthroughGraph(&self, effectinputindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPassthroughGraph)(windows_core::Interface::as_raw(self), effectinputindex).ok()
    }
}
unsafe impl Send for ID2D1TransformGraph {}
unsafe impl Sync for ID2D1TransformGraph {}
#[repr(C)]
pub struct ID2D1TransformGraph_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub SetSingleTransformNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ConnectToEffectInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetPassthroughGraph: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ID2D1TransformNode, ID2D1TransformNode_Vtbl, 0xb2efe1e7_729f_4102_949f_505fa21bf666);
impl core::ops::Deref for ID2D1TransformNode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1TransformNode, windows_core::IUnknown);
impl ID2D1TransformNode {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for ID2D1TransformNode {}
unsafe impl Sync for ID2D1TransformNode {}
#[repr(C)]
pub struct ID2D1TransformNode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(ID2D1TransformedGeometry, ID2D1TransformedGeometry_Vtbl, 0x2cd906bb_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1TransformedGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1TransformedGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1TransformedGeometry {
    pub unsafe fn GetSourceGeometry(&self) -> windows_core::Result<ID2D1Geometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceGeometry)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform)
    }
}
unsafe impl Send for ID2D1TransformedGeometry {}
unsafe impl Sync for ID2D1TransformedGeometry {}
#[repr(C)]
pub struct ID2D1TransformedGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub GetSourceGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2),
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransform: usize,
}
windows_core::imp::define_interface!(ID2D1TransformedImageSource, ID2D1TransformedImageSource_Vtbl, 0x7f1f79e5_2796_416c_8f55_700f911445e5);
impl core::ops::Deref for ID2D1TransformedImageSource {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1TransformedImageSource, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
impl ID2D1TransformedImageSource {
    pub unsafe fn GetSource(&self) -> windows_core::Result<ID2D1ImageSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__);
        windows_core::Type::from_abi(result__)
    }
    pub unsafe fn GetProperties(&self, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), properties)
    }
}
unsafe impl Send for ID2D1TransformedImageSource {}
unsafe impl Sync for ID2D1TransformedImageSource {}
#[repr(C)]
pub struct ID2D1TransformedImageSource_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES),
}
windows_core::imp::define_interface!(ID2D1VertexBuffer, ID2D1VertexBuffer_Vtbl, 0x9b8b1336_00a5_4668_92b7_ced5d8bf9b7b);
impl core::ops::Deref for ID2D1VertexBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1VertexBuffer, windows_core::IUnknown);
impl ID2D1VertexBuffer {
    pub unsafe fn Map(&self, data: *mut *mut u8, buffersize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), data, buffersize).ok()
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1VertexBuffer {}
unsafe impl Sync for ID2D1VertexBuffer {}
#[repr(C)]
pub struct ID2D1VertexBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, u32) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CLSID_D2D12DAffineTransform: windows_core::GUID = windows_core::GUID::from_u128(0x6aa97485_6354_4cfc_908c_e4a74f62c96c);
pub const CLSID_D2D13DPerspectiveTransform: windows_core::GUID = windows_core::GUID::from_u128(0xc2844d0b_3d86_46e7_85ba_526c9240f3fb);
pub const CLSID_D2D13DTransform: windows_core::GUID = windows_core::GUID::from_u128(0xe8467b04_ec61_4b8a_b5de_d4d73debea5a);
pub const CLSID_D2D1AlphaMask: windows_core::GUID = windows_core::GUID::from_u128(0xc80ecff0_3fd5_4f05_8328_c5d1724b4f0a);
pub const CLSID_D2D1ArithmeticComposite: windows_core::GUID = windows_core::GUID::from_u128(0xfc151437_049a_4784_a24a_f1c4daf20987);
pub const CLSID_D2D1Atlas: windows_core::GUID = windows_core::GUID::from_u128(0x913e2be4_fdcf_4fe2_a5f0_2454f14ff408);
pub const CLSID_D2D1BitmapSource: windows_core::GUID = windows_core::GUID::from_u128(0x5fb6c24d_c6dd_4231_9404_50f4d5c3252d);
pub const CLSID_D2D1Blend: windows_core::GUID = windows_core::GUID::from_u128(0x81c5b77b_13f8_4cdd_ad20_c890547ac65d);
pub const CLSID_D2D1Border: windows_core::GUID = windows_core::GUID::from_u128(0x2a2d49c0_4acf_43c7_8c6a_7c4a27874d27);
pub const CLSID_D2D1Brightness: windows_core::GUID = windows_core::GUID::from_u128(0x8cea8d1e_77b0_4986_b3b9_2f0c0eae7887);
pub const CLSID_D2D1ChromaKey: windows_core::GUID = windows_core::GUID::from_u128(0x74c01f5b_2a0d_408c_88e2_c7a3c7197742);
pub const CLSID_D2D1ColorManagement: windows_core::GUID = windows_core::GUID::from_u128(0x1a28524c_fdd6_4aa4_ae8f_837eb8267b37);
pub const CLSID_D2D1ColorMatrix: windows_core::GUID = windows_core::GUID::from_u128(0x921f03d6_641c_47df_852d_b4bb6153ae11);
pub const CLSID_D2D1Composite: windows_core::GUID = windows_core::GUID::from_u128(0x48fc9f51_f6ac_48f1_8b58_3b28ac46f76d);
pub const CLSID_D2D1Contrast: windows_core::GUID = windows_core::GUID::from_u128(0xb648a78a_0ed5_4f80_a94a_8e825aca6b77);
pub const CLSID_D2D1ConvolveMatrix: windows_core::GUID = windows_core::GUID::from_u128(0x407f8c08_5533_4331_a341_23cc3877843e);
pub const CLSID_D2D1Crop: windows_core::GUID = windows_core::GUID::from_u128(0xe23f7110_0e9a_4324_af47_6a2c0c46f35b);
pub const CLSID_D2D1CrossFade: windows_core::GUID = windows_core::GUID::from_u128(0x12f575e8_4db1_485f_9a84_03a07dd3829f);
pub const CLSID_D2D1DirectionalBlur: windows_core::GUID = windows_core::GUID::from_u128(0x174319a6_58e9_49b2_bb63_caf2c811a3db);
pub const CLSID_D2D1DiscreteTransfer: windows_core::GUID = windows_core::GUID::from_u128(0x90866fcd_488e_454b_af06_e5041b66c36c);
pub const CLSID_D2D1DisplacementMap: windows_core::GUID = windows_core::GUID::from_u128(0xedc48364_0417_4111_9450_43845fa9f890);
pub const CLSID_D2D1DistantDiffuse: windows_core::GUID = windows_core::GUID::from_u128(0x3e7efd62_a32d_46d4_a83c_5278889ac954);
pub const CLSID_D2D1DistantSpecular: windows_core::GUID = windows_core::GUID::from_u128(0x428c1ee5_77b8_4450_8ab5_72219c21abda);
pub const CLSID_D2D1DpiCompensation: windows_core::GUID = windows_core::GUID::from_u128(0x6c26c5c7_34e0_46fc_9cfd_e5823706e228);
pub const CLSID_D2D1EdgeDetection: windows_core::GUID = windows_core::GUID::from_u128(0xeff583ca_cb07_4aa9_ac5d_2cc44c76460f);
pub const CLSID_D2D1Emboss: windows_core::GUID = windows_core::GUID::from_u128(0xb1c5eb2b_0348_43f0_8107_4957cacba2ae);
pub const CLSID_D2D1Exposure: windows_core::GUID = windows_core::GUID::from_u128(0xb56c8cfa_f634_41ee_bee0_ffa617106004);
pub const CLSID_D2D1Flood: windows_core::GUID = windows_core::GUID::from_u128(0x61c23c20_ae69_4d8e_94cf_50078df638f2);
pub const CLSID_D2D1GammaTransfer: windows_core::GUID = windows_core::GUID::from_u128(0x409444c4_c419_41a0_b0c1_8cd0c0a18e42);
pub const CLSID_D2D1GaussianBlur: windows_core::GUID = windows_core::GUID::from_u128(0x1feb6d69_2fe6_4ac9_8c58_1d7f93e7a6a5);
pub const CLSID_D2D1Grayscale: windows_core::GUID = windows_core::GUID::from_u128(0x36dde0eb_3725_42e0_836d_52fb20aee644);
pub const CLSID_D2D1HdrToneMap: windows_core::GUID = windows_core::GUID::from_u128(0x7b0b748d_4610_4486_a90c_999d9a2e2b11);
pub const CLSID_D2D1HighlightsShadows: windows_core::GUID = windows_core::GUID::from_u128(0xcadc8384_323f_4c7e_a361_2e2b24df6ee4);
pub const CLSID_D2D1Histogram: windows_core::GUID = windows_core::GUID::from_u128(0x881db7d0_f7ee_4d4d_a6d2_4697acc66ee8);
pub const CLSID_D2D1HueRotation: windows_core::GUID = windows_core::GUID::from_u128(0x0f4458ec_4b32_491b_9e85_bd73f44d3eb6);
pub const CLSID_D2D1HueToRgb: windows_core::GUID = windows_core::GUID::from_u128(0x7b78a6bd_0141_4def_8a52_6356ee0cbdd5);
pub const CLSID_D2D1Invert: windows_core::GUID = windows_core::GUID::from_u128(0xe0c3784d_cb39_4e84_b6fd_6b72f0810263);
pub const CLSID_D2D1LinearTransfer: windows_core::GUID = windows_core::GUID::from_u128(0xad47c8fd_63ef_4acc_9b51_67979c036c06);
pub const CLSID_D2D1LookupTable3D: windows_core::GUID = windows_core::GUID::from_u128(0x349e0eda_0088_4a79_9ca3_c7e300202020);
pub const CLSID_D2D1LuminanceToAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x41251ab7_0beb_46f8_9da7_59e93fcce5de);
pub const CLSID_D2D1Morphology: windows_core::GUID = windows_core::GUID::from_u128(0xeae6c40d_626a_4c2d_bfcb_391001abe202);
pub const CLSID_D2D1Opacity: windows_core::GUID = windows_core::GUID::from_u128(0x811d79a4_de28_4454_8094_c64685f8bd4c);
pub const CLSID_D2D1OpacityMetadata: windows_core::GUID = windows_core::GUID::from_u128(0x6c53006a_4450_4199_aa5b_ad1656fece5e);
pub const CLSID_D2D1PointDiffuse: windows_core::GUID = windows_core::GUID::from_u128(0xb9e303c3_c08c_4f91_8b7b_38656bc48c20);
pub const CLSID_D2D1PointSpecular: windows_core::GUID = windows_core::GUID::from_u128(0x09c3ca26_3ae2_4f09_9ebc_ed3865d53f22);
pub const CLSID_D2D1Posterize: windows_core::GUID = windows_core::GUID::from_u128(0x2188945e_33a3_4366_b7bc_086bd02d0884);
pub const CLSID_D2D1Premultiply: windows_core::GUID = windows_core::GUID::from_u128(0x06eab419_deed_4018_80d2_3e1d471adeb2);
pub const CLSID_D2D1RgbToHue: windows_core::GUID = windows_core::GUID::from_u128(0x23f3e5ec_91e8_4d3d_ad0a_afadc1004aa1);
pub const CLSID_D2D1Saturation: windows_core::GUID = windows_core::GUID::from_u128(0x5cb2d9cf_327d_459f_a0ce_40c0b2086bf7);
pub const CLSID_D2D1Scale: windows_core::GUID = windows_core::GUID::from_u128(0x9daf9369_3846_4d0e_a44e_0c607934a5d7);
pub const CLSID_D2D1Sepia: windows_core::GUID = windows_core::GUID::from_u128(0x3a1af410_5f1d_4dbe_84df_915da79b7153);
pub const CLSID_D2D1Shadow: windows_core::GUID = windows_core::GUID::from_u128(0xc67ea361_1863_4e69_89db_695d3e9a5b6b);
pub const CLSID_D2D1Sharpen: windows_core::GUID = windows_core::GUID::from_u128(0xc9b887cb_c5ff_4dc5_9779_273dcf417c7d);
pub const CLSID_D2D1SpotDiffuse: windows_core::GUID = windows_core::GUID::from_u128(0x818a1105_7932_44f4_aa86_08ae7b2f2c93);
pub const CLSID_D2D1SpotSpecular: windows_core::GUID = windows_core::GUID::from_u128(0xedae421e_7654_4a37_9db8_71acc1beb3c1);
pub const CLSID_D2D1Straighten: windows_core::GUID = windows_core::GUID::from_u128(0x4da47b12_79a3_4fb0_8237_bbc3b2a4de08);
pub const CLSID_D2D1TableTransfer: windows_core::GUID = windows_core::GUID::from_u128(0x5bf818c3_5e43_48cb_b631_868396d6a1d4);
pub const CLSID_D2D1TemperatureTint: windows_core::GUID = windows_core::GUID::from_u128(0x89176087_8af9_4a08_aeb1_895f38db1766);
pub const CLSID_D2D1Tile: windows_core::GUID = windows_core::GUID::from_u128(0xb0784138_3b76_4bc5_b13b_0fa2ad02659f);
pub const CLSID_D2D1Tint: windows_core::GUID = windows_core::GUID::from_u128(0x36312b17_f7dd_4014_915d_ffca768cf211);
pub const CLSID_D2D1Turbulence: windows_core::GUID = windows_core::GUID::from_u128(0xcf2bb6ae_889a_4ad7_ba29_a2fd732c9fc9);
pub const CLSID_D2D1UnPremultiply: windows_core::GUID = windows_core::GUID::from_u128(0xfb9ac489_ad8d_41ed_9999_bb6347d110f7);
pub const CLSID_D2D1Vignette: windows_core::GUID = windows_core::GUID::from_u128(0xc00c40be_5e67_4ca3_95b4_f4b02c115135);
pub const CLSID_D2D1WhiteLevelAdjustment: windows_core::GUID = windows_core::GUID::from_u128(0x44a1cadb_6cdd_4818_8ff4_26c1cfe95bdb);
pub const CLSID_D2D1YCbCr: windows_core::GUID = windows_core::GUID::from_u128(0x99503cc1_66c7_45c9_a875_8ad8a7914401);
pub const D2D1_2DAFFINETRANSFORM_PROP_BORDER_MODE: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(1i32);
pub const D2D1_2DAFFINETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(0i32);
pub const D2D1_2DAFFINETRANSFORM_PROP_SHARPNESS: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(3i32);
pub const D2D1_2DAFFINETRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_2DAFFINETRANSFORM_PROP = D2D1_2DAFFINETRANSFORM_PROP(2i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(4i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(2i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(1i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(3i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(0i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_BORDER_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(1i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_DEPTH: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(2i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_GLOBAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(5i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(0i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_LOCAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(4i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_PERSPECTIVE_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(3i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(7i32);
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = D2D1_3DPERSPECTIVETRANSFORM_PROP(6i32);
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(4i32);
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(2i32);
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(1i32);
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(3i32);
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = D2D1_3DTRANSFORM_INTERPOLATION_MODE(0i32);
pub const D2D1_3DTRANSFORM_PROP_BORDER_MODE: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(1i32);
pub const D2D1_3DTRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(0i32);
pub const D2D1_3DTRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_3DTRANSFORM_PROP = D2D1_3DTRANSFORM_PROP(2i32);
pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE = D2D1_ANTIALIAS_MODE(1i32);
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = D2D1_ANTIALIAS_MODE(0i32);
pub const D2D1_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = D2D1_ARC_SIZE(1i32);
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = D2D1_ARC_SIZE(0i32);
pub const D2D1_ARITHMETICCOMPOSITE_PROP_CLAMP_OUTPUT: D2D1_ARITHMETICCOMPOSITE_PROP = D2D1_ARITHMETICCOMPOSITE_PROP(1i32);
pub const D2D1_ARITHMETICCOMPOSITE_PROP_COEFFICIENTS: D2D1_ARITHMETICCOMPOSITE_PROP = D2D1_ARITHMETICCOMPOSITE_PROP(0i32);
pub const D2D1_ATLAS_PROP_INPUT_PADDING_RECT: D2D1_ATLAS_PROP = D2D1_ATLAS_PROP(1i32);
pub const D2D1_ATLAS_PROP_INPUT_RECT: D2D1_ATLAS_PROP = D2D1_ATLAS_PROP(0i32);
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_PREMULTIPLIED: D2D1_BITMAPSOURCE_ALPHA_MODE = D2D1_BITMAPSOURCE_ALPHA_MODE(1i32);
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_STRAIGHT: D2D1_BITMAPSOURCE_ALPHA_MODE = D2D1_BITMAPSOURCE_ALPHA_MODE(2i32);
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_CUBIC: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(2i32);
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FANT: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(6i32);
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(1i32);
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_MIPMAP_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(7i32);
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = D2D1_BITMAPSOURCE_INTERPOLATION_MODE(0i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_DEFAULT: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(1i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(2i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(3i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(4i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(8i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(5i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(6i32);
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = D2D1_BITMAPSOURCE_ORIENTATION(7i32);
pub const D2D1_BITMAPSOURCE_PROP_ALPHA_MODE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(4i32);
pub const D2D1_BITMAPSOURCE_PROP_ENABLE_DPI_CORRECTION: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(3i32);
pub const D2D1_BITMAPSOURCE_PROP_INTERPOLATION_MODE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(2i32);
pub const D2D1_BITMAPSOURCE_PROP_ORIENTATION: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(5i32);
pub const D2D1_BITMAPSOURCE_PROP_SCALE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(1i32);
pub const D2D1_BITMAPSOURCE_PROP_WIC_BITMAP_SOURCE: D2D1_BITMAPSOURCE_PROP = D2D1_BITMAPSOURCE_PROP(0i32);
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(1i32);
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(0i32);
pub const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(2i32);
pub const D2D1_BITMAP_OPTIONS_CPU_READ: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(4i32);
pub const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(8i32);
pub const D2D1_BITMAP_OPTIONS_NONE: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(0i32);
pub const D2D1_BITMAP_OPTIONS_TARGET: D2D1_BITMAP_OPTIONS = D2D1_BITMAP_OPTIONS(1i32);
pub const D2D1_BLEND_BLEND_FACTOR: D2D1_BLEND = D2D1_BLEND(14i32);
pub const D2D1_BLEND_DEST_ALPHA: D2D1_BLEND = D2D1_BLEND(7i32);
pub const D2D1_BLEND_DEST_COLOR: D2D1_BLEND = D2D1_BLEND(9i32);
pub const D2D1_BLEND_INV_BLEND_FACTOR: D2D1_BLEND = D2D1_BLEND(15i32);
pub const D2D1_BLEND_INV_DEST_ALPHA: D2D1_BLEND = D2D1_BLEND(8i32);
pub const D2D1_BLEND_INV_DEST_COLOR: D2D1_BLEND = D2D1_BLEND(10i32);
pub const D2D1_BLEND_INV_SRC_ALPHA: D2D1_BLEND = D2D1_BLEND(6i32);
pub const D2D1_BLEND_INV_SRC_COLOR: D2D1_BLEND = D2D1_BLEND(4i32);
pub const D2D1_BLEND_ONE: D2D1_BLEND = D2D1_BLEND(2i32);
pub const D2D1_BLEND_OPERATION_ADD: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(1i32);
pub const D2D1_BLEND_OPERATION_MAX: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(5i32);
pub const D2D1_BLEND_OPERATION_MIN: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(4i32);
pub const D2D1_BLEND_OPERATION_REV_SUBTRACT: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(3i32);
pub const D2D1_BLEND_OPERATION_SUBTRACT: D2D1_BLEND_OPERATION = D2D1_BLEND_OPERATION(2i32);
pub const D2D1_BLEND_PROP_MODE: D2D1_BLEND_PROP = D2D1_BLEND_PROP(0i32);
pub const D2D1_BLEND_SRC_ALPHA: D2D1_BLEND = D2D1_BLEND(5i32);
pub const D2D1_BLEND_SRC_ALPHA_SAT: D2D1_BLEND = D2D1_BLEND(11i32);
pub const D2D1_BLEND_SRC_COLOR: D2D1_BLEND = D2D1_BLEND(3i32);
pub const D2D1_BLEND_ZERO: D2D1_BLEND = D2D1_BLEND(1i32);
pub const D2D1_BORDER_EDGE_MODE_CLAMP: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(0i32);
pub const D2D1_BORDER_EDGE_MODE_MIRROR: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(2i32);
pub const D2D1_BORDER_EDGE_MODE_WRAP: D2D1_BORDER_EDGE_MODE = D2D1_BORDER_EDGE_MODE(1i32);
pub const D2D1_BORDER_PROP_EDGE_MODE_X: D2D1_BORDER_PROP = D2D1_BORDER_PROP(0i32);
pub const D2D1_BORDER_PROP_EDGE_MODE_Y: D2D1_BORDER_PROP = D2D1_BORDER_PROP(1i32);
pub const D2D1_BRIGHTNESS_PROP_BLACK_POINT: D2D1_BRIGHTNESS_PROP = D2D1_BRIGHTNESS_PROP(1i32);
pub const D2D1_BRIGHTNESS_PROP_WHITE_POINT: D2D1_BRIGHTNESS_PROP = D2D1_BRIGHTNESS_PROP(0i32);
pub const D2D1_BUFFER_PRECISION_16BPC_FLOAT: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(4i32);
pub const D2D1_BUFFER_PRECISION_16BPC_UNORM: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(3i32);
pub const D2D1_BUFFER_PRECISION_32BPC_FLOAT: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(5i32);
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(1i32);
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(2i32);
pub const D2D1_BUFFER_PRECISION_UNKNOWN: D2D1_BUFFER_PRECISION = D2D1_BUFFER_PRECISION(0i32);
pub const D2D1_CAP_STYLE_FLAT: D2D1_CAP_STYLE = D2D1_CAP_STYLE(0i32);
pub const D2D1_CAP_STYLE_ROUND: D2D1_CAP_STYLE = D2D1_CAP_STYLE(2i32);
pub const D2D1_CAP_STYLE_SQUARE: D2D1_CAP_STYLE = D2D1_CAP_STYLE(1i32);
pub const D2D1_CAP_STYLE_TRIANGLE: D2D1_CAP_STYLE = D2D1_CAP_STYLE(3i32);
pub const D2D1_CHANGE_TYPE_CONTEXT: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(2i32);
pub const D2D1_CHANGE_TYPE_GRAPH: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(3i32);
pub const D2D1_CHANGE_TYPE_NONE: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(0i32);
pub const D2D1_CHANGE_TYPE_PROPERTIES: D2D1_CHANGE_TYPE = D2D1_CHANGE_TYPE(1i32);
pub const D2D1_CHANNEL_DEPTH_1: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(1i32);
pub const D2D1_CHANNEL_DEPTH_4: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(4i32);
pub const D2D1_CHANNEL_DEPTH_DEFAULT: D2D1_CHANNEL_DEPTH = D2D1_CHANNEL_DEPTH(0i32);
pub const D2D1_CHANNEL_SELECTOR_A: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(3i32);
pub const D2D1_CHANNEL_SELECTOR_B: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(2i32);
pub const D2D1_CHANNEL_SELECTOR_G: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(1i32);
pub const D2D1_CHANNEL_SELECTOR_R: D2D1_CHANNEL_SELECTOR = D2D1_CHANNEL_SELECTOR(0i32);
pub const D2D1_CHROMAKEY_PROP_COLOR: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(0i32);
pub const D2D1_CHROMAKEY_PROP_FEATHER: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(3i32);
pub const D2D1_CHROMAKEY_PROP_INVERT_ALPHA: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(2i32);
pub const D2D1_CHROMAKEY_PROP_TOLERANCE: D2D1_CHROMAKEY_PROP = D2D1_CHROMAKEY_PROP(1i32);
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMANAGEMENT_ALPHA_MODE = D2D1_COLORMANAGEMENT_ALPHA_MODE(1i32);
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_STRAIGHT: D2D1_COLORMANAGEMENT_ALPHA_MODE = D2D1_COLORMANAGEMENT_ALPHA_MODE(2i32);
pub const D2D1_COLORMANAGEMENT_PROP_ALPHA_MODE: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(4i32);
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(2i32);
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(3i32);
pub const D2D1_COLORMANAGEMENT_PROP_QUALITY: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(5i32);
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(0i32);
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = D2D1_COLORMANAGEMENT_PROP(1i32);
pub const D2D1_COLORMANAGEMENT_QUALITY_BEST: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(2i32);
pub const D2D1_COLORMANAGEMENT_QUALITY_NORMAL: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(1i32);
pub const D2D1_COLORMANAGEMENT_QUALITY_PROOF: D2D1_COLORMANAGEMENT_QUALITY = D2D1_COLORMANAGEMENT_QUALITY(0i32);
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(3i32);
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_PERCEPTUAL: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(0i32);
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_RELATIVE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(1i32);
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_SATURATION: D2D1_COLORMANAGEMENT_RENDERING_INTENT = D2D1_COLORMANAGEMENT_RENDERING_INTENT(2i32);
pub const D2D1_COLORMATRIX_PROP_ALPHA_MODE: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(1i32);
pub const D2D1_COLORMATRIX_PROP_CLAMP_OUTPUT: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(2i32);
pub const D2D1_COLORMATRIX_PROP_COLOR_MATRIX: D2D1_COLORMATRIX_PROP = D2D1_COLORMATRIX_PROP(0i32);
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(0i32);
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(1i32);
pub const D2D1_COLOR_CONTEXT_TYPE_DXGI: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(2i32);
pub const D2D1_COLOR_CONTEXT_TYPE_ICC: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(0i32);
pub const D2D1_COLOR_CONTEXT_TYPE_SIMPLE: D2D1_COLOR_CONTEXT_TYPE = D2D1_COLOR_CONTEXT_TYPE(1i32);
pub const D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED: D2D1_COLOR_INTERPOLATION_MODE = D2D1_COLOR_INTERPOLATION_MODE(1i32);
pub const D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT: D2D1_COLOR_INTERPOLATION_MODE = D2D1_COLOR_INTERPOLATION_MODE(0i32);
pub const D2D1_COLOR_SPACE_CUSTOM: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(0i32);
pub const D2D1_COLOR_SPACE_SCRGB: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(2i32);
pub const D2D1_COLOR_SPACE_SRGB: D2D1_COLOR_SPACE = D2D1_COLOR_SPACE(1i32);
pub const D2D1_COMBINE_MODE_EXCLUDE: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(3i32);
pub const D2D1_COMBINE_MODE_INTERSECT: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(1i32);
pub const D2D1_COMBINE_MODE_UNION: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(0i32);
pub const D2D1_COMBINE_MODE_XOR: D2D1_COMBINE_MODE = D2D1_COMBINE_MODE(2i32);
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(1i32);
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(0i32);
pub const D2D1_COMPOSITE_PROP_MODE: D2D1_COMPOSITE_PROP = D2D1_COMPOSITE_PROP(0i32);
pub const D2D1_CONTRAST_PROP_CLAMP_INPUT: D2D1_CONTRAST_PROP = D2D1_CONTRAST_PROP(1i32);
pub const D2D1_CONTRAST_PROP_CONTRAST: D2D1_CONTRAST_PROP = D2D1_CONTRAST_PROP(0i32);
pub const D2D1_CONVOLVEMATRIX_PROP_BIAS: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(6i32);
pub const D2D1_CONVOLVEMATRIX_PROP_BORDER_MODE: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(9i32);
pub const D2D1_CONVOLVEMATRIX_PROP_CLAMP_OUTPUT: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(10i32);
pub const D2D1_CONVOLVEMATRIX_PROP_DIVISOR: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(5i32);
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_MATRIX: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(4i32);
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_OFFSET: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(7i32);
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_X: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(2i32);
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_Y: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(3i32);
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_UNIT_LENGTH: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(0i32);
pub const D2D1_CONVOLVEMATRIX_PROP_PRESERVE_ALPHA: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(8i32);
pub const D2D1_CONVOLVEMATRIX_PROP_SCALE_MODE: D2D1_CONVOLVEMATRIX_PROP = D2D1_CONVOLVEMATRIX_PROP(1i32);
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_ANISOTROPIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(4i32);
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(2i32);
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(5i32);
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(1i32);
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(3i32);
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_CONVOLVEMATRIX_SCALE_MODE = D2D1_CONVOLVEMATRIX_SCALE_MODE(0i32);
pub const D2D1_CROP_PROP_BORDER_MODE: D2D1_CROP_PROP = D2D1_CROP_PROP(1i32);
pub const D2D1_CROP_PROP_RECT: D2D1_CROP_PROP = D2D1_CROP_PROP(0i32);
pub const D2D1_CROSSFADE_PROP_WEIGHT: D2D1_CROSSFADE_PROP = D2D1_CROSSFADE_PROP(0i32);
pub const D2D1_DASH_STYLE_CUSTOM: D2D1_DASH_STYLE = D2D1_DASH_STYLE(5i32);
pub const D2D1_DASH_STYLE_DASH: D2D1_DASH_STYLE = D2D1_DASH_STYLE(1i32);
pub const D2D1_DASH_STYLE_DASH_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(3i32);
pub const D2D1_DASH_STYLE_DASH_DOT_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(4i32);
pub const D2D1_DASH_STYLE_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(2i32);
pub const D2D1_DASH_STYLE_SOLID: D2D1_DASH_STYLE = D2D1_DASH_STYLE(0i32);
pub const D2D1_DC_INITIALIZE_MODE_CLEAR: D2D1_DC_INITIALIZE_MODE = D2D1_DC_INITIALIZE_MODE(1i32);
pub const D2D1_DC_INITIALIZE_MODE_COPY: D2D1_DC_INITIALIZE_MODE = D2D1_DC_INITIALIZE_MODE(0i32);
pub const D2D1_DEBUG_LEVEL_ERROR: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(1i32);
pub const D2D1_DEBUG_LEVEL_INFORMATION: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(3i32);
pub const D2D1_DEBUG_LEVEL_NONE: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(0i32);
pub const D2D1_DEBUG_LEVEL_WARNING: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(2i32);
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25f32;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS: D2D1_DEVICE_CONTEXT_OPTIONS = D2D1_DEVICE_CONTEXT_OPTIONS(1i32);
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = D2D1_DEVICE_CONTEXT_OPTIONS(0i32);
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_BALANCED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(1i32);
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_QUALITY: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(2i32);
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_SPEED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = D2D1_DIRECTIONALBLUR_OPTIMIZATION(0i32);
pub const D2D1_DIRECTIONALBLUR_PROP_ANGLE: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(1i32);
pub const D2D1_DIRECTIONALBLUR_PROP_BORDER_MODE: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(3i32);
pub const D2D1_DIRECTIONALBLUR_PROP_OPTIMIZATION: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(2i32);
pub const D2D1_DIRECTIONALBLUR_PROP_STANDARD_DEVIATION: D2D1_DIRECTIONALBLUR_PROP = D2D1_DIRECTIONALBLUR_PROP(0i32);
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(7i32);
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(6i32);
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(5i32);
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(4i32);
pub const D2D1_DISCRETETRANSFER_PROP_CLAMP_OUTPUT: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(8i32);
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(3i32);
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(2i32);
pub const D2D1_DISCRETETRANSFER_PROP_RED_DISABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(1i32);
pub const D2D1_DISCRETETRANSFER_PROP_RED_TABLE: D2D1_DISCRETETRANSFER_PROP = D2D1_DISCRETETRANSFER_PROP(0i32);
pub const D2D1_DISPLACEMENTMAP_PROP_SCALE: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(0i32);
pub const D2D1_DISPLACEMENTMAP_PROP_X_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(1i32);
pub const D2D1_DISPLACEMENTMAP_PROP_Y_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = D2D1_DISPLACEMENTMAP_PROP(2i32);
pub const D2D1_DISTANTDIFFUSE_PROP_AZIMUTH: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(0i32);
pub const D2D1_DISTANTDIFFUSE_PROP_COLOR: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(4i32);
pub const D2D1_DISTANTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(2i32);
pub const D2D1_DISTANTDIFFUSE_PROP_ELEVATION: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(1i32);
pub const D2D1_DISTANTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(5i32);
pub const D2D1_DISTANTDIFFUSE_PROP_SCALE_MODE: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(6i32);
pub const D2D1_DISTANTDIFFUSE_PROP_SURFACE_SCALE: D2D1_DISTANTDIFFUSE_PROP = D2D1_DISTANTDIFFUSE_PROP(3i32);
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(4i32);
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(2i32);
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(5i32);
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(1i32);
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(3i32);
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTDIFFUSE_SCALE_MODE = D2D1_DISTANTDIFFUSE_SCALE_MODE(0i32);
pub const D2D1_DISTANTSPECULAR_PROP_AZIMUTH: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(0i32);
pub const D2D1_DISTANTSPECULAR_PROP_COLOR: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(5i32);
pub const D2D1_DISTANTSPECULAR_PROP_ELEVATION: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(1i32);
pub const D2D1_DISTANTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(6i32);
pub const D2D1_DISTANTSPECULAR_PROP_SCALE_MODE: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(7i32);
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(3i32);
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(2i32);
pub const D2D1_DISTANTSPECULAR_PROP_SURFACE_SCALE: D2D1_DISTANTSPECULAR_PROP = D2D1_DISTANTSPECULAR_PROP(4i32);
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(4i32);
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(2i32);
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(5i32);
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(1i32);
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(3i32);
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTSPECULAR_SCALE_MODE = D2D1_DISTANTSPECULAR_SCALE_MODE(0i32);
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_ANISOTROPIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(4i32);
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(2i32);
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(5i32);
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(1i32);
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(3i32);
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = D2D1_DPICOMPENSATION_INTERPOLATION_MODE(0i32);
pub const D2D1_DPICOMPENSATION_PROP_BORDER_MODE: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(1i32);
pub const D2D1_DPICOMPENSATION_PROP_INPUT_DPI: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(2i32);
pub const D2D1_DPICOMPENSATION_PROP_INTERPOLATION_MODE: D2D1_DPICOMPENSATION_PROP = D2D1_DPICOMPENSATION_PROP(0i32);
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(2i32);
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(8i32);
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(4i32);
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(0i32);
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(1i32);
pub const D2D1_EDGEDETECTION_MODE_PREWITT: D2D1_EDGEDETECTION_MODE = D2D1_EDGEDETECTION_MODE(1i32);
pub const D2D1_EDGEDETECTION_MODE_SOBEL: D2D1_EDGEDETECTION_MODE = D2D1_EDGEDETECTION_MODE(0i32);
pub const D2D1_EDGEDETECTION_PROP_ALPHA_MODE: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(4i32);
pub const D2D1_EDGEDETECTION_PROP_BLUR_RADIUS: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(1i32);
pub const D2D1_EDGEDETECTION_PROP_MODE: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(2i32);
pub const D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(3i32);
pub const D2D1_EDGEDETECTION_PROP_STRENGTH: D2D1_EDGEDETECTION_PROP = D2D1_EDGEDETECTION_PROP(0i32);
pub const D2D1_EMBOSS_PROP_DIRECTION: D2D1_EMBOSS_PROP = D2D1_EMBOSS_PROP(1i32);
pub const D2D1_EMBOSS_PROP_HEIGHT: D2D1_EMBOSS_PROP = D2D1_EMBOSS_PROP(0i32);
pub const D2D1_EXPOSURE_PROP_EXPOSURE_VALUE: D2D1_EXPOSURE_PROP = D2D1_EXPOSURE_PROP(0i32);
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(0i32);
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(2i32);
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(1i32);
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE = D2D1_FACTORY_TYPE(1i32);
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = D2D1_FACTORY_TYPE(0i32);
pub const D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS: D2D1_FEATURE = D2D1_FEATURE(1i32);
pub const D2D1_FEATURE_DOUBLES: D2D1_FEATURE = D2D1_FEATURE(0i32);
pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(40960i32);
pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(37120i32);
pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(0i32);
pub const D2D1_FILTER_ANISOTROPIC: D2D1_FILTER = D2D1_FILTER(85i32);
pub const D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT: D2D1_FILTER = D2D1_FILTER(16i32);
pub const D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(17i32);
pub const D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT: D2D1_FILTER = D2D1_FILTER(20i32);
pub const D2D1_FILTER_MIN_MAG_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(21i32);
pub const D2D1_FILTER_MIN_MAG_MIP_POINT: D2D1_FILTER = D2D1_FILTER(0i32);
pub const D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(1i32);
pub const D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D2D1_FILTER = D2D1_FILTER(4i32);
pub const D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR: D2D1_FILTER = D2D1_FILTER(5i32);
pub const D2D1_FLOOD_PROP_COLOR: D2D1_FLOOD_PROP = D2D1_FLOOD_PROP(0i32);
pub const D2D1_GAMMA1_G10: D2D1_GAMMA1 = D2D1_GAMMA1(1i32);
pub const D2D1_GAMMA1_G2084: D2D1_GAMMA1 = D2D1_GAMMA1(2i32);
pub const D2D1_GAMMA1_G22: D2D1_GAMMA1 = D2D1_GAMMA1(0i32);
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(12i32);
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(15i32);
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(13i32);
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(14i32);
pub const D2D1_GAMMATRANSFER_PROP_BLUE_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(8i32);
pub const D2D1_GAMMATRANSFER_PROP_BLUE_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(11i32);
pub const D2D1_GAMMATRANSFER_PROP_BLUE_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(9i32);
pub const D2D1_GAMMATRANSFER_PROP_BLUE_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(10i32);
pub const D2D1_GAMMATRANSFER_PROP_CLAMP_OUTPUT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(16i32);
pub const D2D1_GAMMATRANSFER_PROP_GREEN_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(4i32);
pub const D2D1_GAMMATRANSFER_PROP_GREEN_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(7i32);
pub const D2D1_GAMMATRANSFER_PROP_GREEN_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(5i32);
pub const D2D1_GAMMATRANSFER_PROP_GREEN_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(6i32);
pub const D2D1_GAMMATRANSFER_PROP_RED_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(0i32);
pub const D2D1_GAMMATRANSFER_PROP_RED_DISABLE: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(3i32);
pub const D2D1_GAMMATRANSFER_PROP_RED_EXPONENT: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(1i32);
pub const D2D1_GAMMATRANSFER_PROP_RED_OFFSET: D2D1_GAMMATRANSFER_PROP = D2D1_GAMMATRANSFER_PROP(2i32);
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = D2D1_GAMMA(1i32);
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = D2D1_GAMMA(0i32);
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(1i32);
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_QUALITY: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(2i32);
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED: D2D1_GAUSSIANBLUR_OPTIMIZATION = D2D1_GAUSSIANBLUR_OPTIMIZATION(0i32);
pub const D2D1_GAUSSIANBLUR_PROP_BORDER_MODE: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(2i32);
pub const D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(1i32);
pub const D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION: D2D1_GAUSSIANBLUR_PROP = D2D1_GAUSSIANBLUR_PROP(0i32);
pub const D2D1_GEOMETRY_RELATION_CONTAINS: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(3i32);
pub const D2D1_GEOMETRY_RELATION_DISJOINT: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(1i32);
pub const D2D1_GEOMETRY_RELATION_IS_CONTAINED: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(2i32);
pub const D2D1_GEOMETRY_RELATION_OVERLAP: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(4i32);
pub const D2D1_GEOMETRY_RELATION_UNKNOWN: D2D1_GEOMETRY_RELATION = D2D1_GEOMETRY_RELATION(0i32);
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = D2D1_GEOMETRY_SIMPLIFICATION_OPTION(0i32);
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = D2D1_GEOMETRY_SIMPLIFICATION_OPTION(1i32);
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_HDR: D2D1_HDRTONEMAP_DISPLAY_MODE = D2D1_HDRTONEMAP_DISPLAY_MODE(1i32);
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_SDR: D2D1_HDRTONEMAP_DISPLAY_MODE = D2D1_HDRTONEMAP_DISPLAY_MODE(0i32);
pub const D2D1_HDRTONEMAP_PROP_DISPLAY_MODE: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(2i32);
pub const D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(0i32);
pub const D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = D2D1_HDRTONEMAP_PROP(1i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(0i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(1i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(2i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(0i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(3i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(4i32);
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = D2D1_HIGHLIGHTSANDSHADOWS_PROP(1i32);
pub const D2D1_HISTOGRAM_PROP_CHANNEL_SELECT: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(1i32);
pub const D2D1_HISTOGRAM_PROP_HISTOGRAM_OUTPUT: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(2i32);
pub const D2D1_HISTOGRAM_PROP_NUM_BINS: D2D1_HISTOGRAM_PROP = D2D1_HISTOGRAM_PROP(0i32);
pub const D2D1_HUEROTATION_PROP_ANGLE: D2D1_HUEROTATION_PROP = D2D1_HUEROTATION_PROP(0i32);
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_HUETORGB_INPUT_COLOR_SPACE = D2D1_HUETORGB_INPUT_COLOR_SPACE(1i32);
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_HUETORGB_INPUT_COLOR_SPACE = D2D1_HUETORGB_INPUT_COLOR_SPACE(0i32);
pub const D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE: D2D1_HUETORGB_PROP = D2D1_HUETORGB_PROP(0i32);
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(1i32);
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(0i32);
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(2i32);
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(0i32);
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = D2D1_IMAGE_SOURCE_LOADING_OPTIONS(1i32);
pub const D2D1_INK_NIB_SHAPE_ROUND: D2D1_INK_NIB_SHAPE = D2D1_INK_NIB_SHAPE(0i32);
pub const D2D1_INK_NIB_SHAPE_SQUARE: D2D1_INK_NIB_SHAPE = D2D1_INK_NIB_SHAPE(1i32);
pub const D2D1_INTERPOLATION_MODE_ANISOTROPIC: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(4i32);
pub const D2D1_INTERPOLATION_MODE_CUBIC: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(2i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(4i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(2i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_FANT: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(6i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(5i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(1i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(7i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(3i32);
pub const D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE_DEFINITION = D2D1_INTERPOLATION_MODE_DEFINITION(0i32);
pub const D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(5i32);
pub const D2D1_INTERPOLATION_MODE_LINEAR: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(1i32);
pub const D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(3i32);
pub const D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE = D2D1_INTERPOLATION_MODE(0i32);
pub const D2D1_LAYER_OPTIONS1_IGNORE_ALPHA: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(2i32);
pub const D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(1i32);
pub const D2D1_LAYER_OPTIONS1_NONE: D2D1_LAYER_OPTIONS1 = D2D1_LAYER_OPTIONS1(0i32);
pub const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(1i32);
pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(0i32);
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(11i32);
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(10i32);
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(9i32);
pub const D2D1_LINEARTRANSFER_PROP_BLUE_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(8i32);
pub const D2D1_LINEARTRANSFER_PROP_BLUE_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(7i32);
pub const D2D1_LINEARTRANSFER_PROP_BLUE_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(6i32);
pub const D2D1_LINEARTRANSFER_PROP_CLAMP_OUTPUT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(12i32);
pub const D2D1_LINEARTRANSFER_PROP_GREEN_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(5i32);
pub const D2D1_LINEARTRANSFER_PROP_GREEN_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(4i32);
pub const D2D1_LINEARTRANSFER_PROP_GREEN_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(3i32);
pub const D2D1_LINEARTRANSFER_PROP_RED_DISABLE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(2i32);
pub const D2D1_LINEARTRANSFER_PROP_RED_SLOPE: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(1i32);
pub const D2D1_LINEARTRANSFER_PROP_RED_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = D2D1_LINEARTRANSFER_PROP(0i32);
pub const D2D1_LINE_JOIN_BEVEL: D2D1_LINE_JOIN = D2D1_LINE_JOIN(1i32);
pub const D2D1_LINE_JOIN_MITER: D2D1_LINE_JOIN = D2D1_LINE_JOIN(0i32);
pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: D2D1_LINE_JOIN = D2D1_LINE_JOIN(3i32);
pub const D2D1_LINE_JOIN_ROUND: D2D1_LINE_JOIN = D2D1_LINE_JOIN(2i32);
pub const D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE: D2D1_LOOKUPTABLE3D_PROP = D2D1_LOOKUPTABLE3D_PROP(1i32);
pub const D2D1_LOOKUPTABLE3D_PROP_LUT: D2D1_LOOKUPTABLE3D_PROP = D2D1_LOOKUPTABLE3D_PROP(0i32);
pub const D2D1_MAP_OPTIONS_DISCARD: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(4i32);
pub const D2D1_MAP_OPTIONS_NONE: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(0i32);
pub const D2D1_MAP_OPTIONS_READ: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(1i32);
pub const D2D1_MAP_OPTIONS_WRITE: D2D1_MAP_OPTIONS = D2D1_MAP_OPTIONS(2i32);
pub const D2D1_MORPHOLOGY_MODE_DILATE: D2D1_MORPHOLOGY_MODE = D2D1_MORPHOLOGY_MODE(1i32);
pub const D2D1_MORPHOLOGY_MODE_ERODE: D2D1_MORPHOLOGY_MODE = D2D1_MORPHOLOGY_MODE(0i32);
pub const D2D1_MORPHOLOGY_PROP_HEIGHT: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(2i32);
pub const D2D1_MORPHOLOGY_PROP_MODE: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(0i32);
pub const D2D1_MORPHOLOGY_PROP_WIDTH: D2D1_MORPHOLOGY_PROP = D2D1_MORPHOLOGY_PROP(1i32);
pub const D2D1_OPACITYMETADATA_PROP_INPUT_OPAQUE_RECT: D2D1_OPACITYMETADATA_PROP = D2D1_OPACITYMETADATA_PROP(0i32);
pub const D2D1_OPACITY_MASK_CONTENT_GRAPHICS: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(0i32);
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(2i32);
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL: D2D1_OPACITY_MASK_CONTENT = D2D1_OPACITY_MASK_CONTENT(1i32);
pub const D2D1_OPACITY_PROP_OPACITY: D2D1_OPACITY_PROP = D2D1_OPACITY_PROP(0i32);
pub const D2D1_ORIENTATION_DEFAULT: D2D1_ORIENTATION = D2D1_ORIENTATION(1i32);
pub const D2D1_ORIENTATION_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(2i32);
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_ORIENTATION = D2D1_ORIENTATION(3i32);
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(4i32);
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_ORIENTATION = D2D1_ORIENTATION(6i32);
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(7i32);
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_ORIENTATION = D2D1_ORIENTATION(8i32);
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_ORIENTATION = D2D1_ORIENTATION(5i32);
pub const D2D1_PATCH_EDGE_MODE_ALIASED: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(0i32);
pub const D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(2i32);
pub const D2D1_PATCH_EDGE_MODE_ANTIALIASED: D2D1_PATCH_EDGE_MODE = D2D1_PATCH_EDGE_MODE(1i32);
pub const D2D1_PIXEL_OPTIONS_NONE: D2D1_PIXEL_OPTIONS = D2D1_PIXEL_OPTIONS(0i32);
pub const D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING: D2D1_PIXEL_OPTIONS = D2D1_PIXEL_OPTIONS(1i32);
pub const D2D1_POINTDIFFUSE_PROP_COLOR: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(3i32);
pub const D2D1_POINTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(1i32);
pub const D2D1_POINTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(4i32);
pub const D2D1_POINTDIFFUSE_PROP_LIGHT_POSITION: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(0i32);
pub const D2D1_POINTDIFFUSE_PROP_SCALE_MODE: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(5i32);
pub const D2D1_POINTDIFFUSE_PROP_SURFACE_SCALE: D2D1_POINTDIFFUSE_PROP = D2D1_POINTDIFFUSE_PROP(2i32);
pub const D2D1_POINTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(4i32);
pub const D2D1_POINTDIFFUSE_SCALE_MODE_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(2i32);
pub const D2D1_POINTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(5i32);
pub const D2D1_POINTDIFFUSE_SCALE_MODE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(1i32);
pub const D2D1_POINTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(3i32);
pub const D2D1_POINTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTDIFFUSE_SCALE_MODE = D2D1_POINTDIFFUSE_SCALE_MODE(0i32);
pub const D2D1_POINTSPECULAR_PROP_COLOR: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(4i32);
pub const D2D1_POINTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(5i32);
pub const D2D1_POINTSPECULAR_PROP_LIGHT_POSITION: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(0i32);
pub const D2D1_POINTSPECULAR_PROP_SCALE_MODE: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(6i32);
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(2i32);
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(1i32);
pub const D2D1_POINTSPECULAR_PROP_SURFACE_SCALE: D2D1_POINTSPECULAR_PROP = D2D1_POINTSPECULAR_PROP(3i32);
pub const D2D1_POINTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(4i32);
pub const D2D1_POINTSPECULAR_SCALE_MODE_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(2i32);
pub const D2D1_POINTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(5i32);
pub const D2D1_POINTSPECULAR_SCALE_MODE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(1i32);
pub const D2D1_POINTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(3i32);
pub const D2D1_POINTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTSPECULAR_SCALE_MODE = D2D1_POINTSPECULAR_SCALE_MODE(0i32);
pub const D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(2i32);
pub const D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(1i32);
pub const D2D1_POSTERIZE_PROP_RED_VALUE_COUNT: D2D1_POSTERIZE_PROP = D2D1_POSTERIZE_PROP(0i32);
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(2i32);
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(0i32);
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = D2D1_PRESENT_OPTIONS(1i32);
pub const D2D1_PRIMITIVE_BLEND_ADD: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(3i32);
pub const D2D1_PRIMITIVE_BLEND_COPY: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(1i32);
pub const D2D1_PRIMITIVE_BLEND_MAX: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(4i32);
pub const D2D1_PRIMITIVE_BLEND_MIN: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(2i32);
pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: D2D1_PRIMITIVE_BLEND = D2D1_PRIMITIVE_BLEND(0i32);
pub const D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(0i32);
pub const D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(1i32);
pub const D2D1_PRINT_FONT_SUBSET_MODE_NONE: D2D1_PRINT_FONT_SUBSET_MODE = D2D1_PRINT_FONT_SUBSET_MODE(2i32);
pub const D2D1_PROPERTY_AUTHOR: D2D1_PROPERTY = D2D1_PROPERTY(-2147483646i32);
pub const D2D1_PROPERTY_CACHED: D2D1_PROPERTY = D2D1_PROPERTY(-2147483642i32);
pub const D2D1_PROPERTY_CATEGORY: D2D1_PROPERTY = D2D1_PROPERTY(-2147483645i32);
pub const D2D1_PROPERTY_CLSID: D2D1_PROPERTY = D2D1_PROPERTY(-2147483648i32);
pub const D2D1_PROPERTY_DESCRIPTION: D2D1_PROPERTY = D2D1_PROPERTY(-2147483644i32);
pub const D2D1_PROPERTY_DISPLAYNAME: D2D1_PROPERTY = D2D1_PROPERTY(-2147483647i32);
pub const D2D1_PROPERTY_INPUTS: D2D1_PROPERTY = D2D1_PROPERTY(-2147483643i32);
pub const D2D1_PROPERTY_MAX_INPUTS: D2D1_PROPERTY = D2D1_PROPERTY(-2147483639i32);
pub const D2D1_PROPERTY_MIN_INPUTS: D2D1_PROPERTY = D2D1_PROPERTY(-2147483640i32);
pub const D2D1_PROPERTY_PRECISION: D2D1_PROPERTY = D2D1_PROPERTY(-2147483641i32);
pub const D2D1_PROPERTY_TYPE_ARRAY: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(12i32);
pub const D2D1_PROPERTY_TYPE_BLOB: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(9i32);
pub const D2D1_PROPERTY_TYPE_BOOL: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(2i32);
pub const D2D1_PROPERTY_TYPE_CLSID: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(13i32);
pub const D2D1_PROPERTY_TYPE_COLOR_CONTEXT: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(18i32);
pub const D2D1_PROPERTY_TYPE_ENUM: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(11i32);
pub const D2D1_PROPERTY_TYPE_FLOAT: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(5i32);
pub const D2D1_PROPERTY_TYPE_INT32: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(4i32);
pub const D2D1_PROPERTY_TYPE_IUNKNOWN: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(10i32);
pub const D2D1_PROPERTY_TYPE_MATRIX_3X2: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(14i32);
pub const D2D1_PROPERTY_TYPE_MATRIX_4X3: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(15i32);
pub const D2D1_PROPERTY_TYPE_MATRIX_4X4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(16i32);
pub const D2D1_PROPERTY_TYPE_MATRIX_5X4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(17i32);
pub const D2D1_PROPERTY_TYPE_STRING: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(1i32);
pub const D2D1_PROPERTY_TYPE_UINT32: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(3i32);
pub const D2D1_PROPERTY_TYPE_UNKNOWN: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(0i32);
pub const D2D1_PROPERTY_TYPE_VECTOR2: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(6i32);
pub const D2D1_PROPERTY_TYPE_VECTOR3: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(7i32);
pub const D2D1_PROPERTY_TYPE_VECTOR4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(8i32);
pub const D2D1_RENDERING_PRIORITY_LOW: D2D1_RENDERING_PRIORITY = D2D1_RENDERING_PRIORITY(1i32);
pub const D2D1_RENDERING_PRIORITY_NORMAL: D2D1_RENDERING_PRIORITY = D2D1_RENDERING_PRIORITY(0i32);
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(0i32);
pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(2i32);
pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(1i32);
pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(1i32);
pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(2i32);
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = D2D1_RENDER_TARGET_USAGE(0i32);
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(1i32);
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(0i32);
pub const D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE: D2D1_RGBTOHUE_PROP = D2D1_RGBTOHUE_PROP(0i32);
pub const D2D1_SATURATION_PROP_SATURATION: D2D1_SATURATION_PROP = D2D1_SATURATION_PROP(0i32);
pub const D2D1_SCALE_INTERPOLATION_MODE_ANISOTROPIC: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(4i32);
pub const D2D1_SCALE_INTERPOLATION_MODE_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(2i32);
pub const D2D1_SCALE_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(5i32);
pub const D2D1_SCALE_INTERPOLATION_MODE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(1i32);
pub const D2D1_SCALE_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(3i32);
pub const D2D1_SCALE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_SCALE_INTERPOLATION_MODE = D2D1_SCALE_INTERPOLATION_MODE(0i32);
pub const D2D1_SCALE_PROP_BORDER_MODE: D2D1_SCALE_PROP = D2D1_SCALE_PROP(3i32);
pub const D2D1_SCALE_PROP_CENTER_POINT: D2D1_SCALE_PROP = D2D1_SCALE_PROP(1i32);
pub const D2D1_SCALE_PROP_INTERPOLATION_MODE: D2D1_SCALE_PROP = D2D1_SCALE_PROP(2i32);
pub const D2D1_SCALE_PROP_SCALE: D2D1_SCALE_PROP = D2D1_SCALE_PROP(0i32);
pub const D2D1_SCALE_PROP_SHARPNESS: D2D1_SCALE_PROP = D2D1_SCALE_PROP(4i32);
pub const D2D1_SCENE_REFERRED_SDR_WHITE_LEVEL: f32 = 80f32;
pub const D2D1_SEPIA_PROP_ALPHA_MODE: D2D1_SEPIA_PROP = D2D1_SEPIA_PROP(1i32);
pub const D2D1_SEPIA_PROP_INTENSITY: D2D1_SEPIA_PROP = D2D1_SEPIA_PROP(0i32);
pub const D2D1_SHADOW_OPTIMIZATION_BALANCED: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(1i32);
pub const D2D1_SHADOW_OPTIMIZATION_QUALITY: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(2i32);
pub const D2D1_SHADOW_OPTIMIZATION_SPEED: D2D1_SHADOW_OPTIMIZATION = D2D1_SHADOW_OPTIMIZATION(0i32);
pub const D2D1_SHADOW_PROP_BLUR_STANDARD_DEVIATION: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(0i32);
pub const D2D1_SHADOW_PROP_COLOR: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(1i32);
pub const D2D1_SHADOW_PROP_OPTIMIZATION: D2D1_SHADOW_PROP = D2D1_SHADOW_PROP(2i32);
pub const D2D1_SHARPEN_PROP_SHARPNESS: D2D1_SHARPEN_PROP = D2D1_SHARPEN_PROP(0i32);
pub const D2D1_SHARPEN_PROP_THRESHOLD: D2D1_SHARPEN_PROP = D2D1_SHARPEN_PROP(1i32);
pub const D2D1_SPOTDIFFUSE_PROP_COLOR: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(6i32);
pub const D2D1_SPOTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(4i32);
pub const D2D1_SPOTDIFFUSE_PROP_FOCUS: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(2i32);
pub const D2D1_SPOTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(7i32);
pub const D2D1_SPOTDIFFUSE_PROP_LIGHT_POSITION: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(0i32);
pub const D2D1_SPOTDIFFUSE_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(3i32);
pub const D2D1_SPOTDIFFUSE_PROP_POINTS_AT: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(1i32);
pub const D2D1_SPOTDIFFUSE_PROP_SCALE_MODE: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(8i32);
pub const D2D1_SPOTDIFFUSE_PROP_SURFACE_SCALE: D2D1_SPOTDIFFUSE_PROP = D2D1_SPOTDIFFUSE_PROP(5i32);
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(4i32);
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(2i32);
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(5i32);
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(1i32);
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(3i32);
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTDIFFUSE_SCALE_MODE = D2D1_SPOTDIFFUSE_SCALE_MODE(0i32);
pub const D2D1_SPOTSPECULAR_PROP_COLOR: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(7i32);
pub const D2D1_SPOTSPECULAR_PROP_FOCUS: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(2i32);
pub const D2D1_SPOTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(8i32);
pub const D2D1_SPOTSPECULAR_PROP_LIGHT_POSITION: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(0i32);
pub const D2D1_SPOTSPECULAR_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(3i32);
pub const D2D1_SPOTSPECULAR_PROP_POINTS_AT: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(1i32);
pub const D2D1_SPOTSPECULAR_PROP_SCALE_MODE: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(9i32);
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(5i32);
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(4i32);
pub const D2D1_SPOTSPECULAR_PROP_SURFACE_SCALE: D2D1_SPOTSPECULAR_PROP = D2D1_SPOTSPECULAR_PROP(6i32);
pub const D2D1_SPOTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(4i32);
pub const D2D1_SPOTSPECULAR_SCALE_MODE_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(2i32);
pub const D2D1_SPOTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(5i32);
pub const D2D1_SPOTSPECULAR_SCALE_MODE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(1i32);
pub const D2D1_SPOTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(3i32);
pub const D2D1_SPOTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTSPECULAR_SCALE_MODE = D2D1_SPOTSPECULAR_SCALE_MODE(0i32);
pub const D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE: D2D1_SPRITE_OPTIONS = D2D1_SPRITE_OPTIONS(1i32);
pub const D2D1_SPRITE_OPTIONS_NONE: D2D1_SPRITE_OPTIONS = D2D1_SPRITE_OPTIONS(0i32);
pub const D2D1_STRAIGHTEN_PROP_ANGLE: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(0i32);
pub const D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(1i32);
pub const D2D1_STRAIGHTEN_PROP_SCALE_MODE: D2D1_STRAIGHTEN_PROP = D2D1_STRAIGHTEN_PROP(2i32);
pub const D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(4i32);
pub const D2D1_STRAIGHTEN_SCALE_MODE_CUBIC: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(2i32);
pub const D2D1_STRAIGHTEN_SCALE_MODE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(1i32);
pub const D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(3i32);
pub const D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_STRAIGHTEN_SCALE_MODE = D2D1_STRAIGHTEN_SCALE_MODE(0i32);
pub const D2D1_STROKE_TRANSFORM_TYPE_FIXED: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(1i32);
pub const D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(2i32);
pub const D2D1_STROKE_TRANSFORM_TYPE_NORMAL: D2D1_STROKE_TRANSFORM_TYPE = D2D1_STROKE_TRANSFORM_TYPE(0i32);
pub const D2D1_SUBPROPERTY_DEFAULT: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483644i32);
pub const D2D1_SUBPROPERTY_DISPLAYNAME: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483648i32);
pub const D2D1_SUBPROPERTY_FIELDS: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483643i32);
pub const D2D1_SUBPROPERTY_INDEX: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483642i32);
pub const D2D1_SUBPROPERTY_ISREADONLY: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483647i32);
pub const D2D1_SUBPROPERTY_MAX: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483645i32);
pub const D2D1_SUBPROPERTY_MIN: D2D1_SUBPROPERTY = D2D1_SUBPROPERTY(-2147483646i32);
pub const D2D1_SVG_ASPECT_ALIGN_NONE: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(0i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(9i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(6i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(3i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(8i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(5i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(2i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(7i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(4i32);
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN: D2D1_SVG_ASPECT_ALIGN = D2D1_SVG_ASPECT_ALIGN(1i32);
pub const D2D1_SVG_ASPECT_SCALING_MEET: D2D1_SVG_ASPECT_SCALING = D2D1_SVG_ASPECT_SCALING(0i32);
pub const D2D1_SVG_ASPECT_SCALING_SLICE: D2D1_SVG_ASPECT_SCALING = D2D1_SVG_ASPECT_SCALING(1i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(1i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(3i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(10i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(2i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(0i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(13i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(5i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(6i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(8i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(4i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(11i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(9i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(12i32);
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY: D2D1_SVG_ATTRIBUTE_POD_TYPE = D2D1_SVG_ATTRIBUTE_POD_TYPE(7i32);
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID: D2D1_SVG_ATTRIBUTE_STRING_TYPE = D2D1_SVG_ATTRIBUTE_STRING_TYPE(1i32);
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG: D2D1_SVG_ATTRIBUTE_STRING_TYPE = D2D1_SVG_ATTRIBUTE_STRING_TYPE(0i32);
pub const D2D1_SVG_DISPLAY_INLINE: D2D1_SVG_DISPLAY = D2D1_SVG_DISPLAY(0i32);
pub const D2D1_SVG_DISPLAY_NONE: D2D1_SVG_DISPLAY = D2D1_SVG_DISPLAY(1i32);
pub const D2D1_SVG_LENGTH_UNITS_NUMBER: D2D1_SVG_LENGTH_UNITS = D2D1_SVG_LENGTH_UNITS(0i32);
pub const D2D1_SVG_LENGTH_UNITS_PERCENTAGE: D2D1_SVG_LENGTH_UNITS = D2D1_SVG_LENGTH_UNITS(1i32);
pub const D2D1_SVG_LINE_CAP_BUTT: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(0i32);
pub const D2D1_SVG_LINE_CAP_ROUND: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(2i32);
pub const D2D1_SVG_LINE_CAP_SQUARE: D2D1_SVG_LINE_CAP = D2D1_SVG_LINE_CAP(1i32);
pub const D2D1_SVG_LINE_JOIN_BEVEL: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(1i32);
pub const D2D1_SVG_LINE_JOIN_MITER: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(3i32);
pub const D2D1_SVG_LINE_JOIN_ROUND: D2D1_SVG_LINE_JOIN = D2D1_SVG_LINE_JOIN(2i32);
pub const D2D1_SVG_OVERFLOW_HIDDEN: D2D1_SVG_OVERFLOW = D2D1_SVG_OVERFLOW(1i32);
pub const D2D1_SVG_OVERFLOW_VISIBLE: D2D1_SVG_OVERFLOW = D2D1_SVG_OVERFLOW(0i32);
pub const D2D1_SVG_PAINT_TYPE_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(1i32);
pub const D2D1_SVG_PAINT_TYPE_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(2i32);
pub const D2D1_SVG_PAINT_TYPE_NONE: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(0i32);
pub const D2D1_SVG_PAINT_TYPE_URI: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(3i32);
pub const D2D1_SVG_PAINT_TYPE_URI_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(5i32);
pub const D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(6i32);
pub const D2D1_SVG_PAINT_TYPE_URI_NONE: D2D1_SVG_PAINT_TYPE = D2D1_SVG_PAINT_TYPE(4i32);
pub const D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(9i32);
pub const D2D1_SVG_PATH_COMMAND_ARC_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(10i32);
pub const D2D1_SVG_PATH_COMMAND_CLOSE_PATH: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(0i32);
pub const D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(5i32);
pub const D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(6i32);
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(15i32);
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(16i32);
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(11i32);
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(12i32);
pub const D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(3i32);
pub const D2D1_SVG_PATH_COMMAND_LINE_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(4i32);
pub const D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(1i32);
pub const D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(2i32);
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(7i32);
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(8i32);
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(17i32);
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(18i32);
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(13i32);
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE: D2D1_SVG_PATH_COMMAND = D2D1_SVG_PATH_COMMAND(14i32);
pub const D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX: D2D1_SVG_UNIT_TYPE = D2D1_SVG_UNIT_TYPE(1i32);
pub const D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE: D2D1_SVG_UNIT_TYPE = D2D1_SVG_UNIT_TYPE(0i32);
pub const D2D1_SVG_VISIBILITY_HIDDEN: D2D1_SVG_VISIBILITY = D2D1_SVG_VISIBILITY(1i32);
pub const D2D1_SVG_VISIBILITY_VISIBLE: D2D1_SVG_VISIBILITY = D2D1_SVG_VISIBILITY(0i32);
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = D2D1_SWEEP_DIRECTION(1i32);
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = D2D1_SWEEP_DIRECTION(0i32);
pub const D2D1_TABLETRANSFER_PROP_ALPHA_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(7i32);
pub const D2D1_TABLETRANSFER_PROP_ALPHA_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(6i32);
pub const D2D1_TABLETRANSFER_PROP_BLUE_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(5i32);
pub const D2D1_TABLETRANSFER_PROP_BLUE_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(4i32);
pub const D2D1_TABLETRANSFER_PROP_CLAMP_OUTPUT: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(8i32);
pub const D2D1_TABLETRANSFER_PROP_GREEN_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(3i32);
pub const D2D1_TABLETRANSFER_PROP_GREEN_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(2i32);
pub const D2D1_TABLETRANSFER_PROP_RED_DISABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(1i32);
pub const D2D1_TABLETRANSFER_PROP_RED_TABLE: D2D1_TABLETRANSFER_PROP = D2D1_TABLETRANSFER_PROP(0i32);
pub const D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE: D2D1_TEMPERATUREANDTINT_PROP = D2D1_TEMPERATUREANDTINT_PROP(0i32);
pub const D2D1_TEMPERATUREANDTINT_PROP_TINT: D2D1_TEMPERATUREANDTINT_PROP = D2D1_TEMPERATUREANDTINT_PROP(1i32);
pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(3i32);
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(1i32);
pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(0i32);
pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(2i32);
pub const D2D1_THREADING_MODE_MULTI_THREADED: D2D1_THREADING_MODE = D2D1_THREADING_MODE(1i32);
pub const D2D1_THREADING_MODE_SINGLE_THREADED: D2D1_THREADING_MODE = D2D1_THREADING_MODE(0i32);
pub const D2D1_TILE_PROP_RECT: D2D1_TILE_PROP = D2D1_TILE_PROP(0i32);
pub const D2D1_TINT_PROP_CLAMP_OUTPUT: D2D1_TINT_PROP = D2D1_TINT_PROP(1i32);
pub const D2D1_TINT_PROP_COLOR: D2D1_TINT_PROP = D2D1_TINT_PROP(0i32);
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(1i32);
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(0i32);
pub const D2D1_TURBULENCE_PROP_BASE_FREQUENCY: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(2i32);
pub const D2D1_TURBULENCE_PROP_NOISE: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(5i32);
pub const D2D1_TURBULENCE_PROP_NUM_OCTAVES: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(3i32);
pub const D2D1_TURBULENCE_PROP_OFFSET: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(0i32);
pub const D2D1_TURBULENCE_PROP_SEED: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(4i32);
pub const D2D1_TURBULENCE_PROP_SIZE: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(1i32);
pub const D2D1_TURBULENCE_PROP_STITCHABLE: D2D1_TURBULENCE_PROP = D2D1_TURBULENCE_PROP(6i32);
pub const D2D1_UNIT_MODE_DIPS: D2D1_UNIT_MODE = D2D1_UNIT_MODE(0i32);
pub const D2D1_UNIT_MODE_PIXELS: D2D1_UNIT_MODE = D2D1_UNIT_MODE(1i32);
pub const D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(4i32);
pub const D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(1i32);
pub const D2D1_VERTEX_OPTIONS_NONE: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(0i32);
pub const D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER: D2D1_VERTEX_OPTIONS = D2D1_VERTEX_OPTIONS(2i32);
pub const D2D1_VERTEX_USAGE_DYNAMIC: D2D1_VERTEX_USAGE = D2D1_VERTEX_USAGE(1i32);
pub const D2D1_VERTEX_USAGE_STATIC: D2D1_VERTEX_USAGE = D2D1_VERTEX_USAGE(0i32);
pub const D2D1_VIGNETTE_PROP_COLOR: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(0i32);
pub const D2D1_VIGNETTE_PROP_STRENGTH: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(2i32);
pub const D2D1_VIGNETTE_PROP_TRANSITION_SIZE: D2D1_VIGNETTE_PROP = D2D1_VIGNETTE_PROP(1i32);
pub const D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = D2D1_WHITELEVELADJUSTMENT_PROP(0i32);
pub const D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = D2D1_WHITELEVELADJUSTMENT_PROP(1i32);
pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(0i32);
pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(1i32);
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_420: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(1i32);
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_422: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(2i32);
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_440: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(4i32);
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_444: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(3i32);
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO: D2D1_YCBCR_CHROMA_SUBSAMPLING = D2D1_YCBCR_CHROMA_SUBSAMPLING(0i32);
pub const D2D1_YCBCR_INTERPOLATION_MODE_ANISOTROPIC: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(4i32);
pub const D2D1_YCBCR_INTERPOLATION_MODE_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(2i32);
pub const D2D1_YCBCR_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(5i32);
pub const D2D1_YCBCR_INTERPOLATION_MODE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(1i32);
pub const D2D1_YCBCR_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(3i32);
pub const D2D1_YCBCR_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_YCBCR_INTERPOLATION_MODE = D2D1_YCBCR_INTERPOLATION_MODE(0i32);
pub const D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(0i32);
pub const D2D1_YCBCR_PROP_INTERPOLATION_MODE: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(2i32);
pub const D2D1_YCBCR_PROP_TRANSFORM_MATRIX: D2D1_YCBCR_PROP = D2D1_YCBCR_PROP(1i32);
pub const FACILITY_D2D: u32 = 2201u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_2DAFFINETRANSFORM_PROP(pub i32);
impl windows_core::TypeKind for D2D1_2DAFFINETRANSFORM_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_2DAFFINETRANSFORM_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_2DAFFINETRANSFORM_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_3DPERSPECTIVETRANSFORM_PROP(pub i32);
impl windows_core::TypeKind for D2D1_3DPERSPECTIVETRANSFORM_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_3DPERSPECTIVETRANSFORM_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_3DPERSPECTIVETRANSFORM_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_3DTRANSFORM_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_3DTRANSFORM_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_3DTRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_3DTRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_3DTRANSFORM_PROP(pub i32);
impl windows_core::TypeKind for D2D1_3DTRANSFORM_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_3DTRANSFORM_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_3DTRANSFORM_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_ANTIALIAS_MODE(pub i32);
impl windows_core::TypeKind for D2D1_ANTIALIAS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_ARC_SIZE(pub i32);
impl windows_core::TypeKind for D2D1_ARC_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_ARC_SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_ARC_SIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_ARITHMETICCOMPOSITE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_ARITHMETICCOMPOSITE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_ARITHMETICCOMPOSITE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_ARITHMETICCOMPOSITE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_ATLAS_PROP(pub i32);
impl windows_core::TypeKind for D2D1_ATLAS_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_ATLAS_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_ATLAS_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BITMAPSOURCE_ALPHA_MODE(pub i32);
impl windows_core::TypeKind for D2D1_BITMAPSOURCE_ALPHA_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BITMAPSOURCE_ALPHA_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_ALPHA_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BITMAPSOURCE_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BITMAPSOURCE_ORIENTATION(pub i32);
impl windows_core::TypeKind for D2D1_BITMAPSOURCE_ORIENTATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BITMAPSOURCE_ORIENTATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_ORIENTATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BITMAPSOURCE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_BITMAPSOURCE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BITMAPSOURCE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BITMAP_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_BITMAP_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BITMAP_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_BITMAP_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BITMAP_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BITMAP_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_BITMAP_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_BITMAP_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_BITMAP_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_BITMAP_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_BITMAP_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_BITMAP_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BLEND(pub i32);
impl windows_core::TypeKind for D2D1_BLEND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BLEND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BLEND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BLEND_OPERATION(pub i32);
impl windows_core::TypeKind for D2D1_BLEND_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BLEND_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BLEND_PROP(pub i32);
impl windows_core::TypeKind for D2D1_BLEND_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BLEND_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BORDER_EDGE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_BORDER_EDGE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BORDER_EDGE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_EDGE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BORDER_PROP(pub i32);
impl windows_core::TypeKind for D2D1_BORDER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BORDER_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BRIGHTNESS_PROP(pub i32);
impl windows_core::TypeKind for D2D1_BRIGHTNESS_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BRIGHTNESS_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BRIGHTNESS_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BUFFER_PRECISION(pub i32);
impl windows_core::TypeKind for D2D1_BUFFER_PRECISION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BUFFER_PRECISION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BUFFER_PRECISION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CAP_STYLE(pub i32);
impl windows_core::TypeKind for D2D1_CAP_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CAP_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CAP_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl D2D1_CHANGE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_CHANGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_CHANGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_CHANGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_CHANGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_CHANGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CHANNEL_DEPTH(pub i32);
impl windows_core::TypeKind for D2D1_CHANNEL_DEPTH {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CHANNEL_DEPTH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CHANNEL_DEPTH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CHANNEL_SELECTOR(pub i32);
impl windows_core::TypeKind for D2D1_CHANNEL_SELECTOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CHANNEL_SELECTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CHANNEL_SELECTOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CHROMAKEY_PROP(pub i32);
impl windows_core::TypeKind for D2D1_CHROMAKEY_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CHROMAKEY_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CHROMAKEY_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLORMANAGEMENT_ALPHA_MODE(pub i32);
impl windows_core::TypeKind for D2D1_COLORMANAGEMENT_ALPHA_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLORMANAGEMENT_ALPHA_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_ALPHA_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLORMANAGEMENT_PROP(pub i32);
impl windows_core::TypeKind for D2D1_COLORMANAGEMENT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLORMANAGEMENT_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLORMANAGEMENT_QUALITY(pub i32);
impl windows_core::TypeKind for D2D1_COLORMANAGEMENT_QUALITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLORMANAGEMENT_QUALITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_QUALITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLORMANAGEMENT_RENDERING_INTENT(pub i32);
impl windows_core::TypeKind for D2D1_COLORMANAGEMENT_RENDERING_INTENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLORMANAGEMENT_RENDERING_INTENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_RENDERING_INTENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLORMATRIX_PROP(pub i32);
impl windows_core::TypeKind for D2D1_COLORMATRIX_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLORMATRIX_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLORMATRIX_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(pub i32);
impl windows_core::TypeKind for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLOR_CONTEXT_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_COLOR_CONTEXT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLOR_CONTEXT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLOR_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_COLOR_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLOR_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLOR_SPACE(pub i32);
impl windows_core::TypeKind for D2D1_COLOR_SPACE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLOR_SPACE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_SPACE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COMBINE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_COMBINE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COMBINE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COMBINE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COMPOSITE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_COMPOSITE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COMPOSITE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COMPOSITE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CONTRAST_PROP(pub i32);
impl windows_core::TypeKind for D2D1_CONTRAST_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CONTRAST_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CONTRAST_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CONVOLVEMATRIX_PROP(pub i32);
impl windows_core::TypeKind for D2D1_CONVOLVEMATRIX_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CONVOLVEMATRIX_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CONVOLVEMATRIX_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CONVOLVEMATRIX_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_CONVOLVEMATRIX_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CONVOLVEMATRIX_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CONVOLVEMATRIX_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CROP_PROP(pub i32);
impl windows_core::TypeKind for D2D1_CROP_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CROP_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CROP_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_CROSSFADE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_CROSSFADE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_CROSSFADE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_CROSSFADE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DASH_STYLE(pub i32);
impl windows_core::TypeKind for D2D1_DASH_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DASH_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DASH_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DC_INITIALIZE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_DC_INITIALIZE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DC_INITIALIZE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DC_INITIALIZE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DEBUG_LEVEL(pub i32);
impl windows_core::TypeKind for D2D1_DEBUG_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DEBUG_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DEBUG_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DEVICE_CONTEXT_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_DEVICE_CONTEXT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DEVICE_CONTEXT_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_DEVICE_CONTEXT_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DIRECTIONALBLUR_OPTIMIZATION(pub i32);
impl windows_core::TypeKind for D2D1_DIRECTIONALBLUR_OPTIMIZATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DIRECTIONALBLUR_OPTIMIZATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DIRECTIONALBLUR_OPTIMIZATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DIRECTIONALBLUR_PROP(pub i32);
impl windows_core::TypeKind for D2D1_DIRECTIONALBLUR_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DIRECTIONALBLUR_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DIRECTIONALBLUR_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DISCRETETRANSFER_PROP(pub i32);
impl windows_core::TypeKind for D2D1_DISCRETETRANSFER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DISCRETETRANSFER_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DISCRETETRANSFER_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DISPLACEMENTMAP_PROP(pub i32);
impl windows_core::TypeKind for D2D1_DISPLACEMENTMAP_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DISPLACEMENTMAP_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DISPLACEMENTMAP_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DISTANTDIFFUSE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_DISTANTDIFFUSE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DISTANTDIFFUSE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTDIFFUSE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DISTANTDIFFUSE_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_DISTANTDIFFUSE_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DISTANTDIFFUSE_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTDIFFUSE_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DISTANTSPECULAR_PROP(pub i32);
impl windows_core::TypeKind for D2D1_DISTANTSPECULAR_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DISTANTSPECULAR_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTSPECULAR_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DISTANTSPECULAR_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_DISTANTSPECULAR_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DISTANTSPECULAR_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTSPECULAR_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DPICOMPENSATION_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DPICOMPENSATION_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DPICOMPENSATION_PROP(pub i32);
impl windows_core::TypeKind for D2D1_DPICOMPENSATION_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DPICOMPENSATION_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DPICOMPENSATION_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_DRAW_TEXT_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_DRAW_TEXT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_DRAW_TEXT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_DRAW_TEXT_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_DRAW_TEXT_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_DRAW_TEXT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_DRAW_TEXT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_DRAW_TEXT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_DRAW_TEXT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_DRAW_TEXT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_EDGEDETECTION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_EDGEDETECTION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_EDGEDETECTION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_EDGEDETECTION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_EDGEDETECTION_PROP(pub i32);
impl windows_core::TypeKind for D2D1_EDGEDETECTION_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_EDGEDETECTION_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_EDGEDETECTION_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_EMBOSS_PROP(pub i32);
impl windows_core::TypeKind for D2D1_EMBOSS_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_EMBOSS_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_EMBOSS_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_EXPOSURE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_EXPOSURE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_EXPOSURE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_EXPOSURE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_EXTEND_MODE(pub i32);
impl windows_core::TypeKind for D2D1_EXTEND_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_EXTEND_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_EXTEND_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FACTORY_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_FACTORY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FACTORY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FACTORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FEATURE(pub i32);
impl windows_core::TypeKind for D2D1_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FEATURE_LEVEL(pub i32);
impl windows_core::TypeKind for D2D1_FEATURE_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FEATURE_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FEATURE_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FILTER(pub i32);
impl windows_core::TypeKind for D2D1_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FLOOD_PROP(pub i32);
impl windows_core::TypeKind for D2D1_FLOOD_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FLOOD_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FLOOD_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GAMMA(pub i32);
impl windows_core::TypeKind for D2D1_GAMMA {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GAMMA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GAMMA").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GAMMA1(pub i32);
impl windows_core::TypeKind for D2D1_GAMMA1 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GAMMA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GAMMA1").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GAMMATRANSFER_PROP(pub i32);
impl windows_core::TypeKind for D2D1_GAMMATRANSFER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GAMMATRANSFER_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GAMMATRANSFER_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GAUSSIANBLUR_OPTIMIZATION(pub i32);
impl windows_core::TypeKind for D2D1_GAUSSIANBLUR_OPTIMIZATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GAUSSIANBLUR_OPTIMIZATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GAUSSIANBLUR_OPTIMIZATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GAUSSIANBLUR_PROP(pub i32);
impl windows_core::TypeKind for D2D1_GAUSSIANBLUR_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GAUSSIANBLUR_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GAUSSIANBLUR_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GEOMETRY_RELATION(pub i32);
impl windows_core::TypeKind for D2D1_GEOMETRY_RELATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GEOMETRY_RELATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GEOMETRY_RELATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_GEOMETRY_SIMPLIFICATION_OPTION(pub i32);
impl windows_core::TypeKind for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_GEOMETRY_SIMPLIFICATION_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HDRTONEMAP_DISPLAY_MODE(pub i32);
impl windows_core::TypeKind for D2D1_HDRTONEMAP_DISPLAY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HDRTONEMAP_DISPLAY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HDRTONEMAP_DISPLAY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HDRTONEMAP_PROP(pub i32);
impl windows_core::TypeKind for D2D1_HDRTONEMAP_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HDRTONEMAP_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HDRTONEMAP_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(pub i32);
impl windows_core::TypeKind for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HIGHLIGHTSANDSHADOWS_PROP(pub i32);
impl windows_core::TypeKind for D2D1_HIGHLIGHTSANDSHADOWS_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HIGHLIGHTSANDSHADOWS_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HIGHLIGHTSANDSHADOWS_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HISTOGRAM_PROP(pub i32);
impl windows_core::TypeKind for D2D1_HISTOGRAM_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HISTOGRAM_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HISTOGRAM_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HUEROTATION_PROP(pub i32);
impl windows_core::TypeKind for D2D1_HUEROTATION_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HUEROTATION_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HUEROTATION_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HUETORGB_INPUT_COLOR_SPACE(pub i32);
impl windows_core::TypeKind for D2D1_HUETORGB_INPUT_COLOR_SPACE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HUETORGB_INPUT_COLOR_SPACE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HUETORGB_INPUT_COLOR_SPACE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_HUETORGB_PROP(pub i32);
impl windows_core::TypeKind for D2D1_HUETORGB_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_HUETORGB_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_HUETORGB_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_IMAGE_SOURCE_LOADING_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_IMAGE_SOURCE_LOADING_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_INK_NIB_SHAPE(pub i32);
impl windows_core::TypeKind for D2D1_INK_NIB_SHAPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_INK_NIB_SHAPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_INK_NIB_SHAPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_INTERPOLATION_MODE_DEFINITION(pub i32);
impl windows_core::TypeKind for D2D1_INTERPOLATION_MODE_DEFINITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_INTERPOLATION_MODE_DEFINITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_INTERPOLATION_MODE_DEFINITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_LAYER_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_LAYER_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_LAYER_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_LAYER_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_LAYER_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_LAYER_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_LAYER_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_LAYER_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_LAYER_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_LAYER_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_LAYER_OPTIONS1(pub i32);
impl windows_core::TypeKind for D2D1_LAYER_OPTIONS1 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_LAYER_OPTIONS1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_LAYER_OPTIONS1").field(&self.0).finish()
    }
}
impl D2D1_LAYER_OPTIONS1 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_LAYER_OPTIONS1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_LAYER_OPTIONS1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_LAYER_OPTIONS1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_LAYER_OPTIONS1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_LAYER_OPTIONS1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_LINEARTRANSFER_PROP(pub i32);
impl windows_core::TypeKind for D2D1_LINEARTRANSFER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_LINEARTRANSFER_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_LINEARTRANSFER_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_LINE_JOIN(pub i32);
impl windows_core::TypeKind for D2D1_LINE_JOIN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_LINE_JOIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_LINE_JOIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_LOOKUPTABLE3D_PROP(pub i32);
impl windows_core::TypeKind for D2D1_LOOKUPTABLE3D_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_LOOKUPTABLE3D_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_LOOKUPTABLE3D_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_MAP_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_MAP_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_MAP_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_MAP_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_MAP_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_MAP_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_MAP_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_MAP_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_MAP_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_MAP_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_MORPHOLOGY_MODE(pub i32);
impl windows_core::TypeKind for D2D1_MORPHOLOGY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_MORPHOLOGY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_MORPHOLOGY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_MORPHOLOGY_PROP(pub i32);
impl windows_core::TypeKind for D2D1_MORPHOLOGY_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_MORPHOLOGY_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_MORPHOLOGY_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_OPACITYMETADATA_PROP(pub i32);
impl windows_core::TypeKind for D2D1_OPACITYMETADATA_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_OPACITYMETADATA_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_OPACITYMETADATA_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_OPACITY_MASK_CONTENT(pub i32);
impl windows_core::TypeKind for D2D1_OPACITY_MASK_CONTENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_OPACITY_MASK_CONTENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_OPACITY_MASK_CONTENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_OPACITY_PROP(pub i32);
impl windows_core::TypeKind for D2D1_OPACITY_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_OPACITY_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_OPACITY_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_ORIENTATION(pub i32);
impl windows_core::TypeKind for D2D1_ORIENTATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_ORIENTATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_ORIENTATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PATCH_EDGE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_PATCH_EDGE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PATCH_EDGE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PATCH_EDGE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PIXEL_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_PIXEL_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PIXEL_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PIXEL_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_PIXEL_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_PIXEL_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_PIXEL_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_PIXEL_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_PIXEL_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_PIXEL_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_POINTDIFFUSE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_POINTDIFFUSE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_POINTDIFFUSE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_POINTDIFFUSE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_POINTDIFFUSE_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_POINTDIFFUSE_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_POINTDIFFUSE_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_POINTDIFFUSE_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_POINTSPECULAR_PROP(pub i32);
impl windows_core::TypeKind for D2D1_POINTSPECULAR_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_POINTSPECULAR_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_POINTSPECULAR_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_POINTSPECULAR_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_POINTSPECULAR_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_POINTSPECULAR_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_POINTSPECULAR_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_POSTERIZE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_POSTERIZE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_POSTERIZE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_POSTERIZE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PRESENT_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_PRESENT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PRESENT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PRESENT_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_PRESENT_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_PRESENT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_PRESENT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_PRESENT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_PRESENT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_PRESENT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PRIMITIVE_BLEND(pub i32);
impl windows_core::TypeKind for D2D1_PRIMITIVE_BLEND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PRIMITIVE_BLEND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PRIMITIVE_BLEND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PRINT_FONT_SUBSET_MODE(pub i32);
impl windows_core::TypeKind for D2D1_PRINT_FONT_SUBSET_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PRINT_FONT_SUBSET_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PRINT_FONT_SUBSET_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PROPERTY(pub i32);
impl windows_core::TypeKind for D2D1_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PROPERTY_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_PROPERTY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PROPERTY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_RENDERING_PRIORITY(pub i32);
impl windows_core::TypeKind for D2D1_RENDERING_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_RENDERING_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_RENDERING_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_RENDER_TARGET_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_RENDER_TARGET_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_RENDER_TARGET_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_RENDER_TARGET_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_RENDER_TARGET_USAGE(pub i32);
impl windows_core::TypeKind for D2D1_RENDER_TARGET_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_RENDER_TARGET_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_RENDER_TARGET_USAGE").field(&self.0).finish()
    }
}
impl D2D1_RENDER_TARGET_USAGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_RENDER_TARGET_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_RENDER_TARGET_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_RENDER_TARGET_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_RENDER_TARGET_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_RENDER_TARGET_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(pub i32);
impl windows_core::TypeKind for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_RGBTOHUE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_RGBTOHUE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_RGBTOHUE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_RGBTOHUE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SATURATION_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SATURATION_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SATURATION_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SATURATION_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SCALE_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_SCALE_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SCALE_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SCALE_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SCALE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SCALE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SCALE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SCALE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SEPIA_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SEPIA_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SEPIA_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SEPIA_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SHADOW_OPTIMIZATION(pub i32);
impl windows_core::TypeKind for D2D1_SHADOW_OPTIMIZATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SHADOW_OPTIMIZATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SHADOW_OPTIMIZATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SHADOW_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SHADOW_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SHADOW_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SHADOW_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SHARPEN_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SHARPEN_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SHARPEN_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SHARPEN_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SPOTDIFFUSE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SPOTDIFFUSE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SPOTDIFFUSE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SPOTDIFFUSE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SPOTDIFFUSE_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_SPOTDIFFUSE_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SPOTDIFFUSE_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SPOTDIFFUSE_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SPOTSPECULAR_PROP(pub i32);
impl windows_core::TypeKind for D2D1_SPOTSPECULAR_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SPOTSPECULAR_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SPOTSPECULAR_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SPOTSPECULAR_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_SPOTSPECULAR_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SPOTSPECULAR_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SPOTSPECULAR_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SPRITE_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_SPRITE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SPRITE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SPRITE_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_SPRITE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_SPRITE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_SPRITE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_SPRITE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_SPRITE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_SPRITE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_STRAIGHTEN_PROP(pub i32);
impl windows_core::TypeKind for D2D1_STRAIGHTEN_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_STRAIGHTEN_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_STRAIGHTEN_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_STRAIGHTEN_SCALE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_STRAIGHTEN_SCALE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_STRAIGHTEN_SCALE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_STRAIGHTEN_SCALE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_STROKE_TRANSFORM_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_STROKE_TRANSFORM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_STROKE_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_STROKE_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SUBPROPERTY(pub i32);
impl windows_core::TypeKind for D2D1_SUBPROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SUBPROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SUBPROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_ASPECT_ALIGN(pub i32);
impl windows_core::TypeKind for D2D1_SVG_ASPECT_ALIGN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_ASPECT_ALIGN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ASPECT_ALIGN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_ASPECT_SCALING(pub i32);
impl windows_core::TypeKind for D2D1_SVG_ASPECT_SCALING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_ASPECT_SCALING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ASPECT_SCALING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_ATTRIBUTE_POD_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_SVG_ATTRIBUTE_POD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_ATTRIBUTE_POD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ATTRIBUTE_POD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_ATTRIBUTE_STRING_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ATTRIBUTE_STRING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_DISPLAY(pub i32);
impl windows_core::TypeKind for D2D1_SVG_DISPLAY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_DISPLAY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_DISPLAY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_LENGTH_UNITS(pub i32);
impl windows_core::TypeKind for D2D1_SVG_LENGTH_UNITS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_LENGTH_UNITS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_LENGTH_UNITS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_LINE_CAP(pub i32);
impl windows_core::TypeKind for D2D1_SVG_LINE_CAP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_LINE_CAP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_LINE_CAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_LINE_JOIN(pub i32);
impl windows_core::TypeKind for D2D1_SVG_LINE_JOIN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_LINE_JOIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_LINE_JOIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_OVERFLOW(pub i32);
impl windows_core::TypeKind for D2D1_SVG_OVERFLOW {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_OVERFLOW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_OVERFLOW").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_PAINT_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_SVG_PAINT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_PAINT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_PAINT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_PATH_COMMAND(pub i32);
impl windows_core::TypeKind for D2D1_SVG_PATH_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_PATH_COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_PATH_COMMAND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_UNIT_TYPE(pub i32);
impl windows_core::TypeKind for D2D1_SVG_UNIT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_UNIT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_UNIT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SVG_VISIBILITY(pub i32);
impl windows_core::TypeKind for D2D1_SVG_VISIBILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SVG_VISIBILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SVG_VISIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_SWEEP_DIRECTION(pub i32);
impl windows_core::TypeKind for D2D1_SWEEP_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_SWEEP_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_SWEEP_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TABLETRANSFER_PROP(pub i32);
impl windows_core::TypeKind for D2D1_TABLETRANSFER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TABLETRANSFER_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TABLETRANSFER_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TEMPERATUREANDTINT_PROP(pub i32);
impl windows_core::TypeKind for D2D1_TEMPERATUREANDTINT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TEMPERATUREANDTINT_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TEMPERATUREANDTINT_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TEXT_ANTIALIAS_MODE(pub i32);
impl windows_core::TypeKind for D2D1_TEXT_ANTIALIAS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TEXT_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TEXT_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_THREADING_MODE(pub i32);
impl windows_core::TypeKind for D2D1_THREADING_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_THREADING_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_THREADING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TILE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_TILE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TILE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TILE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TINT_PROP(pub i32);
impl windows_core::TypeKind for D2D1_TINT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TINT_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TINT_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TURBULENCE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_TURBULENCE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TURBULENCE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TURBULENCE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_UNIT_MODE(pub i32);
impl windows_core::TypeKind for D2D1_UNIT_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_UNIT_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_UNIT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_VERTEX_OPTIONS(pub i32);
impl windows_core::TypeKind for D2D1_VERTEX_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_VERTEX_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_VERTEX_OPTIONS").field(&self.0).finish()
    }
}
impl D2D1_VERTEX_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_VERTEX_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_VERTEX_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_VERTEX_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_VERTEX_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_VERTEX_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_VERTEX_USAGE(pub i32);
impl windows_core::TypeKind for D2D1_VERTEX_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_VERTEX_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_VERTEX_USAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_VIGNETTE_PROP(pub i32);
impl windows_core::TypeKind for D2D1_VIGNETTE_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_VIGNETTE_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_VIGNETTE_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_WHITELEVELADJUSTMENT_PROP(pub i32);
impl windows_core::TypeKind for D2D1_WHITELEVELADJUSTMENT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_WHITELEVELADJUSTMENT_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_WHITELEVELADJUSTMENT_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_WINDOW_STATE(pub i32);
impl windows_core::TypeKind for D2D1_WINDOW_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_WINDOW_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_WINDOW_STATE").field(&self.0).finish()
    }
}
impl D2D1_WINDOW_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_WINDOW_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_WINDOW_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_WINDOW_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_WINDOW_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_WINDOW_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_YCBCR_CHROMA_SUBSAMPLING(pub i32);
impl windows_core::TypeKind for D2D1_YCBCR_CHROMA_SUBSAMPLING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_YCBCR_CHROMA_SUBSAMPLING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_YCBCR_CHROMA_SUBSAMPLING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_YCBCR_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_YCBCR_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_YCBCR_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_YCBCR_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_YCBCR_PROP(pub i32);
impl windows_core::TypeKind for D2D1_YCBCR_PROP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_YCBCR_PROP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_YCBCR_PROP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PAINT_FEATURE_LEVEL(pub i32);
impl windows_core::TypeKind for DWRITE_PAINT_FEATURE_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PAINT_FEATURE_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PAINT_FEATURE_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_ARC_SEGMENT {
    pub point: Common::D2D_POINT_2F,
    pub size: Common::D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_ARC_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_ARC_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
impl windows_core::TypeKind for D2D1_BITMAP_BRUSH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
impl windows_core::TypeKind for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D2D1_BITMAP_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D2D1_BITMAP_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Debug, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: core::mem::ManuallyDrop<Option<ID2D1ColorContext>>,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl Clone for D2D1_BITMAP_PROPERTIES1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D2D1_BITMAP_PROPERTIES1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D2D1_BITMAP_PROPERTIES1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_BLEND_DESCRIPTION {
    pub sourceBlend: D2D1_BLEND,
    pub destinationBlend: D2D1_BLEND,
    pub blendOperation: D2D1_BLEND_OPERATION,
    pub sourceBlendAlpha: D2D1_BLEND,
    pub destinationBlendAlpha: D2D1_BLEND,
    pub blendOperationAlpha: D2D1_BLEND_OPERATION,
    pub blendFactor: [f32; 4],
}
impl windows_core::TypeKind for D2D1_BLEND_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_BLEND_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for D2D1_BRUSH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for D2D1_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_CREATION_PROPERTIES {
    pub threadingMode: D2D1_THREADING_MODE,
    pub debugLevel: D2D1_DEBUG_LEVEL,
    pub options: D2D1_DEVICE_CONTEXT_OPTIONS,
}
impl windows_core::TypeKind for D2D1_CREATION_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_CREATION_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    pub shaderBufferWithInputSignature: *const u8,
    pub shaderBufferSize: u32,
    pub inputElements: *const D2D1_INPUT_ELEMENT_DESC,
    pub elementCount: u32,
    pub stride: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for D2D1_DRAWING_STATE_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for D2D1_DRAWING_STATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
    pub unitMode: D2D1_UNIT_MODE,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for D2D1_DRAWING_STATE_DESCRIPTION1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Debug, PartialEq)]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: core::mem::ManuallyDrop<Option<ID2D1Effect>>,
    pub inputIndex: u32,
    pub inputRectangle: Common::D2D_RECT_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Clone for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_EFFECT_INPUT_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_ELLIPSE {
    pub point: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_ELLIPSE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_ELLIPSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
impl windows_core::TypeKind for D2D1_FACTORY_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_FACTORY_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_FEATURE_DATA_DOUBLES {
    pub doublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for D2D1_FEATURE_DATA_DOUBLES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_FEATURE_DATA_DOUBLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_GRADIENT_MESH_PATCH {
    pub point00: Common::D2D_POINT_2F,
    pub point01: Common::D2D_POINT_2F,
    pub point02: Common::D2D_POINT_2F,
    pub point03: Common::D2D_POINT_2F,
    pub point10: Common::D2D_POINT_2F,
    pub point11: Common::D2D_POINT_2F,
    pub point12: Common::D2D_POINT_2F,
    pub point13: Common::D2D_POINT_2F,
    pub point20: Common::D2D_POINT_2F,
    pub point21: Common::D2D_POINT_2F,
    pub point22: Common::D2D_POINT_2F,
    pub point23: Common::D2D_POINT_2F,
    pub point30: Common::D2D_POINT_2F,
    pub point31: Common::D2D_POINT_2F,
    pub point32: Common::D2D_POINT_2F,
    pub point33: Common::D2D_POINT_2F,
    pub color00: Common::D2D1_COLOR_F,
    pub color03: Common::D2D1_COLOR_F,
    pub color30: Common::D2D1_COLOR_F,
    pub color33: Common::D2D1_COLOR_F,
    pub topEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_GRADIENT_MESH_PATCH {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_GRADIENT_MESH_PATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: super::super::Foundation::HWND,
    pub pixelSize: Common::D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES {
    pub sourceRectangle: Common::D2D_RECT_F,
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_IMAGE_BRUSH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_INK_BEZIER_SEGMENT {
    pub point1: D2D1_INK_POINT,
    pub point2: D2D1_INK_POINT,
    pub point3: D2D1_INK_POINT,
}
impl windows_core::TypeKind for D2D1_INK_BEZIER_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_INK_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_INK_POINT {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl windows_core::TypeKind for D2D1_INK_POINT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_INK_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_INK_STYLE_PROPERTIES {
    pub nibShape: D2D1_INK_NIB_SHAPE,
    pub nibTransform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for D2D1_INK_STYLE_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for D2D1_INK_STYLE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_INPUT_DESCRIPTION {
    pub filter: D2D1_FILTER,
    pub levelOfDetailCount: u32,
}
impl windows_core::TypeKind for D2D1_INPUT_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_INPUT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_INPUT_ELEMENT_DESC {
    pub semanticName: windows_core::PCSTR,
    pub semanticIndex: u32,
    pub format: super::Dxgi::Common::DXGI_FORMAT,
    pub inputSlot: u32,
    pub alignedByteOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D2D1_INPUT_ELEMENT_DESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D2D1_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
#[derive(Debug, PartialEq)]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl Clone for D2D1_LAYER_PARAMETERS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::TypeKind for D2D1_LAYER_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl Default for D2D1_LAYER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
#[derive(Debug, PartialEq)]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl Clone for D2D1_LAYER_PARAMETERS1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::TypeKind for D2D1_LAYER_PARAMETERS1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl Default for D2D1_LAYER_PARAMETERS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: Common::D2D_POINT_2F,
    pub endPoint: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub bits: *mut u8,
}
impl windows_core::TypeKind for D2D1_MAPPED_RECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_MAPPED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_POINT_DESCRIPTION {
    pub point: Common::D2D_POINT_2F,
    pub unitTangentVector: Common::D2D_POINT_2F,
    pub endSegment: u32,
    pub endFigure: u32,
    pub lengthToEndSegment: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_POINT_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_POINT_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_PRINT_CONTROL_PROPERTIES {
    pub fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    pub rasterDPI: f32,
    pub colorSpace: D2D1_COLOR_SPACE,
}
impl windows_core::TypeKind for D2D1_PRINT_CONTROL_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_PRINT_CONTROL_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D2D1_PROPERTY_BINDING {
    pub propertyName: windows_core::PCWSTR,
    pub setFunction: PD2D1_PROPERTY_SET_FUNCTION,
    pub getFunction: PD2D1_PROPERTY_GET_FUNCTION,
}
impl windows_core::TypeKind for D2D1_PROPERTY_BINDING {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_PROPERTY_BINDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_QUADRATIC_BEZIER_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: Common::D2D_POINT_2F,
    pub gradientOriginOffset: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: Common::D2D_SIZE_U,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_RENDERING_CONTROLS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_RENDERING_CONTROLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for D2D1_RENDER_TARGET_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for D2D1_RENDER_TARGET_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    pub extents: *const u32,
    pub dimensions: u32,
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub channelDepth: D2D1_CHANNEL_DEPTH,
    pub filter: D2D1_FILTER,
    pub extendModes: *const D2D1_EXTEND_MODE,
}
impl windows_core::TypeKind for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: Common::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_ROUNDED_RECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_ROUNDED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_SIMPLE_COLOR_PROFILE {
    pub redPrimary: Common::D2D_POINT_2F,
    pub greenPrimary: Common::D2D_POINT_2F,
    pub bluePrimary: Common::D2D_POINT_2F,
    pub whitePointXZ: Common::D2D_POINT_2F,
    pub gamma: D2D1_GAMMA1,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_SIMPLE_COLOR_PROFILE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_SIMPLE_COLOR_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
impl windows_core::TypeKind for D2D1_STROKE_STYLE_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_STROKE_STYLE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
    pub transformType: D2D1_STROKE_TRANSFORM_TYPE,
}
impl windows_core::TypeKind for D2D1_STROKE_STYLE_PROPERTIES1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_SVG_LENGTH {
    pub value: f32,
    pub units: D2D1_SVG_LENGTH_UNITS,
}
impl windows_core::TypeKind for D2D1_SVG_LENGTH {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_SVG_LENGTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_SVG_PRESERVE_ASPECT_RATIO {
    pub defer: super::super::Foundation::BOOL,
    pub align: D2D1_SVG_ASPECT_ALIGN,
    pub meetOrSlice: D2D1_SVG_ASPECT_SCALING,
}
impl windows_core::TypeKind for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_SVG_VIEWBOX {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl windows_core::TypeKind for D2D1_SVG_VIEWBOX {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_SVG_VIEWBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    pub orientation: D2D1_ORIENTATION,
    pub scaleX: f32,
    pub scaleY: f32,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
    pub options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}
impl windows_core::TypeKind for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_TRIANGLE {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
    pub point3: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for D2D1_TRIANGLE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_TRIANGLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_VERTEX_BUFFER_PROPERTIES {
    pub inputCount: u32,
    pub usage: D2D1_VERTEX_USAGE,
    pub data: *const u8,
    pub byteWidth: u32,
}
impl windows_core::TypeKind for D2D1_VERTEX_BUFFER_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_VERTEX_RANGE {
    pub startVertex: u32,
    pub vertexCount: u32,
}
impl windows_core::TypeKind for D2D1_VERTEX_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_VERTEX_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PD2D1_EFFECT_FACTORY = Option<unsafe extern "system" fn(effectimpl: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT>;
pub type PD2D1_PROPERTY_GET_FUNCTION = Option<unsafe extern "system" fn(effect: Option<windows_core::IUnknown>, data: *mut u8, datasize: u32, actualsize: *mut u32) -> windows_core::HRESULT>;
pub type PD2D1_PROPERTY_SET_FUNCTION = Option<unsafe extern "system" fn(effect: Option<windows_core::IUnknown>, data: *const u8, datasize: u32) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

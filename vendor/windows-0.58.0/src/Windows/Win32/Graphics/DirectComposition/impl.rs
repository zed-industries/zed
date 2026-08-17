#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionAffineTransform2DEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> windows_core::Result<()>;
    fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()>;
    fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetTransformMatrixElement(&self, row: i32, column: i32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
    fn SetSharpness(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetSharpness2(&self, sharpness: f32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for IDCompositionAffineTransform2DEffect {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionAffineTransform2DEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionAffineTransform2DEffect_Vtbl
    where
        Identity: IDCompositionAffineTransform2DEffect_Impl,
    {
        unsafe extern "system" fn SetInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetInterpolationMode(this, core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetBorderMode(this, core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetTransformMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetTransformMatrix(this, core::mem::transmute_copy(&transformmatrix)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetTransformMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetTransformMatrixElement2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetTransformMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetSharpness<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetSharpness(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetSharpness2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharpness: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionAffineTransform2DEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAffineTransform2DEffect_Impl::SetSharpness2(this, core::mem::transmute_copy(&sharpness)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetInterpolationMode: SetInterpolationMode::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Identity, OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Identity, OFFSET>,
            SetTransformMatrixElement2: SetTransformMatrixElement2::<Identity, OFFSET>,
            SetSharpness: SetSharpness::<Identity, OFFSET>,
            SetSharpness2: SetSharpness2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionAffineTransform2DEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionAnimation_Impl: Sized {
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetAbsoluteBeginTime(&self, begintime: i64) -> windows_core::Result<()>;
    fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()>;
    fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()>;
    fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> windows_core::Result<()>;
    fn End(&self, endoffset: f64, endvalue: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionAnimation {}
impl IDCompositionAnimation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionAnimation_Vtbl
    where
        Identity: IDCompositionAnimation_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionAnimation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAnimation_Impl::Reset(this).into()
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, begintime: i64) -> windows_core::HRESULT
        where
            Identity: IDCompositionAnimation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAnimation_Impl::SetAbsoluteBeginTime(this, core::mem::transmute_copy(&begintime)).into()
        }
        unsafe extern "system" fn AddCubic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionAnimation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAnimation_Impl::AddCubic(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&constantcoefficient), core::mem::transmute_copy(&linearcoefficient), core::mem::transmute_copy(&quadraticcoefficient), core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionAnimation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAnimation_Impl::AddSinusoidal(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&bias), core::mem::transmute_copy(&amplitude), core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&phase)).into()
        }
        unsafe extern "system" fn AddRepeat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> windows_core::HRESULT
        where
            Identity: IDCompositionAnimation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAnimation_Impl::AddRepeat(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&durationtorepeat)).into()
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endoffset: f64, endvalue: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionAnimation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionAnimation_Impl::End(this, core::mem::transmute_copy(&endoffset), core::mem::transmute_copy(&endvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            SetAbsoluteBeginTime: SetAbsoluteBeginTime::<Identity, OFFSET>,
            AddCubic: AddCubic::<Identity, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, OFFSET>,
            AddRepeat: AddRepeat::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionAnimation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionArithmeticCompositeEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clampoutput: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetCoefficient1(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient12(&self, coeffcient1: f32) -> windows_core::Result<()>;
    fn SetCoefficient2(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient22(&self, coefficient2: f32) -> windows_core::Result<()>;
    fn SetCoefficient3(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient32(&self, coefficient3: f32) -> windows_core::Result<()>;
    fn SetCoefficient4(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCoefficient42(&self, coefficient4: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionArithmeticCompositeEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionArithmeticCompositeEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionArithmeticCompositeEffect_Vtbl
    where
        Identity: IDCompositionArithmeticCompositeEffect_Impl,
    {
        unsafe extern "system" fn SetCoefficients<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficients(this, core::mem::transmute_copy(&coefficients)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetCoefficient1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient1(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient12<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coeffcient1: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient12(this, core::mem::transmute_copy(&coeffcient1)).into()
        }
        unsafe extern "system" fn SetCoefficient2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient2(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient22<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficient2: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient22(this, core::mem::transmute_copy(&coefficient2)).into()
        }
        unsafe extern "system" fn SetCoefficient3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient3(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficient3: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient32(this, core::mem::transmute_copy(&coefficient3)).into()
        }
        unsafe extern "system" fn SetCoefficient4<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient4(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCoefficient42<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, coefficient4: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionArithmeticCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionArithmeticCompositeEffect_Impl::SetCoefficient42(this, core::mem::transmute_copy(&coefficient4)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetCoefficients: SetCoefficients::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
            SetCoefficient1: SetCoefficient1::<Identity, OFFSET>,
            SetCoefficient12: SetCoefficient12::<Identity, OFFSET>,
            SetCoefficient2: SetCoefficient2::<Identity, OFFSET>,
            SetCoefficient22: SetCoefficient22::<Identity, OFFSET>,
            SetCoefficient3: SetCoefficient3::<Identity, OFFSET>,
            SetCoefficient32: SetCoefficient32::<Identity, OFFSET>,
            SetCoefficient4: SetCoefficient4::<Identity, OFFSET>,
            SetCoefficient42: SetCoefficient42::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionArithmeticCompositeEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBlendEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionBlendEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBlendEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionBlendEffect_Vtbl
    where
        Identity: IDCompositionBlendEffect_Impl,
    {
        unsafe extern "system" fn SetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionBlendEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBlendEffect_Impl::SetMode(this, core::mem::transmute_copy(&mode)).into()
        }
        Self { base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(), SetMode: SetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionBlendEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBrightnessEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()>;
    fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()>;
    fn SetWhitePointX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetWhitePointX2(&self, whitepointx: f32) -> windows_core::Result<()>;
    fn SetWhitePointY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetWhitePointY2(&self, whitepointy: f32) -> windows_core::Result<()>;
    fn SetBlackPointX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlackPointX2(&self, blackpointx: f32) -> windows_core::Result<()>;
    fn SetBlackPointY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlackPointY2(&self, blackpointy: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionBrightnessEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionBrightnessEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionBrightnessEffect_Vtbl
    where
        Identity: IDCompositionBrightnessEffect_Impl,
    {
        unsafe extern "system" fn SetWhitePoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetWhitePoint(this, core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn SetBlackPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetBlackPoint(this, core::mem::transmute_copy(&blackpoint)).into()
        }
        unsafe extern "system" fn SetWhitePointX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetWhitePointX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepointx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetWhitePointX2(this, core::mem::transmute_copy(&whitepointx)).into()
        }
        unsafe extern "system" fn SetWhitePointY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetWhitePointY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetWhitePointY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepointy: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetWhitePointY2(this, core::mem::transmute_copy(&whitepointy)).into()
        }
        unsafe extern "system" fn SetBlackPointX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetBlackPointX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blackpointx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetBlackPointX2(this, core::mem::transmute_copy(&blackpointx)).into()
        }
        unsafe extern "system" fn SetBlackPointY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetBlackPointY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBlackPointY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blackpointy: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionBrightnessEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionBrightnessEffect_Impl::SetBlackPointY2(this, core::mem::transmute_copy(&blackpointy)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetWhitePoint: SetWhitePoint::<Identity, OFFSET>,
            SetBlackPoint: SetBlackPoint::<Identity, OFFSET>,
            SetWhitePointX: SetWhitePointX::<Identity, OFFSET>,
            SetWhitePointX2: SetWhitePointX2::<Identity, OFFSET>,
            SetWhitePointY: SetWhitePointY::<Identity, OFFSET>,
            SetWhitePointY2: SetWhitePointY2::<Identity, OFFSET>,
            SetBlackPointX: SetBlackPointX::<Identity, OFFSET>,
            SetBlackPointX2: SetBlackPointX2::<Identity, OFFSET>,
            SetBlackPointY: SetBlackPointY::<Identity, OFFSET>,
            SetBlackPointY2: SetBlackPointY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionBrightnessEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionClip_Impl: Sized {}
impl windows_core::RuntimeName for IDCompositionClip {}
impl IDCompositionClip_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionClip_Vtbl
    where
        Identity: IDCompositionClip_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionClip as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionColorMatrixEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> windows_core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
    fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clamp: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionColorMatrixEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionColorMatrixEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionColorMatrixEffect_Vtbl
    where
        Identity: IDCompositionColorMatrixEffect_Impl,
    {
        unsafe extern "system" fn SetMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> windows_core::HRESULT
        where
            Identity: IDCompositionColorMatrixEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionColorMatrixEffect_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionColorMatrixEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionColorMatrixEffect_Impl::SetMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionColorMatrixEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionColorMatrixEffect_Impl::SetMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionColorMatrixEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionColorMatrixEffect_Impl::SetAlphaMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionColorMatrixEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionColorMatrixEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clamp)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionColorMatrixEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionCompositeEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionCompositeEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionCompositeEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionCompositeEffect_Vtbl
    where
        Identity: IDCompositionCompositeEffect_Impl,
    {
        unsafe extern "system" fn SetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionCompositeEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionCompositeEffect_Impl::SetMode(this, core::mem::transmute_copy(&mode)).into()
        }
        Self { base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(), SetMode: SetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionCompositeEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrail_Impl: Sized {
    fn AddTrailPoints(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> windows_core::Result<u32>;
    fn AddTrailPointsWithPrediction(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> windows_core::Result<u32>;
    fn RemoveTrailPoints(&self, generationid: u32) -> windows_core::Result<()>;
    fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionDelegatedInkTrail {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionDelegatedInkTrail_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDelegatedInkTrail_Vtbl
    where
        Identity: IDCompositionDelegatedInkTrail_Impl,
    {
        unsafe extern "system" fn AddTrailPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionDelegatedInkTrail_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDelegatedInkTrail_Impl::AddTrailPoints(this, core::mem::transmute_copy(&inkpoints), core::mem::transmute_copy(&inkpointscount)) {
                Ok(ok__) => {
                    generationid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionDelegatedInkTrail_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDelegatedInkTrail_Impl::AddTrailPointsWithPrediction(this, core::mem::transmute_copy(&inkpoints), core::mem::transmute_copy(&inkpointscount), core::mem::transmute_copy(&predictedinkpoints), core::mem::transmute_copy(&predictedinkpointscount)) {
                Ok(ok__) => {
                    generationid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrailPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, generationid: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionDelegatedInkTrail_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDelegatedInkTrail_Impl::RemoveTrailPoints(this, core::mem::transmute_copy(&generationid)).into()
        }
        unsafe extern "system" fn StartNewTrail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::HRESULT
        where
            Identity: IDCompositionDelegatedInkTrail_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDelegatedInkTrail_Impl::StartNewTrail(this, core::mem::transmute_copy(&color)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTrailPoints: AddTrailPoints::<Identity, OFFSET>,
            AddTrailPointsWithPrediction: AddTrailPointsWithPrediction::<Identity, OFFSET>,
            RemoveTrailPoints: RemoveTrailPoints::<Identity, OFFSET>,
            StartNewTrail: StartNewTrail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDelegatedInkTrail as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDesktopDevice_Impl: Sized + IDCompositionDevice2_Impl {
    fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> windows_core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDesktopDevice {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDesktopDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDesktopDevice_Vtbl
    where
        Identity: IDCompositionDesktopDevice_Impl,
    {
        unsafe extern "system" fn CreateTargetForHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDesktopDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDesktopDevice_Impl::CreateTargetForHwnd(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&topmost)) {
                Ok(ok__) => {
                    target.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDesktopDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDesktopDevice_Impl::CreateSurfaceFromHandle(this, core::mem::transmute_copy(&handle)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDesktopDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDesktopDevice_Impl::CreateSurfaceFromHwnd(this, core::mem::transmute_copy(&hwnd)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDCompositionDevice2_Vtbl::new::<Identity, OFFSET>(),
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDesktopDevice as windows_core::Interface>::IID || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice_Impl: Sized {
    fn Commit(&self) -> windows_core::Result<()>;
    fn WaitForCommitCompletion(&self) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn CreateTargetForHwnd(&self, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> windows_core::Result<IDCompositionTarget>;
    fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual>;
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateSurfaceFromHandle(&self, handle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&self, transforms: *const Option<IDCompositionTransform>, elements: u32) -> windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&self, transforms3d: *const Option<IDCompositionTransform3D>, elements: u32) -> windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation>;
    fn CheckDeviceState(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDevice_Vtbl
    where
        Identity: IDCompositionDevice_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDevice_Impl::Commit(this).into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDevice_Impl::WaitForCommitCompletion(this).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDevice_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&statistics)).into()
        }
        unsafe extern "system" fn CreateTargetForHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateTargetForHwnd(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&topmost)) {
                Ok(ok__) => {
                    target.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVisual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateVisual(this) {
                Ok(ok__) => {
                    visual.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateVirtualSurface(this, core::mem::transmute_copy(&initialwidth), core::mem::transmute_copy(&initialheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    virtualsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateSurfaceFromHandle(this, core::mem::transmute_copy(&handle)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateSurfaceFromHwnd(this, core::mem::transmute_copy(&hwnd)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateTranslateTransform(this) {
                Ok(ok__) => {
                    translatetransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateScaleTransform(this) {
                Ok(ok__) => {
                    scaletransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateRotateTransform(this) {
                Ok(ok__) => {
                    rotatetransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, skewtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateSkewTransform(this) {
                Ok(ok__) => {
                    skewtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateMatrixTransform(this) {
                Ok(ok__) => {
                    matrixtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms: *const *mut core::ffi::c_void, elements: u32, transformgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateTransformGroup(this, core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    transformgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateTranslateTransform3D(this) {
                Ok(ok__) => {
                    translatetransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateScaleTransform3D(this) {
                Ok(ok__) => {
                    scaletransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateRotateTransform3D(this) {
                Ok(ok__) => {
                    rotatetransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateMatrixTransform3D(this) {
                Ok(ok__) => {
                    matrixtransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms3d: *const *mut core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateTransform3DGroup(this, core::mem::transmute_copy(&transforms3d), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    transform3dgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateEffectGroup(this) {
                Ok(ok__) => {
                    effectgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateRectangleClip(this) {
                Ok(ok__) => {
                    clip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CreateAnimation(this) {
                Ok(ok__) => {
                    animation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDeviceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice_Impl::CheckDeviceState(this) {
                Ok(ok__) => {
                    pfvalid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, OFFSET>,
            CreateVisual: CreateVisual::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice2_Impl: Sized {
    fn Commit(&self) -> windows_core::Result<()>;
    fn WaitForCommitCompletion(&self) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual2>;
    fn CreateSurfaceFactory(&self, renderingdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<IDCompositionSurfaceFactory>;
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(&self, transforms: *const Option<IDCompositionTransform>, elements: u32) -> windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self) -> windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(&self, transforms3d: *const Option<IDCompositionTransform3D>, elements: u32) -> windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDevice2_Vtbl
    where
        Identity: IDCompositionDevice2_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDevice2_Impl::Commit(this).into()
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDevice2_Impl::WaitForCommitCompletion(this).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDevice2_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&statistics)).into()
        }
        unsafe extern "system" fn CreateVisual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateVisual(this) {
                Ok(ok__) => {
                    visual.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, surfacefactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateSurfaceFactory(this, windows_core::from_raw_borrowed(&renderingdevice)) {
                Ok(ok__) => {
                    surfacefactory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateVirtualSurface(this, core::mem::transmute_copy(&initialwidth), core::mem::transmute_copy(&initialheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    virtualsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateTranslateTransform(this) {
                Ok(ok__) => {
                    translatetransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateScaleTransform(this) {
                Ok(ok__) => {
                    scaletransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateRotateTransform(this) {
                Ok(ok__) => {
                    rotatetransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, skewtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateSkewTransform(this) {
                Ok(ok__) => {
                    skewtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateMatrixTransform(this) {
                Ok(ok__) => {
                    matrixtransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms: *const *mut core::ffi::c_void, elements: u32, transformgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateTransformGroup(this, core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    transformgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, translatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateTranslateTransform3D(this) {
                Ok(ok__) => {
                    translatetransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaletransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateScaleTransform3D(this) {
                Ok(ok__) => {
                    scaletransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotatetransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateRotateTransform3D(this) {
                Ok(ok__) => {
                    rotatetransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrixtransform3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateMatrixTransform3D(this) {
                Ok(ok__) => {
                    matrixtransform3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transforms3d: *const *mut core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateTransform3DGroup(this, core::mem::transmute_copy(&transforms3d), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    transform3dgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateEffectGroup(this) {
                Ok(ok__) => {
                    effectgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateRectangleClip(this) {
                Ok(ok__) => {
                    clip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice2_Impl::CreateAnimation(this) {
                Ok(ok__) => {
                    animation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            CreateVisual: CreateVisual::<Identity, OFFSET>,
            CreateSurfaceFactory: CreateSurfaceFactory::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice3_Impl: Sized + IDCompositionDevice2_Impl {
    fn CreateGaussianBlurEffect(&self) -> windows_core::Result<IDCompositionGaussianBlurEffect>;
    fn CreateBrightnessEffect(&self) -> windows_core::Result<IDCompositionBrightnessEffect>;
    fn CreateColorMatrixEffect(&self) -> windows_core::Result<IDCompositionColorMatrixEffect>;
    fn CreateShadowEffect(&self) -> windows_core::Result<IDCompositionShadowEffect>;
    fn CreateHueRotationEffect(&self) -> windows_core::Result<IDCompositionHueRotationEffect>;
    fn CreateSaturationEffect(&self) -> windows_core::Result<IDCompositionSaturationEffect>;
    fn CreateTurbulenceEffect(&self) -> windows_core::Result<IDCompositionTurbulenceEffect>;
    fn CreateLinearTransferEffect(&self) -> windows_core::Result<IDCompositionLinearTransferEffect>;
    fn CreateTableTransferEffect(&self) -> windows_core::Result<IDCompositionTableTransferEffect>;
    fn CreateCompositeEffect(&self) -> windows_core::Result<IDCompositionCompositeEffect>;
    fn CreateBlendEffect(&self) -> windows_core::Result<IDCompositionBlendEffect>;
    fn CreateArithmeticCompositeEffect(&self) -> windows_core::Result<IDCompositionArithmeticCompositeEffect>;
    fn CreateAffineTransform2DEffect(&self) -> windows_core::Result<IDCompositionAffineTransform2DEffect>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDevice3_Vtbl
    where
        Identity: IDCompositionDevice3_Impl,
    {
        unsafe extern "system" fn CreateGaussianBlurEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gaussianblureffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateGaussianBlurEffect(this) {
                Ok(ok__) => {
                    gaussianblureffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBrightnessEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brightnesseffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateBrightnessEffect(this) {
                Ok(ok__) => {
                    brightnesseffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colormatrixeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateColorMatrixEffect(this) {
                Ok(ok__) => {
                    colormatrixeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShadowEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, shadoweffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateShadowEffect(this) {
                Ok(ok__) => {
                    shadoweffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHueRotationEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, huerotationeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateHueRotationEffect(this) {
                Ok(ok__) => {
                    huerotationeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSaturationEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, saturationeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateSaturationEffect(this) {
                Ok(ok__) => {
                    saturationeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, turbulenceeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateTurbulenceEffect(this) {
                Ok(ok__) => {
                    turbulenceeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineartransfereffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateLinearTransferEffect(this) {
                Ok(ok__) => {
                    lineartransfereffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableTransferEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tabletransfereffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateTableTransferEffect(this) {
                Ok(ok__) => {
                    tabletransfereffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositeEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, compositeeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateCompositeEffect(this) {
                Ok(ok__) => {
                    compositeeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blendeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateBlendEffect(this) {
                Ok(ok__) => {
                    blendeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, arithmeticcompositeeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateArithmeticCompositeEffect(this) {
                Ok(ok__) => {
                    arithmeticcompositeeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, affinetransform2deffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice3_Impl::CreateAffineTransform2DEffect(this) {
                Ok(ok__) => {
                    affinetransform2deffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDCompositionDevice2_Vtbl::new::<Identity, OFFSET>(),
            CreateGaussianBlurEffect: CreateGaussianBlurEffect::<Identity, OFFSET>,
            CreateBrightnessEffect: CreateBrightnessEffect::<Identity, OFFSET>,
            CreateColorMatrixEffect: CreateColorMatrixEffect::<Identity, OFFSET>,
            CreateShadowEffect: CreateShadowEffect::<Identity, OFFSET>,
            CreateHueRotationEffect: CreateHueRotationEffect::<Identity, OFFSET>,
            CreateSaturationEffect: CreateSaturationEffect::<Identity, OFFSET>,
            CreateTurbulenceEffect: CreateTurbulenceEffect::<Identity, OFFSET>,
            CreateLinearTransferEffect: CreateLinearTransferEffect::<Identity, OFFSET>,
            CreateTableTransferEffect: CreateTableTransferEffect::<Identity, OFFSET>,
            CreateCompositeEffect: CreateCompositeEffect::<Identity, OFFSET>,
            CreateBlendEffect: CreateBlendEffect::<Identity, OFFSET>,
            CreateArithmeticCompositeEffect: CreateArithmeticCompositeEffect::<Identity, OFFSET>,
            CreateAffineTransform2DEffect: CreateAffineTransform2DEffect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice3 as windows_core::Interface>::IID || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice4_Impl: Sized + IDCompositionDevice3_Impl {
    fn CheckCompositionTextureSupport(&self, renderingdevice: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateCompositionTexture(&self, d3dtexture: Option<&windows_core::IUnknown>) -> windows_core::Result<IDCompositionTexture>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionDevice4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionDevice4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDevice4_Vtbl
    where
        Identity: IDCompositionDevice4_Impl,
    {
        unsafe extern "system" fn CheckCompositionTextureSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingdevice: *mut core::ffi::c_void, supportscompositiontextures: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice4_Impl::CheckCompositionTextureSupport(this, windows_core::from_raw_borrowed(&renderingdevice)) {
                Ok(ok__) => {
                    supportscompositiontextures.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompositionTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, d3dtexture: *mut core::ffi::c_void, compositiontexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDevice4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionDevice4_Impl::CreateCompositionTexture(this, windows_core::from_raw_borrowed(&d3dtexture)) {
                Ok(ok__) => {
                    compositiontexture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDCompositionDevice3_Vtbl::new::<Identity, OFFSET>(),
            CheckCompositionTextureSupport: CheckCompositionTextureSupport::<Identity, OFFSET>,
            CreateCompositionTexture: CreateCompositionTexture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice4 as windows_core::Interface>::IID || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID || iid == &<IDCompositionDevice3 as windows_core::Interface>::IID
    }
}
pub trait IDCompositionDeviceDebug_Impl: Sized {
    fn EnableDebugCounters(&self) -> windows_core::Result<()>;
    fn DisableDebugCounters(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionDeviceDebug {}
impl IDCompositionDeviceDebug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionDeviceDebug_Vtbl
    where
        Identity: IDCompositionDeviceDebug_Impl,
    {
        unsafe extern "system" fn EnableDebugCounters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDeviceDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDeviceDebug_Impl::EnableDebugCounters(this).into()
        }
        unsafe extern "system" fn DisableDebugCounters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionDeviceDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionDeviceDebug_Impl::DisableDebugCounters(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableDebugCounters: EnableDebugCounters::<Identity, OFFSET>,
            DisableDebugCounters: DisableDebugCounters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDeviceDebug as windows_core::Interface>::IID
    }
}
pub trait IDCompositionEffect_Impl: Sized {}
impl windows_core::RuntimeName for IDCompositionEffect {}
impl IDCompositionEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionEffect_Vtbl
    where
        Identity: IDCompositionEffect_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionEffectGroup_Impl: Sized + IDCompositionEffect_Impl {
    fn SetOpacity(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()>;
    fn SetTransform3D(&self, transform3d: Option<&IDCompositionTransform3D>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionEffectGroup {}
impl IDCompositionEffectGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionEffectGroup_Vtbl
    where
        Identity: IDCompositionEffectGroup_Impl,
    {
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionEffectGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionEffectGroup_Impl::SetOpacity(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionEffectGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionEffectGroup_Impl::SetOpacity2(this, core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform3d: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionEffectGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionEffectGroup_Impl::SetTransform3D(this, windows_core::from_raw_borrowed(&transform3d)).into()
        }
        Self {
            base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>(),
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, OFFSET>,
            SetTransform3D: SetTransform3D::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionEffectGroup as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionFilterEffect_Impl: Sized + IDCompositionEffect_Impl {
    fn SetInput(&self, index: u32, input: Option<&windows_core::IUnknown>, flags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionFilterEffect {}
impl IDCompositionFilterEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionFilterEffect_Vtbl
    where
        Identity: IDCompositionFilterEffect_Impl,
    {
        unsafe extern "system" fn SetInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, input: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionFilterEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionFilterEffect_Impl::SetInput(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&input), core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>(), SetInput: SetInput::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionGaussianBlurEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionGaussianBlurEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionGaussianBlurEffect_Vtbl
    where
        Identity: IDCompositionGaussianBlurEffect_Impl,
    {
        unsafe extern "system" fn SetStandardDeviation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionGaussianBlurEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionGaussianBlurEffect_Impl::SetStandardDeviation(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionGaussianBlurEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionGaussianBlurEffect_Impl::SetStandardDeviation2(this, core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBorderMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionGaussianBlurEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionGaussianBlurEffect_Impl::SetBorderMode(this, core::mem::transmute_copy(&mode)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Identity, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionGaussianBlurEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionHueRotationEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetAngle(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngle2(&self, amountdegrees: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionHueRotationEffect {}
impl IDCompositionHueRotationEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionHueRotationEffect_Vtbl
    where
        Identity: IDCompositionHueRotationEffect_Impl,
    {
        unsafe extern "system" fn SetAngle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionHueRotationEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionHueRotationEffect_Impl::SetAngle(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amountdegrees: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionHueRotationEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionHueRotationEffect_Impl::SetAngle2(this, core::mem::transmute_copy(&amountdegrees)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetAngle: SetAngle::<Identity, OFFSET>,
            SetAngle2: SetAngle2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionHueRotationEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionInkTrailDevice_Impl: Sized {
    fn CreateDelegatedInkTrail(&self) -> windows_core::Result<IDCompositionDelegatedInkTrail>;
    fn CreateDelegatedInkTrailForSwapChain(&self, swapchain: Option<&windows_core::IUnknown>) -> windows_core::Result<IDCompositionDelegatedInkTrail>;
}
impl windows_core::RuntimeName for IDCompositionInkTrailDevice {}
impl IDCompositionInkTrailDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionInkTrailDevice_Vtbl
    where
        Identity: IDCompositionInkTrailDevice_Impl,
    {
        unsafe extern "system" fn CreateDelegatedInkTrail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inktrail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionInkTrailDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionInkTrailDevice_Impl::CreateDelegatedInkTrail(this) {
                Ok(ok__) => {
                    inktrail.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void, inktrail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionInkTrailDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionInkTrailDevice_Impl::CreateDelegatedInkTrailForSwapChain(this, windows_core::from_raw_borrowed(&swapchain)) {
                Ok(ok__) => {
                    inktrail.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDelegatedInkTrail: CreateDelegatedInkTrail::<Identity, OFFSET>,
            CreateDelegatedInkTrailForSwapChain: CreateDelegatedInkTrailForSwapChain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionInkTrailDevice as windows_core::Interface>::IID
    }
}
pub trait IDCompositionLinearTransferEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetRedYIntercept(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRedYIntercept2(&self, redyintercept: f32) -> windows_core::Result<()>;
    fn SetRedSlope(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRedSlope2(&self, redslope: f32) -> windows_core::Result<()>;
    fn SetRedDisable(&self, reddisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetGreenYIntercept(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreenYIntercept2(&self, greenyintercept: f32) -> windows_core::Result<()>;
    fn SetGreenSlope(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreenSlope2(&self, greenslope: f32) -> windows_core::Result<()>;
    fn SetGreenDisable(&self, greendisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBlueYIntercept(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlueYIntercept2(&self, blueyintercept: f32) -> windows_core::Result<()>;
    fn SetBlueSlope(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlueSlope2(&self, blueslope: f32) -> windows_core::Result<()>;
    fn SetBlueDisable(&self, bluedisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetAlphaYIntercept(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> windows_core::Result<()>;
    fn SetAlphaSlope(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlphaSlope2(&self, alphaslope: f32) -> windows_core::Result<()>;
    fn SetAlphaDisable(&self, alphadisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clampoutput: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionLinearTransferEffect {}
impl IDCompositionLinearTransferEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionLinearTransferEffect_Vtbl
    where
        Identity: IDCompositionLinearTransferEffect_Impl,
    {
        unsafe extern "system" fn SetRedYIntercept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetRedYIntercept(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetRedYIntercept2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, redyintercept: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetRedYIntercept2(this, core::mem::transmute_copy(&redyintercept)).into()
        }
        unsafe extern "system" fn SetRedSlope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetRedSlope(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetRedSlope2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, redslope: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetRedSlope2(this, core::mem::transmute_copy(&redslope)).into()
        }
        unsafe extern "system" fn SetRedDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetRedDisable(this, core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetGreenYIntercept(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetGreenYIntercept2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, greenyintercept: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetGreenYIntercept2(this, core::mem::transmute_copy(&greenyintercept)).into()
        }
        unsafe extern "system" fn SetGreenSlope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetGreenSlope(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetGreenSlope2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, greenslope: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetGreenSlope2(this, core::mem::transmute_copy(&greenslope)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetGreenDisable(this, core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetBlueYIntercept(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBlueYIntercept2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blueyintercept: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetBlueYIntercept2(this, core::mem::transmute_copy(&blueyintercept)).into()
        }
        unsafe extern "system" fn SetBlueSlope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetBlueSlope(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBlueSlope2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blueslope: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetBlueSlope2(this, core::mem::transmute_copy(&blueslope)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetBlueDisable(this, core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetAlphaYIntercept(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaYIntercept2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphayintercept: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetAlphaYIntercept2(this, core::mem::transmute_copy(&alphayintercept)).into()
        }
        unsafe extern "system" fn SetAlphaSlope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetAlphaSlope(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaSlope2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphaslope: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetAlphaSlope2(this, core::mem::transmute_copy(&alphaslope)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetAlphaDisable(this, core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionLinearTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionLinearTransferEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clampoutput)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetRedYIntercept: SetRedYIntercept::<Identity, OFFSET>,
            SetRedYIntercept2: SetRedYIntercept2::<Identity, OFFSET>,
            SetRedSlope: SetRedSlope::<Identity, OFFSET>,
            SetRedSlope2: SetRedSlope2::<Identity, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Identity, OFFSET>,
            SetGreenYIntercept2: SetGreenYIntercept2::<Identity, OFFSET>,
            SetGreenSlope: SetGreenSlope::<Identity, OFFSET>,
            SetGreenSlope2: SetGreenSlope2::<Identity, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Identity, OFFSET>,
            SetBlueYIntercept2: SetBlueYIntercept2::<Identity, OFFSET>,
            SetBlueSlope: SetBlueSlope::<Identity, OFFSET>,
            SetBlueSlope2: SetBlueSlope2::<Identity, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Identity, OFFSET>,
            SetAlphaYIntercept2: SetAlphaYIntercept2::<Identity, OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Identity, OFFSET>,
            SetAlphaSlope2: SetAlphaSlope2::<Identity, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionLinearTransferEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransform_Impl: Sized + IDCompositionTransform_Impl {
    fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeName for IDCompositionMatrixTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl IDCompositionMatrixTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionMatrixTransform_Vtbl
    where
        Identity: IDCompositionMatrixTransform_Impl,
    {
        unsafe extern "system" fn SetMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT
        where
            Identity: IDCompositionMatrixTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionMatrixTransform_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionMatrixTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionMatrixTransform_Impl::SetMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionMatrixTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionMatrixTransform_Impl::SetMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransform3D_Impl: Sized + IDCompositionTransform3D_Impl {
    fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetMatrixElement(&self, row: i32, column: i32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeName for IDCompositionMatrixTransform3D {}
#[cfg(feature = "Foundation_Numerics")]
impl IDCompositionMatrixTransform3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionMatrixTransform3D_Vtbl
    where
        Identity: IDCompositionMatrixTransform3D_Impl,
    {
        unsafe extern "system" fn SetMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT
        where
            Identity: IDCompositionMatrixTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionMatrixTransform3D_Impl::SetMatrix(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetMatrixElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionMatrixTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionMatrixTransform3D_Impl::SetMatrixElement(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionMatrixTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionMatrixTransform3D_Impl::SetMatrixElement2(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetMatrix: SetMatrix::<Identity, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionMatrixTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
pub trait IDCompositionRectangleClip_Impl: Sized + IDCompositionClip_Impl {
    fn SetLeft(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetLeft2(&self, left: f32) -> windows_core::Result<()>;
    fn SetTop(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTop2(&self, top: f32) -> windows_core::Result<()>;
    fn SetRight(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRight2(&self, right: f32) -> windows_core::Result<()>;
    fn SetBottom(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottom2(&self, bottom: f32) -> windows_core::Result<()>;
    fn SetTopLeftRadiusX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetTopLeftRadiusY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetTopRightRadiusX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopRightRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetTopRightRadiusY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetTopRightRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomLeftRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomRightRadiusX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomRightRadiusX2(&self, radius: f32) -> windows_core::Result<()>;
    fn SetBottomRightRadiusY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBottomRightRadiusY2(&self, radius: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionRectangleClip {}
impl IDCompositionRectangleClip_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionRectangleClip_Vtbl
    where
        Identity: IDCompositionRectangleClip_Impl,
    {
        unsafe extern "system" fn SetLeft<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetLeft(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetLeft2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetLeft2(this, core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn SetTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTop(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetTop2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTop2(this, core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn SetRight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetRight(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetRight2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetRight2(this, core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetBottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottom(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBottom2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottom2(this, core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopLeftRadiusX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopLeftRadiusX2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopLeftRadiusY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetTopLeftRadiusY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopLeftRadiusY2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopRightRadiusX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopRightRadiusX2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopRightRadiusY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetTopRightRadiusY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetTopRightRadiusY2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomLeftRadiusX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomLeftRadiusX2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomLeftRadiusY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBottomLeftRadiusY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomLeftRadiusY2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomRightRadiusX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomRightRadiusX2(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomRightRadiusY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBottomRightRadiusY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRectangleClip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRectangleClip_Impl::SetBottomRightRadiusY2(this, core::mem::transmute_copy(&radius)).into()
        }
        Self {
            base__: IDCompositionClip_Vtbl::new::<Identity, OFFSET>(),
            SetLeft: SetLeft::<Identity, OFFSET>,
            SetLeft2: SetLeft2::<Identity, OFFSET>,
            SetTop: SetTop::<Identity, OFFSET>,
            SetTop2: SetTop2::<Identity, OFFSET>,
            SetRight: SetRight::<Identity, OFFSET>,
            SetRight2: SetRight2::<Identity, OFFSET>,
            SetBottom: SetBottom::<Identity, OFFSET>,
            SetBottom2: SetBottom2::<Identity, OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Identity, OFFSET>,
            SetTopLeftRadiusX2: SetTopLeftRadiusX2::<Identity, OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Identity, OFFSET>,
            SetTopLeftRadiusY2: SetTopLeftRadiusY2::<Identity, OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Identity, OFFSET>,
            SetTopRightRadiusX2: SetTopRightRadiusX2::<Identity, OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Identity, OFFSET>,
            SetTopRightRadiusY2: SetTopRightRadiusY2::<Identity, OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Identity, OFFSET>,
            SetBottomLeftRadiusX2: SetBottomLeftRadiusX2::<Identity, OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Identity, OFFSET>,
            SetBottomLeftRadiusY2: SetBottomLeftRadiusY2::<Identity, OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Identity, OFFSET>,
            SetBottomRightRadiusX2: SetBottomRightRadiusX2::<Identity, OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Identity, OFFSET>,
            SetBottomRightRadiusY2: SetBottomRightRadiusY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionRectangleClip as windows_core::Interface>::IID || iid == &<IDCompositionClip as windows_core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform_Impl: Sized + IDCompositionTransform_Impl {
    fn SetAngle(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngle2(&self, angle: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionRotateTransform {}
impl IDCompositionRotateTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionRotateTransform_Vtbl
    where
        Identity: IDCompositionRotateTransform_Impl,
    {
        unsafe extern "system" fn SetAngle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform_Impl::SetAngle(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, angle: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform_Impl::SetAngle2(this, core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform_Impl::SetCenterX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform_Impl::SetCenterY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetAngle: SetAngle::<Identity, OFFSET>,
            SetAngle2: SetAngle2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
pub trait IDCompositionRotateTransform3D_Impl: Sized + IDCompositionTransform3D_Impl {
    fn SetAngle(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngle2(&self, angle: f32) -> windows_core::Result<()>;
    fn SetAxisX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAxisX2(&self, axisx: f32) -> windows_core::Result<()>;
    fn SetAxisY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAxisY2(&self, axisy: f32) -> windows_core::Result<()>;
    fn SetAxisZ(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAxisZ2(&self, axisz: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
    fn SetCenterZ(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionRotateTransform3D {}
impl IDCompositionRotateTransform3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionRotateTransform3D_Vtbl
    where
        Identity: IDCompositionRotateTransform3D_Impl,
    {
        unsafe extern "system" fn SetAngle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAngle(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAngle2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, angle: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAngle2(this, core::mem::transmute_copy(&angle)).into()
        }
        unsafe extern "system" fn SetAxisX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAxisX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAxisX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAxisX2(this, core::mem::transmute_copy(&axisx)).into()
        }
        unsafe extern "system" fn SetAxisY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAxisY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAxisY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisy: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAxisY2(this, core::mem::transmute_copy(&axisy)).into()
        }
        unsafe extern "system" fn SetAxisZ<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAxisZ(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAxisZ2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisz: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetAxisZ2(this, core::mem::transmute_copy(&axisz)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetCenterX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetCenterY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetCenterZ(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerz: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionRotateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionRotateTransform3D_Impl::SetCenterZ2(this, core::mem::transmute_copy(&centerz)).into()
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetAngle: SetAngle::<Identity, OFFSET>,
            SetAngle2: SetAngle2::<Identity, OFFSET>,
            SetAxisX: SetAxisX::<Identity, OFFSET>,
            SetAxisX2: SetAxisX2::<Identity, OFFSET>,
            SetAxisY: SetAxisY::<Identity, OFFSET>,
            SetAxisY2: SetAxisY2::<Identity, OFFSET>,
            SetAxisZ: SetAxisZ::<Identity, OFFSET>,
            SetAxisZ2: SetAxisZ2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionRotateTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
pub trait IDCompositionSaturationEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetSaturation(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetSaturation2(&self, ratio: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionSaturationEffect {}
impl IDCompositionSaturationEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionSaturationEffect_Vtbl
    where
        Identity: IDCompositionSaturationEffect_Impl,
    {
        unsafe extern "system" fn SetSaturation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSaturationEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSaturationEffect_Impl::SetSaturation(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetSaturation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ratio: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionSaturationEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSaturationEffect_Impl::SetSaturation2(this, core::mem::transmute_copy(&ratio)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetSaturation: SetSaturation::<Identity, OFFSET>,
            SetSaturation2: SetSaturation2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSaturationEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform_Impl: Sized + IDCompositionTransform_Impl {
    fn SetScaleX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()>;
    fn SetScaleY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionScaleTransform {}
impl IDCompositionScaleTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionScaleTransform_Vtbl
    where
        Identity: IDCompositionScaleTransform_Impl,
    {
        unsafe extern "system" fn SetScaleX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetScaleX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalex: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetScaleX2(this, core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetScaleY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaley: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetScaleY2(this, core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetCenterX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetCenterY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetScaleX: SetScaleX::<Identity, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, OFFSET>,
            SetScaleY: SetScaleY::<Identity, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
pub trait IDCompositionScaleTransform3D_Impl: Sized + IDCompositionTransform3D_Impl {
    fn SetScaleX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleX2(&self, scalex: f32) -> windows_core::Result<()>;
    fn SetScaleY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleY2(&self, scaley: f32) -> windows_core::Result<()>;
    fn SetScaleZ(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetScaleZ2(&self, scalez: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
    fn SetCenterZ(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterZ2(&self, centerz: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionScaleTransform3D {}
impl IDCompositionScaleTransform3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionScaleTransform3D_Vtbl
    where
        Identity: IDCompositionScaleTransform3D_Impl,
    {
        unsafe extern "system" fn SetScaleX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetScaleX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetScaleX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalex: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetScaleX2(this, core::mem::transmute_copy(&scalex)).into()
        }
        unsafe extern "system" fn SetScaleY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetScaleY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetScaleY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaley: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetScaleY2(this, core::mem::transmute_copy(&scaley)).into()
        }
        unsafe extern "system" fn SetScaleZ<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetScaleZ(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetScaleZ2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalez: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetScaleZ2(this, core::mem::transmute_copy(&scalez)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetCenterX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetCenterY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
        }
        unsafe extern "system" fn SetCenterZ<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetCenterZ(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterZ2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerz: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionScaleTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionScaleTransform3D_Impl::SetCenterZ2(this, core::mem::transmute_copy(&centerz)).into()
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetScaleX: SetScaleX::<Identity, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, OFFSET>,
            SetScaleY: SetScaleY::<Identity, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, OFFSET>,
            SetScaleZ: SetScaleZ::<Identity, OFFSET>,
            SetScaleZ2: SetScaleZ2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionScaleTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionShadowEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetStandardDeviation2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::Result<()>;
    fn SetRed(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRed2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetGreen(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreen2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetBlue(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlue2(&self, amount: f32) -> windows_core::Result<()>;
    fn SetAlpha(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlpha2(&self, amount: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionShadowEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionShadowEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionShadowEffect_Vtbl
    where
        Identity: IDCompositionShadowEffect_Impl,
    {
        unsafe extern "system" fn SetStandardDeviation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetStandardDeviation(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetStandardDeviation2(this, core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetRed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetRed(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetRed2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetRed2(this, core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetGreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetGreen(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetGreen2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetGreen2(this, core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetBlue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetBlue(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBlue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetBlue2(this, core::mem::transmute_copy(&amount)).into()
        }
        unsafe extern "system" fn SetAlpha<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetAlpha(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAlpha2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, amount: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionShadowEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionShadowEffect_Impl::SetAlpha2(this, core::mem::transmute_copy(&amount)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetStandardDeviation: SetStandardDeviation::<Identity, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            SetRed: SetRed::<Identity, OFFSET>,
            SetRed2: SetRed2::<Identity, OFFSET>,
            SetGreen: SetGreen::<Identity, OFFSET>,
            SetGreen2: SetGreen2::<Identity, OFFSET>,
            SetBlue: SetBlue::<Identity, OFFSET>,
            SetBlue2: SetBlue2::<Identity, OFFSET>,
            SetAlpha: SetAlpha::<Identity, OFFSET>,
            SetAlpha2: SetAlpha2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionShadowEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionSkewTransform_Impl: Sized + IDCompositionTransform_Impl {
    fn SetAngleX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngleX2(&self, anglex: f32) -> windows_core::Result<()>;
    fn SetAngleY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAngleY2(&self, angley: f32) -> windows_core::Result<()>;
    fn SetCenterX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterX2(&self, centerx: f32) -> windows_core::Result<()>;
    fn SetCenterY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetCenterY2(&self, centery: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionSkewTransform {}
impl IDCompositionSkewTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionSkewTransform_Vtbl
    where
        Identity: IDCompositionSkewTransform_Impl,
    {
        unsafe extern "system" fn SetAngleX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetAngleX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAngleX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, anglex: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetAngleX2(this, core::mem::transmute_copy(&anglex)).into()
        }
        unsafe extern "system" fn SetAngleY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetAngleY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAngleY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, angley: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetAngleY2(this, core::mem::transmute_copy(&angley)).into()
        }
        unsafe extern "system" fn SetCenterX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetCenterX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetCenterX2(this, core::mem::transmute_copy(&centerx)).into()
        }
        unsafe extern "system" fn SetCenterY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetCenterY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetCenterY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, centery: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionSkewTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSkewTransform_Impl::SetCenterY2(this, core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetAngleX: SetAngleX::<Identity, OFFSET>,
            SetAngleX2: SetAngleX2::<Identity, OFFSET>,
            SetAngleY: SetAngleY::<Identity, OFFSET>,
            SetAngleY2: SetAngleY2::<Identity, OFFSET>,
            SetCenterX: SetCenterX::<Identity, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, OFFSET>,
            SetCenterY: SetCenterY::<Identity, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSkewTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
pub trait IDCompositionSurface_Impl: Sized {
    fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
    fn SuspendDraw(&self) -> windows_core::Result<()>;
    fn ResumeDraw(&self) -> windows_core::Result<()>;
    fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionSurface {}
impl IDCompositionSurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionSurface_Vtbl
    where
        Identity: IDCompositionSurface_Impl,
    {
        unsafe extern "system" fn BeginDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const windows_core::GUID, updateobject: *mut *mut core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSurface_Impl::BeginDraw(this, core::mem::transmute_copy(&updaterect), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&updateobject), core::mem::transmute_copy(&updateoffset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSurface_Impl::EndDraw(this).into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSurface_Impl::SuspendDraw(this).into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSurface_Impl::ResumeDraw(this).into()
        }
        unsafe extern "system" fn Scroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionSurface_Impl::Scroll(this, core::mem::transmute_copy(&scrollrect), core::mem::transmute_copy(&cliprect), core::mem::transmute_copy(&offsetx), core::mem::transmute_copy(&offsety)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSurface as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionSurfaceFactory_Impl: Sized {
    fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<IDCompositionVirtualSurface>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDCompositionSurfaceFactory {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDCompositionSurfaceFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionSurfaceFactory_Vtbl
    where
        Identity: IDCompositionSurfaceFactory_Impl,
    {
        unsafe extern "system" fn CreateSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurfaceFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionSurfaceFactory_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    surface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionSurfaceFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDCompositionSurfaceFactory_Impl::CreateVirtualSurface(this, core::mem::transmute_copy(&initialwidth), core::mem::transmute_copy(&initialheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&alphamode)) {
                Ok(ok__) => {
                    virtualsurface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionSurfaceFactory as windows_core::Interface>::IID
    }
}
pub trait IDCompositionTableTransferEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetRedTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetGreenTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetBlueTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetAlphaTable(&self, tablevalues: *const f32, count: u32) -> windows_core::Result<()>;
    fn SetRedDisable(&self, reddisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetGreenDisable(&self, greendisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBlueDisable(&self, bluedisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetAlphaDisable(&self, alphadisable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetClampOutput(&self, clampoutput: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetRedTableValue(&self, index: u32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetRedTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
    fn SetGreenTableValue(&self, index: u32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetGreenTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
    fn SetBlueTableValue(&self, index: u32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetBlueTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
    fn SetAlphaTableValue(&self, index: u32, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetAlphaTableValue2(&self, index: u32, value: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionTableTransferEffect {}
impl IDCompositionTableTransferEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTableTransferEffect_Vtbl
    where
        Identity: IDCompositionTableTransferEffect_Impl,
    {
        unsafe extern "system" fn SetRedTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetRedTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetGreenTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetGreenTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBlueTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetBlueTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetAlphaTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablevalues: *const f32, count: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetAlphaTable(this, core::mem::transmute_copy(&tablevalues), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetRedDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetRedDisable(this, core::mem::transmute_copy(&reddisable)).into()
        }
        unsafe extern "system" fn SetGreenDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetGreenDisable(this, core::mem::transmute_copy(&greendisable)).into()
        }
        unsafe extern "system" fn SetBlueDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetBlueDisable(this, core::mem::transmute_copy(&bluedisable)).into()
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetAlphaDisable(this, core::mem::transmute_copy(&alphadisable)).into()
        }
        unsafe extern "system" fn SetClampOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetClampOutput(this, core::mem::transmute_copy(&clampoutput)).into()
        }
        unsafe extern "system" fn SetRedTableValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetRedTableValue(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetRedTableValue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetRedTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetGreenTableValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetGreenTableValue(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetGreenTableValue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetGreenTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetBlueTableValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetBlueTableValue(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetBlueTableValue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetBlueTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetAlphaTableValue(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetAlphaTableValue2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTableTransferEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTableTransferEffect_Impl::SetAlphaTableValue2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetRedTable: SetRedTable::<Identity, OFFSET>,
            SetGreenTable: SetGreenTable::<Identity, OFFSET>,
            SetBlueTable: SetBlueTable::<Identity, OFFSET>,
            SetAlphaTable: SetAlphaTable::<Identity, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, OFFSET>,
            SetRedTableValue: SetRedTableValue::<Identity, OFFSET>,
            SetRedTableValue2: SetRedTableValue2::<Identity, OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Identity, OFFSET>,
            SetGreenTableValue2: SetGreenTableValue2::<Identity, OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Identity, OFFSET>,
            SetBlueTableValue2: SetBlueTableValue2::<Identity, OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Identity, OFFSET>,
            SetAlphaTableValue2: SetAlphaTableValue2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTableTransferEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionTarget_Impl: Sized {
    fn SetRoot(&self, visual: Option<&IDCompositionVisual>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionTarget {}
impl IDCompositionTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTarget_Vtbl
    where
        Identity: IDCompositionTarget_Impl,
    {
        unsafe extern "system" fn SetRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTarget_Impl::SetRoot(this, windows_core::from_raw_borrowed(&visual)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRoot: SetRoot::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionTexture_Impl: Sized {
    fn SetSourceRect(&self, sourcerect: *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()>;
    fn GetAvailableFence(&self, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for IDCompositionTexture {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDCompositionTexture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTexture_Vtbl
    where
        Identity: IDCompositionTexture_Impl,
    {
        unsafe extern "system" fn SetSourceRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerect: *const super::Direct2D::Common::D2D_RECT_U) -> windows_core::HRESULT
        where
            Identity: IDCompositionTexture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTexture_Impl::SetSourceRect(this, core::mem::transmute_copy(&sourcerect)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT
        where
            Identity: IDCompositionTexture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTexture_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionTexture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTexture_Impl::SetAlphaMode(this, core::mem::transmute_copy(&alphamode)).into()
        }
        unsafe extern "system" fn GetAvailableFence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fencevalue: *mut u64, iid: *const windows_core::GUID, availablefence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTexture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTexture_Impl::GetAvailableFence(this, core::mem::transmute_copy(&fencevalue), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&availablefence)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, OFFSET>,
            GetAvailableFence: GetAvailableFence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTexture as windows_core::Interface>::IID
    }
}
pub trait IDCompositionTransform_Impl: Sized + IDCompositionTransform3D_Impl {}
impl windows_core::RuntimeName for IDCompositionTransform {}
impl IDCompositionTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTransform_Vtbl
    where
        Identity: IDCompositionTransform_Impl,
    {
        Self { base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
pub trait IDCompositionTransform3D_Impl: Sized + IDCompositionEffect_Impl {}
impl windows_core::RuntimeName for IDCompositionTransform3D {}
impl IDCompositionTransform3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTransform3D_Vtbl
    where
        Identity: IDCompositionTransform3D_Impl,
    {
        Self { base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform_Impl: Sized + IDCompositionTransform_Impl {
    fn SetOffsetX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionTranslateTransform {}
impl IDCompositionTranslateTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTranslateTransform_Vtbl
    where
        Identity: IDCompositionTranslateTransform_Impl,
    {
        unsafe extern "system" fn SetOffsetX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform_Impl::SetOffsetX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform_Impl::SetOffsetY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsety: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety)).into()
        }
        Self {
            base__: IDCompositionTransform_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionTransform as windows_core::Interface>::IID
    }
}
pub trait IDCompositionTranslateTransform3D_Impl: Sized + IDCompositionTransform3D_Impl {
    fn SetOffsetX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
    fn SetOffsetZ(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionTranslateTransform3D {}
impl IDCompositionTranslateTransform3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTranslateTransform3D_Vtbl
    where
        Identity: IDCompositionTranslateTransform3D_Impl,
    {
        unsafe extern "system" fn SetOffsetX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform3D_Impl::SetOffsetX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform3D_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform3D_Impl::SetOffsetY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsety: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform3D_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform3D_Impl::SetOffsetZ(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetz: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTranslateTransform3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTranslateTransform3D_Impl::SetOffsetZ2(this, core::mem::transmute_copy(&offsetz)).into()
        }
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTranslateTransform3D as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionTurbulenceEffect_Impl: Sized + IDCompositionFilterEffect_Impl {
    fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()>;
    fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()>;
    fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::Result<()>;
    fn SetNumOctaves(&self, numoctaves: u32) -> windows_core::Result<()>;
    fn SetSeed(&self, seed: u32) -> windows_core::Result<()>;
    fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::Result<()>;
    fn SetStitchable(&self, stitchable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDCompositionTurbulenceEffect {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDCompositionTurbulenceEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionTurbulenceEffect_Vtbl
    where
        Identity: IDCompositionTurbulenceEffect_Impl,
    {
        unsafe extern "system" fn SetOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetOffset(this, core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetBaseFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetBaseFrequency(this, core::mem::transmute_copy(&frequency)).into()
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetSize(this, core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SetNumOctaves<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numoctaves: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetNumOctaves(this, core::mem::transmute_copy(&numoctaves)).into()
        }
        unsafe extern "system" fn SetSeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, seed: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetSeed(this, core::mem::transmute_copy(&seed)).into()
        }
        unsafe extern "system" fn SetNoise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetNoise(this, core::mem::transmute_copy(&noise)).into()
        }
        unsafe extern "system" fn SetStitchable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionTurbulenceEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionTurbulenceEffect_Impl::SetStitchable(this, core::mem::transmute_copy(&stitchable)).into()
        }
        Self {
            base__: IDCompositionFilterEffect_Vtbl::new::<Identity, OFFSET>(),
            SetOffset: SetOffset::<Identity, OFFSET>,
            SetBaseFrequency: SetBaseFrequency::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SetNumOctaves: SetNumOctaves::<Identity, OFFSET>,
            SetSeed: SetSeed::<Identity, OFFSET>,
            SetNoise: SetNoise::<Identity, OFFSET>,
            SetStitchable: SetStitchable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTurbulenceEffect as windows_core::Interface>::IID || iid == &<IDCompositionEffect as windows_core::Interface>::IID || iid == &<IDCompositionFilterEffect as windows_core::Interface>::IID
    }
}
pub trait IDCompositionVirtualSurface_Impl: Sized + IDCompositionSurface_Impl {
    fn Resize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn Trim(&self, rectangles: *const super::super::Foundation::RECT, count: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCompositionVirtualSurface {}
impl IDCompositionVirtualSurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionVirtualSurface_Vtbl
    where
        Identity: IDCompositionVirtualSurface_Impl,
    {
        unsafe extern "system" fn Resize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionVirtualSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVirtualSurface_Impl::Resize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Trim<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> windows_core::HRESULT
        where
            Identity: IDCompositionVirtualSurface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVirtualSurface_Impl::Trim(this, core::mem::transmute_copy(&rectangles), core::mem::transmute_copy(&count)).into()
        }
        Self { base__: IDCompositionSurface_Vtbl::new::<Identity, OFFSET>(), Resize: Resize::<Identity, OFFSET>, Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVirtualSurface as windows_core::Interface>::IID || iid == &<IDCompositionSurface as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual_Impl: Sized {
    fn SetOffsetX(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: Option<&IDCompositionTransform>) -> windows_core::Result<()>;
    fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()>;
    fn SetTransformParent(&self, visual: Option<&IDCompositionVisual>) -> windows_core::Result<()>;
    fn SetEffect(&self, effect: Option<&IDCompositionEffect>) -> windows_core::Result<()>;
    fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::Result<()>;
    fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::Result<()>;
    fn SetClip(&self, clip: Option<&IDCompositionClip>) -> windows_core::Result<()>;
    fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::Result<()>;
    fn SetContent(&self, content: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddVisual(&self, visual: Option<&IDCompositionVisual>, insertabove: super::super::Foundation::BOOL, referencevisual: Option<&IDCompositionVisual>) -> windows_core::Result<()>;
    fn RemoveVisual(&self, visual: Option<&IDCompositionVisual>) -> windows_core::Result<()>;
    fn RemoveAllVisuals(&self) -> windows_core::Result<()>;
    fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for IDCompositionVisual {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionVisual_Vtbl
    where
        Identity: IDCompositionVisual_Impl,
    {
        unsafe extern "system" fn SetOffsetX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetOffsetX(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetX2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetx: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx)).into()
        }
        unsafe extern "system" fn SetOffsetY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetOffsetY(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetY2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsety: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetTransform(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetTransform2(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetTransformParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetTransformParent(this, windows_core::from_raw_borrowed(&visual)).into()
        }
        unsafe extern "system" fn SetEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetEffect(this, windows_core::from_raw_borrowed(&effect)).into()
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetBitmapInterpolationMode(this, core::mem::transmute_copy(&interpolationmode)).into()
        }
        unsafe extern "system" fn SetBorderMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetBorderMode(this, core::mem::transmute_copy(&bordermode)).into()
        }
        unsafe extern "system" fn SetClip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clip: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetClip(this, windows_core::from_raw_borrowed(&clip)).into()
        }
        unsafe extern "system" fn SetClip2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetClip2(this, core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn SetContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetContent(this, windows_core::from_raw_borrowed(&content)).into()
        }
        unsafe extern "system" fn AddVisual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void, insertabove: super::super::Foundation::BOOL, referencevisual: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::AddVisual(this, windows_core::from_raw_borrowed(&visual), core::mem::transmute_copy(&insertabove), windows_core::from_raw_borrowed(&referencevisual)).into()
        }
        unsafe extern "system" fn RemoveVisual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visual: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::RemoveVisual(this, windows_core::from_raw_borrowed(&visual)).into()
        }
        unsafe extern "system" fn RemoveAllVisuals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::RemoveAllVisuals(this).into()
        }
        unsafe extern "system" fn SetCompositeMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual_Impl::SetCompositeMode(this, core::mem::transmute_copy(&compositemode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetTransform2: SetTransform2::<Identity, OFFSET>,
            SetTransformParent: SetTransformParent::<Identity, OFFSET>,
            SetEffect: SetEffect::<Identity, OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
            SetClip: SetClip::<Identity, OFFSET>,
            SetClip2: SetClip2::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            AddVisual: AddVisual::<Identity, OFFSET>,
            RemoveVisual: RemoveVisual::<Identity, OFFSET>,
            RemoveAllVisuals: RemoveAllVisuals::<Identity, OFFSET>,
            SetCompositeMode: SetCompositeMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual2_Impl: Sized + IDCompositionVisual_Impl {
    fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> windows_core::Result<()>;
    fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for IDCompositionVisual2 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionVisual2_Vtbl
    where
        Identity: IDCompositionVisual2_Impl,
    {
        unsafe extern "system" fn SetOpacityMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual2_Impl::SetOpacityMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetBackFaceVisibility<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual2_Impl::SetBackFaceVisibility(this, core::mem::transmute_copy(&visibility)).into()
        }
        Self {
            base__: IDCompositionVisual_Vtbl::new::<Identity, OFFSET>(),
            SetOpacityMode: SetOpacityMode::<Identity, OFFSET>,
            SetBackFaceVisibility: SetBackFaceVisibility::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual2 as windows_core::Interface>::IID || iid == &<IDCompositionVisual as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual3_Impl: Sized + IDCompositionVisualDebug_Impl {
    fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> windows_core::Result<()>;
    fn SetOffsetZ(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOffsetZ2(&self, offsetz: f32) -> windows_core::Result<()>;
    fn SetOpacity(&self, animation: Option<&IDCompositionAnimation>) -> windows_core::Result<()>;
    fn SetOpacity2(&self, opacity: f32) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: Option<&IDCompositionTransform3D>) -> windows_core::Result<()>;
    fn SetTransform2(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> windows_core::Result<()>;
    fn SetVisible(&self, visible: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for IDCompositionVisual3 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisual3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionVisual3_Vtbl
    where
        Identity: IDCompositionVisual3_Impl,
    {
        unsafe extern "system" fn SetDepthMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetDepthMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetOffsetZ<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetOffsetZ(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetz: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetOffsetZ2(this, core::mem::transmute_copy(&offsetz)).into()
        }
        unsafe extern "system" fn SetOpacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetOpacity(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn SetOpacity2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetOpacity2(this, core::mem::transmute_copy(&opacity)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetTransform(this, windows_core::from_raw_borrowed(&transform)).into()
        }
        unsafe extern "system" fn SetTransform2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetTransform2(this, core::mem::transmute_copy(&matrix)).into()
        }
        unsafe extern "system" fn SetVisible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisual3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisual3_Impl::SetVisible(this, core::mem::transmute_copy(&visible)).into()
        }
        Self {
            base__: IDCompositionVisualDebug_Vtbl::new::<Identity, OFFSET>(),
            SetDepthMode: SetDepthMode::<Identity, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, OFFSET>,
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetTransform2: SetTransform2::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual3 as windows_core::Interface>::IID || iid == &<IDCompositionVisual as windows_core::Interface>::IID || iid == &<IDCompositionVisual2 as windows_core::Interface>::IID || iid == &<IDCompositionVisualDebug as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisualDebug_Impl: Sized + IDCompositionVisual2_Impl {
    fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::Result<()>;
    fn DisableHeatMap(&self) -> windows_core::Result<()>;
    fn EnableRedrawRegions(&self) -> windows_core::Result<()>;
    fn DisableRedrawRegions(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl windows_core::RuntimeName for IDCompositionVisualDebug {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl IDCompositionVisualDebug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCompositionVisualDebug_Vtbl
    where
        Identity: IDCompositionVisualDebug_Impl,
    {
        unsafe extern "system" fn EnableHeatMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisualDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisualDebug_Impl::EnableHeatMap(this, core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn DisableHeatMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisualDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisualDebug_Impl::DisableHeatMap(this).into()
        }
        unsafe extern "system" fn EnableRedrawRegions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisualDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisualDebug_Impl::EnableRedrawRegions(this).into()
        }
        unsafe extern "system" fn DisableRedrawRegions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDCompositionVisualDebug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCompositionVisualDebug_Impl::DisableRedrawRegions(this).into()
        }
        Self {
            base__: IDCompositionVisual2_Vtbl::new::<Identity, OFFSET>(),
            EnableHeatMap: EnableHeatMap::<Identity, OFFSET>,
            DisableHeatMap: DisableHeatMap::<Identity, OFFSET>,
            EnableRedrawRegions: EnableRedrawRegions::<Identity, OFFSET>,
            DisableRedrawRegions: DisableRedrawRegions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisualDebug as windows_core::Interface>::IID || iid == &<IDCompositionVisual as windows_core::Interface>::IID || iid == &<IDCompositionVisual2 as windows_core::Interface>::IID
    }
}

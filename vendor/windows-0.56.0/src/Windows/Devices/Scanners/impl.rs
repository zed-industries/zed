pub trait IImageScannerFormatConfiguration_Impl: Sized {
    fn DefaultFormat(&self) -> windows_core::Result<ImageScannerFormat>;
    fn Format(&self) -> windows_core::Result<ImageScannerFormat>;
    fn SetFormat(&self, value: ImageScannerFormat) -> windows_core::Result<()>;
    fn IsFormatSupported(&self, value: ImageScannerFormat) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IImageScannerFormatConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerFormatConfiguration";
}
impl IImageScannerFormatConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>() -> IImageScannerFormatConfiguration_Vtbl {
        unsafe extern "system" fn DefaultFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerFormat) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerFormatConfiguration_Impl::DefaultFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerFormat) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerFormatConfiguration_Impl::Format(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerFormat) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerFormatConfiguration_Impl::SetFormat(this, value).into()
        }
        unsafe extern "system" fn IsFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerFormatConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerFormat, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerFormatConfiguration_Impl::IsFormatSupported(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IImageScannerFormatConfiguration, OFFSET>(),
            DefaultFormat: DefaultFormat::<Identity, Impl, OFFSET>,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageScannerFormatConfiguration as windows_core::Interface>::IID
    }
}
pub trait IImageScannerSourceConfiguration_Impl: Sized + IImageScannerFormatConfiguration_Impl {
    fn MinScanArea(&self) -> windows_core::Result<super::super::Foundation::Size>;
    fn MaxScanArea(&self) -> windows_core::Result<super::super::Foundation::Size>;
    fn SelectedScanRegion(&self) -> windows_core::Result<super::super::Foundation::Rect>;
    fn SetSelectedScanRegion(&self, value: &super::super::Foundation::Rect) -> windows_core::Result<()>;
    fn AutoCroppingMode(&self) -> windows_core::Result<ImageScannerAutoCroppingMode>;
    fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<()>;
    fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<bool>;
    fn MinResolution(&self) -> windows_core::Result<ImageScannerResolution>;
    fn MaxResolution(&self) -> windows_core::Result<ImageScannerResolution>;
    fn OpticalResolution(&self) -> windows_core::Result<ImageScannerResolution>;
    fn DesiredResolution(&self) -> windows_core::Result<ImageScannerResolution>;
    fn SetDesiredResolution(&self, value: &ImageScannerResolution) -> windows_core::Result<()>;
    fn ActualResolution(&self) -> windows_core::Result<ImageScannerResolution>;
    fn DefaultColorMode(&self) -> windows_core::Result<ImageScannerColorMode>;
    fn ColorMode(&self) -> windows_core::Result<ImageScannerColorMode>;
    fn SetColorMode(&self, value: ImageScannerColorMode) -> windows_core::Result<()>;
    fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> windows_core::Result<bool>;
    fn MinBrightness(&self) -> windows_core::Result<i32>;
    fn MaxBrightness(&self) -> windows_core::Result<i32>;
    fn BrightnessStep(&self) -> windows_core::Result<u32>;
    fn DefaultBrightness(&self) -> windows_core::Result<i32>;
    fn Brightness(&self) -> windows_core::Result<i32>;
    fn SetBrightness(&self, value: i32) -> windows_core::Result<()>;
    fn MinContrast(&self) -> windows_core::Result<i32>;
    fn MaxContrast(&self) -> windows_core::Result<i32>;
    fn ContrastStep(&self) -> windows_core::Result<u32>;
    fn DefaultContrast(&self) -> windows_core::Result<i32>;
    fn Contrast(&self) -> windows_core::Result<i32>;
    fn SetContrast(&self, value: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IImageScannerSourceConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.IImageScannerSourceConfiguration";
}
impl IImageScannerSourceConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>() -> IImageScannerSourceConfiguration_Vtbl {
        unsafe extern "system" fn MinScanArea<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MinScanArea(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScanArea<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MaxScanArea(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedScanRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::SelectedScanRegion(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedScanRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::Rect) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerSourceConfiguration_Impl::SetSelectedScanRegion(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AutoCroppingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerAutoCroppingMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::AutoCroppingMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCroppingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerAutoCroppingMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerSourceConfiguration_Impl::SetAutoCroppingMode(this, value).into()
        }
        unsafe extern "system" fn IsAutoCroppingModeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerAutoCroppingMode, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::IsAutoCroppingModeSupported(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerResolution) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MinResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerResolution) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MaxResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpticalResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerResolution) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::OpticalResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerResolution) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::DesiredResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerResolution) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerSourceConfiguration_Impl::SetDesiredResolution(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ActualResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerResolution) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::ActualResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultColorMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerColorMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::DefaultColorMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ImageScannerColorMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::ColorMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerColorMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerSourceConfiguration_Impl::SetColorMode(this, value).into()
        }
        unsafe extern "system" fn IsColorModeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ImageScannerColorMode, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::IsColorModeSupported(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinBrightness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MinBrightness(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBrightness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MaxBrightness(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrightnessStep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::BrightnessStep(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultBrightness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::DefaultBrightness(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Brightness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::Brightness(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightness<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerSourceConfiguration_Impl::SetBrightness(this, value).into()
        }
        unsafe extern "system" fn MinContrast<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MinContrast(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxContrast<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::MaxContrast(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContrastStep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::ContrastStep(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContrast<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::DefaultContrast(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contrast<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IImageScannerSourceConfiguration_Impl::Contrast(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContrast<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageScannerSourceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IImageScannerSourceConfiguration_Impl::SetContrast(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IImageScannerSourceConfiguration, OFFSET>(),
            MinScanArea: MinScanArea::<Identity, Impl, OFFSET>,
            MaxScanArea: MaxScanArea::<Identity, Impl, OFFSET>,
            SelectedScanRegion: SelectedScanRegion::<Identity, Impl, OFFSET>,
            SetSelectedScanRegion: SetSelectedScanRegion::<Identity, Impl, OFFSET>,
            AutoCroppingMode: AutoCroppingMode::<Identity, Impl, OFFSET>,
            SetAutoCroppingMode: SetAutoCroppingMode::<Identity, Impl, OFFSET>,
            IsAutoCroppingModeSupported: IsAutoCroppingModeSupported::<Identity, Impl, OFFSET>,
            MinResolution: MinResolution::<Identity, Impl, OFFSET>,
            MaxResolution: MaxResolution::<Identity, Impl, OFFSET>,
            OpticalResolution: OpticalResolution::<Identity, Impl, OFFSET>,
            DesiredResolution: DesiredResolution::<Identity, Impl, OFFSET>,
            SetDesiredResolution: SetDesiredResolution::<Identity, Impl, OFFSET>,
            ActualResolution: ActualResolution::<Identity, Impl, OFFSET>,
            DefaultColorMode: DefaultColorMode::<Identity, Impl, OFFSET>,
            ColorMode: ColorMode::<Identity, Impl, OFFSET>,
            SetColorMode: SetColorMode::<Identity, Impl, OFFSET>,
            IsColorModeSupported: IsColorModeSupported::<Identity, Impl, OFFSET>,
            MinBrightness: MinBrightness::<Identity, Impl, OFFSET>,
            MaxBrightness: MaxBrightness::<Identity, Impl, OFFSET>,
            BrightnessStep: BrightnessStep::<Identity, Impl, OFFSET>,
            DefaultBrightness: DefaultBrightness::<Identity, Impl, OFFSET>,
            Brightness: Brightness::<Identity, Impl, OFFSET>,
            SetBrightness: SetBrightness::<Identity, Impl, OFFSET>,
            MinContrast: MinContrast::<Identity, Impl, OFFSET>,
            MaxContrast: MaxContrast::<Identity, Impl, OFFSET>,
            ContrastStep: ContrastStep::<Identity, Impl, OFFSET>,
            DefaultContrast: DefaultContrast::<Identity, Impl, OFFSET>,
            Contrast: Contrast::<Identity, Impl, OFFSET>,
            SetContrast: SetContrast::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageScannerSourceConfiguration as windows_core::Interface>::IID
    }
}

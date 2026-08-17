windows_core::imp::define_interface!(IImageScanner, IImageScanner_Vtbl, 0x53a88f78_5298_48a0_8da3_8087519665e0);
impl windows_core::RuntimeType for IImageScanner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScanner_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultScanSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerScanSource) -> windows_core::HRESULT,
    pub IsScanSourceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerScanSource, *mut bool) -> windows_core::HRESULT,
    pub FlatbedConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FeederConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutoConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPreviewSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerScanSource, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ScanPreviewToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerScanSource, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanPreviewToStreamAsync: usize,
    #[cfg(feature = "Storage")]
    pub ScanFilesToFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerScanSource, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScanFilesToFolderAsync: usize,
}
windows_core::imp::define_interface!(IImageScannerFeederConfiguration, IImageScannerFeederConfiguration_Vtbl, 0x74bdacee_fa97_4c17_8280_40e39c6dcc67);
impl windows_core::RuntimeType for IImageScannerFeederConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScannerFeederConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanAutoDetectPageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutoDetectPageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoDetectPageSize: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub PageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Printing::PrintMediaSize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Printing::PrintMediaSize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageSize: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub PageOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Printing::PrintOrientation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PageOrientation: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub SetPageOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Printing::PrintOrientation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    SetPageOrientation: usize,
    pub PageSizeDimensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub IsPageSizeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Printing::PrintMediaSize, super::super::Graphics::Printing::PrintOrientation, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    IsPageSizeSupported: usize,
    pub MaxNumberOfPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxNumberOfPages: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CanScanDuplex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Duplex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanScanAhead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ScanAhead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetScanAhead: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageScannerFormatConfiguration, IImageScannerFormatConfiguration_Vtbl, 0xae275d11_dadf_4010_bf10_cca5c83dcbb0);
impl core::ops::Deref for IImageScannerFormatConfiguration {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageScannerFormatConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl IImageScannerFormatConfiguration {
    pub fn DefaultFormat(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFormatSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IImageScannerFormatConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScannerFormatConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DefaultFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerFormat) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerFormat) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerFormat) -> windows_core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerFormat, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageScannerPreviewResult, IImageScannerPreviewResult_Vtbl, 0x08b7fe8e_8891_441d_be9c_176fa109c8bb);
impl windows_core::RuntimeType for IImageScannerPreviewResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScannerPreviewResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerFormat) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageScannerScanResult, IImageScannerScanResult_Vtbl, 0xc91624cd_9037_4e48_84c1_ac0975076bc5);
impl windows_core::RuntimeType for IImageScannerScanResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScannerScanResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ScannedFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ScannedFiles: usize,
}
windows_core::imp::define_interface!(IImageScannerSourceConfiguration, IImageScannerSourceConfiguration_Vtbl, 0xbfb50055_0b44_4c82_9e89_205f9c234e59);
impl core::ops::Deref for IImageScannerSourceConfiguration {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageScannerSourceConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IImageScannerSourceConfiguration, IImageScannerFormatConfiguration);
impl IImageScannerSourceConfiguration {
    pub fn MinScanArea(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinScanArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxScanArea(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxScanArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectedScanRegion(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedScanRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelectedScanRegion(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedScanRegion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoCroppingMode(&self) -> windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoCroppingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoCroppingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OpticalResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpticalResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredResolution(&self, value: ImageScannerResolution) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredResolution)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultColorMode(&self) -> windows_core::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorMode(&self) -> windows_core::Result<ImageScannerColorMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColorMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsColorModeSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinBrightness(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBrightness(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BrightnessStep(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrightnessStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultBrightness(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Brightness(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBrightness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxContrast(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContrastStep(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContrastStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultContrast(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Contrast(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContrast)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultFormat(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = &windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = &windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFormatConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFormatSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IImageScannerSourceConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScannerSourceConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinScanArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub MaxScanArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SelectedScanRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetSelectedScanRegion: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub AutoCroppingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerAutoCroppingMode) -> windows_core::HRESULT,
    pub SetAutoCroppingMode: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerAutoCroppingMode) -> windows_core::HRESULT,
    pub IsAutoCroppingModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerAutoCroppingMode, *mut bool) -> windows_core::HRESULT,
    pub MinResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerResolution) -> windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerResolution) -> windows_core::HRESULT,
    pub OpticalResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerResolution) -> windows_core::HRESULT,
    pub DesiredResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerResolution) -> windows_core::HRESULT,
    pub SetDesiredResolution: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerResolution) -> windows_core::HRESULT,
    pub ActualResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerResolution) -> windows_core::HRESULT,
    pub DefaultColorMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerColorMode) -> windows_core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ImageScannerColorMode) -> windows_core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerColorMode) -> windows_core::HRESULT,
    pub IsColorModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ImageScannerColorMode, *mut bool) -> windows_core::HRESULT,
    pub MinBrightness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxBrightness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BrightnessStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DefaultBrightness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Brightness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ContrastStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DefaultContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Contrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageScannerStatics, IImageScannerStatics_Vtbl, 0xbc57e70e_d804_4477_9fb5_b911b5473897);
impl windows_core::RuntimeType for IImageScannerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageScannerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageScanner(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageScanner, windows_core::IUnknown, windows_core::IInspectable);
impl ImageScanner {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultScanSource(&self) -> windows_core::Result<ImageScannerScanSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultScanSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsScanSourceSupported(&self, value: ImageScannerScanSource) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScanSourceSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn FlatbedConfiguration(&self) -> windows_core::Result<ImageScannerFlatbedConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlatbedConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FeederConfiguration(&self) -> windows_core::Result<ImageScannerFeederConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FeederConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoConfiguration(&self) -> windows_core::Result<ImageScannerAutoConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPreviewSupported(&self, scansource: ImageScannerScanSource) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPreviewSupported)(windows_core::Interface::as_raw(this), scansource, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanPreviewToStreamAsync<P0>(&self, scansource: ImageScannerScanSource, targetstream: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ImageScannerPreviewResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanPreviewToStreamAsync)(windows_core::Interface::as_raw(this), scansource, targetstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn ScanFilesToFolderAsync<P0>(&self, scansource: ImageScannerScanSource, storagefolder: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ImageScannerScanResult, u32>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanFilesToFolderAsync)(windows_core::Interface::as_raw(this), scansource, storagefolder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ImageScanner>> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IImageScannerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IImageScannerStatics<R, F: FnOnce(&IImageScannerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ImageScanner, IImageScannerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ImageScanner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageScanner>();
}
unsafe impl windows_core::Interface for ImageScanner {
    type Vtable = IImageScanner_Vtbl;
    const IID: windows_core::GUID = <IImageScanner as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageScanner {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScanner";
}
unsafe impl Send for ImageScanner {}
unsafe impl Sync for ImageScanner {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageScannerAutoConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageScannerAutoConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageScannerAutoConfiguration, IImageScannerFormatConfiguration);
impl ImageScannerAutoConfiguration {
    pub fn DefaultFormat(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFormatSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ImageScannerAutoConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageScannerFormatConfiguration>();
}
unsafe impl windows_core::Interface for ImageScannerAutoConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: windows_core::GUID = <IImageScannerFormatConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageScannerAutoConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerAutoConfiguration";
}
unsafe impl Send for ImageScannerAutoConfiguration {}
unsafe impl Sync for ImageScannerAutoConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageScannerFeederConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageScannerFeederConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageScannerFeederConfiguration, IImageScannerFormatConfiguration, IImageScannerSourceConfiguration);
impl ImageScannerFeederConfiguration {
    pub fn CanAutoDetectPageSize(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanAutoDetectPageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoDetectPageSize(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoDetectPageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoDetectPageSize(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutoDetectPageSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn PageSize(&self) -> windows_core::Result<super::super::Graphics::Printing::PrintMediaSize> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn SetPageSize(&self, value: super::super::Graphics::Printing::PrintMediaSize) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPageSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn PageOrientation(&self) -> windows_core::Result<super::super::Graphics::Printing::PrintOrientation> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn SetPageOrientation(&self, value: super::super::Graphics::Printing::PrintOrientation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPageOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PageSizeDimensions(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageSizeDimensions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Printing")]
    pub fn IsPageSizeSupported(&self, pagesize: super::super::Graphics::Printing::PrintMediaSize, pageorientation: super::super::Graphics::Printing::PrintOrientation) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPageSizeSupported)(windows_core::Interface::as_raw(this), pagesize, pageorientation, &mut result__).map(|| result__)
        }
    }
    pub fn MaxNumberOfPages(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxNumberOfPages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxNumberOfPages(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxNumberOfPages)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanScanDuplex(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanScanDuplex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duplex(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duplex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuplex(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDuplex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanScanAhead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanScanAhead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScanAhead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanAhead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScanAhead(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerFeederConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScanAhead)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultFormat(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFormatSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinScanArea(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinScanArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxScanArea(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxScanArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectedScanRegion(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedScanRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelectedScanRegion(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedScanRegion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoCroppingMode(&self) -> windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoCroppingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutoCroppingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OpticalResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpticalResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredResolution(&self, value: ImageScannerResolution) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredResolution)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultColorMode(&self) -> windows_core::Result<ImageScannerColorMode> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorMode(&self) -> windows_core::Result<ImageScannerColorMode> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColorMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsColorModeSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinBrightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBrightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BrightnessStep(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrightnessStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultBrightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Brightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBrightness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxContrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContrastStep(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContrastStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultContrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Contrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContrast)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ImageScannerFeederConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageScannerFormatConfiguration>();
}
unsafe impl windows_core::Interface for ImageScannerFeederConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: windows_core::GUID = <IImageScannerFormatConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageScannerFeederConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFeederConfiguration";
}
unsafe impl Send for ImageScannerFeederConfiguration {}
unsafe impl Sync for ImageScannerFeederConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageScannerFlatbedConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageScannerFlatbedConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageScannerFlatbedConfiguration, IImageScannerFormatConfiguration, IImageScannerSourceConfiguration);
impl ImageScannerFlatbedConfiguration {
    pub fn DefaultFormat(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: ImageScannerFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFormatSupported(&self, value: ImageScannerFormat) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFormatSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinScanArea(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinScanArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxScanArea(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxScanArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectedScanRegion(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedScanRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelectedScanRegion(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedScanRegion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoCroppingMode(&self) -> windows_core::Result<ImageScannerAutoCroppingMode> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoCroppingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoCroppingMode(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutoCroppingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoCroppingModeSupported(&self, value: ImageScannerAutoCroppingMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoCroppingModeSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OpticalResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpticalResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredResolution(&self, value: ImageScannerResolution) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredResolution)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualResolution(&self) -> windows_core::Result<ImageScannerResolution> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultColorMode(&self) -> windows_core::Result<ImageScannerColorMode> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorMode(&self) -> windows_core::Result<ImageScannerColorMode> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColorMode(&self, value: ImageScannerColorMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColorMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorModeSupported(&self, value: ImageScannerColorMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsColorModeSupported)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn MinBrightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBrightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BrightnessStep(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrightnessStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultBrightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultBrightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Brightness(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brightness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBrightness(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBrightness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinContrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxContrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContrastStep(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContrastStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultContrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Contrast(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContrast(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IImageScannerSourceConfiguration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContrast)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ImageScannerFlatbedConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageScannerFormatConfiguration>();
}
unsafe impl windows_core::Interface for ImageScannerFlatbedConfiguration {
    type Vtable = IImageScannerFormatConfiguration_Vtbl;
    const IID: windows_core::GUID = <IImageScannerFormatConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageScannerFlatbedConfiguration {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerFlatbedConfiguration";
}
unsafe impl Send for ImageScannerFlatbedConfiguration {}
unsafe impl Sync for ImageScannerFlatbedConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageScannerPreviewResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageScannerPreviewResult, windows_core::IUnknown, windows_core::IInspectable);
impl ImageScannerPreviewResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<ImageScannerFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ImageScannerPreviewResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageScannerPreviewResult>();
}
unsafe impl windows_core::Interface for ImageScannerPreviewResult {
    type Vtable = IImageScannerPreviewResult_Vtbl;
    const IID: windows_core::GUID = <IImageScannerPreviewResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageScannerPreviewResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerPreviewResult";
}
unsafe impl Send for ImageScannerPreviewResult {}
unsafe impl Sync for ImageScannerPreviewResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ImageScannerScanResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageScannerScanResult, windows_core::IUnknown, windows_core::IInspectable);
impl ImageScannerScanResult {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ScannedFiles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScannedFiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ImageScannerScanResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageScannerScanResult>();
}
unsafe impl windows_core::Interface for ImageScannerScanResult {
    type Vtable = IImageScannerScanResult_Vtbl;
    const IID: windows_core::GUID = <IImageScannerScanResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageScannerScanResult {
    const NAME: &'static str = "Windows.Devices.Scanners.ImageScannerScanResult";
}
unsafe impl Send for ImageScannerScanResult {}
unsafe impl Sync for ImageScannerScanResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: Self = Self(0i32);
    pub const SingleRegion: Self = Self(1i32);
    pub const MultipleRegion: Self = Self(2i32);
}
impl windows_core::TypeKind for ImageScannerAutoCroppingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageScannerAutoCroppingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageScannerAutoCroppingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ImageScannerAutoCroppingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerAutoCroppingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: Self = Self(0i32);
    pub const Grayscale: Self = Self(1i32);
    pub const Monochrome: Self = Self(2i32);
    pub const AutoColor: Self = Self(3i32);
}
impl windows_core::TypeKind for ImageScannerColorMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageScannerColorMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageScannerColorMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ImageScannerColorMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerColorMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const DeviceIndependentBitmap: Self = Self(2i32);
    pub const Tiff: Self = Self(3i32);
    pub const Xps: Self = Self(4i32);
    pub const OpenXps: Self = Self(5i32);
    pub const Pdf: Self = Self(6i32);
}
impl windows_core::TypeKind for ImageScannerFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageScannerFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageScannerFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ImageScannerFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: Self = Self(0i32);
    pub const Flatbed: Self = Self(1i32);
    pub const Feeder: Self = Self(2i32);
    pub const AutoConfigured: Self = Self(3i32);
}
impl windows_core::TypeKind for ImageScannerScanSource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageScannerScanSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageScannerScanSource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ImageScannerScanSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Scanners.ImageScannerScanSource;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageScannerResolution {
    pub DpiX: f32,
    pub DpiY: f32,
}
impl windows_core::TypeKind for ImageScannerResolution {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ImageScannerResolution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Scanners.ImageScannerResolution;f4;f4)");
}
impl Default for ImageScannerResolution {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

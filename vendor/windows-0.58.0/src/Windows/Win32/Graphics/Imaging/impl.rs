pub trait IWICBitmap_Impl: Sized + IWICBitmapSource_Impl {
    fn Lock(&self, prclock: *const WICRect, flags: u32) -> windows_core::Result<IWICBitmapLock>;
    fn SetPalette(&self, pipalette: Option<&IWICPalette>) -> windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICBitmap {}
impl IWICBitmap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmap_Vtbl
    where
        Identity: IWICBitmap_Impl,
    {
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmap_Impl::Lock(this, core::mem::transmute_copy(&prclock), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppilock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmap_Impl::SetPalette(this, windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn SetResolution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: f64, dpiy: f64) -> windows_core::HRESULT
        where
            Identity: IWICBitmap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmap_Impl::SetResolution(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetResolution: SetResolution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmap as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapClipper_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: Option<&IWICBitmapSource>, prc: *const WICRect) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICBitmapClipper {}
impl IWICBitmapClipper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapClipper_Vtbl
    where
        Identity: IWICBitmapClipper_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, prc: *const WICRect) -> windows_core::HRESULT
        where
            Identity: IWICBitmapClipper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapClipper_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisource), core::mem::transmute_copy(&prc)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapClipper as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapCodecInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetPixelFormats(&self, cformats: u32, pguidpixelformats: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::Result<()>;
    fn GetColorManagementVersion(&self, cchcolormanagementversion: u32, wzcolormanagementversion: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetMimeTypes(&self, cchmimetypes: u32, wzmimetypes: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetFileExtensions(&self, cchfileextensions: u32, wzfileextensions: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn DoesSupportAnimation(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportChromakey(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportLossless(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportMultiframe(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn MatchesMimeType(&self, wzmimetype: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWICBitmapCodecInfo {}
impl IWICBitmapCodecInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapCodecInfo_Vtbl
    where
        Identity: IWICBitmapCodecInfo_Impl,
    {
        unsafe extern "system" fn GetContainerFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapCodecInfo_Impl::GetContainerFormat(this) {
                Ok(ok__) => {
                    pguidcontainerformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32, pguidpixelformats: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecInfo_Impl::GetPixelFormats(this, core::mem::transmute_copy(&cformats), core::mem::transmute_copy(&pguidpixelformats), core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn GetColorManagementVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecInfo_Impl::GetColorManagementVersion(this, core::mem::transmute_copy(&cchcolormanagementversion), core::mem::transmute(&wzcolormanagementversion), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecInfo_Impl::GetDeviceManufacturer(this, core::mem::transmute_copy(&cchdevicemanufacturer), core::mem::transmute(&wzdevicemanufacturer), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecInfo_Impl::GetDeviceModels(this, core::mem::transmute_copy(&cchdevicemodels), core::mem::transmute(&wzdevicemodels), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetMimeTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchmimetypes: u32, wzmimetypes: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecInfo_Impl::GetMimeTypes(this, core::mem::transmute_copy(&cchmimetypes), core::mem::transmute(&wzmimetypes), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFileExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchfileextensions: u32, wzfileextensions: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecInfo_Impl::GetFileExtensions(this, core::mem::transmute_copy(&cchfileextensions), core::mem::transmute(&wzfileextensions), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesSupportAnimation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapCodecInfo_Impl::DoesSupportAnimation(this) {
                Ok(ok__) => {
                    pfsupportanimation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportChromakey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapCodecInfo_Impl::DoesSupportChromakey(this) {
                Ok(ok__) => {
                    pfsupportchromakey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportLossless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapCodecInfo_Impl::DoesSupportLossless(this) {
                Ok(ok__) => {
                    pfsupportlossless.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportMultiframe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapCodecInfo_Impl::DoesSupportMultiframe(this) {
                Ok(ok__) => {
                    pfsupportmultiframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesMimeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzmimetype: windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapCodecInfo_Impl::MatchesMimeType(this, core::mem::transmute(&wzmimetype)) {
                Ok(ok__) => {
                    pfmatches.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetPixelFormats: GetPixelFormats::<Identity, OFFSET>,
            GetColorManagementVersion: GetColorManagementVersion::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, OFFSET>,
            GetMimeTypes: GetMimeTypes::<Identity, OFFSET>,
            GetFileExtensions: GetFileExtensions::<Identity, OFFSET>,
            DoesSupportAnimation: DoesSupportAnimation::<Identity, OFFSET>,
            DoesSupportChromakey: DoesSupportChromakey::<Identity, OFFSET>,
            DoesSupportLossless: DoesSupportLossless::<Identity, OFFSET>,
            DoesSupportMultiframe: DoesSupportMultiframe::<Identity, OFFSET>,
            MatchesMimeType: MatchesMimeType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapCodecInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapCodecProgressNotification_Impl: Sized {
    fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: *const core::ffi::c_void, dwprogressflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICBitmapCodecProgressNotification {}
impl IWICBitmapCodecProgressNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapCodecProgressNotification_Vtbl
    where
        Identity: IWICBitmapCodecProgressNotification_Impl,
    {
        unsafe extern "system" fn RegisterProgressNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfnprogressnotification: PFNProgressNotification, pvdata: *const core::ffi::c_void, dwprogressflags: u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapCodecProgressNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapCodecProgressNotification_Impl::RegisterProgressNotification(this, core::mem::transmute_copy(&pfnprogressnotification), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&dwprogressflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RegisterProgressNotification: RegisterProgressNotification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapCodecProgressNotification as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoder_Impl: Sized {
    fn QueryCapability(&self, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<u32>;
    fn Initialize(&self, pistream: Option<&super::super::System::Com::IStream>, cacheoptions: WICDecodeOptions) -> windows_core::Result<()>;
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDecoderInfo(&self) -> windows_core::Result<IWICBitmapDecoderInfo>;
    fn CopyPalette(&self, pipalette: Option<&IWICPalette>) -> windows_core::Result<()>;
    fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader>;
    fn GetPreview(&self) -> windows_core::Result<IWICBitmapSource>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut Option<IWICColorContext>, pcactualcount: *mut u32) -> windows_core::Result<()>;
    fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource>;
    fn GetFrameCount(&self) -> windows_core::Result<u32>;
    fn GetFrame(&self, index: u32) -> windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICBitmapDecoder {}
#[cfg(feature = "Win32_System_Com")]
impl IWICBitmapDecoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapDecoder_Vtbl
    where
        Identity: IWICBitmapDecoder_Impl,
    {
        unsafe extern "system" fn QueryCapability<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pdwcapability: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::QueryCapability(this, windows_core::from_raw_borrowed(&pistream)) {
                Ok(ok__) => {
                    pdwcapability.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, cacheoptions: WICDecodeOptions) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapDecoder_Impl::Initialize(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&cacheoptions)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetContainerFormat(this) {
                Ok(ok__) => {
                    pguidcontainerformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDecoderInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidecoderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetDecoderInfo(this) {
                Ok(ok__) => {
                    ppidecoderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapDecoder_Impl::CopyPalette(this, windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn GetMetadataQueryReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataqueryreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetMetadataQueryReader(this) {
                Ok(ok__) => {
                    ppimetadataqueryreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetPreview(this) {
                Ok(ok__) => {
                    ppibitmapsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut core::ffi::c_void, pcactualcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapDecoder_Impl::GetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontexts), core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppithumbnail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetThumbnail(this) {
                Ok(ok__) => {
                    ppithumbnail.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetFrameCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppibitmapframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoder_Impl::GetFrame(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppibitmapframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCapability: QueryCapability::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetDecoderInfo: GetDecoderInfo::<Identity, OFFSET>,
            CopyPalette: CopyPalette::<Identity, OFFSET>,
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, OFFSET>,
            GetPreview: GetPreview::<Identity, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
            GetFrameCount: GetFrameCount::<Identity, OFFSET>,
            GetFrame: GetFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapDecoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoderInfo_Impl: Sized + IWICBitmapCodecInfo_Impl {
    fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> windows_core::Result<()>;
    fn MatchesPattern(&self, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&self) -> windows_core::Result<IWICBitmapDecoder>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICBitmapDecoderInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IWICBitmapDecoderInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapDecoderInfo_Vtbl
    where
        Identity: IWICBitmapDecoderInfo_Impl,
    {
        unsafe extern "system" fn GetPatterns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapDecoderInfo_Impl::GetPatterns(this, core::mem::transmute_copy(&cbsizepatterns), core::mem::transmute_copy(&ppatterns), core::mem::transmute_copy(&pcpatterns), core::mem::transmute_copy(&pcbpatternsactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoderInfo_Impl::MatchesPattern(this, windows_core::from_raw_borrowed(&pistream)) {
                Ok(ok__) => {
                    pfmatches.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapdecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapDecoderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapDecoderInfo_Impl::CreateInstance(this) {
                Ok(ok__) => {
                    ppibitmapdecoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICBitmapCodecInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapDecoderInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICBitmapCodecInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapEncoder_Impl: Sized {
    fn Initialize(&self, pistream: Option<&super::super::System::Com::IStream>, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::Result<()>;
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetEncoderInfo(&self) -> windows_core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const Option<IWICColorContext>) -> windows_core::Result<()>;
    fn SetPalette(&self, pipalette: Option<&IWICPalette>) -> windows_core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: Option<&IWICBitmapSource>) -> windows_core::Result<()>;
    fn SetPreview(&self, pipreview: Option<&IWICBitmapSource>) -> windows_core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWICBitmapEncoder {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapEncoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapEncoder_Vtbl
    where
        Identity: IWICBitmapEncoder_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::Initialize(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&cacheoption)).into()
        }
        unsafe extern "system" fn GetContainerFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapEncoder_Impl::GetContainerFormat(this) {
                Ok(ok__) => {
                    pguidcontainerformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncoderInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiencoderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapEncoder_Impl::GetEncoderInfo(this) {
                Ok(ok__) => {
                    ppiencoderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorContexts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::SetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::SetPalette(this, windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pithumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::SetThumbnail(this, windows_core::from_raw_borrowed(&pithumbnail)).into()
        }
        unsafe extern "system" fn SetPreview<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipreview: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::SetPreview(this, windows_core::from_raw_borrowed(&pipreview)).into()
        }
        unsafe extern "system" fn CreateNewFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiframeencode: *mut *mut core::ffi::c_void, ppiencoderoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::CreateNewFrame(this, core::mem::transmute_copy(&ppiframeencode), core::mem::transmute_copy(&ppiencoderoptions)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapEncoder_Impl::Commit(this).into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapEncoder_Impl::GetMetadataQueryWriter(this) {
                Ok(ok__) => {
                    ppimetadataquerywriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetEncoderInfo: GetEncoderInfo::<Identity, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
            SetPreview: SetPreview::<Identity, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapEncoderInfo_Impl: Sized + IWICBitmapCodecInfo_Impl {
    fn CreateInstance(&self) -> windows_core::Result<IWICBitmapEncoder>;
}
impl windows_core::RuntimeName for IWICBitmapEncoderInfo {}
impl IWICBitmapEncoderInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapEncoderInfo_Vtbl
    where
        Identity: IWICBitmapEncoderInfo_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapEncoderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapEncoderInfo_Impl::CreateInstance(this) {
                Ok(ok__) => {
                    ppibitmapencoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWICBitmapCodecInfo_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoderInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICBitmapCodecInfo as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapFlipRotator_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: Option<&IWICBitmapSource>, options: WICBitmapTransformOptions) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICBitmapFlipRotator {}
impl IWICBitmapFlipRotator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapFlipRotator_Vtbl
    where
        Identity: IWICBitmapFlipRotator_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, options: WICBitmapTransformOptions) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFlipRotator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFlipRotator_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisource), core::mem::transmute_copy(&options)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFlipRotator as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapFrameDecode_Impl: Sized + IWICBitmapSource_Impl {
    fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader>;
    fn GetColorContexts(&self, ccount: u32, ppicolorcontexts: *mut Option<IWICColorContext>, pcactualcount: *mut u32) -> windows_core::Result<()>;
    fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource>;
}
impl windows_core::RuntimeName for IWICBitmapFrameDecode {}
impl IWICBitmapFrameDecode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapFrameDecode_Vtbl
    where
        Identity: IWICBitmapFrameDecode_Impl,
    {
        unsafe extern "system" fn GetMetadataQueryReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataqueryreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapFrameDecode_Impl::GetMetadataQueryReader(this) {
                Ok(ok__) => {
                    ppimetadataqueryreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContexts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut core::ffi::c_void, pcactualcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameDecode_Impl::GetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontexts), core::mem::transmute_copy(&pcactualcount)).into()
        }
        unsafe extern "system" fn GetThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppithumbnail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapFrameDecode_Impl::GetThumbnail(this) {
                Ok(ok__) => {
                    ppithumbnail.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameDecode as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapFrameEncode_Impl: Sized {
    fn Initialize(&self, piencoderoptions: Option<&super::super::System::Com::StructuredStorage::IPropertyBag2>) -> windows_core::Result<()>;
    fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()>;
    fn SetPixelFormat(&self, ppixelformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetColorContexts(&self, ccount: u32, ppicolorcontext: *const Option<IWICColorContext>) -> windows_core::Result<()>;
    fn SetPalette(&self, pipalette: Option<&IWICPalette>) -> windows_core::Result<()>;
    fn SetThumbnail(&self, pithumbnail: Option<&IWICBitmapSource>) -> windows_core::Result<()>;
    fn WritePixels(&self, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> windows_core::Result<()>;
    fn WriteSource(&self, pibitmapsource: Option<&IWICBitmapSource>, prc: *const WICRect) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWICBitmapFrameEncode {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapFrameEncode_Vtbl
    where
        Identity: IWICBitmapFrameEncode_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piencoderoptions: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::Initialize(this, windows_core::from_raw_borrowed(&piencoderoptions)).into()
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::SetSize(this, core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight)).into()
        }
        unsafe extern "system" fn SetResolution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: f64, dpiy: f64) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::SetResolution(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
        }
        unsafe extern "system" fn SetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::SetPixelFormat(this, core::mem::transmute_copy(&ppixelformat)).into()
        }
        unsafe extern "system" fn SetColorContexts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::SetColorContexts(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&ppicolorcontext)).into()
        }
        unsafe extern "system" fn SetPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::SetPalette(this, windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn SetThumbnail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pithumbnail: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::SetThumbnail(this, windows_core::from_raw_borrowed(&pithumbnail)).into()
        }
        unsafe extern "system" fn WritePixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::WritePixels(this, core::mem::transmute_copy(&linecount), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbpixels)).into()
        }
        unsafe extern "system" fn WriteSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, prc: *const WICRect) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::WriteSource(this, windows_core::from_raw_borrowed(&pibitmapsource), core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapFrameEncode_Impl::Commit(this).into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapFrameEncode_Impl::GetMetadataQueryWriter(this) {
                Ok(ok__) => {
                    ppimetadataquerywriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SetResolution: SetResolution::<Identity, OFFSET>,
            SetPixelFormat: SetPixelFormat::<Identity, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
            WritePixels: WritePixels::<Identity, OFFSET>,
            WriteSource: WriteSource::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapLock_Impl: Sized {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetStride(&self) -> windows_core::Result<u32>;
    fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IWICBitmapLock {}
impl IWICBitmapLock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapLock_Vtbl
    where
        Identity: IWICBitmapLock_Impl,
    {
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapLock_Impl::GetSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetStride<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbstride: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapLock_Impl::GetStride(this) {
                Ok(ok__) => {
                    pcbstride.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IWICBitmapLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapLock_Impl::GetDataPointer(this, core::mem::transmute_copy(&pcbbuffersize), core::mem::transmute_copy(&ppbdata)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapLock_Impl::GetPixelFormat(this) {
                Ok(ok__) => {
                    ppixelformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetStride: GetStride::<Identity, OFFSET>,
            GetDataPointer: GetDataPointer::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapLock as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapScaler_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: Option<&IWICBitmapSource>, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICBitmapScaler {}
impl IWICBitmapScaler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapScaler_Vtbl
    where
        Identity: IWICBitmapScaler_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> windows_core::HRESULT
        where
            Identity: IWICBitmapScaler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapScaler_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisource), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&mode)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapScaler as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapSource_Impl: Sized {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::Result<()>;
    fn CopyPalette(&self, pipalette: Option<&IWICPalette>) -> windows_core::Result<()>;
    fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICBitmapSource {}
impl IWICBitmapSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapSource_Vtbl
    where
        Identity: IWICBitmapSource_Impl,
    {
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSource_Impl::GetSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapSource_Impl::GetPixelFormat(this) {
                Ok(ok__) => {
                    ppixelformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSource_Impl::GetResolution(this, core::mem::transmute_copy(&pdpix), core::mem::transmute_copy(&pdpiy)).into()
        }
        unsafe extern "system" fn CopyPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSource_Impl::CopyPalette(this, windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn CopyPixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSource_Impl::CopyPixels(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetResolution: GetResolution::<Identity, OFFSET>,
            CopyPalette: CopyPalette::<Identity, OFFSET>,
            CopyPixels: CopyPixels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICBitmapSourceTransform_Impl: Sized {
    fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::Result<()>;
    fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetClosestPixelFormat(&self, pguiddstformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWICBitmapSourceTransform {}
impl IWICBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICBitmapSourceTransform_Vtbl
    where
        Identity: IWICBitmapSourceTransform_Impl,
    {
        unsafe extern "system" fn CopyPixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSourceTransform_Impl::CopyPixels(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pguiddstformat), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&nstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbbuffer)).into()
        }
        unsafe extern "system" fn GetClosestSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSourceTransform_Impl::GetClosestSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
        }
        unsafe extern "system" fn GetClosestPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguiddstformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICBitmapSourceTransform_Impl::GetClosestPixelFormat(this, core::mem::transmute_copy(&pguiddstformat)).into()
        }
        unsafe extern "system" fn DoesSupportTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICBitmapSourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICBitmapSourceTransform_Impl::DoesSupportTransform(this, core::mem::transmute_copy(&dsttransform)) {
                Ok(ok__) => {
                    pfissupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CopyPixels: CopyPixels::<Identity, OFFSET>,
            GetClosestSize: GetClosestSize::<Identity, OFFSET>,
            GetClosestPixelFormat: GetClosestPixelFormat::<Identity, OFFSET>,
            DoesSupportTransform: DoesSupportTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapSourceTransform as windows_core::Interface>::IID
    }
}
pub trait IWICColorContext_Impl: Sized {
    fn InitializeFromFilename(&self, wzfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::Result<()>;
    fn InitializeFromExifColorSpace(&self, value: u32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<WICColorContextType>;
    fn GetProfileBytes(&self, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::Result<()>;
    fn GetExifColorSpace(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWICColorContext {}
impl IWICColorContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICColorContext_Vtbl
    where
        Identity: IWICColorContext_Impl,
    {
        unsafe extern "system" fn InitializeFromFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWICColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICColorContext_Impl::InitializeFromFilename(this, core::mem::transmute(&wzfilename)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::HRESULT
        where
            Identity: IWICColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICColorContext_Impl::InitializeFromMemory(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: IWICColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICColorContext_Impl::InitializeFromExifColorSpace(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WICColorContextType) -> windows_core::HRESULT
        where
            Identity: IWICColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICColorContext_Impl::GetType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICColorContext_Impl::GetProfileBytes(this, core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn GetExifColorSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICColorContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICColorContext_Impl::GetExifColorSpace(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromFilename: InitializeFromFilename::<Identity, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, OFFSET>,
            InitializeFromExifColorSpace: InitializeFromExifColorSpace::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetProfileBytes: GetProfileBytes::<Identity, OFFSET>,
            GetExifColorSpace: GetExifColorSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICColorContext as windows_core::Interface>::IID
    }
}
pub trait IWICColorTransform_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pibitmapsource: Option<&IWICBitmapSource>, picontextsource: Option<&IWICColorContext>, picontextdest: Option<&IWICColorContext>, pixelfmtdest: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICColorTransform {}
impl IWICColorTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICColorTransform_Vtbl
    where
        Identity: IWICColorTransform_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, picontextsource: *mut core::ffi::c_void, picontextdest: *mut core::ffi::c_void, pixelfmtdest: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICColorTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICColorTransform_Impl::Initialize(this, windows_core::from_raw_borrowed(&pibitmapsource), windows_core::from_raw_borrowed(&picontextsource), windows_core::from_raw_borrowed(&picontextdest), core::mem::transmute_copy(&pixelfmtdest)).into()
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICColorTransform as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICComponentFactory_Impl: Sized + IWICImagingFactory_Impl {
    fn CreateMetadataReader(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataReaderFromContainer(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwmetadataoptions: u32) -> windows_core::Result<IWICMetadataWriter>;
    fn CreateMetadataWriterFromReader(&self, pireader: Option<&IWICMetadataReader>, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataWriter>;
    fn CreateQueryReaderFromBlockReader(&self, piblockreader: Option<&IWICMetadataBlockReader>) -> windows_core::Result<IWICMetadataQueryReader>;
    fn CreateQueryWriterFromBlockWriter(&self, piblockwriter: Option<&IWICMetadataBlockWriter>) -> windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateEncoderPropertyBag(&self, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWICComponentFactory {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICComponentFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICComponentFactory_Vtbl
    where
        Identity: IWICComponentFactory_Impl,
    {
        unsafe extern "system" fn CreateMetadataReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateMetadataReader(this, core::mem::transmute_copy(&guidmetadataformat), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwoptions), windows_core::from_raw_borrowed(&pistream)) {
                Ok(ok__) => {
                    ppireader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateMetadataReaderFromContainer(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwoptions), windows_core::from_raw_borrowed(&pistream)) {
                Ok(ok__) => {
                    ppireader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwmetadataoptions: u32, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateMetadataWriter(this, core::mem::transmute_copy(&guidmetadataformat), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwmetadataoptions)) {
                Ok(ok__) => {
                    ppiwriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pireader: *mut core::ffi::c_void, pguidvendor: *const windows_core::GUID, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateMetadataWriterFromReader(this, windows_core::from_raw_borrowed(&pireader), core::mem::transmute_copy(&pguidvendor)) {
                Ok(ok__) => {
                    ppiwriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piblockreader: *mut core::ffi::c_void, ppiqueryreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateQueryReaderFromBlockReader(this, windows_core::from_raw_borrowed(&piblockreader)) {
                Ok(ok__) => {
                    ppiqueryreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piblockwriter: *mut core::ffi::c_void, ppiquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateQueryWriterFromBlockWriter(this, windows_core::from_raw_borrowed(&piblockwriter)) {
                Ok(ok__) => {
                    ppiquerywriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICComponentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentFactory_Impl::CreateEncoderPropertyBag(this, core::mem::transmute_copy(&ppropoptions), core::mem::transmute_copy(&ccount)) {
                Ok(ok__) => {
                    ppipropertybag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICImagingFactory_Vtbl::new::<Identity, OFFSET>(),
            CreateMetadataReader: CreateMetadataReader::<Identity, OFFSET>,
            CreateMetadataReaderFromContainer: CreateMetadataReaderFromContainer::<Identity, OFFSET>,
            CreateMetadataWriter: CreateMetadataWriter::<Identity, OFFSET>,
            CreateMetadataWriterFromReader: CreateMetadataWriterFromReader::<Identity, OFFSET>,
            CreateQueryReaderFromBlockReader: CreateQueryReaderFromBlockReader::<Identity, OFFSET>,
            CreateQueryWriterFromBlockWriter: CreateQueryWriterFromBlockWriter::<Identity, OFFSET>,
            CreateEncoderPropertyBag: CreateEncoderPropertyBag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICComponentFactory as windows_core::Interface>::IID || iid == &<IWICImagingFactory as windows_core::Interface>::IID
    }
}
pub trait IWICComponentInfo_Impl: Sized {
    fn GetComponentType(&self) -> windows_core::Result<WICComponentType>;
    fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSigningStatus(&self) -> windows_core::Result<u32>;
    fn GetAuthor(&self, cchauthor: u32, wzauthor: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetVendorGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetVersion(&self, cchversion: u32, wzversion: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetSpecVersion(&self, cchspecversion: u32, wzspecversion: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetFriendlyName(&self, cchfriendlyname: u32, wzfriendlyname: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICComponentInfo {}
impl IWICComponentInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICComponentInfo_Vtbl
    where
        Identity: IWICComponentInfo_Impl,
    {
        unsafe extern "system" fn GetComponentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WICComponentType) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentInfo_Impl::GetComponentType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentInfo_Impl::GetCLSID(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentInfo_Impl::GetSigningStatus(this) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchauthor: u32, wzauthor: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICComponentInfo_Impl::GetAuthor(this, core::mem::transmute_copy(&cchauthor), core::mem::transmute(&wzauthor), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetVendorGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidvendor: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICComponentInfo_Impl::GetVendorGUID(this) {
                Ok(ok__) => {
                    pguidvendor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchversion: u32, wzversion: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICComponentInfo_Impl::GetVersion(this, core::mem::transmute_copy(&cchversion), core::mem::transmute(&wzversion), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetSpecVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchspecversion: u32, wzspecversion: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICComponentInfo_Impl::GetSpecVersion(this, core::mem::transmute_copy(&cchspecversion), core::mem::transmute(&wzspecversion), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICComponentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICComponentInfo_Impl::GetFriendlyName(this, core::mem::transmute_copy(&cchfriendlyname), core::mem::transmute(&wzfriendlyname), core::mem::transmute_copy(&pcchactual)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetComponentType: GetComponentType::<Identity, OFFSET>,
            GetCLSID: GetCLSID::<Identity, OFFSET>,
            GetSigningStatus: GetSigningStatus::<Identity, OFFSET>,
            GetAuthor: GetAuthor::<Identity, OFFSET>,
            GetVendorGUID: GetVendorGUID::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetSpecVersion: GetSpecVersion::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsDecoder_Impl: Sized {
    fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::Result<()>;
    fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IWICDdsDecoder {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsDecoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICDdsDecoder_Vtbl
    where
        Identity: IWICDdsDecoder_Impl,
    {
        unsafe extern "system" fn GetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut WICDdsParameters) -> windows_core::HRESULT
        where
            Identity: IWICDdsDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDdsDecoder_Impl::GetParameters(this, core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICDdsDecoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDdsDecoder_Impl::GetFrame(this, core::mem::transmute_copy(&arrayindex), core::mem::transmute_copy(&miplevel), core::mem::transmute_copy(&sliceindex)) {
                Ok(ok__) => {
                    ppibitmapframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, OFFSET>,
            GetFrame: GetFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDdsDecoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsEncoder_Impl: Sized {
    fn SetParameters(&self, pparameters: *const WICDdsParameters) -> windows_core::Result<()>;
    fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::Result<()>;
    fn CreateNewFrame(&self, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IWICDdsEncoder {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsEncoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICDdsEncoder_Vtbl
    where
        Identity: IWICDdsEncoder_Impl,
    {
        unsafe extern "system" fn SetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *const WICDdsParameters) -> windows_core::HRESULT
        where
            Identity: IWICDdsEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDdsEncoder_Impl::SetParameters(this, core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut WICDdsParameters) -> windows_core::HRESULT
        where
            Identity: IWICDdsEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDdsEncoder_Impl::GetParameters(this, core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn CreateNewFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiframeencode: *mut *mut core::ffi::c_void, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICDdsEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDdsEncoder_Impl::CreateNewFrame(this, core::mem::transmute_copy(&ppiframeencode), core::mem::transmute_copy(&parrayindex), core::mem::transmute_copy(&pmiplevel), core::mem::transmute_copy(&psliceindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDdsEncoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsFrameDecode_Impl: Sized {
    fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> windows_core::Result<()>;
    fn GetFormatInfo(&self) -> windows_core::Result<WICDdsFormatInfo>;
    fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IWICDdsFrameDecode {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICDdsFrameDecode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICDdsFrameDecode_Vtbl
    where
        Identity: IWICDdsFrameDecode_Impl,
    {
        unsafe extern "system" fn GetSizeInBlocks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICDdsFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDdsFrameDecode_Impl::GetSizeInBlocks(this, core::mem::transmute_copy(&pwidthinblocks), core::mem::transmute_copy(&pheightinblocks)).into()
        }
        unsafe extern "system" fn GetFormatInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> windows_core::HRESULT
        where
            Identity: IWICDdsFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDdsFrameDecode_Impl::GetFormatInfo(this) {
                Ok(ok__) => {
                    pformatinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyBlocks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWICDdsFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDdsFrameDecode_Impl::CopyBlocks(this, core::mem::transmute_copy(&prcboundsinblocks), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSizeInBlocks: GetSizeInBlocks::<Identity, OFFSET>,
            GetFormatInfo: GetFormatInfo::<Identity, OFFSET>,
            CopyBlocks: CopyBlocks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDdsFrameDecode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICDevelopRaw_Impl: Sized + IWICBitmapFrameDecode_Impl {
    fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> windows_core::Result<()>;
    fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> windows_core::Result<()>;
    fn GetCurrentParameterSet(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
    fn SetExposureCompensation(&self, ev: f64) -> windows_core::Result<()>;
    fn GetExposureCompensation(&self) -> windows_core::Result<f64>;
    fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> windows_core::Result<()>;
    fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> windows_core::Result<()>;
    fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> windows_core::Result<()>;
    fn GetNamedWhitePoint(&self) -> windows_core::Result<WICNamedWhitePoint>;
    fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> windows_core::Result<()>;
    fn GetWhitePointKelvin(&self) -> windows_core::Result<u32>;
    fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> windows_core::Result<()>;
    fn SetContrast(&self, contrast: f64) -> windows_core::Result<()>;
    fn GetContrast(&self) -> windows_core::Result<f64>;
    fn SetGamma(&self, gamma: f64) -> windows_core::Result<()>;
    fn GetGamma(&self) -> windows_core::Result<f64>;
    fn SetSharpness(&self, sharpness: f64) -> windows_core::Result<()>;
    fn GetSharpness(&self) -> windows_core::Result<f64>;
    fn SetSaturation(&self, saturation: f64) -> windows_core::Result<()>;
    fn GetSaturation(&self) -> windows_core::Result<f64>;
    fn SetTint(&self, tint: f64) -> windows_core::Result<()>;
    fn GetTint(&self) -> windows_core::Result<f64>;
    fn SetNoiseReduction(&self, noisereduction: f64) -> windows_core::Result<()>;
    fn GetNoiseReduction(&self) -> windows_core::Result<f64>;
    fn SetDestinationColorContext(&self, pcolorcontext: Option<&IWICColorContext>) -> windows_core::Result<()>;
    fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> windows_core::Result<()>;
    fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> windows_core::Result<()>;
    fn SetRotation(&self, rotation: f64) -> windows_core::Result<()>;
    fn GetRotation(&self) -> windows_core::Result<f64>;
    fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> windows_core::Result<()>;
    fn GetRenderMode(&self) -> windows_core::Result<WICRawRenderMode>;
    fn SetNotificationCallback(&self, pcallback: Option<&IWICDevelopRawNotificationCallback>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWICDevelopRaw {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWICDevelopRaw_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICDevelopRaw_Vtbl
    where
        Identity: IWICDevelopRaw_Impl,
    {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::QueryRawCapabilitiesInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn LoadParameterSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterset: WICRawParameterSet) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::LoadParameterSet(this, core::mem::transmute_copy(&parameterset)).into()
        }
        unsafe extern "system" fn GetCurrentParameterSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcurrentparameterset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetCurrentParameterSet(this) {
                Ok(ok__) => {
                    ppcurrentparameterset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExposureCompensation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ev: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetExposureCompensation(this, core::mem::transmute_copy(&ev)).into()
        }
        unsafe extern "system" fn GetExposureCompensation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pev: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetExposureCompensation(this) {
                Ok(ok__) => {
                    pev.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointRGB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, red: u32, green: u32, blue: u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetWhitePointRGB(this, core::mem::transmute_copy(&red), core::mem::transmute_copy(&green), core::mem::transmute_copy(&blue)).into()
        }
        unsafe extern "system" fn GetWhitePointRGB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::GetWhitePointRGB(this, core::mem::transmute_copy(&pred), core::mem::transmute_copy(&pgreen), core::mem::transmute_copy(&pblue)).into()
        }
        unsafe extern "system" fn SetNamedWhitePoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetNamedWhitePoint(this, core::mem::transmute_copy(&whitepoint)).into()
        }
        unsafe extern "system" fn GetNamedWhitePoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetNamedWhitePoint(this) {
                Ok(ok__) => {
                    pwhitepoint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWhitePointKelvin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitepointkelvin: u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetWhitePointKelvin(this, core::mem::transmute_copy(&whitepointkelvin)).into()
        }
        unsafe extern "system" fn GetWhitePointKelvin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwhitepointkelvin: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetWhitePointKelvin(this) {
                Ok(ok__) => {
                    pwhitepointkelvin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::GetKelvinRangeInfo(this, core::mem::transmute_copy(&pminkelvintemp), core::mem::transmute_copy(&pmaxkelvintemp), core::mem::transmute_copy(&pkelvintempstepvalue)).into()
        }
        unsafe extern "system" fn SetContrast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contrast: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetContrast(this, core::mem::transmute_copy(&contrast)).into()
        }
        unsafe extern "system" fn GetContrast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontrast: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetContrast(this) {
                Ok(ok__) => {
                    pcontrast.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGamma<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetGamma(this, core::mem::transmute_copy(&gamma)).into()
        }
        unsafe extern "system" fn GetGamma<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgamma: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetGamma(this) {
                Ok(ok__) => {
                    pgamma.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharpness<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharpness: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetSharpness(this, core::mem::transmute_copy(&sharpness)).into()
        }
        unsafe extern "system" fn GetSharpness<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharpness: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetSharpness(this) {
                Ok(ok__) => {
                    psharpness.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaturation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, saturation: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetSaturation(this, core::mem::transmute_copy(&saturation)).into()
        }
        unsafe extern "system" fn GetSaturation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psaturation: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetSaturation(this) {
                Ok(ok__) => {
                    psaturation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tint: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetTint(this, core::mem::transmute_copy(&tint)).into()
        }
        unsafe extern "system" fn GetTint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptint: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetTint(this) {
                Ok(ok__) => {
                    ptint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoiseReduction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, noisereduction: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetNoiseReduction(this, core::mem::transmute_copy(&noisereduction)).into()
        }
        unsafe extern "system" fn GetNoiseReduction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnoisereduction: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetNoiseReduction(this) {
                Ok(ok__) => {
                    pnoisereduction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolorcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetDestinationColorContext(this, windows_core::from_raw_borrowed(&pcolorcontext)).into()
        }
        unsafe extern "system" fn SetToneCurve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetToneCurve(this, core::mem::transmute_copy(&cbtonecurvesize), core::mem::transmute_copy(&ptonecurve)).into()
        }
        unsafe extern "system" fn GetToneCurve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::GetToneCurve(this, core::mem::transmute_copy(&cbtonecurvebuffersize), core::mem::transmute_copy(&ptonecurve), core::mem::transmute_copy(&pcbactualtonecurvebuffersize)).into()
        }
        unsafe extern "system" fn SetRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetRotation(this) {
                Ok(ok__) => {
                    protation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendermode: WICRawRenderMode) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetRenderMode(this, core::mem::transmute_copy(&rendermode)).into()
        }
        unsafe extern "system" fn GetRenderMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICDevelopRaw_Impl::GetRenderMode(this) {
                Ok(ok__) => {
                    prendermode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRaw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRaw_Impl::SetNotificationCallback(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        Self {
            base__: IWICBitmapFrameDecode_Vtbl::new::<Identity, OFFSET>(),
            QueryRawCapabilitiesInfo: QueryRawCapabilitiesInfo::<Identity, OFFSET>,
            LoadParameterSet: LoadParameterSet::<Identity, OFFSET>,
            GetCurrentParameterSet: GetCurrentParameterSet::<Identity, OFFSET>,
            SetExposureCompensation: SetExposureCompensation::<Identity, OFFSET>,
            GetExposureCompensation: GetExposureCompensation::<Identity, OFFSET>,
            SetWhitePointRGB: SetWhitePointRGB::<Identity, OFFSET>,
            GetWhitePointRGB: GetWhitePointRGB::<Identity, OFFSET>,
            SetNamedWhitePoint: SetNamedWhitePoint::<Identity, OFFSET>,
            GetNamedWhitePoint: GetNamedWhitePoint::<Identity, OFFSET>,
            SetWhitePointKelvin: SetWhitePointKelvin::<Identity, OFFSET>,
            GetWhitePointKelvin: GetWhitePointKelvin::<Identity, OFFSET>,
            GetKelvinRangeInfo: GetKelvinRangeInfo::<Identity, OFFSET>,
            SetContrast: SetContrast::<Identity, OFFSET>,
            GetContrast: GetContrast::<Identity, OFFSET>,
            SetGamma: SetGamma::<Identity, OFFSET>,
            GetGamma: GetGamma::<Identity, OFFSET>,
            SetSharpness: SetSharpness::<Identity, OFFSET>,
            GetSharpness: GetSharpness::<Identity, OFFSET>,
            SetSaturation: SetSaturation::<Identity, OFFSET>,
            GetSaturation: GetSaturation::<Identity, OFFSET>,
            SetTint: SetTint::<Identity, OFFSET>,
            GetTint: GetTint::<Identity, OFFSET>,
            SetNoiseReduction: SetNoiseReduction::<Identity, OFFSET>,
            GetNoiseReduction: GetNoiseReduction::<Identity, OFFSET>,
            SetDestinationColorContext: SetDestinationColorContext::<Identity, OFFSET>,
            SetToneCurve: SetToneCurve::<Identity, OFFSET>,
            GetToneCurve: GetToneCurve::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
            SetRenderMode: SetRenderMode::<Identity, OFFSET>,
            GetRenderMode: GetRenderMode::<Identity, OFFSET>,
            SetNotificationCallback: SetNotificationCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDevelopRaw as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID || iid == &<IWICBitmapFrameDecode as windows_core::Interface>::IID
    }
}
pub trait IWICDevelopRawNotificationCallback_Impl: Sized {
    fn Notify(&self, notificationmask: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICDevelopRawNotificationCallback {}
impl IWICDevelopRawNotificationCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICDevelopRawNotificationCallback_Vtbl
    where
        Identity: IWICDevelopRawNotificationCallback_Impl,
    {
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationmask: u32) -> windows_core::HRESULT
        where
            Identity: IWICDevelopRawNotificationCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICDevelopRawNotificationCallback_Impl::Notify(this, core::mem::transmute_copy(&notificationmask)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICDevelopRawNotificationCallback as windows_core::Interface>::IID
    }
}
pub trait IWICEnumMetadataItem_Impl: Sized {
    fn Next(&self, celt: u32, rgeltschema: *mut windows_core::PROPVARIANT, rgeltid: *mut windows_core::PROPVARIANT, rgeltvalue: *mut windows_core::PROPVARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWICEnumMetadataItem>;
}
impl windows_core::RuntimeName for IWICEnumMetadataItem {}
impl IWICEnumMetadataItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICEnumMetadataItem_Vtbl
    where
        Identity: IWICEnumMetadataItem_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgeltschema: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, rgeltid: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, rgeltvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICEnumMetadataItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICEnumMetadataItem_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgeltschema), core::mem::transmute_copy(&rgeltid), core::mem::transmute_copy(&rgeltvalue), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IWICEnumMetadataItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICEnumMetadataItem_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICEnumMetadataItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICEnumMetadataItem_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienummetadataitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICEnumMetadataItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICEnumMetadataItem_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienummetadataitem.write(core::mem::transmute(ok__));
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
        iid == &<IWICEnumMetadataItem as windows_core::Interface>::IID
    }
}
pub trait IWICFastMetadataEncoder_Impl: Sized {
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
impl windows_core::RuntimeName for IWICFastMetadataEncoder {}
impl IWICFastMetadataEncoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICFastMetadataEncoder_Vtbl
    where
        Identity: IWICFastMetadataEncoder_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICFastMetadataEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICFastMetadataEncoder_Impl::Commit(this).into()
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimetadataquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICFastMetadataEncoder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICFastMetadataEncoder_Impl::GetMetadataQueryWriter(this) {
                Ok(ok__) => {
                    ppimetadataquerywriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICFastMetadataEncoder as windows_core::Interface>::IID
    }
}
pub trait IWICFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, pisource: Option<&IWICBitmapSource>, dstformat: *const windows_core::GUID, dither: WICBitmapDitherType, pipalette: Option<&IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::Result<()>;
    fn CanConvert(&self, srcpixelformat: *const windows_core::GUID, dstpixelformat: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWICFormatConverter {}
impl IWICFormatConverter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICFormatConverter_Vtbl
    where
        Identity: IWICFormatConverter_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisource: *mut core::ffi::c_void, dstformat: *const windows_core::GUID, dither: WICBitmapDitherType, pipalette: *mut core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::HRESULT
        where
            Identity: IWICFormatConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICFormatConverter_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisource), core::mem::transmute_copy(&dstformat), core::mem::transmute_copy(&dither), windows_core::from_raw_borrowed(&pipalette), core::mem::transmute_copy(&alphathresholdpercent), core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcpixelformat: *const windows_core::GUID, dstpixelformat: *const windows_core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICFormatConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICFormatConverter_Impl::CanConvert(this, core::mem::transmute_copy(&srcpixelformat), core::mem::transmute_copy(&dstpixelformat)) {
                Ok(ok__) => {
                    pfcanconvert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, CanConvert: CanConvert::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICFormatConverter as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICFormatConverterInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetPixelFormats(&self, cformats: u32, ppixelformatguids: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::Result<()>;
    fn CreateInstance(&self) -> windows_core::Result<IWICFormatConverter>;
}
impl windows_core::RuntimeName for IWICFormatConverterInfo {}
impl IWICFormatConverterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICFormatConverterInfo_Vtbl
    where
        Identity: IWICFormatConverterInfo_Impl,
    {
        unsafe extern "system" fn GetPixelFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32, ppixelformatguids: *mut windows_core::GUID, pcactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICFormatConverterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICFormatConverterInfo_Impl::GetPixelFormats(this, core::mem::transmute_copy(&cformats), core::mem::transmute_copy(&ppixelformatguids), core::mem::transmute_copy(&pcactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiconverter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICFormatConverterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICFormatConverterInfo_Impl::CreateInstance(this) {
                Ok(ok__) => {
                    ppiconverter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPixelFormats: GetPixelFormats::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICFormatConverterInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory_Impl: Sized {
    fn CreateDecoderFromFilename(&self, wzfilename: &windows_core::PCWSTR, pguidvendor: *const windows_core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromStream(&self, pistream: Option<&super::super::System::Com::IStream>, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateComponentInfo(&self, clsidcomponent: *const windows_core::GUID) -> windows_core::Result<IWICComponentInfo>;
    fn CreateDecoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapDecoder>;
    fn CreateEncoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapEncoder>;
    fn CreatePalette(&self) -> windows_core::Result<IWICPalette>;
    fn CreateFormatConverter(&self) -> windows_core::Result<IWICFormatConverter>;
    fn CreateBitmapScaler(&self) -> windows_core::Result<IWICBitmapScaler>;
    fn CreateBitmapClipper(&self) -> windows_core::Result<IWICBitmapClipper>;
    fn CreateBitmapFlipRotator(&self) -> windows_core::Result<IWICBitmapFlipRotator>;
    fn CreateStream(&self) -> windows_core::Result<IWICStream>;
    fn CreateColorContext(&self) -> windows_core::Result<IWICColorContext>;
    fn CreateColorTransformer(&self) -> windows_core::Result<IWICColorTransform>;
    fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSource(&self, pibitmapsource: Option<&IWICBitmapSource>, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSourceRect(&self, pibitmapsource: Option<&IWICBitmapSource>, x: u32, y: u32, width: u32, height: u32) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHBITMAP(&self, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption) -> windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHICON(&self, hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<IWICBitmap>;
    fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn CreateFastMetadataEncoderFromDecoder(&self, pidecoder: Option<&IWICBitmapDecoder>) -> windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateFastMetadataEncoderFromFrameDecode(&self, piframedecoder: Option<&IWICBitmapFrameDecode>) -> windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateQueryWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateQueryWriterFromReader(&self, piqueryreader: Option<&IWICMetadataQueryReader>, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWICImagingFactory {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICImagingFactory_Vtbl
    where
        Identity: IWICImagingFactory_Impl,
    {
        unsafe extern "system" fn CreateDecoderFromFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzfilename: windows_core::PCWSTR, pguidvendor: *const windows_core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateDecoderFromFilename(this, core::mem::transmute(&wzfilename), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&metadataoptions)) {
                Ok(ok__) => {
                    ppidecoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateDecoderFromStream(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&metadataoptions)) {
                Ok(ok__) => {
                    ppidecoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateDecoderFromFileHandle(this, core::mem::transmute_copy(&hfile), core::mem::transmute_copy(&pguidvendor), core::mem::transmute_copy(&metadataoptions)) {
                Ok(ok__) => {
                    ppidecoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidcomponent: *const windows_core::GUID, ppiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateComponentInfo(this, core::mem::transmute_copy(&clsidcomponent)) {
                Ok(ok__) => {
                    ppiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecoder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, ppidecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateDecoder(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pguidvendor)) {
                Ok(ok__) => {
                    ppidecoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncoder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, ppiencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateEncoder(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&pguidvendor)) {
                Ok(ok__) => {
                    ppiencoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipalette: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreatePalette(this) {
                Ok(ok__) => {
                    ppipalette.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFormatConverter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiformatconverter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateFormatConverter(this) {
                Ok(ok__) => {
                    ppiformatconverter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapScaler<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapscaler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapScaler(this) {
                Ok(ok__) => {
                    ppibitmapscaler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapClipper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapclipper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapClipper(this) {
                Ok(ok__) => {
                    ppibitmapclipper.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppibitmapfliprotator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapFlipRotator(this) {
                Ok(ok__) => {
                    ppibitmapfliprotator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwicstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateStream(this) {
                Ok(ok__) => {
                    ppiwicstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiccolorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateColorContext(this) {
                Ok(ok__) => {
                    ppiwiccolorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateColorTransformer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiccolortransform: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateColorTransformer(this) {
                Ok(ok__) => {
                    ppiwiccolortransform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmap(this, core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&option)) {
                Ok(ok__) => {
                    ppibitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapFromSource(this, windows_core::from_raw_borrowed(&pibitmapsource), core::mem::transmute_copy(&option)) {
                Ok(ok__) => {
                    ppibitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pibitmapsource: *mut core::ffi::c_void, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapFromSourceRect(this, windows_core::from_raw_borrowed(&pibitmapsource), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)) {
                Ok(ok__) => {
                    ppibitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapFromMemory(this, core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&pixelformat), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbbuffersize), core::mem::transmute_copy(&pbbuffer)) {
                Ok(ok__) => {
                    ppibitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapFromHBITMAP(this, core::mem::transmute_copy(&hbitmap), core::mem::transmute_copy(&hpalette), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppibitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateBitmapFromHICON(this, core::mem::transmute_copy(&hicon)) {
                Ok(ok__) => {
                    ppibitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComponentEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateComponentEnumerator(this, core::mem::transmute_copy(&componenttypes), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppienumunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidecoder: *mut core::ffi::c_void, ppifastencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateFastMetadataEncoderFromDecoder(this, windows_core::from_raw_borrowed(&pidecoder)) {
                Ok(ok__) => {
                    ppifastencoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piframedecoder: *mut core::ffi::c_void, ppifastencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateFastMetadataEncoderFromFrameDecode(this, windows_core::from_raw_borrowed(&piframedecoder)) {
                Ok(ok__) => {
                    ppifastencoder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, ppiquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateQueryWriter(this, core::mem::transmute_copy(&guidmetadataformat), core::mem::transmute_copy(&pguidvendor)) {
                Ok(ok__) => {
                    ppiquerywriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piqueryreader: *mut core::ffi::c_void, pguidvendor: *const windows_core::GUID, ppiquerywriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICImagingFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICImagingFactory_Impl::CreateQueryWriterFromReader(this, windows_core::from_raw_borrowed(&piqueryreader), core::mem::transmute_copy(&pguidvendor)) {
                Ok(ok__) => {
                    ppiquerywriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDecoderFromFilename: CreateDecoderFromFilename::<Identity, OFFSET>,
            CreateDecoderFromStream: CreateDecoderFromStream::<Identity, OFFSET>,
            CreateDecoderFromFileHandle: CreateDecoderFromFileHandle::<Identity, OFFSET>,
            CreateComponentInfo: CreateComponentInfo::<Identity, OFFSET>,
            CreateDecoder: CreateDecoder::<Identity, OFFSET>,
            CreateEncoder: CreateEncoder::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateFormatConverter: CreateFormatConverter::<Identity, OFFSET>,
            CreateBitmapScaler: CreateBitmapScaler::<Identity, OFFSET>,
            CreateBitmapClipper: CreateBitmapClipper::<Identity, OFFSET>,
            CreateBitmapFlipRotator: CreateBitmapFlipRotator::<Identity, OFFSET>,
            CreateStream: CreateStream::<Identity, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, OFFSET>,
            CreateColorTransformer: CreateColorTransformer::<Identity, OFFSET>,
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromSource: CreateBitmapFromSource::<Identity, OFFSET>,
            CreateBitmapFromSourceRect: CreateBitmapFromSourceRect::<Identity, OFFSET>,
            CreateBitmapFromMemory: CreateBitmapFromMemory::<Identity, OFFSET>,
            CreateBitmapFromHBITMAP: CreateBitmapFromHBITMAP::<Identity, OFFSET>,
            CreateBitmapFromHICON: CreateBitmapFromHICON::<Identity, OFFSET>,
            CreateComponentEnumerator: CreateComponentEnumerator::<Identity, OFFSET>,
            CreateFastMetadataEncoderFromDecoder: CreateFastMetadataEncoderFromDecoder::<Identity, OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode: CreateFastMetadataEncoderFromFrameDecode::<Identity, OFFSET>,
            CreateQueryWriter: CreateQueryWriter::<Identity, OFFSET>,
            CreateQueryWriterFromReader: CreateQueryWriterFromReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameDecode_Impl: Sized {
    fn DoesSupportIndexing(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> windows_core::Result<()>;
    fn ClearIndexing(&self) -> windows_core::Result<()>;
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::Result<()>;
    fn GetFrameHeader(&self, pframeheader: *mut WICJpegFrameHeader) -> windows_core::Result<()>;
    fn GetScanHeader(&self, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> windows_core::Result<()>;
    fn CopyScan(&self, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> windows_core::Result<()>;
    fn CopyMinimalStream(&self, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IWICJpegFrameDecode {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICJpegFrameDecode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICJpegFrameDecode_Vtbl
    where
        Identity: IWICJpegFrameDecode_Impl,
    {
        unsafe extern "system" fn DoesSupportIndexing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICJpegFrameDecode_Impl::DoesSupportIndexing(this) {
                Ok(ok__) => {
                    pfindexingsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::SetIndexing(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&horizontalintervalsize)).into()
        }
        unsafe extern "system" fn ClearIndexing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::ClearIndexing(this).into()
        }
        unsafe extern "system" fn GetAcHuffmanTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::GetAcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pachuffmantable)).into()
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::GetDcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pdchuffmantable)).into()
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::GetQuantizationTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pquantizationtable)).into()
        }
        unsafe extern "system" fn GetFrameHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::GetFrameHeader(this, core::mem::transmute_copy(&pframeheader)).into()
        }
        unsafe extern "system" fn GetScanHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::GetScanHeader(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&pscanheader)).into()
        }
        unsafe extern "system" fn CopyScan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::CopyScan(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&scanoffset), core::mem::transmute_copy(&cbscandata), core::mem::transmute_copy(&pbscandata), core::mem::transmute_copy(&pcbscandataactual)).into()
        }
        unsafe extern "system" fn CopyMinimalStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameDecode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameDecode_Impl::CopyMinimalStream(this, core::mem::transmute_copy(&streamoffset), core::mem::transmute_copy(&cbstreamdata), core::mem::transmute_copy(&pbstreamdata), core::mem::transmute_copy(&pcbstreamdataactual)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportIndexing: DoesSupportIndexing::<Identity, OFFSET>,
            SetIndexing: SetIndexing::<Identity, OFFSET>,
            ClearIndexing: ClearIndexing::<Identity, OFFSET>,
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, OFFSET>,
            GetFrameHeader: GetFrameHeader::<Identity, OFFSET>,
            GetScanHeader: GetScanHeader::<Identity, OFFSET>,
            CopyScan: CopyScan::<Identity, OFFSET>,
            CopyMinimalStream: CopyMinimalStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICJpegFrameDecode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameEncode_Impl: Sized {
    fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::Result<()>;
    fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::Result<()>;
    fn WriteScan(&self, cbscandata: u32, pbscandata: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IWICJpegFrameEncode {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IWICJpegFrameEncode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICJpegFrameEncode_Vtbl
    where
        Identity: IWICJpegFrameEncode_Impl,
    {
        unsafe extern "system" fn GetAcHuffmanTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameEncode_Impl::GetAcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pachuffmantable)).into()
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameEncode_Impl::GetDcHuffmanTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pdchuffmantable)).into()
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameEncode_Impl::GetQuantizationTable(this, core::mem::transmute_copy(&scanindex), core::mem::transmute_copy(&tableindex), core::mem::transmute_copy(&pquantizationtable)).into()
        }
        unsafe extern "system" fn WriteScan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> windows_core::HRESULT
        where
            Identity: IWICJpegFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICJpegFrameEncode_Impl::WriteScan(this, core::mem::transmute_copy(&cbscandata), core::mem::transmute_copy(&pbscandata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, OFFSET>,
            WriteScan: WriteScan::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICJpegFrameEncode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockReader_Impl: Sized {
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetReaderByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataReader>;
    fn GetEnumerator(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICMetadataBlockReader {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataBlockReader_Vtbl
    where
        Identity: IWICMetadataBlockReader_Impl,
    {
        unsafe extern "system" fn GetContainerFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataBlockReader_Impl::GetContainerFormat(this) {
                Ok(ok__) => {
                    pguidcontainerformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataBlockReader_Impl::GetCount(this) {
                Ok(ok__) => {
                    pccount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReaderByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppimetadatareader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataBlockReader_Impl::GetReaderByIndex(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppimetadatareader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienummetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataBlockReader_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    ppienummetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetReaderByIndex: GetReaderByIndex::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataBlockReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockWriter_Impl: Sized + IWICMetadataBlockReader_Impl {
    fn InitializeFromBlockReader(&self, pimdblockreader: Option<&IWICMetadataBlockReader>) -> windows_core::Result<()>;
    fn GetWriterByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataWriter>;
    fn AddWriter(&self, pimetadatawriter: Option<&IWICMetadataWriter>) -> windows_core::Result<()>;
    fn SetWriterByIndex(&self, nindex: u32, pimetadatawriter: Option<&IWICMetadataWriter>) -> windows_core::Result<()>;
    fn RemoveWriterByIndex(&self, nindex: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICMetadataBlockWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataBlockWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataBlockWriter_Vtbl
    where
        Identity: IWICMetadataBlockWriter_Impl,
    {
        unsafe extern "system" fn InitializeFromBlockReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimdblockreader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataBlockWriter_Impl::InitializeFromBlockReader(this, windows_core::from_raw_borrowed(&pimdblockreader)).into()
        }
        unsafe extern "system" fn GetWriterByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataBlockWriter_Impl::GetWriterByIndex(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppimetadatawriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimetadatawriter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataBlockWriter_Impl::AddWriter(this, windows_core::from_raw_borrowed(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn SetWriterByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pimetadatawriter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataBlockWriter_Impl::SetWriterByIndex(this, core::mem::transmute_copy(&nindex), windows_core::from_raw_borrowed(&pimetadatawriter)).into()
        }
        unsafe extern "system" fn RemoveWriterByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataBlockWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataBlockWriter_Impl::RemoveWriterByIndex(this, core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base__: IWICMetadataBlockReader_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromBlockReader: InitializeFromBlockReader::<Identity, OFFSET>,
            GetWriterByIndex: GetWriterByIndex::<Identity, OFFSET>,
            AddWriter: AddWriter::<Identity, OFFSET>,
            SetWriterByIndex: SetWriterByIndex::<Identity, OFFSET>,
            RemoveWriterByIndex: RemoveWriterByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataBlockWriter as windows_core::Interface>::IID || iid == &<IWICMetadataBlockReader as windows_core::Interface>::IID
    }
}
pub trait IWICMetadataHandlerInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetContainerFormats(&self, ccontainerformats: u32, pguidcontainerformats: *mut windows_core::GUID, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceManufacturer(&self, cchdevicemanufacturer: u32, wzdevicemanufacturer: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceModels(&self, cchdevicemodels: u32, wzdevicemodels: &windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::Result<()>;
    fn DoesRequireFullStream(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportPadding(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesRequireFixedSize(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWICMetadataHandlerInfo {}
impl IWICMetadataHandlerInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataHandlerInfo_Vtbl
    where
        Identity: IWICMetadataHandlerInfo_Impl,
    {
        unsafe extern "system" fn GetMetadataFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidmetadataformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataHandlerInfo_Impl::GetMetadataFormat(this) {
                Ok(ok__) => {
                    pguidmetadataformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut windows_core::GUID, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataHandlerInfo_Impl::GetContainerFormats(this, core::mem::transmute_copy(&ccontainerformats), core::mem::transmute_copy(&pguidcontainerformats), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataHandlerInfo_Impl::GetDeviceManufacturer(this, core::mem::transmute_copy(&cchdevicemanufacturer), core::mem::transmute(&wzdevicemanufacturer), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn GetDeviceModels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: windows_core::PWSTR, pcchactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataHandlerInfo_Impl::GetDeviceModels(this, core::mem::transmute_copy(&cchdevicemodels), core::mem::transmute(&wzdevicemodels), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn DoesRequireFullStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataHandlerInfo_Impl::DoesRequireFullStream(this) {
                Ok(ok__) => {
                    pfrequiresfullstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesSupportPadding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataHandlerInfo_Impl::DoesSupportPadding(this) {
                Ok(ok__) => {
                    pfsupportspadding.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoesRequireFixedSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICMetadataHandlerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataHandlerInfo_Impl::DoesRequireFixedSize(this) {
                Ok(ok__) => {
                    pffixedsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, OFFSET>,
            GetContainerFormats: GetContainerFormats::<Identity, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, OFFSET>,
            DoesRequireFullStream: DoesRequireFullStream::<Identity, OFFSET>,
            DoesSupportPadding: DoesSupportPadding::<Identity, OFFSET>,
            DoesRequireFixedSize: DoesRequireFixedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataHandlerInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataQueryReader_Impl: Sized {
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetLocation(&self, cchmaxlength: u32, wznamespace: &windows_core::PWSTR, pcchactuallength: *mut u32) -> windows_core::Result<()>;
    fn GetMetadataByName(&self, wzname: &windows_core::PCWSTR, pvarvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICMetadataQueryReader {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataQueryReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataQueryReader_Vtbl
    where
        Identity: IWICMetadataQueryReader_Impl,
    {
        unsafe extern "system" fn GetContainerFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontainerformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICMetadataQueryReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataQueryReader_Impl::GetContainerFormat(this) {
                Ok(ok__) => {
                    pguidcontainerformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchmaxlength: u32, wznamespace: windows_core::PWSTR, pcchactuallength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataQueryReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataQueryReader_Impl::GetLocation(this, core::mem::transmute_copy(&cchmaxlength), core::mem::transmute(&wznamespace), core::mem::transmute_copy(&pcchactuallength)).into()
        }
        unsafe extern "system" fn GetMetadataByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzname: windows_core::PCWSTR, pvarvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataQueryReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataQueryReader_Impl::GetMetadataByName(this, core::mem::transmute(&wzname), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataQueryReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataQueryReader_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    ppienumstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetLocation: GetLocation::<Identity, OFFSET>,
            GetMetadataByName: GetMetadataByName::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataQueryReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataQueryWriter_Impl: Sized + IWICMetadataQueryReader_Impl {
    fn SetMetadataByName(&self, wzname: &windows_core::PCWSTR, pvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn RemoveMetadataByName(&self, wzname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICMetadataQueryWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataQueryWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataQueryWriter_Vtbl
    where
        Identity: IWICMetadataQueryWriter_Impl,
    {
        unsafe extern "system" fn SetMetadataByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzname: windows_core::PCWSTR, pvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataQueryWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataQueryWriter_Impl::SetMetadataByName(this, core::mem::transmute(&wzname), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveMetadataByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWICMetadataQueryWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataQueryWriter_Impl::RemoveMetadataByName(this, core::mem::transmute(&wzname)).into()
        }
        Self {
            base__: IWICMetadataQueryReader_Vtbl::new::<Identity, OFFSET>(),
            SetMetadataByName: SetMetadataByName::<Identity, OFFSET>,
            RemoveMetadataByName: RemoveMetadataByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataQueryWriter as windows_core::Interface>::IID || iid == &<IWICMetadataQueryReader as windows_core::Interface>::IID
    }
}
pub trait IWICMetadataReader_Impl: Sized {
    fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetMetadataHandlerInfo(&self) -> windows_core::Result<IWICMetadataHandlerInfo>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut windows_core::PROPVARIANT, pvarid: *mut windows_core::PROPVARIANT, pvarvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValue(&self, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT, pvarvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IWICEnumMetadataItem>;
}
impl windows_core::RuntimeName for IWICMetadataReader {}
impl IWICMetadataReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataReader_Vtbl
    where
        Identity: IWICMetadataReader_Impl,
    {
        unsafe extern "system" fn GetMetadataFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidmetadataformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataReader_Impl::GetMetadataFormat(this) {
                Ok(ok__) => {
                    pguidmetadataformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppihandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataReader_Impl::GetMetadataHandlerInfo(this) {
                Ok(ok__) => {
                    ppihandler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataReader_Impl::GetCount(this) {
                Ok(ok__) => {
                    pccount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pvarschema: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarid: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataReader_Impl::GetValueByIndex(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarschema: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataReader_Impl::GetValue(this, core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienummetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataReader_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    ppienummetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMetadataFormat: GetMetadataFormat::<Identity, OFFSET>,
            GetMetadataHandlerInfo: GetMetadataHandlerInfo::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValueByIndex: GetValueByIndex::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataReaderInfo_Impl: Sized + IWICMetadataHandlerInfo_Impl {
    fn GetPatterns(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> windows_core::Result<()>;
    fn MatchesPattern(&self, guidcontainerformat: *const windows_core::GUID, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(&self) -> windows_core::Result<IWICMetadataReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICMetadataReaderInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IWICMetadataReaderInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataReaderInfo_Vtbl
    where
        Identity: IWICMetadataReaderInfo_Impl,
    {
        unsafe extern "system" fn GetPatterns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataReaderInfo_Impl::GetPatterns(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&ppattern), core::mem::transmute_copy(&pccount), core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn MatchesPattern<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, pistream: *mut core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataReaderInfo_Impl::MatchesPattern(this, core::mem::transmute_copy(&guidcontainerformat), windows_core::from_raw_borrowed(&pistream)) {
                Ok(ok__) => {
                    pfmatches.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppireader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataReaderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataReaderInfo_Impl::CreateInstance(this) {
                Ok(ok__) => {
                    ppireader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICMetadataHandlerInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPatterns: GetPatterns::<Identity, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataReaderInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICMetadataHandlerInfo as windows_core::Interface>::IID
    }
}
pub trait IWICMetadataWriter_Impl: Sized + IWICMetadataReader_Impl {
    fn SetValue(&self, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT, pvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn SetValueByIndex(&self, nindex: u32, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT, pvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn RemoveValue(&self, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn RemoveValueByIndex(&self, nindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICMetadataWriter {}
impl IWICMetadataWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataWriter_Vtbl
    where
        Identity: IWICMetadataWriter_Impl,
    {
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarschema: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataWriter_Impl::SetValue(this, core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn SetValueByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pvarschema: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataWriter_Impl::SetValueByIndex(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn RemoveValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarschema: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pvarid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWICMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataWriter_Impl::RemoveValue(this, core::mem::transmute_copy(&pvarschema), core::mem::transmute_copy(&pvarid)).into()
        }
        unsafe extern "system" fn RemoveValueByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataWriter_Impl::RemoveValueByIndex(this, core::mem::transmute_copy(&nindex)).into()
        }
        Self {
            base__: IWICMetadataReader_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, OFFSET>,
            SetValueByIndex: SetValueByIndex::<Identity, OFFSET>,
            RemoveValue: RemoveValue::<Identity, OFFSET>,
            RemoveValueByIndex: RemoveValueByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataWriter as windows_core::Interface>::IID || iid == &<IWICMetadataReader as windows_core::Interface>::IID
    }
}
pub trait IWICMetadataWriterInfo_Impl: Sized + IWICMetadataHandlerInfo_Impl {
    fn GetHeader(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> windows_core::Result<()>;
    fn CreateInstance(&self) -> windows_core::Result<IWICMetadataWriter>;
}
impl windows_core::RuntimeName for IWICMetadataWriterInfo {}
impl IWICMetadataWriterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICMetadataWriterInfo_Vtbl
    where
        Identity: IWICMetadataWriterInfo_Impl,
    {
        unsafe extern "system" fn GetHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcontainerformat: *const windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICMetadataWriterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICMetadataWriterInfo_Impl::GetHeader(this, core::mem::transmute_copy(&guidcontainerformat), core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pheader), core::mem::transmute_copy(&pcbactual)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICMetadataWriterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICMetadataWriterInfo_Impl::CreateInstance(this) {
                Ok(ok__) => {
                    ppiwriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICMetadataHandlerInfo_Vtbl::new::<Identity, OFFSET>(),
            GetHeader: GetHeader::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICMetadataWriterInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICMetadataHandlerInfo as windows_core::Interface>::IID
    }
}
pub trait IWICPalette_Impl: Sized {
    fn InitializePredefined(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InitializeCustom(&self, pcolors: *const u32, ccount: u32) -> windows_core::Result<()>;
    fn InitializeFromBitmap(&self, pisurface: Option<&IWICBitmapSource>, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InitializeFromPalette(&self, pipalette: Option<&IWICPalette>) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<WICBitmapPaletteType>;
    fn GetColorCount(&self) -> windows_core::Result<u32>;
    fn GetColors(&self, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> windows_core::Result<()>;
    fn IsBlackWhite(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsGrayscale(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn HasAlpha(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWICPalette {}
impl IWICPalette_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPalette_Vtbl
    where
        Identity: IWICPalette_Impl,
    {
        unsafe extern "system" fn InitializePredefined<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPalette_Impl::InitializePredefined(this, core::mem::transmute_copy(&epalettetype), core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeCustom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolors: *const u32, ccount: u32) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPalette_Impl::InitializeCustom(this, core::mem::transmute_copy(&pcolors), core::mem::transmute_copy(&ccount)).into()
        }
        unsafe extern "system" fn InitializeFromBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisurface: *mut core::ffi::c_void, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPalette_Impl::InitializeFromBitmap(this, windows_core::from_raw_borrowed(&pisurface), core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&faddtransparentcolor)).into()
        }
        unsafe extern "system" fn InitializeFromPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipalette: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPalette_Impl::InitializeFromPalette(this, windows_core::from_raw_borrowed(&pipalette)).into()
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPalette_Impl::GetType(this) {
                Ok(ok__) => {
                    pepalettetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPalette_Impl::GetColorCount(this) {
                Ok(ok__) => {
                    pccount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPalette_Impl::GetColors(this, core::mem::transmute_copy(&ccount), core::mem::transmute_copy(&pcolors), core::mem::transmute_copy(&pcactualcolors)).into()
        }
        unsafe extern "system" fn IsBlackWhite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPalette_Impl::IsBlackWhite(this) {
                Ok(ok__) => {
                    pfisblackwhite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGrayscale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPalette_Impl::IsGrayscale(this) {
                Ok(ok__) => {
                    pfisgrayscale.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasAlpha<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPalette_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPalette_Impl::HasAlpha(this) {
                Ok(ok__) => {
                    pfhasalpha.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializePredefined: InitializePredefined::<Identity, OFFSET>,
            InitializeCustom: InitializeCustom::<Identity, OFFSET>,
            InitializeFromBitmap: InitializeFromBitmap::<Identity, OFFSET>,
            InitializeFromPalette: InitializeFromPalette::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetColorCount: GetColorCount::<Identity, OFFSET>,
            GetColors: GetColors::<Identity, OFFSET>,
            IsBlackWhite: IsBlackWhite::<Identity, OFFSET>,
            IsGrayscale: IsGrayscale::<Identity, OFFSET>,
            HasAlpha: HasAlpha::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPalette as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICPersistStream_Impl: Sized + super::super::System::Com::IPersistStream_Impl {
    fn LoadEx(&self, pistream: Option<&super::super::System::Com::IStream>, pguidpreferredvendor: *const windows_core::GUID, dwpersistoptions: u32) -> windows_core::Result<()>;
    fn SaveEx(&self, pistream: Option<&super::super::System::Com::IStream>, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICPersistStream {}
#[cfg(feature = "Win32_System_Com")]
impl IWICPersistStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPersistStream_Vtbl
    where
        Identity: IWICPersistStream_Impl,
    {
        unsafe extern "system" fn LoadEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pguidpreferredvendor: *const windows_core::GUID, dwpersistoptions: u32) -> windows_core::HRESULT
        where
            Identity: IWICPersistStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPersistStream_Impl::LoadEx(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&pguidpreferredvendor), core::mem::transmute_copy(&dwpersistoptions)).into()
        }
        unsafe extern "system" fn SaveEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPersistStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPersistStream_Impl::SaveEx(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&dwpersistoptions), core::mem::transmute_copy(&fcleardirty)).into()
        }
        Self {
            base__: super::super::System::Com::IPersistStream_Vtbl::new::<Identity, OFFSET>(),
            LoadEx: LoadEx::<Identity, OFFSET>,
            SaveEx: SaveEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPersistStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersistStream as windows_core::Interface>::IID
    }
}
pub trait IWICPixelFormatInfo_Impl: Sized + IWICComponentInfo_Impl {
    fn GetFormatGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetColorContext(&self) -> windows_core::Result<IWICColorContext>;
    fn GetBitsPerPixel(&self) -> windows_core::Result<u32>;
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetChannelMask(&self, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICPixelFormatInfo {}
impl IWICPixelFormatInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPixelFormatInfo_Vtbl
    where
        Identity: IWICPixelFormatInfo_Impl,
    {
        unsafe extern "system" fn GetFormatGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPixelFormatInfo_Impl::GetFormatGUID(this) {
                Ok(ok__) => {
                    pformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicolorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPixelFormatInfo_Impl::GetColorContext(this) {
                Ok(ok__) => {
                    ppicolorcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitsPerPixel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puibitsperpixel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPixelFormatInfo_Impl::GetBitsPerPixel(this) {
                Ok(ok__) => {
                    puibitsperpixel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puichannelcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPixelFormatInfo_Impl::GetChannelCount(this) {
                Ok(ok__) => {
                    puichannelcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPixelFormatInfo_Impl::GetChannelMask(this, core::mem::transmute_copy(&uichannelindex), core::mem::transmute_copy(&cbmaskbuffer), core::mem::transmute_copy(&pbmaskbuffer), core::mem::transmute_copy(&pcbactual)).into()
        }
        Self {
            base__: IWICComponentInfo_Vtbl::new::<Identity, OFFSET>(),
            GetFormatGUID: GetFormatGUID::<Identity, OFFSET>,
            GetColorContext: GetColorContext::<Identity, OFFSET>,
            GetBitsPerPixel: GetBitsPerPixel::<Identity, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            GetChannelMask: GetChannelMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID
    }
}
pub trait IWICPixelFormatInfo2_Impl: Sized + IWICPixelFormatInfo_Impl {
    fn SupportsTransparency(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetNumericRepresentation(&self) -> windows_core::Result<WICPixelFormatNumericRepresentation>;
}
impl windows_core::RuntimeName for IWICPixelFormatInfo2 {}
impl IWICPixelFormatInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPixelFormatInfo2_Vtbl
    where
        Identity: IWICPixelFormatInfo2_Impl,
    {
        unsafe extern "system" fn SupportsTransparency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPixelFormatInfo2_Impl::SupportsTransparency(this) {
                Ok(ok__) => {
                    pfsupportstransparency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericRepresentation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> windows_core::HRESULT
        where
            Identity: IWICPixelFormatInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPixelFormatInfo2_Impl::GetNumericRepresentation(this) {
                Ok(ok__) => {
                    pnumericrepresentation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWICPixelFormatInfo_Vtbl::new::<Identity, OFFSET>(),
            SupportsTransparency: SupportsTransparency::<Identity, OFFSET>,
            GetNumericRepresentation: GetNumericRepresentation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPixelFormatInfo2 as windows_core::Interface>::IID || iid == &<IWICComponentInfo as windows_core::Interface>::IID || iid == &<IWICPixelFormatInfo as windows_core::Interface>::IID
    }
}
pub trait IWICPlanarBitmapFrameEncode_Impl: Sized {
    fn WritePixels(&self, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::Result<()>;
    fn WriteSource(&self, ppplanes: *const Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICPlanarBitmapFrameEncode {}
impl IWICPlanarBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPlanarBitmapFrameEncode_Vtbl
    where
        Identity: IWICPlanarBitmapFrameEncode_Impl,
    {
        unsafe extern "system" fn WritePixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::HRESULT
        where
            Identity: IWICPlanarBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPlanarBitmapFrameEncode_Impl::WritePixels(this, core::mem::transmute_copy(&linecount), core::mem::transmute_copy(&pplanes), core::mem::transmute_copy(&cplanes)).into()
        }
        unsafe extern "system" fn WriteSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplanes: *const *mut core::ffi::c_void, cplanes: u32, prcsource: *const WICRect) -> windows_core::HRESULT
        where
            Identity: IWICPlanarBitmapFrameEncode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPlanarBitmapFrameEncode_Impl::WriteSource(this, core::mem::transmute_copy(&ppplanes), core::mem::transmute_copy(&cplanes), core::mem::transmute_copy(&prcsource)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WritePixels: WritePixels::<Identity, OFFSET>,
            WriteSource: WriteSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPlanarBitmapFrameEncode as windows_core::Interface>::IID
    }
}
pub trait IWICPlanarBitmapSourceTransform_Impl: Sized {
    fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICPlanarBitmapSourceTransform {}
impl IWICPlanarBitmapSourceTransform_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPlanarBitmapSourceTransform_Vtbl
    where
        Identity: IWICPlanarBitmapSourceTransform_Impl,
    {
        unsafe extern "system" fn DoesSupportTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPlanarBitmapSourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPlanarBitmapSourceTransform_Impl::DoesSupportTransform(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&dstplanaroptions), core::mem::transmute_copy(&pguiddstformats), core::mem::transmute_copy(&pplanedescriptions), core::mem::transmute_copy(&cplanes), core::mem::transmute_copy(&pfissupported)).into()
        }
        unsafe extern "system" fn CopyPixels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> windows_core::HRESULT
        where
            Identity: IWICPlanarBitmapSourceTransform_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPlanarBitmapSourceTransform_Impl::CopyPixels(this, core::mem::transmute_copy(&prcsource), core::mem::transmute_copy(&uiwidth), core::mem::transmute_copy(&uiheight), core::mem::transmute_copy(&dsttransform), core::mem::transmute_copy(&dstplanaroptions), core::mem::transmute_copy(&pdstplanes), core::mem::transmute_copy(&cplanes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoesSupportTransform: DoesSupportTransform::<Identity, OFFSET>,
            CopyPixels: CopyPixels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPlanarBitmapSourceTransform as windows_core::Interface>::IID
    }
}
pub trait IWICPlanarFormatConverter_Impl: Sized + IWICBitmapSource_Impl {
    fn Initialize(&self, ppplanes: *const Option<IWICBitmapSource>, cplanes: u32, dstformat: *const windows_core::GUID, dither: WICBitmapDitherType, pipalette: Option<&IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::Result<()>;
    fn CanConvert(&self, psrcpixelformats: *const windows_core::GUID, csrcplanes: u32, dstpixelformat: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWICPlanarFormatConverter {}
impl IWICPlanarFormatConverter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICPlanarFormatConverter_Vtbl
    where
        Identity: IWICPlanarFormatConverter_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplanes: *const *mut core::ffi::c_void, cplanes: u32, dstformat: *const windows_core::GUID, dither: WICBitmapDitherType, pipalette: *mut core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::HRESULT
        where
            Identity: IWICPlanarFormatConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICPlanarFormatConverter_Impl::Initialize(this, core::mem::transmute_copy(&ppplanes), core::mem::transmute_copy(&cplanes), core::mem::transmute_copy(&dstformat), core::mem::transmute_copy(&dither), windows_core::from_raw_borrowed(&pipalette), core::mem::transmute_copy(&alphathresholdpercent), core::mem::transmute_copy(&palettetranslate)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcpixelformats: *const windows_core::GUID, csrcplanes: u32, dstpixelformat: *const windows_core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWICPlanarFormatConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICPlanarFormatConverter_Impl::CanConvert(this, core::mem::transmute_copy(&psrcpixelformats), core::mem::transmute_copy(&csrcplanes), core::mem::transmute_copy(&dstpixelformat)) {
                Ok(ok__) => {
                    pfcanconvert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWICBitmapSource_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, CanConvert: CanConvert::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICPlanarFormatConverter as windows_core::Interface>::IID || iid == &<IWICBitmapSource as windows_core::Interface>::IID
    }
}
pub trait IWICProgressCallback_Impl: Sized {
    fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICProgressCallback {}
impl IWICProgressCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICProgressCallback_Vtbl
    where
        Identity: IWICProgressCallback_Impl,
    {
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::HRESULT
        where
            Identity: IWICProgressCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICProgressCallback_Impl::Notify(this, core::mem::transmute_copy(&uframenum), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&dblprogress)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICProgressCallback as windows_core::Interface>::IID
    }
}
pub trait IWICProgressiveLevelControl_Impl: Sized {
    fn GetLevelCount(&self) -> windows_core::Result<u32>;
    fn GetCurrentLevel(&self) -> windows_core::Result<u32>;
    fn SetCurrentLevel(&self, nlevel: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWICProgressiveLevelControl {}
impl IWICProgressiveLevelControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICProgressiveLevelControl_Vtbl
    where
        Identity: IWICProgressiveLevelControl_Impl,
    {
        unsafe extern "system" fn GetLevelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclevels: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICProgressiveLevelControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICProgressiveLevelControl_Impl::GetLevelCount(this) {
                Ok(ok__) => {
                    pclevels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICProgressiveLevelControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICProgressiveLevelControl_Impl::GetCurrentLevel(this) {
                Ok(ok__) => {
                    pnlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nlevel: u32) -> windows_core::HRESULT
        where
            Identity: IWICProgressiveLevelControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICProgressiveLevelControl_Impl::SetCurrentLevel(this, core::mem::transmute_copy(&nlevel)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, OFFSET>,
            SetCurrentLevel: SetCurrentLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICProgressiveLevelControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStream_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn InitializeFromIStream(&self, pistream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn InitializeFromFilename(&self, wzfilename: &windows_core::PCWSTR, dwdesiredaccess: u32) -> windows_core::Result<()>;
    fn InitializeFromMemory(&self, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::Result<()>;
    fn InitializeFromIStreamRegion(&self, pistream: Option<&super::super::System::Com::IStream>, uloffset: u64, ulmaxsize: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICStream {}
#[cfg(feature = "Win32_System_Com")]
impl IWICStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICStream_Vtbl
    where
        Identity: IWICStream_Impl,
    {
        unsafe extern "system" fn InitializeFromIStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICStream_Impl::InitializeFromIStream(this, windows_core::from_raw_borrowed(&pistream)).into()
        }
        unsafe extern "system" fn InitializeFromFilename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzfilename: windows_core::PCWSTR, dwdesiredaccess: u32) -> windows_core::HRESULT
        where
            Identity: IWICStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICStream_Impl::InitializeFromFilename(this, core::mem::transmute(&wzfilename), core::mem::transmute_copy(&dwdesiredaccess)).into()
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> windows_core::HRESULT
        where
            Identity: IWICStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICStream_Impl::InitializeFromMemory(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&cbbuffersize)).into()
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, uloffset: u64, ulmaxsize: u64) -> windows_core::HRESULT
        where
            Identity: IWICStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICStream_Impl::InitializeFromIStreamRegion(this, windows_core::from_raw_borrowed(&pistream), core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&ulmaxsize)).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromIStream: InitializeFromIStream::<Identity, OFFSET>,
            InitializeFromFilename: InitializeFromFilename::<Identity, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, OFFSET>,
            InitializeFromIStreamRegion: InitializeFromIStreamRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamProvider_Impl: Sized {
    fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn GetPersistOptions(&self) -> windows_core::Result<u32>;
    fn GetPreferredVendorGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn RefreshStream(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWICStreamProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IWICStreamProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWICStreamProvider_Vtbl
    where
        Identity: IWICStreamProvider_Impl,
    {
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICStreamProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICStreamProvider_Impl::GetStream(this) {
                Ok(ok__) => {
                    ppistream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpersistoptions: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWICStreamProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICStreamProvider_Impl::GetPersistOptions(this) {
                Ok(ok__) => {
                    pdwpersistoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidpreferredvendor: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWICStreamProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWICStreamProvider_Impl::GetPreferredVendorGUID(this) {
                Ok(ok__) => {
                    pguidpreferredvendor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWICStreamProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWICStreamProvider_Impl::RefreshStream(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStream: GetStream::<Identity, OFFSET>,
            GetPersistOptions: GetPersistOptions::<Identity, OFFSET>,
            GetPreferredVendorGUID: GetPreferredVendorGUID::<Identity, OFFSET>,
            RefreshStream: RefreshStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICStreamProvider as windows_core::Interface>::IID
    }
}

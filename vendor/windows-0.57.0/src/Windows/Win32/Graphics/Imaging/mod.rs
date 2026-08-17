#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
#[inline]
pub unsafe fn WICConvertBitmapSource<P0>(dstformat: *const windows_core::GUID, pisrc: P0) -> windows_core::Result<IWICBitmapSource>
where
    P0: windows_core::Param<IWICBitmapSource>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICConvertBitmapSource(dstformat : *const windows_core::GUID, pisrc : * mut core::ffi::c_void, ppidst : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WICConvertBitmapSource(dstformat, pisrc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WICCreateBitmapFromSection<P0>(width: u32, height: u32, pixelformat: *const windows_core::GUID, hsection: P0, stride: u32, offset: u32) -> windows_core::Result<IWICBitmap>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICCreateBitmapFromSection(width : u32, height : u32, pixelformat : *const windows_core::GUID, hsection : super::super::Foundation:: HANDLE, stride : u32, offset : u32, ppibitmap : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WICCreateBitmapFromSection(width, height, pixelformat, hsection.param().abi(), stride, offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WICCreateBitmapFromSectionEx<P0>(width: u32, height: u32, pixelformat: *const windows_core::GUID, hsection: P0, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel) -> windows_core::Result<IWICBitmap>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICCreateBitmapFromSectionEx(width : u32, height : u32, pixelformat : *const windows_core::GUID, hsection : super::super::Foundation:: HANDLE, stride : u32, offset : u32, desiredaccesslevel : WICSectionAccessLevel, ppibitmap : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WICCreateBitmapFromSectionEx(width, height, pixelformat, hsection.param().abi(), stride, offset, desiredaccesslevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WICGetMetadataContentSize<P0>(guidcontainerformat: *const windows_core::GUID, piwriter: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<IWICMetadataWriter>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICGetMetadataContentSize(guidcontainerformat : *const windows_core::GUID, piwriter : * mut core::ffi::c_void, pcbsize : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WICGetMetadataContentSize(guidcontainerformat, piwriter.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WICMapGuidToShortName(guid: *const windows_core::GUID, wzname: Option<&mut [u16]>, pcchactual: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("windowscodecs.dll" "system" fn WICMapGuidToShortName(guid : *const windows_core::GUID, cchname : u32, wzname : windows_core::PWSTR, pcchactual : *mut u32) -> windows_core::HRESULT);
    WICMapGuidToShortName(guid, wzname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(wzname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcchactual).ok()
}
#[inline]
pub unsafe fn WICMapSchemaToName<P0>(guidmetadataformat: *const windows_core::GUID, pwzschema: P0, wzname: Option<&mut [u16]>, pcchactual: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICMapSchemaToName(guidmetadataformat : *const windows_core::GUID, pwzschema : windows_core::PCWSTR, cchname : u32, wzname : windows_core::PWSTR, pcchactual : *mut u32) -> windows_core::HRESULT);
    WICMapSchemaToName(guidmetadataformat, pwzschema.param().abi(), wzname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(wzname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcchactual).ok()
}
#[inline]
pub unsafe fn WICMapShortNameToGuid<P0>(wzname: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICMapShortNameToGuid(wzname : windows_core::PCWSTR, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WICMapShortNameToGuid(wzname.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WICMatchMetadataContent<P0>(guidcontainerformat: *const windows_core::GUID, pguidvendor: Option<*const windows_core::GUID>, pistream: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICMatchMetadataContent(guidcontainerformat : *const windows_core::GUID, pguidvendor : *const windows_core::GUID, pistream : * mut core::ffi::c_void, pguidmetadataformat : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WICMatchMetadataContent(guidcontainerformat, core::mem::transmute(pguidvendor.unwrap_or(std::ptr::null())), pistream.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WICSerializeMetadataContent<P0, P1>(guidcontainerformat: *const windows_core::GUID, piwriter: P0, dwpersistoptions: u32, pistream: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<IWICMetadataWriter>,
    P1: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("windowscodecs.dll" "system" fn WICSerializeMetadataContent(guidcontainerformat : *const windows_core::GUID, piwriter : * mut core::ffi::c_void, dwpersistoptions : u32, pistream : * mut core::ffi::c_void) -> windows_core::HRESULT);
    WICSerializeMetadataContent(guidcontainerformat, piwriter.param().abi(), dwpersistoptions, pistream.param().abi()).ok()
}
windows_core::imp::define_interface!(IWICBitmap, IWICBitmap_Vtbl, 0x00000121_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmap {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmap, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmap {
    pub unsafe fn Lock(&self, prclock: *const WICRect, flags: u32) -> windows_core::Result<IWICBitmapLock> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), prclock, flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()).ok()
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResolution)(windows_core::Interface::as_raw(self), dpix, dpiy).ok()
    }
}
#[repr(C)]
pub struct IWICBitmap_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapClipper, IWICBitmapClipper_Vtbl, 0xe4fbcf03_223d_4e81_9333_d635556dd1b5);
impl core::ops::Deref for IWICBitmapClipper {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapClipper, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapClipper {
    pub unsafe fn Initialize<P0>(&self, pisource: P0, prc: *const WICRect) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), prc).ok()
    }
}
#[repr(C)]
pub struct IWICBitmapClipper_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WICRect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapCodecInfo, IWICBitmapCodecInfo_Vtbl, 0xe87a44c4_b76e_4c47_8b09_298eb12a2714);
impl core::ops::Deref for IWICBitmapCodecInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapCodecInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICBitmapCodecInfo {
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPixelFormats(&self, pguidpixelformats: &mut [windows_core::GUID], pcactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPixelFormats)(windows_core::Interface::as_raw(self), pguidpixelformats.len().try_into().unwrap(), core::mem::transmute(pguidpixelformats.as_ptr()), pcactual).ok()
    }
    pub unsafe fn GetColorManagementVersion(&self, wzcolormanagementversion: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColorManagementVersion)(windows_core::Interface::as_raw(self), wzcolormanagementversion.len().try_into().unwrap(), core::mem::transmute(wzcolormanagementversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceManufacturer)(windows_core::Interface::as_raw(self), wzdevicemanufacturer.len().try_into().unwrap(), core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceModels)(windows_core::Interface::as_raw(self), wzdevicemodels.len().try_into().unwrap(), core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMimeTypes(&self, wzmimetypes: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMimeTypes)(windows_core::Interface::as_raw(self), wzmimetypes.len().try_into().unwrap(), core::mem::transmute(wzmimetypes.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFileExtensions(&self, wzfileextensions: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileExtensions)(windows_core::Interface::as_raw(self), wzfileextensions.len().try_into().unwrap(), core::mem::transmute(wzfileextensions.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn DoesSupportAnimation(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportAnimation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DoesSupportChromakey(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportChromakey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DoesSupportLossless(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportLossless)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DoesSupportMultiframe(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportMultiframe)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MatchesMimeType)(windows_core::Interface::as_raw(self), wzmimetype.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICBitmapCodecInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPixelFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetColorManagementVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceModels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetMimeTypes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub DoesSupportAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DoesSupportChromakey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DoesSupportLossless: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DoesSupportMultiframe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MatchesMimeType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapCodecProgressNotification, IWICBitmapCodecProgressNotification_Vtbl, 0x64c1024e_c3cf_4462_8078_88c2b11c46d9);
impl core::ops::Deref for IWICBitmapCodecProgressNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapCodecProgressNotification, windows_core::IUnknown);
impl IWICBitmapCodecProgressNotification {
    pub unsafe fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: Option<*const core::ffi::c_void>, dwprogressflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterProgressNotification)(windows_core::Interface::as_raw(self), pfnprogressnotification, core::mem::transmute(pvdata.unwrap_or(std::ptr::null())), dwprogressflags).ok()
    }
}
#[repr(C)]
pub struct IWICBitmapCodecProgressNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProgressNotification: unsafe extern "system" fn(*mut core::ffi::c_void, PFNProgressNotification, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapDecoder, IWICBitmapDecoder_Vtbl, 0x9edde9e7_8dee_47ea_99df_e6faf2ed44bf);
impl core::ops::Deref for IWICBitmapDecoder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapDecoder, windows_core::IUnknown);
impl IWICBitmapDecoder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCapability<P0>(&self, pistream: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryCapability)(windows_core::Interface::as_raw(self), pistream.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoptions: WICDecodeOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pistream.param().abi(), cacheoptions).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDecoderInfo(&self) -> windows_core::Result<IWICBitmapDecoderInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDecoderInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).CopyPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataQueryReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPreview(&self) -> windows_core::Result<IWICBitmapSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreview)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetColorContexts(&self, ppicolorcontexts: &mut [Option<IWICColorContext>], pcactualcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColorContexts)(windows_core::Interface::as_raw(self), ppicolorcontexts.len().try_into().unwrap(), core::mem::transmute(ppicolorcontexts.as_ptr()), pcactualcount).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFrameCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFrame(&self, index: u32) -> windows_core::Result<IWICBitmapFrameDecode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrame)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICBitmapDecoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryCapability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryCapability: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICDecodeOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDecoderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapDecoderInfo, IWICBitmapDecoderInfo_Vtbl, 0xd8cd007f_d08f_4191_9bfc_236ea7f0e4b5);
impl core::ops::Deref for IWICBitmapDecoderInfo {
    type Target = IWICBitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapDecoderInfo, windows_core::IUnknown, IWICComponentInfo, IWICBitmapCodecInfo);
impl IWICBitmapDecoderInfo {
    pub unsafe fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: Option<*mut WICBitmapPattern>, pcpatterns: Option<*mut u32>, pcbpatternsactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPatterns)(windows_core::Interface::as_raw(self), cbsizepatterns, core::mem::transmute(ppatterns.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcpatterns.unwrap_or(std::ptr::null_mut())), pcbpatternsactual).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MatchesPattern<P0>(&self, pistream: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MatchesPattern)(windows_core::Interface::as_raw(self), pistream.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICBitmapDecoder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICBitmapDecoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    pub GetPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICBitmapPattern, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MatchesPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MatchesPattern: usize,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapEncoder, IWICBitmapEncoder_Vtbl, 0x00000103_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmapEncoder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapEncoder, windows_core::IUnknown);
impl IWICBitmapEncoder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pistream.param().abi(), cacheoption).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEncoderInfo(&self) -> windows_core::Result<IWICBitmapEncoderInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEncoderInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetColorContexts(&self, ppicolorcontext: &[Option<IWICColorContext>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorContexts)(windows_core::Interface::as_raw(self), ppicolorcontext.len().try_into().unwrap(), core::mem::transmute(ppicolorcontext.as_ptr())).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()).ok()
    }
    pub unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).SetThumbnail)(windows_core::Interface::as_raw(self), pithumbnail.param().abi()).ok()
    }
    pub unsafe fn SetPreview<P0>(&self, pipreview: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).SetPreview)(windows_core::Interface::as_raw(self), pipreview.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateNewFrame)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiframeencode), core::mem::transmute(ppiencoderoptions)).ok()
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICBitmapEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapEncoderCacheOption) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetEncoderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateNewFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateNewFrame: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapEncoderInfo, IWICBitmapEncoderInfo_Vtbl, 0x94c9b4ee_a09f_4f92_8a1e_4a9bce7e76fb);
impl core::ops::Deref for IWICBitmapEncoderInfo {
    type Target = IWICBitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapEncoderInfo, windows_core::IUnknown, IWICComponentInfo, IWICBitmapCodecInfo);
impl IWICBitmapEncoderInfo {
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICBitmapEncoder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICBitmapEncoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapFlipRotator, IWICBitmapFlipRotator_Vtbl, 0x5009834f_2d6a_41ce_9e1b_17c5aff7a782);
impl core::ops::Deref for IWICBitmapFlipRotator {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapFlipRotator, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapFlipRotator {
    pub unsafe fn Initialize<P0>(&self, pisource: P0, options: WICBitmapTransformOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), options).ok()
    }
}
#[repr(C)]
pub struct IWICBitmapFlipRotator_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapTransformOptions) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapFrameDecode, IWICBitmapFrameDecode_Vtbl, 0x3b16811b_6a43_4ec9_a813_3d930c13b940);
impl core::ops::Deref for IWICBitmapFrameDecode {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapFrameDecode, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapFrameDecode {
    pub unsafe fn GetMetadataQueryReader(&self) -> windows_core::Result<IWICMetadataQueryReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataQueryReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetColorContexts(&self, ppicolorcontexts: &mut [Option<IWICColorContext>], pcactualcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColorContexts)(windows_core::Interface::as_raw(self), ppicolorcontexts.len().try_into().unwrap(), core::mem::transmute(ppicolorcontexts.as_ptr()), pcactualcount).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> windows_core::Result<IWICBitmapSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThumbnail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICBitmapFrameDecode_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub GetMetadataQueryReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapFrameEncode, IWICBitmapFrameEncode_Vtbl, 0x00000105_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmapFrameEncode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapFrameEncode, windows_core::IUnknown);
impl IWICBitmapFrameEncode {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0>(&self, piencoderoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IPropertyBag2>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), piencoderoptions.param().abi()).ok()
    }
    pub unsafe fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), uiwidth, uiheight).ok()
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResolution)(windows_core::Interface::as_raw(self), dpix, dpiy).ok()
    }
    pub unsafe fn SetPixelFormat(&self, ppixelformat: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPixelFormat)(windows_core::Interface::as_raw(self), ppixelformat).ok()
    }
    pub unsafe fn SetColorContexts(&self, ppicolorcontext: &[Option<IWICColorContext>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorContexts)(windows_core::Interface::as_raw(self), ppicolorcontext.len().try_into().unwrap(), core::mem::transmute(ppicolorcontext.as_ptr())).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()).ok()
    }
    pub unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).SetThumbnail)(windows_core::Interface::as_raw(self), pithumbnail.param().abi()).ok()
    }
    pub unsafe fn WritePixels(&self, linecount: u32, cbstride: u32, pbpixels: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WritePixels)(windows_core::Interface::as_raw(self), linecount, cbstride, pbpixels.len().try_into().unwrap(), core::mem::transmute(pbpixels.as_ptr())).ok()
    }
    pub unsafe fn WriteSource<P0>(&self, pibitmapsource: P0, prc: *const WICRect) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).WriteSource)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), prc).ok()
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICBitmapFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub SetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WritePixels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *const u8) -> windows_core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WICRect) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapLock, IWICBitmapLock_Vtbl, 0x00000123_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmapLock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapLock, windows_core::IUnknown);
impl IWICBitmapLock {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetStride(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataPointer)(windows_core::Interface::as_raw(self), pcbbuffersize, ppbdata).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICBitmapLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapScaler, IWICBitmapScaler_Vtbl, 0x00000302_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmapScaler {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapScaler, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmapScaler {
    pub unsafe fn Initialize<P0>(&self, pisource: P0, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), uiwidth, uiheight, mode).ok()
    }
}
#[repr(C)]
pub struct IWICBitmapScaler_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, WICBitmapInterpolationMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapSource, IWICBitmapSource_Vtbl, 0x00000120_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmapSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
impl IWICBitmapSource {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResolution)(windows_core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).CopyPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prc, cbstride, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub CopyPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICBitmapSourceTransform, IWICBitmapSourceTransform_Vtbl, 0x3b16811b_6a43_4ec9_b713_3d5a0c13b940);
impl core::ops::Deref for IWICBitmapSourceTransform {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmapSourceTransform, windows_core::IUnknown);
impl IWICBitmapSourceTransform {
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, pbbuffer: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prc, uiwidth, uiheight, pguiddstformat, dsttransform, nstride, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClosestSize)(windows_core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetClosestPixelFormat(&self, pguiddstformat: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClosestPixelFormat)(windows_core::Interface::as_raw(self), pguiddstformat).ok()
    }
    pub unsafe fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportTransform)(windows_core::Interface::as_raw(self), dsttransform, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICBitmapSourceTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *const windows_core::GUID, WICBitmapTransformOptions, u32, u32, *mut u8) -> windows_core::HRESULT,
    pub GetClosestSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetClosestPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DoesSupportTransform: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapTransformOptions, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICColorContext, IWICColorContext_Vtbl, 0x3c613a02_34b2_44ea_9a7c_45aea9c6fd6d);
impl core::ops::Deref for IWICColorContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICColorContext, windows_core::IUnknown);
impl IWICColorContext {
    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi()).ok()
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromMemory)(windows_core::Interface::as_raw(self), core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn InitializeFromExifColorSpace(&self, value: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromExifColorSpace)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<WICColorContextType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProfileBytes(&self, pbbuffer: &mut [u8], pcbactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProfileBytes)(windows_core::Interface::as_raw(self), pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr()), pcbactual).ok()
    }
    pub unsafe fn GetExifColorSpace(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExifColorSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICColorContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub InitializeFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub InitializeFromExifColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICColorContextType) -> windows_core::HRESULT,
    pub GetProfileBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetExifColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICColorTransform, IWICColorTransform_Vtbl, 0xb66f034f_d0e2_40ab_b436_6de39e321a94);
impl core::ops::Deref for IWICColorTransform {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICColorTransform, windows_core::IUnknown, IWICBitmapSource);
impl IWICColorTransform {
    pub unsafe fn Initialize<P0, P1, P2>(&self, pibitmapsource: P0, picontextsource: P1, picontextdest: P2, pixelfmtdest: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
        P1: windows_core::Param<IWICColorContext>,
        P2: windows_core::Param<IWICColorContext>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), picontextsource.param().abi(), picontextdest.param().abi(), pixelfmtdest).ok()
    }
}
#[repr(C)]
pub struct IWICColorTransform_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICComponentFactory, IWICComponentFactory_Vtbl, 0x412d0c3a_9650_44fa_af5b_dd2a06c8e8fb);
impl core::ops::Deref for IWICComponentFactory {
    type Target = IWICImagingFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICComponentFactory, windows_core::IUnknown, IWICImagingFactory);
impl IWICComponentFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMetadataReader<P0>(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: P0) -> windows_core::Result<IWICMetadataReader>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMetadataReader)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, dwoptions, pistream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMetadataReaderFromContainer<P0>(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwoptions: u32, pistream: P0) -> windows_core::Result<IWICMetadataReader>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMetadataReaderFromContainer)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, dwoptions, pistream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMetadataWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID, dwmetadataoptions: u32) -> windows_core::Result<IWICMetadataWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMetadataWriter)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, dwmetadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMetadataWriterFromReader<P0>(&self, pireader: P0, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataWriter>
    where
        P0: windows_core::Param<IWICMetadataReader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMetadataWriterFromReader)(windows_core::Interface::as_raw(self), pireader.param().abi(), pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateQueryReaderFromBlockReader<P0>(&self, piblockreader: P0) -> windows_core::Result<IWICMetadataQueryReader>
    where
        P0: windows_core::Param<IWICMetadataBlockReader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQueryReaderFromBlockReader)(windows_core::Interface::as_raw(self), piblockreader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateQueryWriterFromBlockWriter<P0>(&self, piblockwriter: P0) -> windows_core::Result<IWICMetadataQueryWriter>
    where
        P0: windows_core::Param<IWICMetadataBlockWriter>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQueryWriterFromBlockWriter)(windows_core::Interface::as_raw(self), piblockwriter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateEncoderPropertyBag(&self, ppropoptions: &[super::super::System::Com::StructuredStorage::PROPBAG2]) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncoderPropertyBag)(windows_core::Interface::as_raw(self), core::mem::transmute(ppropoptions.as_ptr()), ppropoptions.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICComponentFactory_Vtbl {
    pub base__: IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMetadataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMetadataReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMetadataReaderFromContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMetadataReaderFromContainer: usize,
    pub CreateMetadataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMetadataWriterFromReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryReaderFromBlockReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWriterFromBlockWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub CreateEncoderPropertyBag: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::StructuredStorage::PROPBAG2, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    CreateEncoderPropertyBag: usize,
}
windows_core::imp::define_interface!(IWICComponentInfo, IWICComponentInfo_Vtbl, 0x23bc3f0a_698b_4357_886b_f24d50671334);
impl core::ops::Deref for IWICComponentInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICComponentInfo, windows_core::IUnknown);
impl IWICComponentInfo {
    pub unsafe fn GetComponentType(&self) -> windows_core::Result<WICComponentType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetComponentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSigningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAuthor)(windows_core::Interface::as_raw(self), wzauthor.len().try_into().unwrap(), core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVendorGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), wzversion.len().try_into().unwrap(), core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSpecVersion)(windows_core::Interface::as_raw(self), wzspecversion.len().try_into().unwrap(), core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), wzfriendlyname.len().try_into().unwrap(), core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
}
#[repr(C)]
pub struct IWICComponentInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetComponentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICComponentType) -> windows_core::HRESULT,
    pub GetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetSigningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetVendorGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetSpecVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICDdsDecoder, IWICDdsDecoder_Vtbl, 0x409cd537_8532_40cb_9774_e2feb2df4e9c);
impl core::ops::Deref for IWICDdsDecoder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDdsDecoder, windows_core::IUnknown);
impl IWICDdsDecoder {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pparameters).ok()
    }
    pub unsafe fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> windows_core::Result<IWICBitmapFrameDecode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrame)(windows_core::Interface::as_raw(self), arrayindex, miplevel, sliceindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICDdsDecoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICDdsParameters) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetParameters: usize,
    pub GetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICDdsEncoder, IWICDdsEncoder_Vtbl, 0x5cacdb4c_407e_41b3_b936_d0f010cd6732);
impl core::ops::Deref for IWICDdsEncoder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDdsEncoder, windows_core::IUnknown);
impl IWICDdsEncoder {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetParameters(&self, pparameters: *const WICDdsParameters) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), pparameters).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pparameters).ok()
    }
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateNewFrame)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiframeencode), parrayindex, pmiplevel, psliceindex).ok()
    }
}
#[repr(C)]
pub struct IWICDdsEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICDdsParameters) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetParameters: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICDdsParameters) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetParameters: usize,
    pub CreateNewFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICDdsFrameDecode, IWICDdsFrameDecode_Vtbl, 0x3d4c0c61_18a4_41e4_bd80_481a4fc9f464);
impl core::ops::Deref for IWICDdsFrameDecode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDdsFrameDecode, windows_core::IUnknown);
impl IWICDdsFrameDecode {
    pub unsafe fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSizeInBlocks)(windows_core::Interface::as_raw(self), pwidthinblocks, pheightinblocks).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFormatInfo(&self) -> windows_core::Result<WICDdsFormatInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormatInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyBlocks)(windows_core::Interface::as_raw(self), prcboundsinblocks, cbstride, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IWICDdsFrameDecode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSizeInBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFormatInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICDdsFormatInfo) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFormatInfo: usize,
    pub CopyBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICDevelopRaw, IWICDevelopRaw_Vtbl, 0xfbec5e44_f7be_4b65_b7f8_c0c81fef026d);
impl core::ops::Deref for IWICDevelopRaw {
    type Target = IWICBitmapFrameDecode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDevelopRaw, windows_core::IUnknown, IWICBitmapSource, IWICBitmapFrameDecode);
impl IWICDevelopRaw {
    pub unsafe fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryRawCapabilitiesInfo)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadParameterSet)(windows_core::Interface::as_raw(self), parameterset).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetCurrentParameterSet(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentParameterSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExposureCompensation(&self, ev: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExposureCompensation)(windows_core::Interface::as_raw(self), ev).ok()
    }
    pub unsafe fn GetExposureCompensation(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExposureCompensation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWhitePointRGB)(windows_core::Interface::as_raw(self), red, green, blue).ok()
    }
    pub unsafe fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWhitePointRGB)(windows_core::Interface::as_raw(self), pred, pgreen, pblue).ok()
    }
    pub unsafe fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNamedWhitePoint)(windows_core::Interface::as_raw(self), whitepoint).ok()
    }
    pub unsafe fn GetNamedWhitePoint(&self) -> windows_core::Result<WICNamedWhitePoint> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamedWhitePoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWhitePointKelvin)(windows_core::Interface::as_raw(self), whitepointkelvin).ok()
    }
    pub unsafe fn GetWhitePointKelvin(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWhitePointKelvin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKelvinRangeInfo)(windows_core::Interface::as_raw(self), pminkelvintemp, pmaxkelvintemp, pkelvintempstepvalue).ok()
    }
    pub unsafe fn SetContrast(&self, contrast: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContrast)(windows_core::Interface::as_raw(self), contrast).ok()
    }
    pub unsafe fn GetContrast(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContrast)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGamma(&self, gamma: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGamma)(windows_core::Interface::as_raw(self), gamma).ok()
    }
    pub unsafe fn GetGamma(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGamma)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSharpness(&self, sharpness: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSharpness)(windows_core::Interface::as_raw(self), sharpness).ok()
    }
    pub unsafe fn GetSharpness(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSharpness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSaturation(&self, saturation: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSaturation)(windows_core::Interface::as_raw(self), saturation).ok()
    }
    pub unsafe fn GetSaturation(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSaturation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTint(&self, tint: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTint)(windows_core::Interface::as_raw(self), tint).ok()
    }
    pub unsafe fn GetTint(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNoiseReduction(&self, noisereduction: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNoiseReduction)(windows_core::Interface::as_raw(self), noisereduction).ok()
    }
    pub unsafe fn GetNoiseReduction(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNoiseReduction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDestinationColorContext<P0>(&self, pcolorcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICColorContext>,
    {
        (windows_core::Interface::vtable(self).SetDestinationColorContext)(windows_core::Interface::as_raw(self), pcolorcontext.param().abi()).ok()
    }
    pub unsafe fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetToneCurve)(windows_core::Interface::as_raw(self), cbtonecurvesize, ptonecurve).ok()
    }
    pub unsafe fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: Option<*mut WICRawToneCurve>, pcbactualtonecurvebuffersize: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetToneCurve)(windows_core::Interface::as_raw(self), cbtonecurvebuffersize, core::mem::transmute(ptonecurve.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbactualtonecurvebuffersize.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetRotation(&self, rotation: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), rotation).ok()
    }
    pub unsafe fn GetRotation(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRenderMode)(windows_core::Interface::as_raw(self), rendermode).ok()
    }
    pub unsafe fn GetRenderMode(&self) -> windows_core::Result<WICRawRenderMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRenderMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotificationCallback<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICDevelopRawNotificationCallback>,
    {
        (windows_core::Interface::vtable(self).SetNotificationCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWICDevelopRaw_Vtbl {
    pub base__: IWICBitmapFrameDecode_Vtbl,
    pub QueryRawCapabilitiesInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICRawCapabilitiesInfo) -> windows_core::HRESULT,
    pub LoadParameterSet: unsafe extern "system" fn(*mut core::ffi::c_void, WICRawParameterSet) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetCurrentParameterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetCurrentParameterSet: usize,
    pub SetExposureCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetExposureCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetWhitePointRGB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetWhitePointRGB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetNamedWhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, WICNamedWhitePoint) -> windows_core::HRESULT,
    pub GetNamedWhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICNamedWhitePoint) -> windows_core::HRESULT,
    pub SetWhitePointKelvin: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetWhitePointKelvin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetKelvinRangeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetGamma: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetGamma: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetSharpness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSaturation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetSaturation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetTint: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetTint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNoiseReduction: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetNoiseReduction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDestinationColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetToneCurve: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const WICRawToneCurve) -> windows_core::HRESULT,
    pub GetToneCurve: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICRawToneCurve, *mut u32) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, WICRawRenderMode) -> windows_core::HRESULT,
    pub GetRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICRawRenderMode) -> windows_core::HRESULT,
    pub SetNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICDevelopRawNotificationCallback, IWICDevelopRawNotificationCallback_Vtbl, 0x95c75a6e_3e8c_4ec2_85a8_aebcc551e59b);
impl core::ops::Deref for IWICDevelopRawNotificationCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICDevelopRawNotificationCallback, windows_core::IUnknown);
impl IWICDevelopRawNotificationCallback {
    pub unsafe fn Notify(&self, notificationmask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), notificationmask).ok()
    }
}
#[repr(C)]
pub struct IWICDevelopRawNotificationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICEnumMetadataItem, IWICEnumMetadataItem_Vtbl, 0xdc2bb46d_3f07_481e_8625_220c4aedbb33);
impl core::ops::Deref for IWICEnumMetadataItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICEnumMetadataItem, windows_core::IUnknown);
impl IWICEnumMetadataItem {
    pub unsafe fn Next(&self, celt: u32, rgeltschema: *mut windows_core::PROPVARIANT, rgeltid: *mut windows_core::PROPVARIANT, rgeltvalue: *mut windows_core::PROPVARIANT, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgeltschema), core::mem::transmute(rgeltid), core::mem::transmute(rgeltvalue), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWICEnumMetadataItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICEnumMetadataItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICFastMetadataEncoder, IWICFastMetadataEncoder_Vtbl, 0xb84e2c09_78c9_4ac4_8bd3_524ae1663a2f);
impl core::ops::Deref for IWICFastMetadataEncoder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICFastMetadataEncoder, windows_core::IUnknown);
impl IWICFastMetadataEncoder {
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICFastMetadataEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICFormatConverter, IWICFormatConverter_Vtbl, 0x00000301_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICFormatConverter {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICFormatConverter, windows_core::IUnknown, IWICBitmapSource);
impl IWICFormatConverter {
    pub unsafe fn Initialize<P0, P1>(&self, pisource: P0, dstformat: *const windows_core::GUID, dither: WICBitmapDitherType, pipalette: P1, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
        P1: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisource.param().abi(), dstformat, dither, pipalette.param().abi(), alphathresholdpercent, palettetranslate).ok()
    }
    pub unsafe fn CanConvert(&self, srcpixelformat: *const windows_core::GUID, dstpixelformat: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), srcpixelformat, dstpixelformat, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, WICBitmapDitherType, *mut core::ffi::c_void, f64, WICBitmapPaletteType) -> windows_core::HRESULT,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICFormatConverterInfo, IWICFormatConverterInfo_Vtbl, 0x9f34fb65_13f4_4f15_bc57_3726b5e53d9f);
impl core::ops::Deref for IWICFormatConverterInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICFormatConverterInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICFormatConverterInfo {
    pub unsafe fn GetPixelFormats(&self, ppixelformatguids: &mut [windows_core::GUID], pcactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPixelFormats)(windows_core::Interface::as_raw(self), ppixelformatguids.len().try_into().unwrap(), core::mem::transmute(ppixelformatguids.as_ptr()), pcactual).ok()
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICFormatConverter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICFormatConverterInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetPixelFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICImagingFactory, IWICImagingFactory_Vtbl, 0xec5ec8a9_c395_4314_9c77_54d7a935ff70);
impl core::ops::Deref for IWICImagingFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICImagingFactory, windows_core::IUnknown);
impl IWICImagingFactory {
    pub unsafe fn CreateDecoderFromFilename<P0>(&self, wzfilename: P0, pguidvendor: Option<*const windows_core::GUID>, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDecoderFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi(), core::mem::transmute(pguidvendor.unwrap_or(std::ptr::null())), dwdesiredaccess, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<P0>(&self, pistream: P0, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDecoderFromStream)(windows_core::Interface::as_raw(self), pistream.param().abi(), pguidvendor, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const windows_core::GUID, metadataoptions: WICDecodeOptions) -> windows_core::Result<IWICBitmapDecoder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDecoderFromFileHandle)(windows_core::Interface::as_raw(self), hfile, pguidvendor, metadataoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const windows_core::GUID) -> windows_core::Result<IWICComponentInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateComponentInfo)(windows_core::Interface::as_raw(self), clsidcomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapDecoder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDecoder)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICBitmapEncoder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncoder)(windows_core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePalette(&self) -> windows_core::Result<IWICPalette> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFormatConverter(&self) -> windows_core::Result<IWICFormatConverter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFormatConverter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapScaler(&self) -> windows_core::Result<IWICBitmapScaler> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapScaler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapClipper(&self) -> windows_core::Result<IWICBitmapClipper> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> windows_core::Result<IWICBitmapFlipRotator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFlipRotator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> windows_core::Result<IWICStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorContext(&self) -> windows_core::Result<IWICColorContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateColorTransformer(&self) -> windows_core::Result<IWICColorTransform> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateColorTransformer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromSource)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromSourceRect)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), x, y, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, cbstride: u32, pbbuffer: &[u8]) -> windows_core::Result<IWICBitmap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromMemory)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<P0, P1>(&self, hbitmap: P0, hpalette: P1, options: WICBitmapAlphaChannelOption) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<super::Gdi::HBITMAP>,
        P1: windows_core::Param<super::Gdi::HPALETTE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromHBITMAP)(windows_core::Interface::as_raw(self), hbitmap.param().abi(), hpalette.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<P0>(&self, hicon: P0) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HICON>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapFromHICON)(windows_core::Interface::as_raw(self), hicon.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateComponentEnumerator)(windows_core::Interface::as_raw(self), componenttypes, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<P0>(&self, pidecoder: P0) -> windows_core::Result<IWICFastMetadataEncoder>
    where
        P0: windows_core::Param<IWICBitmapDecoder>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFastMetadataEncoderFromDecoder)(windows_core::Interface::as_raw(self), pidecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<P0>(&self, piframedecoder: P0) -> windows_core::Result<IWICFastMetadataEncoder>
    where
        P0: windows_core::Param<IWICBitmapFrameDecode>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFastMetadataEncoderFromFrameDecode)(windows_core::Interface::as_raw(self), piframedecoder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const windows_core::GUID, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQueryWriter)(windows_core::Interface::as_raw(self), guidmetadataformat, pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateQueryWriterFromReader<P0>(&self, piqueryreader: P0, pguidvendor: *const windows_core::GUID) -> windows_core::Result<IWICMetadataQueryWriter>
    where
        P0: windows_core::Param<IWICMetadataQueryReader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQueryWriterFromReader)(windows_core::Interface::as_raw(self), piqueryreader.param().abi(), pguidvendor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICImagingFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDecoderFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, super::super::Foundation::GENERIC_ACCESS_RIGHTS, WICDecodeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDecoderFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, WICDecodeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDecoderFromStream: usize,
    pub CreateDecoderFromFileHandle: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, WICDecodeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComponentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFormatConverter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapScaler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFlipRotator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    pub CreateColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateColorTransformer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, WICBitmapCreateCacheOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapCreateCacheOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, u32, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateBitmapFromHBITMAP: unsafe extern "system" fn(*mut core::ffi::c_void, super::Gdi::HBITMAP, super::Gdi::HPALETTE, WICBitmapAlphaChannelOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateBitmapFromHBITMAP: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub CreateBitmapFromHICON: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowsAndMessaging::HICON, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    CreateBitmapFromHICON: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateComponentEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateComponentEnumerator: usize,
    pub CreateFastMetadataEncoderFromDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFastMetadataEncoderFromFrameDecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWriterFromReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICJpegFrameDecode, IWICJpegFrameDecode_Vtbl, 0x8939f66e_c46a_4c21_a9d1_98b327ce1679);
impl core::ops::Deref for IWICJpegFrameDecode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICJpegFrameDecode, windows_core::IUnknown);
impl IWICJpegFrameDecode {
    pub unsafe fn DoesSupportIndexing(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportIndexing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIndexing)(windows_core::Interface::as_raw(self), options, horizontalintervalsize).ok()
    }
    pub unsafe fn ClearIndexing(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearIndexing)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pachuffmantable).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pdchuffmantable).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuantizationTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pquantizationtable).ok()
    }
    pub unsafe fn GetFrameHeader(&self, pframeheader: *mut WICJpegFrameHeader) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFrameHeader)(windows_core::Interface::as_raw(self), pframeheader).ok()
    }
    pub unsafe fn GetScanHeader(&self, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScanHeader)(windows_core::Interface::as_raw(self), scanindex, pscanheader).ok()
    }
    pub unsafe fn CopyScan(&self, scanindex: u32, scanoffset: u32, pbscandata: &mut [u8], pcbscandataactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyScan)(windows_core::Interface::as_raw(self), scanindex, scanoffset, pbscandata.len().try_into().unwrap(), core::mem::transmute(pbscandata.as_ptr()), pcbscandataactual).ok()
    }
    pub unsafe fn CopyMinimalStream(&self, streamoffset: u32, pbstreamdata: &mut [u8], pcbstreamdataactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyMinimalStream)(windows_core::Interface::as_raw(self), streamoffset, pbstreamdata.len().try_into().unwrap(), core::mem::transmute(pbstreamdata.as_ptr()), pcbstreamdataactual).ok()
    }
}
#[repr(C)]
pub struct IWICJpegFrameDecode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoesSupportIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetIndexing: unsafe extern "system" fn(*mut core::ffi::c_void, WICJpegIndexingOptions, u32) -> windows_core::HRESULT,
    pub ClearIndexing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetAcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetAcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetQuantizationTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetQuantizationTable: usize,
    pub GetFrameHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICJpegFrameHeader) -> windows_core::HRESULT,
    pub GetScanHeader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WICJpegScanHeader) -> windows_core::HRESULT,
    pub CopyScan: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub CopyMinimalStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICJpegFrameEncode, IWICJpegFrameEncode_Vtbl, 0x2f0c601f_d2c6_468c_abfa_49495d983ed1);
impl core::ops::Deref for IWICJpegFrameEncode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICJpegFrameEncode, windows_core::IUnknown);
impl IWICJpegFrameEncode {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pachuffmantable).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDcHuffmanTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pdchuffmantable).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuantizationTable)(windows_core::Interface::as_raw(self), scanindex, tableindex, pquantizationtable).ok()
    }
    pub unsafe fn WriteScan(&self, pbscandata: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteScan)(windows_core::Interface::as_raw(self), pbscandata.len().try_into().unwrap(), core::mem::transmute(pbscandata.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IWICJpegFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetAcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetAcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDcHuffmanTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetQuantizationTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetQuantizationTable: usize,
    pub WriteScan: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataBlockReader, IWICMetadataBlockReader_Vtbl, 0xfeaa2a8d_b3f3_43e4_b25c_d1de990a1ae1);
impl core::ops::Deref for IWICMetadataBlockReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataBlockReader, windows_core::IUnknown);
impl IWICMetadataBlockReader {
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReaderByIndex)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICMetadataBlockReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetReaderByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnumerator: usize,
}
windows_core::imp::define_interface!(IWICMetadataBlockWriter, IWICMetadataBlockWriter_Vtbl, 0x08fb9676_b444_41e8_8dbe_6a53a542bff1);
impl core::ops::Deref for IWICMetadataBlockWriter {
    type Target = IWICMetadataBlockReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataBlockWriter, windows_core::IUnknown, IWICMetadataBlockReader);
impl IWICMetadataBlockWriter {
    pub unsafe fn InitializeFromBlockReader<P0>(&self, pimdblockreader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICMetadataBlockReader>,
    {
        (windows_core::Interface::vtable(self).InitializeFromBlockReader)(windows_core::Interface::as_raw(self), pimdblockreader.param().abi()).ok()
    }
    pub unsafe fn GetWriterByIndex(&self, nindex: u32) -> windows_core::Result<IWICMetadataWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWriterByIndex)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddWriter<P0>(&self, pimetadatawriter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICMetadataWriter>,
    {
        (windows_core::Interface::vtable(self).AddWriter)(windows_core::Interface::as_raw(self), pimetadatawriter.param().abi()).ok()
    }
    pub unsafe fn SetWriterByIndex<P0>(&self, nindex: u32, pimetadatawriter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICMetadataWriter>,
    {
        (windows_core::Interface::vtable(self).SetWriterByIndex)(windows_core::Interface::as_raw(self), nindex, pimetadatawriter.param().abi()).ok()
    }
    pub unsafe fn RemoveWriterByIndex(&self, nindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveWriterByIndex)(windows_core::Interface::as_raw(self), nindex).ok()
    }
}
#[repr(C)]
pub struct IWICMetadataBlockWriter_Vtbl {
    pub base__: IWICMetadataBlockReader_Vtbl,
    pub InitializeFromBlockReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWriterByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWriterByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveWriterByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataHandlerInfo, IWICMetadataHandlerInfo_Vtbl, 0xaba958bf_c672_44d1_8d61_ce6df2e682c2);
impl core::ops::Deref for IWICMetadataHandlerInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataHandlerInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICMetadataHandlerInfo {
    pub unsafe fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetContainerFormats(&self, pguidcontainerformats: &mut [windows_core::GUID], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContainerFormats)(windows_core::Interface::as_raw(self), pguidcontainerformats.len().try_into().unwrap(), core::mem::transmute(pguidcontainerformats.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceManufacturer)(windows_core::Interface::as_raw(self), wzdevicemanufacturer.len().try_into().unwrap(), core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceModels)(windows_core::Interface::as_raw(self), wzdevicemodels.len().try_into().unwrap(), core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn DoesRequireFullStream(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesRequireFullStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DoesSupportPadding(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesSupportPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DoesRequireFixedSize(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoesRequireFixedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICMetadataHandlerInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetMetadataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetContainerFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceModels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub DoesRequireFullStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DoesSupportPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DoesRequireFixedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataQueryReader, IWICMetadataQueryReader_Vtbl, 0x30989668_e1c9_4597_b395_458eedb808df);
impl core::ops::Deref for IWICMetadataQueryReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataQueryReader, windows_core::IUnknown);
impl IWICMetadataQueryReader {
    pub unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContainerFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLocation(&self, wznamespace: &mut [u16], pcchactuallength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocation)(windows_core::Interface::as_raw(self), wznamespace.len().try_into().unwrap(), core::mem::transmute(wznamespace.as_ptr()), pcchactuallength).ok()
    }
    pub unsafe fn GetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetMetadataByName)(windows_core::Interface::as_raw(self), wzname.param().abi(), core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICMetadataQueryReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetMetadataByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnumerator: usize,
}
windows_core::imp::define_interface!(IWICMetadataQueryWriter, IWICMetadataQueryWriter_Vtbl, 0xa721791a_0def_4d06_bd91_2118bf1db10b);
impl core::ops::Deref for IWICMetadataQueryWriter {
    type Target = IWICMetadataQueryReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataQueryWriter, windows_core::IUnknown, IWICMetadataQueryReader);
impl IWICMetadataQueryWriter {
    pub unsafe fn SetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMetadataByName)(windows_core::Interface::as_raw(self), wzname.param().abi(), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn RemoveMetadataByName<P0>(&self, wzname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveMetadataByName)(windows_core::Interface::as_raw(self), wzname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWICMetadataQueryWriter_Vtbl {
    pub base__: IWICMetadataQueryReader_Vtbl,
    pub SetMetadataByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub RemoveMetadataByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataReader, IWICMetadataReader_Vtbl, 0x9204fe99_d8fc_4fd5_a001_9536b067a899);
impl core::ops::Deref for IWICMetadataReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataReader, windows_core::IUnknown);
impl IWICMetadataReader {
    pub unsafe fn GetMetadataFormat(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMetadataHandlerInfo(&self) -> windows_core::Result<IWICMetadataHandlerInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadataHandlerInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut windows_core::PROPVARIANT, pvarid: *mut windows_core::PROPVARIANT, pvarvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValueByIndex)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute(pvarschema), core::mem::transmute(pvarid), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn GetValue(&self, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT, pvarvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarschema), core::mem::transmute(pvarid), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IWICEnumMetadataItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICMetadataReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMetadataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMetadataHandlerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataReaderInfo, IWICMetadataReaderInfo_Vtbl, 0xeebf1f5b_07c1_4447_a3ab_22acaf78a804);
impl core::ops::Deref for IWICMetadataReaderInfo {
    type Target = IWICMetadataHandlerInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataReaderInfo, windows_core::IUnknown, IWICComponentInfo, IWICMetadataHandlerInfo);
impl IWICMetadataReaderInfo {
    pub unsafe fn GetPatterns(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, ppattern: Option<*mut WICMetadataPattern>, pccount: Option<*mut u32>, pcbactual: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPatterns)(windows_core::Interface::as_raw(self), guidcontainerformat, cbsize, core::mem::transmute(ppattern.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pccount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MatchesPattern<P0>(&self, guidcontainerformat: *const windows_core::GUID, pistream: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MatchesPattern)(windows_core::Interface::as_raw(self), guidcontainerformat, pistream.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICMetadataReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICMetadataReaderInfo_Vtbl {
    pub base__: IWICMetadataHandlerInfo_Vtbl,
    pub GetPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut WICMetadataPattern, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MatchesPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MatchesPattern: usize,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataWriter, IWICMetadataWriter_Vtbl, 0xf7836e16_3be0_470b_86bb_160d0aecd7de);
impl core::ops::Deref for IWICMetadataWriter {
    type Target = IWICMetadataReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataWriter, windows_core::IUnknown, IWICMetadataReader);
impl IWICMetadataWriter {
    pub unsafe fn SetValue(&self, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT, pvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarschema), core::mem::transmute(pvarid), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn SetValueByIndex(&self, nindex: u32, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT, pvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueByIndex)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute(pvarschema), core::mem::transmute(pvarid), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn RemoveValue(&self, pvarschema: *const windows_core::PROPVARIANT, pvarid: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarschema), core::mem::transmute(pvarid)).ok()
    }
    pub unsafe fn RemoveValueByIndex(&self, nindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveValueByIndex)(windows_core::Interface::as_raw(self), nindex).ok()
    }
}
#[repr(C)]
pub struct IWICMetadataWriter_Vtbl {
    pub base__: IWICMetadataReader_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub SetValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub RemoveValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub RemoveValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICMetadataWriterInfo, IWICMetadataWriterInfo_Vtbl, 0xb22e3fba_3925_4323_b5c1_9ebfc430f236);
impl core::ops::Deref for IWICMetadataWriterInfo {
    type Target = IWICMetadataHandlerInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICMetadataWriterInfo, windows_core::IUnknown, IWICComponentInfo, IWICMetadataHandlerInfo);
impl IWICMetadataWriterInfo {
    pub unsafe fn GetHeader(&self, guidcontainerformat: *const windows_core::GUID, cbsize: u32, pheader: Option<*mut WICMetadataHeader>, pcbactual: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHeader)(windows_core::Interface::as_raw(self), guidcontainerformat, cbsize, core::mem::transmute(pheader.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateInstance(&self) -> windows_core::Result<IWICMetadataWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICMetadataWriterInfo_Vtbl {
    pub base__: IWICMetadataHandlerInfo_Vtbl,
    pub GetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut WICMetadataHeader, *mut u32) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICPalette, IWICPalette_Vtbl, 0x00000040_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICPalette {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPalette, windows_core::IUnknown);
impl IWICPalette {
    pub unsafe fn InitializePredefined<P0>(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).InitializePredefined)(windows_core::Interface::as_raw(self), epalettetype, faddtransparentcolor.param().abi()).ok()
    }
    pub unsafe fn InitializeCustom(&self, pcolors: &[u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeCustom)(windows_core::Interface::as_raw(self), core::mem::transmute(pcolors.as_ptr()), pcolors.len().try_into().unwrap()).ok()
    }
    pub unsafe fn InitializeFromBitmap<P0, P1>(&self, pisurface: P0, ccount: u32, faddtransparentcolor: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICBitmapSource>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).InitializeFromBitmap)(windows_core::Interface::as_raw(self), pisurface.param().abi(), ccount, faddtransparentcolor.param().abi()).ok()
    }
    pub unsafe fn InitializeFromPalette<P0>(&self, pipalette: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).InitializeFromPalette)(windows_core::Interface::as_raw(self), pipalette.param().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<WICBitmapPaletteType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColorCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColors(&self, pcolors: &mut [u32], pcactualcolors: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColors)(windows_core::Interface::as_raw(self), pcolors.len().try_into().unwrap(), core::mem::transmute(pcolors.as_ptr()), pcactualcolors).ok()
    }
    pub unsafe fn IsBlackWhite(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsBlackWhite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsGrayscale(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsGrayscale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HasAlpha(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasAlpha)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICPalette_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializePredefined: unsafe extern "system" fn(*mut core::ffi::c_void, WICBitmapPaletteType, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub InitializeCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    pub InitializeFromBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub InitializeFromPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICBitmapPaletteType) -> windows_core::HRESULT,
    pub GetColorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub IsBlackWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsGrayscale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub HasAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWICPersistStream, IWICPersistStream_Vtbl, 0x00675040_6908_45f8_86a3_49c7dfd6d9ad);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWICPersistStream {
    type Target = super::super::System::Com::IPersistStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWICPersistStream, windows_core::IUnknown, super::super::System::Com::IPersist, super::super::System::Com::IPersistStream);
#[cfg(feature = "Win32_System_Com")]
impl IWICPersistStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadEx<P0>(&self, pistream: P0, pguidpreferredvendor: *const windows_core::GUID, dwpersistoptions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).LoadEx)(windows_core::Interface::as_raw(self), pistream.param().abi(), pguidpreferredvendor, dwpersistoptions).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveEx<P0, P1>(&self, pistream: P0, dwpersistoptions: u32, fcleardirty: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SaveEx)(windows_core::Interface::as_raw(self), pistream.param().abi(), dwpersistoptions, fcleardirty.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWICPersistStream_Vtbl {
    pub base__: super::super::System::Com::IPersistStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveEx: usize,
}
windows_core::imp::define_interface!(IWICPixelFormatInfo, IWICPixelFormatInfo_Vtbl, 0xe8eda601_3d48_431a_ab44_69059be88bbe);
impl core::ops::Deref for IWICPixelFormatInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPixelFormatInfo, windows_core::IUnknown, IWICComponentInfo);
impl IWICPixelFormatInfo {
    pub unsafe fn GetFormatGUID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormatGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColorContext(&self) -> windows_core::Result<IWICColorContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColorContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBitsPerPixel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitsPerPixel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, pbmaskbuffer: &mut [u8], pcbactual: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChannelMask)(windows_core::Interface::as_raw(self), uichannelindex, pbmaskbuffer.len().try_into().unwrap(), core::mem::transmute(pbmaskbuffer.as_ptr()), pcbactual).ok()
    }
}
#[repr(C)]
pub struct IWICPixelFormatInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetFormatGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBitsPerPixel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICPixelFormatInfo2, IWICPixelFormatInfo2_Vtbl, 0xa9db33a2_af5f_43c7_b679_74f5984b5aa4);
impl core::ops::Deref for IWICPixelFormatInfo2 {
    type Target = IWICPixelFormatInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPixelFormatInfo2, windows_core::IUnknown, IWICComponentInfo, IWICPixelFormatInfo);
impl IWICPixelFormatInfo2 {
    pub unsafe fn SupportsTransparency(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportsTransparency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNumericRepresentation(&self) -> windows_core::Result<WICPixelFormatNumericRepresentation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumericRepresentation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICPixelFormatInfo2_Vtbl {
    pub base__: IWICPixelFormatInfo_Vtbl,
    pub SupportsTransparency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetNumericRepresentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICPixelFormatNumericRepresentation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICPlanarBitmapFrameEncode, IWICPlanarBitmapFrameEncode_Vtbl, 0xf928b7b8_2221_40c1_b72e_7e82f1974d1a);
impl core::ops::Deref for IWICPlanarBitmapFrameEncode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPlanarBitmapFrameEncode, windows_core::IUnknown);
impl IWICPlanarBitmapFrameEncode {
    pub unsafe fn WritePixels(&self, linecount: u32, pplanes: &[WICBitmapPlane]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WritePixels)(windows_core::Interface::as_raw(self), linecount, core::mem::transmute(pplanes.as_ptr()), pplanes.len().try_into().unwrap()).ok()
    }
    pub unsafe fn WriteSource(&self, ppplanes: &[Option<IWICBitmapSource>], prcsource: *const WICRect) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteSource)(windows_core::Interface::as_raw(self), core::mem::transmute(ppplanes.as_ptr()), ppplanes.len().try_into().unwrap(), prcsource).ok()
    }
}
#[repr(C)]
pub struct IWICPlanarBitmapFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WritePixels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const WICBitmapPlane, u32) -> windows_core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *const WICRect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICPlanarBitmapSourceTransform, IWICPlanarBitmapSourceTransform_Vtbl, 0x3aff9cce_be95_4303_b927_e7d16ff4a613);
impl core::ops::Deref for IWICPlanarBitmapSourceTransform {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPlanarBitmapSourceTransform, windows_core::IUnknown);
impl IWICPlanarBitmapSourceTransform {
    pub unsafe fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoesSupportTransform)(windows_core::Interface::as_raw(self), puiwidth, puiheight, dsttransform, dstplanaroptions, pguiddstformats, pplanedescriptions, cplanes, pfissupported).ok()
    }
    pub unsafe fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: &[WICBitmapPlane]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prcsource, uiwidth, uiheight, dsttransform, dstplanaroptions, core::mem::transmute(pdstplanes.as_ptr()), pdstplanes.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IWICPlanarBitmapSourceTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoesSupportTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, WICBitmapTransformOptions, WICPlanarOptions, *const windows_core::GUID, *mut WICBitmapPlaneDescription, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, WICBitmapTransformOptions, WICPlanarOptions, *const WICBitmapPlane, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICPlanarFormatConverter, IWICPlanarFormatConverter_Vtbl, 0xbebee9cb_83b0_4dcc_8132_b0aaa55eac96);
impl core::ops::Deref for IWICPlanarFormatConverter {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICPlanarFormatConverter, windows_core::IUnknown, IWICBitmapSource);
impl IWICPlanarFormatConverter {
    pub unsafe fn Initialize<P0>(&self, ppplanes: &[Option<IWICBitmapSource>], dstformat: *const windows_core::GUID, dither: WICBitmapDitherType, pipalette: P0, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWICPalette>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(ppplanes.as_ptr()), ppplanes.len().try_into().unwrap(), dstformat, dither, pipalette.param().abi(), alphathresholdpercent, palettetranslate).ok()
    }
    pub unsafe fn CanConvert(&self, psrcpixelformats: &[windows_core::GUID], dstpixelformat: *const windows_core::GUID) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), core::mem::transmute(psrcpixelformats.as_ptr()), psrcpixelformats.len().try_into().unwrap(), dstpixelformat, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWICPlanarFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *const windows_core::GUID, WICBitmapDitherType, *mut core::ffi::c_void, f64, WICBitmapPaletteType) -> windows_core::HRESULT,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICProgressCallback, IWICProgressCallback_Vtbl, 0x4776f9cd_9517_45fa_bf24_e89c5ec5c60c);
impl core::ops::Deref for IWICProgressCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICProgressCallback, windows_core::IUnknown);
impl IWICProgressCallback {
    pub unsafe fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), uframenum, operation, dblprogress).ok()
    }
}
#[repr(C)]
pub struct IWICProgressCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, WICProgressOperation, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWICProgressiveLevelControl, IWICProgressiveLevelControl_Vtbl, 0xdaac296f_7aa5_4dbf_8d15_225c5976f891);
impl core::ops::Deref for IWICProgressiveLevelControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICProgressiveLevelControl, windows_core::IUnknown);
impl IWICProgressiveLevelControl {
    pub unsafe fn GetLevelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentLevel(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCurrentLevel(&self, nlevel: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentLevel)(windows_core::Interface::as_raw(self), nlevel).ok()
    }
}
#[repr(C)]
pub struct IWICProgressiveLevelControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWICStream, IWICStream_Vtbl, 0x135ff860_22b7_4ddf_b0f6_218f4f299a43);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWICStream {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWICStream, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl IWICStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromIStream<P0>(&self, pistream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).InitializeFromIStream)(windows_core::Interface::as_raw(self), pistream.param().abi()).ok()
    }
    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0, dwdesiredaccess: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InitializeFromFilename)(windows_core::Interface::as_raw(self), wzfilename.param().abi(), dwdesiredaccess).ok()
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeFromMemory)(windows_core::Interface::as_raw(self), core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromIStreamRegion<P0>(&self, pistream: P0, uloffset: u64, ulmaxsize: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).InitializeFromIStreamRegion)(windows_core::Interface::as_raw(self), pistream.param().abi(), uloffset, ulmaxsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWICStream_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromIStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromIStream: usize,
    pub InitializeFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub InitializeFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromIStreamRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromIStreamRegion: usize,
}
windows_core::imp::define_interface!(IWICStreamProvider, IWICStreamProvider_Vtbl, 0x449494bc_b468_4927_96d7_ba90d31ab505);
impl core::ops::Deref for IWICStreamProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICStreamProvider, windows_core::IUnknown);
impl IWICStreamProvider {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPersistOptions(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPersistOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPreferredVendorGUID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreferredVendorGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RefreshStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshStream)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWICStreamProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    pub GetPersistOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPreferredVendorGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RefreshStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CATID_WICBitmapDecoders: windows_core::GUID = windows_core::GUID::from_u128(0x7ed96837_96f0_4812_b211_f13c24117ed3);
pub const CATID_WICBitmapEncoders: windows_core::GUID = windows_core::GUID::from_u128(0xac757296_3522_4e11_9862_c17be5a1767e);
pub const CATID_WICFormatConverters: windows_core::GUID = windows_core::GUID::from_u128(0x7835eae8_bf14_49d1_93ce_533a407b2248);
pub const CATID_WICMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x05af94d8_7174_4cd2_be4a_4124b80ee4b8);
pub const CATID_WICMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xabe3b9a4_257d_4b97_bd1a_294af496222e);
pub const CATID_WICPixelFormats: windows_core::GUID = windows_core::GUID::from_u128(0x2b46e70f_cda7_473e_89f6_dc9630a2390b);
pub const CLSID_WIC8BIMIPTCDigestMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x02805f1e_d5aa_415b_82c5_61c033a988a6);
pub const CLSID_WIC8BIMIPTCDigestMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x2db5e62b_0d67_495f_8f9d_c2f0188647ac);
pub const CLSID_WIC8BIMIPTCMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x0010668c_0801_4da6_a4a4_826522b6d28f);
pub const CLSID_WIC8BIMIPTCMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x00108226_ee41_44a2_9e9c_4be4d5b1d2cd);
pub const CLSID_WIC8BIMResolutionInfoMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x5805137a_e348_4f7c_b3cc_6db9965a0599);
pub const CLSID_WIC8BIMResolutionInfoMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x4ff2fe0e_e74a_4b71_98c4_ab7dc16707ba);
pub const CLSID_WICAPEMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x1767b93a_b021_44ea_920f_863c11f4f768);
pub const CLSID_WICAPEMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xbd6edfca_2890_482f_b233_8d7339a1cf8d);
pub const CLSID_WICAdngDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x981d9411_909e_42a7_8f5d_a747ff052edb);
pub const CLSID_WICApp0MetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x43324b33_a78f_480f_9111_9638aaccc832);
pub const CLSID_WICApp0MetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xf3c633a2_46c8_498e_8fbb_cc6f721bbcde);
pub const CLSID_WICApp13MetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xaa7e3c50_864c_4604_bc04_8b0b76e637f6);
pub const CLSID_WICApp13MetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x7b19a919_a9d6_49e5_bd45_02c34e4e4cd5);
pub const CLSID_WICApp1MetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xdde33513_774e_4bcd_ae79_02f4adfe62fc);
pub const CLSID_WICApp1MetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xee366069_1832_420f_b381_0479ad066f19);
pub const CLSID_WICBmpDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x6b462062_7cbf_400d_9fdb_813dd10f2778);
pub const CLSID_WICBmpEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x69be8bb4_d66d_47c8_865a_ed1589433782);
pub const CLSID_WICDdsDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x9053699f_a341_429d_9e90_ee437cf80c73);
pub const CLSID_WICDdsEncoder: windows_core::GUID = windows_core::GUID::from_u128(0xa61dde94_66ce_4ac1_881b_71680588895e);
pub const CLSID_WICDdsMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x276c88ca_7533_4a86_b676_66b36080d484);
pub const CLSID_WICDdsMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xfd688bbd_31ed_4db7_a723_934927d38367);
pub const CLSID_WICDefaultFormatConverter: windows_core::GUID = windows_core::GUID::from_u128(0x1a3f11dc_b514_4b17_8c5f_2154513852f1);
pub const CLSID_WICExifMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xd9403860_297f_4a49_bf9b_77898150a442);
pub const CLSID_WICExifMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xc9a14cda_c339_460b_9078_d4debcfabe91);
pub const CLSID_WICFormatConverterHighColor: windows_core::GUID = windows_core::GUID::from_u128(0xac75d454_9f37_48f8_b972_4e19bc856011);
pub const CLSID_WICFormatConverterNChannel: windows_core::GUID = windows_core::GUID::from_u128(0xc17cabb2_d4a3_47d7_a557_339b2efbd4f1);
pub const CLSID_WICFormatConverterWMPhoto: windows_core::GUID = windows_core::GUID::from_u128(0x9cb5172b_d600_46ba_ab77_77bb7e3a00d9);
pub const CLSID_WICGCEMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xb92e345d_f52d_41f3_b562_081bc772e3b9);
pub const CLSID_WICGCEMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xaf95dc76_16b2_47f4_b3ea_3c31796693e7);
pub const CLSID_WICGifCommentMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x32557d3b_69dc_4f95_836e_f5972b2f6159);
pub const CLSID_WICGifCommentMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa02797fc_c4ae_418c_af95_e637c7ead2a1);
pub const CLSID_WICGifDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x381dda3c_9ce9_4834_a23e_1f98f8fc52be);
pub const CLSID_WICGifEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x114f5598_0b22_40a0_86a1_c83ea495adbd);
pub const CLSID_WICGpsMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x3697790b_223b_484e_9925_c4869218f17a);
pub const CLSID_WICGpsMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xcb8c13e4_62b5_4c96_a48b_6ba6ace39c76);
pub const CLSID_WICHeifDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xe9a4a80a_44fe_4de4_8971_7150b10a5199);
pub const CLSID_WICHeifEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x0dbecec1_9eb3_4860_9c6f_ddbe86634575);
pub const CLSID_WICHeifHDRMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x2438de3d_94d9_4be8_84a8_4de95a575e75);
pub const CLSID_WICHeifMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xacddfc3f_85ec_41bc_bdef_1bc262e4db05);
pub const CLSID_WICHeifMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x3ae45e79_40bc_4401_ace5_dd3cb16e6afe);
pub const CLSID_WICIMDMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x7447a267_0015_42c8_a8f1_fb3b94c68361);
pub const CLSID_WICIMDMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x8c89071f_452e_4e95_9682_9d1024627172);
pub const CLSID_WICIPTCMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x03012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICIPTCMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1249b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICIRBMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xd4dcd3d7_b4c2_47d9_a6bf_b89ba396a4a3);
pub const CLSID_WICIRBMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x5c5c1935_0235_4434_80bc_251bc1ec39c6);
pub const CLSID_WICIcoDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xc61bfcdf_2e0f_4aad_a8d7_e06bafebcdfe);
pub const CLSID_WICIfdMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x8f914656_9d0a_4eb2_9019_0bf96d8a9ee6);
pub const CLSID_WICIfdMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xb1ebfc28_c9bd_47a2_8d33_b948769777a7);
pub const CLSID_WICImagingCategories: windows_core::GUID = windows_core::GUID::from_u128(0xfae3d380_fea4_4623_8c75_c6b61110b681);
pub const CLSID_WICImagingFactory: windows_core::GUID = windows_core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory1: windows_core::GUID = windows_core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const CLSID_WICImagingFactory2: windows_core::GUID = windows_core::GUID::from_u128(0x317d06e8_5f24_433d_bdf7_79ce68d8abc2);
pub const CLSID_WICInteropMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xb5c8b898_0074_459f_b700_860d4651ea14);
pub const CLSID_WICInteropMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x122ec645_cd7e_44d8_b186_2c8c20c3b50f);
pub const CLSID_WICJpegChrominanceMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x50b1904b_f28f_4574_93f4_0bade82c69e9);
pub const CLSID_WICJpegChrominanceMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x3ff566f0_6e6b_49d4_96e6_b78886692c62);
pub const CLSID_WICJpegCommentMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x9f66347c_60c4_4c4d_ab58_d2358685f607);
pub const CLSID_WICJpegCommentMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xe573236f_55b1_4eda_81ea_9f65db0290d3);
pub const CLSID_WICJpegDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x9456a480_e88b_43ea_9e73_0b2d9b71b1ca);
pub const CLSID_WICJpegEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x1a34f5c1_4a5a_46dc_b644_1f4567e7a676);
pub const CLSID_WICJpegLuminanceMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x356f2f88_05a6_4728_b9a4_1bfbce04d838);
pub const CLSID_WICJpegLuminanceMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1d583abc_8a0e_4657_9982_a380ca58fb4b);
pub const CLSID_WICJpegQualcommPhoneEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x68ed5c62_f534_4979_b2b3_686a12b2b34c);
pub const CLSID_WICLSDMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x41070793_59e4_479a_a1f7_954adc2ef5fc);
pub const CLSID_WICLSDMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x73c037e7_e5d9_4954_876a_6da81d6e5768);
pub const CLSID_WICPlanarFormatConverter: windows_core::GUID = windows_core::GUID::from_u128(0x184132b8_32f8_4784_9131_dd7224b23438);
pub const CLSID_WICPngBkgdMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x0ce7a4a6_03e8_4a60_9d15_282ef32ee7da);
pub const CLSID_WICPngBkgdMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x68e3f2fd_31ae_4441_bb6a_fd7047525f90);
pub const CLSID_WICPngChrmMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xf90b5f36_367b_402a_9dd1_bc0fd59d8f62);
pub const CLSID_WICPngChrmMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xe23ce3eb_5608_4e83_bcef_27b1987e51d7);
pub const CLSID_WICPngDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder1: windows_core::GUID = windows_core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
pub const CLSID_WICPngDecoder2: windows_core::GUID = windows_core::GUID::from_u128(0xe018945b_aa86_4008_9bd4_6777a1e40c11);
pub const CLSID_WICPngEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x27949969_876a_41d7_9447_568f6a35a4dc);
pub const CLSID_WICPngGamaMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x3692ca39_e082_4350_9e1f_3704cb083cd5);
pub const CLSID_WICPngGamaMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xff036d13_5d4b_46dd_b10f_106693d9fe4f);
pub const CLSID_WICPngHistMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x877a0bb7_a313_4491_87b5_2e6d0594f520);
pub const CLSID_WICPngHistMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x8a03e749_672e_446e_bf1f_2c11d233b6ff);
pub const CLSID_WICPngIccpMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xf5d3e63b_cb0f_4628_a478_6d8244be36b1);
pub const CLSID_WICPngIccpMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x16671e5f_0ce6_4cc4_9768_e89fe5018ade);
pub const CLSID_WICPngItxtMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xaabfb2fa_3e1e_4a8f_8977_5556fb94ea23);
pub const CLSID_WICPngItxtMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x31879719_e751_4df8_981d_68dff67704ed);
pub const CLSID_WICPngSrgbMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xfb40360c_547e_4956_a3b9_d4418859ba66);
pub const CLSID_WICPngSrgbMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa6ee35c6_87ec_47df_9f22_1d5aad840c82);
pub const CLSID_WICPngTextMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x4b59afcc_b8c3_408a_b670_89e5fab6fda7);
pub const CLSID_WICPngTextMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xb5ebafb9_253e_4a72_a744_0762d2685683);
pub const CLSID_WICPngTimeMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xd94edf02_efe5_4f0d_85c8_f5a68b3000b1);
pub const CLSID_WICPngTimeMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1ab78400_b5a3_4d91_8ace_33fcd1499be6);
pub const CLSID_WICRAWDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x41945702_8302_44a6_9445_ac98e8afa086);
pub const CLSID_WICSubIfdMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x50d42f09_ecd1_4b41_b65d_da1fdaa75663);
pub const CLSID_WICSubIfdMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x8ade5386_8e9b_4f4c_acf2_f0008706b238);
pub const CLSID_WICThumbnailMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xfb012959_f4f6_44d7_9d09_daa087a9db57);
pub const CLSID_WICThumbnailMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xd049b20c_5dd0_44fe_b0b3_8f92c8e6d080);
pub const CLSID_WICTiffDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xb54e85d9_fe23_499f_8b88_6acea713752b);
pub const CLSID_WICTiffEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x0131be10_2001_4c5f_a9b0_cc88fab64ce8);
pub const CLSID_WICUnknownMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x699745c2_5066_4b82_a8e3_d40478dbec8c);
pub const CLSID_WICUnknownMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xa09cca86_27ba_4f39_9053_121fa4dc08fc);
pub const CLSID_WICWebpAnimMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x076f9911_a348_465c_a807_a252f3f2d3de);
pub const CLSID_WICWebpAnmfMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x85a10b03_c9f6_439f_be5e_c0fbef67807c);
pub const CLSID_WICWebpDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x7693e886_51c9_4070_8419_9f70738ec8fa);
pub const CLSID_WICWmpDecoder: windows_core::GUID = windows_core::GUID::from_u128(0xa26cec36_234c_4950_ae16_e34aace71d0d);
pub const CLSID_WICWmpEncoder: windows_core::GUID = windows_core::GUID::from_u128(0xac4ce3cb_e1c1_44cd_8215_5a1665509ec2);
pub const CLSID_WICXMPAltMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xaa94dcc2_b8b0_4898_b835_000aabd74393);
pub const CLSID_WICXMPAltMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x076c2a6c_f78f_4c46_a723_3583e70876ea);
pub const CLSID_WICXMPBagMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0xe7e79a30_4f2c_4fab_8d00_394f2d6bbebe);
pub const CLSID_WICXMPBagMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0xed822c8c_d6be_4301_a631_0e1416bad28f);
pub const CLSID_WICXMPMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x72b624df_ae11_4948_a65c_351eb0829419);
pub const CLSID_WICXMPMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x1765e14e_1bd4_462e_b6b1_590bf1262ac6);
pub const CLSID_WICXMPSeqMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x7f12e753_fc71_43d7_a51d_92f35977abb5);
pub const CLSID_WICXMPSeqMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x6d68d1de_d432_4b0f_923a_091183a9bda7);
pub const CLSID_WICXMPStructMetadataReader: windows_core::GUID = windows_core::GUID::from_u128(0x01b90d9a_8209_47f7_9c52_e1244bf50ced);
pub const CLSID_WICXMPStructMetadataWriter: windows_core::GUID = windows_core::GUID::from_u128(0x22c21f93_7ddb_411c_9b17_c5b7bd064abc);
pub const FACILITY_WINCODEC_ERR: u32 = 2200u32;
pub const GUID_ContainerFormatAdng: windows_core::GUID = windows_core::GUID::from_u128(0xf3ff6d0d_38c0_41c4_b1fe_1f3824f17b84);
pub const GUID_ContainerFormatBmp: windows_core::GUID = windows_core::GUID::from_u128(0x0af1d87e_fcfe_4188_bdeb_a7906471cbe3);
pub const GUID_ContainerFormatDds: windows_core::GUID = windows_core::GUID::from_u128(0x9967cb95_2e85_4ac8_8ca2_83d7ccd425c9);
pub const GUID_ContainerFormatGif: windows_core::GUID = windows_core::GUID::from_u128(0x1f8a5601_7d4d_4cbd_9c82_1bc8d4eeb9a5);
pub const GUID_ContainerFormatHeif: windows_core::GUID = windows_core::GUID::from_u128(0xe1e62521_6787_405b_a339_500715b5763f);
pub const GUID_ContainerFormatIco: windows_core::GUID = windows_core::GUID::from_u128(0xa3a860c4_338f_4c17_919a_fba4b5628f21);
pub const GUID_ContainerFormatJpeg: windows_core::GUID = windows_core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const GUID_ContainerFormatPng: windows_core::GUID = windows_core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
pub const GUID_ContainerFormatRaw: windows_core::GUID = windows_core::GUID::from_u128(0xfe99ce60_f19c_433c_a3ae_00acefa9ca21);
pub const GUID_ContainerFormatTiff: windows_core::GUID = windows_core::GUID::from_u128(0x163bcc30_e2e9_4f0b_961d_a3e9fdb788a3);
pub const GUID_ContainerFormatWebp: windows_core::GUID = windows_core::GUID::from_u128(0xe094b0e2_67f2_45b3_b0ea_115337ca7cf3);
pub const GUID_ContainerFormatWmp: windows_core::GUID = windows_core::GUID::from_u128(0x57a37caa_367a_4540_916b_f183c5093a4b);
pub const GUID_MetadataFormat8BIMIPTC: windows_core::GUID = windows_core::GUID::from_u128(0x0010568c_0852_4e6a_b191_5c33ac5b0430);
pub const GUID_MetadataFormat8BIMIPTCDigest: windows_core::GUID = windows_core::GUID::from_u128(0x1ca32285_9ccd_4786_8bd8_79539db6a006);
pub const GUID_MetadataFormat8BIMResolutionInfo: windows_core::GUID = windows_core::GUID::from_u128(0x739f305d_81db_43cb_ac5e_55013ef9f003);
pub const GUID_MetadataFormatAPE: windows_core::GUID = windows_core::GUID::from_u128(0x2e043dc2_c967_4e05_875e_618bf67e85c3);
pub const GUID_MetadataFormatApp0: windows_core::GUID = windows_core::GUID::from_u128(0x79007028_268d_45d6_a3c2_354e6a504bc9);
pub const GUID_MetadataFormatApp1: windows_core::GUID = windows_core::GUID::from_u128(0x8fd3dfc3_f951_492b_817f_69c2e6d9a5b0);
pub const GUID_MetadataFormatApp13: windows_core::GUID = windows_core::GUID::from_u128(0x326556a2_f502_4354_9cc0_8e3f48eaf6b5);
pub const GUID_MetadataFormatChunkbKGD: windows_core::GUID = windows_core::GUID::from_u128(0xe14d3571_6b47_4dea_b60a_87ce0a78dfb7);
pub const GUID_MetadataFormatChunkcHRM: windows_core::GUID = windows_core::GUID::from_u128(0x9db3655b_2842_44b3_8067_12e9b375556a);
pub const GUID_MetadataFormatChunkgAMA: windows_core::GUID = windows_core::GUID::from_u128(0xf00935a5_1d5d_4cd1_81b2_9324d7eca781);
pub const GUID_MetadataFormatChunkhIST: windows_core::GUID = windows_core::GUID::from_u128(0xc59a82da_db74_48a4_bd6a_b69c4931ef95);
pub const GUID_MetadataFormatChunkiCCP: windows_core::GUID = windows_core::GUID::from_u128(0xeb4349ab_b685_450f_91b5_e802e892536c);
pub const GUID_MetadataFormatChunkiTXt: windows_core::GUID = windows_core::GUID::from_u128(0xc2bec729_0b68_4b77_aa0e_6295a6ac1814);
pub const GUID_MetadataFormatChunksRGB: windows_core::GUID = windows_core::GUID::from_u128(0xc115fd36_cc6f_4e3f_8363_524b87c6b0d9);
pub const GUID_MetadataFormatChunktEXt: windows_core::GUID = windows_core::GUID::from_u128(0x568d8936_c0a9_4923_905d_df2b38238fbc);
pub const GUID_MetadataFormatChunktIME: windows_core::GUID = windows_core::GUID::from_u128(0x6b00ae2d_e24b_460a_98b6_878bd03072fd);
pub const GUID_MetadataFormatDds: windows_core::GUID = windows_core::GUID::from_u128(0x4a064603_8c33_4e60_9c29_136231702d08);
pub const GUID_MetadataFormatExif: windows_core::GUID = windows_core::GUID::from_u128(0x1c3c4f9d_b84a_467d_9493_36cfbd59ea57);
pub const GUID_MetadataFormatGCE: windows_core::GUID = windows_core::GUID::from_u128(0x2a25cad8_deeb_4c69_a788_0ec2266dcafd);
pub const GUID_MetadataFormatGifComment: windows_core::GUID = windows_core::GUID::from_u128(0xc4b6e0e0_cfb4_4ad3_ab33_9aad2355a34a);
pub const GUID_MetadataFormatGps: windows_core::GUID = windows_core::GUID::from_u128(0x7134ab8a_9351_44ad_af62_448db6b502ec);
pub const GUID_MetadataFormatHeif: windows_core::GUID = windows_core::GUID::from_u128(0x817ef3e1_1288_45f4_a852_260d9e7cce83);
pub const GUID_MetadataFormatHeifHDR: windows_core::GUID = windows_core::GUID::from_u128(0x568b8d8a_1e65_438c_8968_d60e1012beb9);
pub const GUID_MetadataFormatIMD: windows_core::GUID = windows_core::GUID::from_u128(0xbd2bb086_4d52_48dd_9677_db483e85ae8f);
pub const GUID_MetadataFormatIPTC: windows_core::GUID = windows_core::GUID::from_u128(0x4fab0914_e129_4087_a1d1_bc812d45a7b5);
pub const GUID_MetadataFormatIRB: windows_core::GUID = windows_core::GUID::from_u128(0x16100d66_8570_4bb9_b92d_fda4b23ece67);
pub const GUID_MetadataFormatIfd: windows_core::GUID = windows_core::GUID::from_u128(0x537396c6_2d8a_4bb6_9bf8_2f0a8e2a3adf);
pub const GUID_MetadataFormatInterop: windows_core::GUID = windows_core::GUID::from_u128(0xed686f8e_681f_4c8b_bd41_a8addbf6b3fc);
pub const GUID_MetadataFormatJpegChrominance: windows_core::GUID = windows_core::GUID::from_u128(0xf73d0dcf_cec6_4f85_9b0e_1c3956b1bef7);
pub const GUID_MetadataFormatJpegComment: windows_core::GUID = windows_core::GUID::from_u128(0x220e5f33_afd3_474e_9d31_7d4fe730f557);
pub const GUID_MetadataFormatJpegLuminance: windows_core::GUID = windows_core::GUID::from_u128(0x86908007_edfc_4860_8d4b_4ee6e83e6058);
pub const GUID_MetadataFormatLSD: windows_core::GUID = windows_core::GUID::from_u128(0xe256031e_6299_4929_b98d_5ac884afba92);
pub const GUID_MetadataFormatSubIfd: windows_core::GUID = windows_core::GUID::from_u128(0x58a2e128_2db9_4e57_bb14_5177891ed331);
pub const GUID_MetadataFormatThumbnail: windows_core::GUID = windows_core::GUID::from_u128(0x243dcee9_8703_40ee_8ef0_22a600b8058c);
pub const GUID_MetadataFormatUnknown: windows_core::GUID = windows_core::GUID::from_u128(0xa45e592f_9078_4a7c_adb5_4edc4fd61b1f);
pub const GUID_MetadataFormatWebpANIM: windows_core::GUID = windows_core::GUID::from_u128(0x6dc4fda6_78e6_4102_ae35_bcfa1edcc78b);
pub const GUID_MetadataFormatWebpANMF: windows_core::GUID = windows_core::GUID::from_u128(0x43c105ee_b93b_4abb_b003_a08c0d870471);
pub const GUID_MetadataFormatXMP: windows_core::GUID = windows_core::GUID::from_u128(0xbb5acc38_f216_4cec_a6c5_5f6e739763a9);
pub const GUID_MetadataFormatXMPAlt: windows_core::GUID = windows_core::GUID::from_u128(0x7b08a675_91aa_481b_a798_4da94908613b);
pub const GUID_MetadataFormatXMPBag: windows_core::GUID = windows_core::GUID::from_u128(0x833cca5f_dcb7_4516_806f_6596ab26dce4);
pub const GUID_MetadataFormatXMPSeq: windows_core::GUID = windows_core::GUID::from_u128(0x63e8df02_eb6c_456c_a224_b25e794fd648);
pub const GUID_MetadataFormatXMPStruct: windows_core::GUID = windows_core::GUID::from_u128(0x22383cf1_ed17_4e2e_af17_d85b8f6b30d0);
pub const GUID_VendorMicrosoft: windows_core::GUID = windows_core::GUID::from_u128(0xf0e749ca_edef_4589_a73a_ee0e626a2a2b);
pub const GUID_VendorMicrosoftBuiltIn: windows_core::GUID = windows_core::GUID::from_u128(0x257a30fd_06b6_462b_aea4_63f70b86e533);
pub const GUID_WICPixelFormat112bpp6ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc937);
pub const GUID_WICPixelFormat112bpp7Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92a);
pub const GUID_WICPixelFormat128bpp7ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc938);
pub const GUID_WICPixelFormat128bpp8Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92b);
pub const GUID_WICPixelFormat128bppPRGBAFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91a);
pub const GUID_WICPixelFormat128bppRGBAFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91e);
pub const GUID_WICPixelFormat128bppRGBAFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc919);
pub const GUID_WICPixelFormat128bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc941);
pub const GUID_WICPixelFormat128bppRGBFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91b);
pub const GUID_WICPixelFormat144bpp8ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc939);
pub const GUID_WICPixelFormat16bppBGR555: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc909);
pub const GUID_WICPixelFormat16bppBGR565: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90a);
pub const GUID_WICPixelFormat16bppBGRA5551: windows_core::GUID = windows_core::GUID::from_u128(0x05ec7c2b_f1e6_4961_ad46_e1cc810a87d2);
pub const GUID_WICPixelFormat16bppCbCr: windows_core::GUID = windows_core::GUID::from_u128(0xff95ba6e_11e0_4263_bb45_01721f3460a4);
pub const GUID_WICPixelFormat16bppCbQuantizedDctCoefficients: windows_core::GUID = windows_core::GUID::from_u128(0xd2c4ff61_56a5_49c2_8b5c_4c1925964837);
pub const GUID_WICPixelFormat16bppCrQuantizedDctCoefficients: windows_core::GUID = windows_core::GUID::from_u128(0x2fe354f0_1680_42d8_9231_e73c0565bfc1);
pub const GUID_WICPixelFormat16bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90b);
pub const GUID_WICPixelFormat16bppGrayFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc913);
pub const GUID_WICPixelFormat16bppGrayHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93e);
pub const GUID_WICPixelFormat16bppYQuantizedDctCoefficients: windows_core::GUID = windows_core::GUID::from_u128(0xa355f433_48e8_4a42_84d8_e2aa26ca80a4);
pub const GUID_WICPixelFormat1bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc901);
pub const GUID_WICPixelFormat24bpp3Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc920);
pub const GUID_WICPixelFormat24bppBGR: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
pub const GUID_WICPixelFormat24bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90d);
pub const GUID_WICPixelFormat2bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc906);
pub const GUID_WICPixelFormat2bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc902);
pub const GUID_WICPixelFormat32bpp3ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92e);
pub const GUID_WICPixelFormat32bpp4Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc921);
pub const GUID_WICPixelFormat32bppBGR: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
pub const GUID_WICPixelFormat32bppBGR101010: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc914);
pub const GUID_WICPixelFormat32bppBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90f);
pub const GUID_WICPixelFormat32bppCMYK: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91c);
pub const GUID_WICPixelFormat32bppGrayFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93f);
pub const GUID_WICPixelFormat32bppGrayFloat: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc911);
pub const GUID_WICPixelFormat32bppPBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
pub const GUID_WICPixelFormat32bppPRGBA: windows_core::GUID = windows_core::GUID::from_u128(0x3cc4a650_a527_4d37_a916_3142c7ebedba);
pub const GUID_WICPixelFormat32bppR10G10B10A2: windows_core::GUID = windows_core::GUID::from_u128(0x604e1bb5_8a3c_4b65_b11c_bc0b8dd75b7f);
pub const GUID_WICPixelFormat32bppR10G10B10A2HDR10: windows_core::GUID = windows_core::GUID::from_u128(0x9c215c5d_1acc_4f0e_a4bc_70fb3ae8fd28);
pub const GUID_WICPixelFormat32bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0xd98c6b95_3efe_47d6_bb25_eb1748ab0cf1);
pub const GUID_WICPixelFormat32bppRGBA: windows_core::GUID = windows_core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
pub const GUID_WICPixelFormat32bppRGBA1010102: windows_core::GUID = windows_core::GUID::from_u128(0x25238d72_fcf9_4522_b514_5578e5ad55e0);
pub const GUID_WICPixelFormat32bppRGBA1010102XR: windows_core::GUID = windows_core::GUID::from_u128(0x00de6b9a_c101_434b_b502_d0165ee1122c);
pub const GUID_WICPixelFormat32bppRGBE: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93d);
pub const GUID_WICPixelFormat40bpp4ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92f);
pub const GUID_WICPixelFormat40bpp5Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc922);
pub const GUID_WICPixelFormat40bppCMYKAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92c);
pub const GUID_WICPixelFormat48bpp3Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc926);
pub const GUID_WICPixelFormat48bpp5ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc930);
pub const GUID_WICPixelFormat48bpp6Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc923);
pub const GUID_WICPixelFormat48bppBGR: windows_core::GUID = windows_core::GUID::from_u128(0xe605a384_b468_46ce_bb2e_36f180e64313);
pub const GUID_WICPixelFormat48bppBGRFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x49ca140e_cab6_493b_9ddf_60187c37532a);
pub const GUID_WICPixelFormat48bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc915);
pub const GUID_WICPixelFormat48bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc912);
pub const GUID_WICPixelFormat48bppRGBHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93b);
pub const GUID_WICPixelFormat4bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc907);
pub const GUID_WICPixelFormat4bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc903);
pub const GUID_WICPixelFormat56bpp6ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc931);
pub const GUID_WICPixelFormat56bpp7Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc924);
pub const GUID_WICPixelFormat64bpp3ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc934);
pub const GUID_WICPixelFormat64bpp4Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc927);
pub const GUID_WICPixelFormat64bpp7ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc932);
pub const GUID_WICPixelFormat64bpp8Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc925);
pub const GUID_WICPixelFormat64bppBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x1562ff7c_d352_46f9_979e_42976b792246);
pub const GUID_WICPixelFormat64bppBGRAFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x356de33c_54d2_4a23_bb04_9b7bf9b1d42d);
pub const GUID_WICPixelFormat64bppCMYK: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91f);
pub const GUID_WICPixelFormat64bppPBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x8c518e8e_a4ec_468b_ae70_c9a35a9c5530);
pub const GUID_WICPixelFormat64bppPRGBA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc917);
pub const GUID_WICPixelFormat64bppPRGBAHalf: windows_core::GUID = windows_core::GUID::from_u128(0x58ad26c2_c623_4d9d_b320_387e49f8c442);
pub const GUID_WICPixelFormat64bppRGB: windows_core::GUID = windows_core::GUID::from_u128(0xa1182111_186d_4d42_bc6a_9c8303a8dff9);
pub const GUID_WICPixelFormat64bppRGBA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc916);
pub const GUID_WICPixelFormat64bppRGBAFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91d);
pub const GUID_WICPixelFormat64bppRGBAHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93a);
pub const GUID_WICPixelFormat64bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc940);
pub const GUID_WICPixelFormat64bppRGBHalf: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc942);
pub const GUID_WICPixelFormat72bpp8ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc933);
pub const GUID_WICPixelFormat80bpp4ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc935);
pub const GUID_WICPixelFormat80bpp5Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc928);
pub const GUID_WICPixelFormat80bppCMYKAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92d);
pub const GUID_WICPixelFormat8bppAlpha: windows_core::GUID = windows_core::GUID::from_u128(0xe6cd0116_eeba_4161_aa85_27dd9fb3a895);
pub const GUID_WICPixelFormat8bppCb: windows_core::GUID = windows_core::GUID::from_u128(0x1339f224_6bfe_4c3e_9302_e4f3a6d0ca2a);
pub const GUID_WICPixelFormat8bppCr: windows_core::GUID = windows_core::GUID::from_u128(0xb8145053_2116_49f0_8835_ed844b205c51);
pub const GUID_WICPixelFormat8bppGray: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc908);
pub const GUID_WICPixelFormat8bppIndexed: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc904);
pub const GUID_WICPixelFormat8bppY: windows_core::GUID = windows_core::GUID::from_u128(0x91b4db54_2df9_42f0_b449_2909bb3df88e);
pub const GUID_WICPixelFormat96bpp5ChannelsAlpha: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc936);
pub const GUID_WICPixelFormat96bpp6Channels: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc929);
pub const GUID_WICPixelFormat96bppRGBFixedPoint: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc918);
pub const GUID_WICPixelFormat96bppRGBFloat: windows_core::GUID = windows_core::GUID::from_u128(0xe3fed78f_e8db_4acf_84c1_e97f6136b327);
pub const GUID_WICPixelFormatBlackWhite: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc905);
pub const GUID_WICPixelFormatDontCare: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc900);
pub const WIC8BIMIptcDigestIptcDigest: WIC8BIMIptcDigestProperties = WIC8BIMIptcDigestProperties(2i32);
pub const WIC8BIMIptcDigestPString: WIC8BIMIptcDigestProperties = WIC8BIMIptcDigestProperties(1i32);
pub const WIC8BIMIptcEmbeddedIPTC: WIC8BIMIptcProperties = WIC8BIMIptcProperties(1i32);
pub const WIC8BIMIptcPString: WIC8BIMIptcProperties = WIC8BIMIptcProperties(0i32);
pub const WIC8BIMResolutionInfoHResolution: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(2i32);
pub const WIC8BIMResolutionInfoHResolutionUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(3i32);
pub const WIC8BIMResolutionInfoHeightUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(7i32);
pub const WIC8BIMResolutionInfoPString: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(1i32);
pub const WIC8BIMResolutionInfoVResolution: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(5i32);
pub const WIC8BIMResolutionInfoVResolutionUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(6i32);
pub const WIC8BIMResolutionInfoWidthUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(4i32);
pub const WICAllComponents: WICComponentType = WICComponentType(63i32);
pub const WICAsShotParameterSet: WICRawParameterSet = WICRawParameterSet(1i32);
pub const WICAutoAdjustedParameterSet: WICRawParameterSet = WICRawParameterSet(3i32);
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(1i32);
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(2i32);
pub const WICBitmapDecoderCapabilityCanDecodeAllImages: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(2i32);
pub const WICBitmapDecoderCapabilityCanDecodeSomeImages: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(4i32);
pub const WICBitmapDecoderCapabilityCanDecodeThumbnail: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(16i32);
pub const WICBitmapDecoderCapabilityCanEnumerateMetadata: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(8i32);
pub const WICBitmapDecoderCapabilitySameEncoder: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(1i32);
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = WICBitmapDitherType(6i32);
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = WICBitmapDitherType(7i32);
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = WICBitmapDitherType(8i32);
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = WICBitmapDitherType(0i32);
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = WICBitmapDitherType(3i32);
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = WICBitmapDitherType(1i32);
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = WICBitmapDitherType(2i32);
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = WICBitmapDitherType(0i32);
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = WICBitmapDitherType(4i32);
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = WICBitmapDitherType(5i32);
pub const WICBitmapEncoderCacheInMemory: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(0i32);
pub const WICBitmapEncoderCacheTempFile: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(1i32);
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(2i32);
pub const WICBitmapIgnoreAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(2i32);
pub const WICBitmapInterpolationModeCubic: WICBitmapInterpolationMode = WICBitmapInterpolationMode(2i32);
pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = WICBitmapInterpolationMode(3i32);
pub const WICBitmapInterpolationModeHighQualityCubic: WICBitmapInterpolationMode = WICBitmapInterpolationMode(4i32);
pub const WICBitmapInterpolationModeLinear: WICBitmapInterpolationMode = WICBitmapInterpolationMode(1i32);
pub const WICBitmapInterpolationModeNearestNeighbor: WICBitmapInterpolationMode = WICBitmapInterpolationMode(0i32);
pub const WICBitmapLockRead: WICBitmapLockFlags = WICBitmapLockFlags(1i32);
pub const WICBitmapLockWrite: WICBitmapLockFlags = WICBitmapLockFlags(2i32);
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(0i32);
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = WICBitmapPaletteType(0i32);
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = WICBitmapPaletteType(2i32);
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = WICBitmapPaletteType(11i32);
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = WICBitmapPaletteType(12i32);
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = WICBitmapPaletteType(10i32);
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = WICBitmapPaletteType(6i32);
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = WICBitmapPaletteType(7i32);
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = WICBitmapPaletteType(8i32);
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = WICBitmapPaletteType(9i32);
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = WICBitmapPaletteType(4i32);
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = WICBitmapPaletteType(5i32);
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = WICBitmapPaletteType(3i32);
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = WICBitmapPaletteType(7i32);
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = WICBitmapPaletteType(1i32);
pub const WICBitmapTransformFlipHorizontal: WICBitmapTransformOptions = WICBitmapTransformOptions(8i32);
pub const WICBitmapTransformFlipVertical: WICBitmapTransformOptions = WICBitmapTransformOptions(16i32);
pub const WICBitmapTransformRotate0: WICBitmapTransformOptions = WICBitmapTransformOptions(0i32);
pub const WICBitmapTransformRotate180: WICBitmapTransformOptions = WICBitmapTransformOptions(2i32);
pub const WICBitmapTransformRotate270: WICBitmapTransformOptions = WICBitmapTransformOptions(3i32);
pub const WICBitmapTransformRotate90: WICBitmapTransformOptions = WICBitmapTransformOptions(1i32);
pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(0i32);
pub const WICBitmapUsePremultipliedAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(1i32);
pub const WICColorContextExifColorSpace: WICColorContextType = WICColorContextType(2i32);
pub const WICColorContextProfile: WICColorContextType = WICColorContextType(1i32);
pub const WICColorContextUninitialized: WICColorContextType = WICColorContextType(0i32);
pub const WICComponentDisabled: WICComponentSigning = WICComponentSigning(-2147483648i32);
pub const WICComponentEnumerateBuiltInOnly: WICComponentEnumerateOptions = WICComponentEnumerateOptions(536870912i32);
pub const WICComponentEnumerateDefault: WICComponentEnumerateOptions = WICComponentEnumerateOptions(0i32);
pub const WICComponentEnumerateDisabled: WICComponentEnumerateOptions = WICComponentEnumerateOptions(-2147483648i32);
pub const WICComponentEnumerateRefresh: WICComponentEnumerateOptions = WICComponentEnumerateOptions(1i32);
pub const WICComponentEnumerateUnsigned: WICComponentEnumerateOptions = WICComponentEnumerateOptions(1073741824i32);
pub const WICComponentSafe: WICComponentSigning = WICComponentSigning(4i32);
pub const WICComponentSigned: WICComponentSigning = WICComponentSigning(1i32);
pub const WICComponentUnsigned: WICComponentSigning = WICComponentSigning(2i32);
pub const WICDdsAlphaModeCustom: WICDdsAlphaMode = WICDdsAlphaMode(4i32);
pub const WICDdsAlphaModeOpaque: WICDdsAlphaMode = WICDdsAlphaMode(3i32);
pub const WICDdsAlphaModePremultiplied: WICDdsAlphaMode = WICDdsAlphaMode(2i32);
pub const WICDdsAlphaModeStraight: WICDdsAlphaMode = WICDdsAlphaMode(1i32);
pub const WICDdsAlphaModeUnknown: WICDdsAlphaMode = WICDdsAlphaMode(0i32);
pub const WICDdsTexture1D: WICDdsDimension = WICDdsDimension(0i32);
pub const WICDdsTexture2D: WICDdsDimension = WICDdsDimension(1i32);
pub const WICDdsTexture3D: WICDdsDimension = WICDdsDimension(2i32);
pub const WICDdsTextureCube: WICDdsDimension = WICDdsDimension(3i32);
pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = WICDecodeOptions(0i32);
pub const WICDecodeMetadataCacheOnLoad: WICDecodeOptions = WICDecodeOptions(1i32);
pub const WICDecoder: WICComponentType = WICComponentType(1i32);
pub const WICEncoder: WICComponentType = WICComponentType(2i32);
pub const WICGifApplicationExtensionApplication: WICGifApplicationExtensionProperties = WICGifApplicationExtensionProperties(1i32);
pub const WICGifApplicationExtensionData: WICGifApplicationExtensionProperties = WICGifApplicationExtensionProperties(2i32);
pub const WICGifCommentExtensionText: WICGifCommentExtensionProperties = WICGifCommentExtensionProperties(1i32);
pub const WICGifGraphicControlExtensionDelay: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(4i32);
pub const WICGifGraphicControlExtensionDisposal: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(1i32);
pub const WICGifGraphicControlExtensionTransparencyFlag: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(3i32);
pub const WICGifGraphicControlExtensionTransparentColorIndex: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(5i32);
pub const WICGifGraphicControlExtensionUserInputFlag: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(2i32);
pub const WICGifImageDescriptorHeight: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(4i32);
pub const WICGifImageDescriptorInterlaceFlag: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(6i32);
pub const WICGifImageDescriptorLeft: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(1i32);
pub const WICGifImageDescriptorLocalColorTableFlag: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(5i32);
pub const WICGifImageDescriptorLocalColorTableSize: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(8i32);
pub const WICGifImageDescriptorSortFlag: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(7i32);
pub const WICGifImageDescriptorTop: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(2i32);
pub const WICGifImageDescriptorWidth: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(3i32);
pub const WICGifLogicalScreenDescriptorBackgroundColorIndex: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(8i32);
pub const WICGifLogicalScreenDescriptorColorResolution: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(5i32);
pub const WICGifLogicalScreenDescriptorGlobalColorTableFlag: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(4i32);
pub const WICGifLogicalScreenDescriptorGlobalColorTableSize: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(7i32);
pub const WICGifLogicalScreenDescriptorHeight: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(3i32);
pub const WICGifLogicalScreenDescriptorPixelAspectRatio: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(9i32);
pub const WICGifLogicalScreenDescriptorSortFlag: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(6i32);
pub const WICGifLogicalScreenDescriptorWidth: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(2i32);
pub const WICGifLogicalScreenSignature: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(1i32);
pub const WICHeifHdrCustomVideoPrimaries: WICHeifHdrProperties = WICHeifHdrProperties(5i32);
pub const WICHeifHdrMaximumFrameAverageLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(2i32);
pub const WICHeifHdrMaximumLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(1i32);
pub const WICHeifHdrMaximumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(4i32);
pub const WICHeifHdrMinimumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(3i32);
pub const WICHeifOrientation: WICHeifProperties = WICHeifProperties(1i32);
pub const WICJpegChrominanceTable: WICJpegChrominanceProperties = WICJpegChrominanceProperties(1i32);
pub const WICJpegCommentText: WICJpegCommentProperties = WICJpegCommentProperties(1i32);
pub const WICJpegIndexingOptionsGenerateOnDemand: WICJpegIndexingOptions = WICJpegIndexingOptions(0i32);
pub const WICJpegIndexingOptionsGenerateOnLoad: WICJpegIndexingOptions = WICJpegIndexingOptions(1i32);
pub const WICJpegLuminanceTable: WICJpegLuminanceProperties = WICJpegLuminanceProperties(1i32);
pub const WICJpegScanTypeInterleaved: WICJpegScanType = WICJpegScanType(0i32);
pub const WICJpegScanTypePlanarComponents: WICJpegScanType = WICJpegScanType(1i32);
pub const WICJpegScanTypeProgressive: WICJpegScanType = WICJpegScanType(2i32);
pub const WICJpegTransferMatrixBT601: WICJpegTransferMatrix = WICJpegTransferMatrix(1i32);
pub const WICJpegTransferMatrixIdentity: WICJpegTransferMatrix = WICJpegTransferMatrix(0i32);
pub const WICJpegYCrCbSubsampling420: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(1i32);
pub const WICJpegYCrCbSubsampling422: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(2i32);
pub const WICJpegYCrCbSubsampling440: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(4i32);
pub const WICJpegYCrCbSubsampling444: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(3i32);
pub const WICJpegYCrCbSubsamplingDefault: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(0i32);
pub const WICMetadataCreationAllowUnknown: WICMetadataCreationOptions = WICMetadataCreationOptions(0i32);
pub const WICMetadataCreationDefault: WICMetadataCreationOptions = WICMetadataCreationOptions(0i32);
pub const WICMetadataCreationFailUnknown: WICMetadataCreationOptions = WICMetadataCreationOptions(65536i32);
pub const WICMetadataCreationMask: WICMetadataCreationOptions = WICMetadataCreationOptions(-65536i32);
pub const WICMetadataReader: WICComponentType = WICComponentType(8i32);
pub const WICMetadataWriter: WICComponentType = WICComponentType(16i32);
pub const WICPersistOptionBigEndian: WICPersistOptions = WICPersistOptions(1i32);
pub const WICPersistOptionDefault: WICPersistOptions = WICPersistOptions(0i32);
pub const WICPersistOptionLittleEndian: WICPersistOptions = WICPersistOptions(0i32);
pub const WICPersistOptionMask: WICPersistOptions = WICPersistOptions(65535i32);
pub const WICPersistOptionNoCacheStream: WICPersistOptions = WICPersistOptions(4i32);
pub const WICPersistOptionPreferUTF8: WICPersistOptions = WICPersistOptions(8i32);
pub const WICPersistOptionStrictFormat: WICPersistOptions = WICPersistOptions(2i32);
pub const WICPixelFormat: WICComponentType = WICComponentType(32i32);
pub const WICPixelFormatConverter: WICComponentType = WICComponentType(4i32);
pub const WICPixelFormatNumericRepresentationFixed: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(4i32);
pub const WICPixelFormatNumericRepresentationFloat: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(5i32);
pub const WICPixelFormatNumericRepresentationIndexed: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(1i32);
pub const WICPixelFormatNumericRepresentationSignedInteger: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(3i32);
pub const WICPixelFormatNumericRepresentationUnsignedInteger: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(2i32);
pub const WICPixelFormatNumericRepresentationUnspecified: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(0i32);
pub const WICPlanarOptionsDefault: WICPlanarOptions = WICPlanarOptions(0i32);
pub const WICPlanarOptionsPreserveSubsampling: WICPlanarOptions = WICPlanarOptions(1i32);
pub const WICPngBkgdBackgroundColor: WICPngBkgdProperties = WICPngBkgdProperties(1i32);
pub const WICPngChrmBlueX: WICPngChrmProperties = WICPngChrmProperties(7i32);
pub const WICPngChrmBlueY: WICPngChrmProperties = WICPngChrmProperties(8i32);
pub const WICPngChrmGreenX: WICPngChrmProperties = WICPngChrmProperties(5i32);
pub const WICPngChrmGreenY: WICPngChrmProperties = WICPngChrmProperties(6i32);
pub const WICPngChrmRedX: WICPngChrmProperties = WICPngChrmProperties(3i32);
pub const WICPngChrmRedY: WICPngChrmProperties = WICPngChrmProperties(4i32);
pub const WICPngChrmWhitePointX: WICPngChrmProperties = WICPngChrmProperties(1i32);
pub const WICPngChrmWhitePointY: WICPngChrmProperties = WICPngChrmProperties(2i32);
pub const WICPngFilterAdaptive: WICPngFilterOption = WICPngFilterOption(6i32);
pub const WICPngFilterAverage: WICPngFilterOption = WICPngFilterOption(4i32);
pub const WICPngFilterNone: WICPngFilterOption = WICPngFilterOption(1i32);
pub const WICPngFilterPaeth: WICPngFilterOption = WICPngFilterOption(5i32);
pub const WICPngFilterSub: WICPngFilterOption = WICPngFilterOption(2i32);
pub const WICPngFilterUnspecified: WICPngFilterOption = WICPngFilterOption(0i32);
pub const WICPngFilterUp: WICPngFilterOption = WICPngFilterOption(3i32);
pub const WICPngGamaGamma: WICPngGamaProperties = WICPngGamaProperties(1i32);
pub const WICPngHistFrequencies: WICPngHistProperties = WICPngHistProperties(1i32);
pub const WICPngIccpProfileData: WICPngIccpProperties = WICPngIccpProperties(2i32);
pub const WICPngIccpProfileName: WICPngIccpProperties = WICPngIccpProperties(1i32);
pub const WICPngItxtCompressionFlag: WICPngItxtProperties = WICPngItxtProperties(2i32);
pub const WICPngItxtKeyword: WICPngItxtProperties = WICPngItxtProperties(1i32);
pub const WICPngItxtLanguageTag: WICPngItxtProperties = WICPngItxtProperties(3i32);
pub const WICPngItxtText: WICPngItxtProperties = WICPngItxtProperties(5i32);
pub const WICPngItxtTranslatedKeyword: WICPngItxtProperties = WICPngItxtProperties(4i32);
pub const WICPngSrgbRenderingIntent: WICPngSrgbProperties = WICPngSrgbProperties(1i32);
pub const WICPngTimeDay: WICPngTimeProperties = WICPngTimeProperties(3i32);
pub const WICPngTimeHour: WICPngTimeProperties = WICPngTimeProperties(4i32);
pub const WICPngTimeMinute: WICPngTimeProperties = WICPngTimeProperties(5i32);
pub const WICPngTimeMonth: WICPngTimeProperties = WICPngTimeProperties(2i32);
pub const WICPngTimeSecond: WICPngTimeProperties = WICPngTimeProperties(6i32);
pub const WICPngTimeYear: WICPngTimeProperties = WICPngTimeProperties(1i32);
pub const WICProgressNotificationAll: WICProgressNotification = WICProgressNotification(-65536i32);
pub const WICProgressNotificationBegin: WICProgressNotification = WICProgressNotification(65536i32);
pub const WICProgressNotificationEnd: WICProgressNotification = WICProgressNotification(131072i32);
pub const WICProgressNotificationFrequent: WICProgressNotification = WICProgressNotification(262144i32);
pub const WICProgressOperationAll: WICProgressOperation = WICProgressOperation(65535i32);
pub const WICProgressOperationCopyPixels: WICProgressOperation = WICProgressOperation(1i32);
pub const WICProgressOperationWritePixels: WICProgressOperation = WICProgressOperation(2i32);
pub const WICRawCapabilityFullySupported: WICRawCapabilities = WICRawCapabilities(2i32);
pub const WICRawCapabilityGetSupported: WICRawCapabilities = WICRawCapabilities(1i32);
pub const WICRawCapabilityNotSupported: WICRawCapabilities = WICRawCapabilities(0i32);
pub const WICRawChangeNotification_Contrast: u32 = 16u32;
pub const WICRawChangeNotification_DestinationColorContext: u32 = 1024u32;
pub const WICRawChangeNotification_ExposureCompensation: u32 = 1u32;
pub const WICRawChangeNotification_Gamma: u32 = 32u32;
pub const WICRawChangeNotification_KelvinWhitePoint: u32 = 4u32;
pub const WICRawChangeNotification_NamedWhitePoint: u32 = 2u32;
pub const WICRawChangeNotification_NoiseReduction: u32 = 512u32;
pub const WICRawChangeNotification_RGBWhitePoint: u32 = 8u32;
pub const WICRawChangeNotification_RenderMode: u32 = 8192u32;
pub const WICRawChangeNotification_Rotation: u32 = 4096u32;
pub const WICRawChangeNotification_Saturation: u32 = 128u32;
pub const WICRawChangeNotification_Sharpness: u32 = 64u32;
pub const WICRawChangeNotification_Tint: u32 = 256u32;
pub const WICRawChangeNotification_ToneCurve: u32 = 2048u32;
pub const WICRawRenderModeBestQuality: WICRawRenderMode = WICRawRenderMode(3i32);
pub const WICRawRenderModeDraft: WICRawRenderMode = WICRawRenderMode(1i32);
pub const WICRawRenderModeNormal: WICRawRenderMode = WICRawRenderMode(2i32);
pub const WICRawRotationCapabilityFullySupported: WICRawRotationCapabilities = WICRawRotationCapabilities(3i32);
pub const WICRawRotationCapabilityGetSupported: WICRawRotationCapabilities = WICRawRotationCapabilities(1i32);
pub const WICRawRotationCapabilityNinetyDegreesSupported: WICRawRotationCapabilities = WICRawRotationCapabilities(2i32);
pub const WICRawRotationCapabilityNotSupported: WICRawRotationCapabilities = WICRawRotationCapabilities(0i32);
pub const WICSectionAccessLevelRead: WICSectionAccessLevel = WICSectionAccessLevel(1i32);
pub const WICSectionAccessLevelReadWrite: WICSectionAccessLevel = WICSectionAccessLevel(3i32);
pub const WICTiffCompressionCCITT3: WICTiffCompressionOption = WICTiffCompressionOption(2i32);
pub const WICTiffCompressionCCITT4: WICTiffCompressionOption = WICTiffCompressionOption(3i32);
pub const WICTiffCompressionDontCare: WICTiffCompressionOption = WICTiffCompressionOption(0i32);
pub const WICTiffCompressionLZW: WICTiffCompressionOption = WICTiffCompressionOption(4i32);
pub const WICTiffCompressionLZWHDifferencing: WICTiffCompressionOption = WICTiffCompressionOption(7i32);
pub const WICTiffCompressionNone: WICTiffCompressionOption = WICTiffCompressionOption(1i32);
pub const WICTiffCompressionRLE: WICTiffCompressionOption = WICTiffCompressionOption(5i32);
pub const WICTiffCompressionZIP: WICTiffCompressionOption = WICTiffCompressionOption(6i32);
pub const WICUserAdjustedParameterSet: WICRawParameterSet = WICRawParameterSet(2i32);
pub const WICWebpAnimLoopCount: WICWebpAnimProperties = WICWebpAnimProperties(1i32);
pub const WICWebpAnmfFrameDuration: WICWebpAnmfProperties = WICWebpAnmfProperties(1i32);
pub const WICWhitePointAsShot: WICNamedWhitePoint = WICNamedWhitePoint(1i32);
pub const WICWhitePointAutoWhiteBalance: WICNamedWhitePoint = WICNamedWhitePoint(512i32);
pub const WICWhitePointCloudy: WICNamedWhitePoint = WICNamedWhitePoint(4i32);
pub const WICWhitePointCustom: WICNamedWhitePoint = WICNamedWhitePoint(256i32);
pub const WICWhitePointDaylight: WICNamedWhitePoint = WICNamedWhitePoint(2i32);
pub const WICWhitePointDefault: WICNamedWhitePoint = WICNamedWhitePoint(1i32);
pub const WICWhitePointFlash: WICNamedWhitePoint = WICNamedWhitePoint(64i32);
pub const WICWhitePointFluorescent: WICNamedWhitePoint = WICNamedWhitePoint(32i32);
pub const WICWhitePointShade: WICNamedWhitePoint = WICNamedWhitePoint(8i32);
pub const WICWhitePointTungsten: WICNamedWhitePoint = WICNamedWhitePoint(16i32);
pub const WICWhitePointUnderwater: WICNamedWhitePoint = WICNamedWhitePoint(128i32);
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: u32 = 0u32;
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: u32 = 1118464u32;
pub const WIC_JPEG_MAX_COMPONENT_COUNT: u32 = 4u32;
pub const WIC_JPEG_MAX_TABLE_INDEX: u32 = 3u32;
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: u32 = 0u32;
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: u32 = 65792u32;
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: u32 = 17u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: u32 = 1118498u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: u32 = 1118497u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: u32 = 1118482u32;
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: u32 = 1118481u32;
pub const WINCODEC_ERR_ABORTED: i32 = -2147467260i32;
pub const WINCODEC_ERR_ACCESSDENIED: i32 = -2147024891i32;
pub const WINCODEC_ERR_BASE: u32 = 8192u32;
pub const WINCODEC_ERR_GENERIC_ERROR: i32 = -2147467259i32;
pub const WINCODEC_ERR_INVALIDPARAMETER: i32 = -2147024809i32;
pub const WINCODEC_ERR_NOTIMPLEMENTED: i32 = -2147467263i32;
pub const WINCODEC_ERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const WINCODEC_SDK_VERSION: u32 = 567u32;
pub const WINCODEC_SDK_VERSION1: u32 = 566u32;
pub const WINCODEC_SDK_VERSION2: u32 = 567u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WIC8BIMIptcDigestProperties(pub i32);
impl windows_core::TypeKind for WIC8BIMIptcDigestProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WIC8BIMIptcDigestProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WIC8BIMIptcDigestProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WIC8BIMIptcProperties(pub i32);
impl windows_core::TypeKind for WIC8BIMIptcProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WIC8BIMIptcProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WIC8BIMIptcProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WIC8BIMResolutionInfoProperties(pub i32);
impl windows_core::TypeKind for WIC8BIMResolutionInfoProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WIC8BIMResolutionInfoProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WIC8BIMResolutionInfoProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapAlphaChannelOption(pub i32);
impl windows_core::TypeKind for WICBitmapAlphaChannelOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapAlphaChannelOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapAlphaChannelOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapCreateCacheOption(pub i32);
impl windows_core::TypeKind for WICBitmapCreateCacheOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapCreateCacheOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapCreateCacheOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapDecoderCapabilities(pub i32);
impl windows_core::TypeKind for WICBitmapDecoderCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapDecoderCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapDecoderCapabilities").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapDitherType(pub i32);
impl windows_core::TypeKind for WICBitmapDitherType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapDitherType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapDitherType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapEncoderCacheOption(pub i32);
impl windows_core::TypeKind for WICBitmapEncoderCacheOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapEncoderCacheOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapEncoderCacheOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapInterpolationMode(pub i32);
impl windows_core::TypeKind for WICBitmapInterpolationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapInterpolationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapInterpolationMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapLockFlags(pub i32);
impl windows_core::TypeKind for WICBitmapLockFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapLockFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapLockFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapPaletteType(pub i32);
impl windows_core::TypeKind for WICBitmapPaletteType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapPaletteType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapPaletteType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICBitmapTransformOptions(pub i32);
impl windows_core::TypeKind for WICBitmapTransformOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICBitmapTransformOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICBitmapTransformOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICColorContextType(pub i32);
impl windows_core::TypeKind for WICColorContextType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICColorContextType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICColorContextType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICComponentEnumerateOptions(pub i32);
impl windows_core::TypeKind for WICComponentEnumerateOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICComponentEnumerateOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICComponentEnumerateOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICComponentSigning(pub i32);
impl windows_core::TypeKind for WICComponentSigning {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICComponentSigning {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICComponentSigning").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICComponentType(pub i32);
impl windows_core::TypeKind for WICComponentType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICComponentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICComponentType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICDdsAlphaMode(pub i32);
impl windows_core::TypeKind for WICDdsAlphaMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICDdsAlphaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICDdsAlphaMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICDdsDimension(pub i32);
impl windows_core::TypeKind for WICDdsDimension {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICDdsDimension {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICDdsDimension").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICDecodeOptions(pub i32);
impl windows_core::TypeKind for WICDecodeOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICDecodeOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICDecodeOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICGifApplicationExtensionProperties(pub i32);
impl windows_core::TypeKind for WICGifApplicationExtensionProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICGifApplicationExtensionProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICGifApplicationExtensionProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICGifCommentExtensionProperties(pub i32);
impl windows_core::TypeKind for WICGifCommentExtensionProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICGifCommentExtensionProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICGifCommentExtensionProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICGifGraphicControlExtensionProperties(pub i32);
impl windows_core::TypeKind for WICGifGraphicControlExtensionProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICGifGraphicControlExtensionProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICGifGraphicControlExtensionProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICGifImageDescriptorProperties(pub i32);
impl windows_core::TypeKind for WICGifImageDescriptorProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICGifImageDescriptorProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICGifImageDescriptorProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICGifLogicalScreenDescriptorProperties(pub i32);
impl windows_core::TypeKind for WICGifLogicalScreenDescriptorProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICGifLogicalScreenDescriptorProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICGifLogicalScreenDescriptorProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICHeifHdrProperties(pub i32);
impl windows_core::TypeKind for WICHeifHdrProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICHeifHdrProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICHeifHdrProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICHeifProperties(pub i32);
impl windows_core::TypeKind for WICHeifProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICHeifProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICHeifProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegChrominanceProperties(pub i32);
impl windows_core::TypeKind for WICJpegChrominanceProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegChrominanceProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegChrominanceProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegCommentProperties(pub i32);
impl windows_core::TypeKind for WICJpegCommentProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegCommentProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegCommentProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegIndexingOptions(pub i32);
impl windows_core::TypeKind for WICJpegIndexingOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegIndexingOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegIndexingOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegLuminanceProperties(pub i32);
impl windows_core::TypeKind for WICJpegLuminanceProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegLuminanceProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegLuminanceProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegScanType(pub i32);
impl windows_core::TypeKind for WICJpegScanType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegScanType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegScanType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegTransferMatrix(pub i32);
impl windows_core::TypeKind for WICJpegTransferMatrix {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegTransferMatrix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegTransferMatrix").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICJpegYCrCbSubsamplingOption(pub i32);
impl windows_core::TypeKind for WICJpegYCrCbSubsamplingOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICJpegYCrCbSubsamplingOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICJpegYCrCbSubsamplingOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICMetadataCreationOptions(pub i32);
impl windows_core::TypeKind for WICMetadataCreationOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICMetadataCreationOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICMetadataCreationOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICNamedWhitePoint(pub i32);
impl windows_core::TypeKind for WICNamedWhitePoint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICNamedWhitePoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICNamedWhitePoint").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPersistOptions(pub i32);
impl windows_core::TypeKind for WICPersistOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPersistOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPersistOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPixelFormatNumericRepresentation(pub i32);
impl windows_core::TypeKind for WICPixelFormatNumericRepresentation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPixelFormatNumericRepresentation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPixelFormatNumericRepresentation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPlanarOptions(pub i32);
impl windows_core::TypeKind for WICPlanarOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPlanarOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPlanarOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngBkgdProperties(pub i32);
impl windows_core::TypeKind for WICPngBkgdProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngBkgdProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngBkgdProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngChrmProperties(pub i32);
impl windows_core::TypeKind for WICPngChrmProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngChrmProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngChrmProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngFilterOption(pub i32);
impl windows_core::TypeKind for WICPngFilterOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngFilterOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngFilterOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngGamaProperties(pub i32);
impl windows_core::TypeKind for WICPngGamaProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngGamaProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngGamaProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngHistProperties(pub i32);
impl windows_core::TypeKind for WICPngHistProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngHistProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngHistProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngIccpProperties(pub i32);
impl windows_core::TypeKind for WICPngIccpProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngIccpProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngIccpProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngItxtProperties(pub i32);
impl windows_core::TypeKind for WICPngItxtProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngItxtProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngItxtProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngSrgbProperties(pub i32);
impl windows_core::TypeKind for WICPngSrgbProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngSrgbProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngSrgbProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICPngTimeProperties(pub i32);
impl windows_core::TypeKind for WICPngTimeProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICPngTimeProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICPngTimeProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICProgressNotification(pub i32);
impl windows_core::TypeKind for WICProgressNotification {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICProgressNotification {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICProgressNotification").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICProgressOperation(pub i32);
impl windows_core::TypeKind for WICProgressOperation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICProgressOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICProgressOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICRawCapabilities(pub i32);
impl windows_core::TypeKind for WICRawCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICRawCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICRawCapabilities").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICRawParameterSet(pub i32);
impl windows_core::TypeKind for WICRawParameterSet {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICRawParameterSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICRawParameterSet").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICRawRenderMode(pub i32);
impl windows_core::TypeKind for WICRawRenderMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICRawRenderMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICRawRenderMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICRawRotationCapabilities(pub i32);
impl windows_core::TypeKind for WICRawRotationCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICRawRotationCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICRawRotationCapabilities").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICSectionAccessLevel(pub i32);
impl windows_core::TypeKind for WICSectionAccessLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICSectionAccessLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICSectionAccessLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICTiffCompressionOption(pub i32);
impl windows_core::TypeKind for WICTiffCompressionOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICTiffCompressionOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICTiffCompressionOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICWebpAnimProperties(pub i32);
impl windows_core::TypeKind for WICWebpAnimProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICWebpAnimProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICWebpAnimProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WICWebpAnmfProperties(pub i32);
impl windows_core::TypeKind for WICWebpAnmfProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WICWebpAnmfProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WICWebpAnmfProperties").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICBitmapPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub EndOfStream: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WICBitmapPattern {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICBitmapPattern {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICBitmapPlane {
    pub Format: windows_core::GUID,
    pub pbBuffer: *mut u8,
    pub cbStride: u32,
    pub cbBufferSize: u32,
}
impl windows_core::TypeKind for WICBitmapPlane {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICBitmapPlane {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICBitmapPlaneDescription {
    pub Format: windows_core::GUID,
    pub Width: u32,
    pub Height: u32,
}
impl windows_core::TypeKind for WICBitmapPlaneDescription {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICBitmapPlaneDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICDdsFormatInfo {
    pub DxgiFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub BytesPerBlock: u32,
    pub BlockWidth: u32,
    pub BlockHeight: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for WICDdsFormatInfo {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for WICDdsFormatInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICDdsParameters {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub DxgiFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub Dimension: WICDdsDimension,
    pub AlphaMode: WICDdsAlphaMode,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for WICDdsParameters {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for WICDdsParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WICImageParameters {
    pub PixelFormat: super::Direct2D::Common::D2D1_PIXEL_FORMAT,
    pub DpiX: f32,
    pub DpiY: f32,
    pub Top: f32,
    pub Left: f32,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::TypeKind for WICImageParameters {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl Default for WICImageParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICJpegFrameHeader {
    pub Width: u32,
    pub Height: u32,
    pub TransferMatrix: WICJpegTransferMatrix,
    pub ScanType: WICJpegScanType,
    pub cComponents: u32,
    pub ComponentIdentifiers: u32,
    pub SampleFactors: u32,
    pub QuantizationTableIndices: u32,
}
impl windows_core::TypeKind for WICJpegFrameHeader {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICJpegFrameHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICJpegScanHeader {
    pub cComponents: u32,
    pub RestartInterval: u32,
    pub ComponentSelectors: u32,
    pub HuffmanTableIndices: u32,
    pub StartSpectralSelection: u8,
    pub EndSpectralSelection: u8,
    pub SuccessiveApproximationHigh: u8,
    pub SuccessiveApproximationLow: u8,
}
impl windows_core::TypeKind for WICJpegScanHeader {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICJpegScanHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICMetadataHeader {
    pub Position: u64,
    pub Length: u32,
    pub Header: *mut u8,
    pub DataOffset: u64,
}
impl windows_core::TypeKind for WICMetadataHeader {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICMetadataHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICMetadataPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub DataOffset: u64,
}
impl windows_core::TypeKind for WICMetadataPattern {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICMetadataPattern {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICRawCapabilitiesInfo {
    pub cbSize: u32,
    pub CodecMajorVersion: u32,
    pub CodecMinorVersion: u32,
    pub ExposureCompensationSupport: WICRawCapabilities,
    pub ContrastSupport: WICRawCapabilities,
    pub RGBWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupportMask: u32,
    pub KelvinWhitePointSupport: WICRawCapabilities,
    pub GammaSupport: WICRawCapabilities,
    pub TintSupport: WICRawCapabilities,
    pub SaturationSupport: WICRawCapabilities,
    pub SharpnessSupport: WICRawCapabilities,
    pub NoiseReductionSupport: WICRawCapabilities,
    pub DestinationColorProfileSupport: WICRawCapabilities,
    pub ToneCurveSupport: WICRawCapabilities,
    pub RotationSupport: WICRawRotationCapabilities,
    pub RenderModeSupport: WICRawCapabilities,
}
impl windows_core::TypeKind for WICRawCapabilitiesInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICRawCapabilitiesInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WICRawToneCurve {
    pub cPoints: u32,
    pub aPoints: [WICRawToneCurvePoint; 1],
}
impl windows_core::TypeKind for WICRawToneCurve {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICRawToneCurve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WICRawToneCurvePoint {
    pub Input: f64,
    pub Output: f64,
}
impl windows_core::TypeKind for WICRawToneCurvePoint {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICRawToneCurvePoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl windows_core::TypeKind for WICRect {
    type TypeKind = windows_core::CopyType;
}
impl Default for WICRect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFNProgressNotification = Option<unsafe extern "system" fn(pvdata: *const core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

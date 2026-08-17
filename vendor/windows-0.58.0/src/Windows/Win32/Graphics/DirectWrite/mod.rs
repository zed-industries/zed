#[inline]
pub unsafe fn DWriteCreateFactory<T>(factorytype: DWRITE_FACTORY_TYPE) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    DWriteCreateFactory(factorytype, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(IDWriteAsyncResult, IDWriteAsyncResult_Vtbl, 0xce25f8fd_863b_4d13_9651_c1f88dc73fe2);
impl core::ops::Deref for IDWriteAsyncResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteAsyncResult, windows_core::IUnknown);
impl IDWriteAsyncResult {
    pub unsafe fn GetWaitHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).GetWaitHandle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetResult(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for IDWriteAsyncResult {}
unsafe impl Sync for IDWriteAsyncResult {}
#[repr(C)]
pub struct IDWriteAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWaitHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget, IDWriteBitmapRenderTarget_Vtbl, 0x5e5a32a3_8dff_4773_9ff6_0696eab77267);
impl core::ops::Deref for IDWriteBitmapRenderTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget, windows_core::IUnknown);
impl IDWriteBitmapRenderTarget {
    pub unsafe fn DrawGlyphRun<P0, P1>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: P0, textcolor: P1, blackboxrect: Option<*mut super::super::Foundation::RECT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
        P1: windows_core::Param<super::super::Foundation::COLORREF>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, glyphrun, renderingparams.param().abi(), textcolor.param().abi(), core::mem::transmute(blackboxrect.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetMemoryDC(&self) -> super::Gdi::HDC {
        (windows_core::Interface::vtable(self).GetMemoryDC)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPixelsPerDip(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetPixelsPerDip)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetPixelsPerDip(&self, pixelsperdip: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPixelsPerDip)(windows_core::Interface::as_raw(self), pixelsperdip).ok()
    }
    pub unsafe fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentTransform)(windows_core::Interface::as_raw(self), transform).ok()
    }
    pub unsafe fn SetCurrentTransform(&self, transform: Option<*const DWRITE_MATRIX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(transform.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height).ok()
    }
}
unsafe impl Send for IDWriteBitmapRenderTarget {}
unsafe impl Sync for IDWriteBitmapRenderTarget {}
#[repr(C)]
pub struct IDWriteBitmapRenderTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, *mut core::ffi::c_void, super::super::Foundation::COLORREF, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetMemoryDC: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetMemoryDC: usize,
    pub GetPixelsPerDip: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub SetPixelsPerDip: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_MATRIX) -> windows_core::HRESULT,
    pub SetCurrentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget1, IDWriteBitmapRenderTarget1_Vtbl, 0x791e8298_3ef3_4230_9880_c9bdecc42064);
impl core::ops::Deref for IDWriteBitmapRenderTarget1 {
    type Target = IDWriteBitmapRenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget1, windows_core::IUnknown, IDWriteBitmapRenderTarget);
impl IDWriteBitmapRenderTarget1 {
    pub unsafe fn GetTextAntialiasMode(&self) -> DWRITE_TEXT_ANTIALIAS_MODE {
        (windows_core::Interface::vtable(self).GetTextAntialiasMode)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextAntialiasMode)(windows_core::Interface::as_raw(self), antialiasmode).ok()
    }
}
unsafe impl Send for IDWriteBitmapRenderTarget1 {}
unsafe impl Sync for IDWriteBitmapRenderTarget1 {}
#[repr(C)]
pub struct IDWriteBitmapRenderTarget1_Vtbl {
    pub base__: IDWriteBitmapRenderTarget_Vtbl,
    pub GetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget2, IDWriteBitmapRenderTarget2_Vtbl, 0xc553a742_fc01_44da_a66e_b8b9ed6c3995);
impl core::ops::Deref for IDWriteBitmapRenderTarget2 {
    type Target = IDWriteBitmapRenderTarget1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget2, windows_core::IUnknown, IDWriteBitmapRenderTarget, IDWriteBitmapRenderTarget1);
impl IDWriteBitmapRenderTarget2 {
    pub unsafe fn GetBitmapData(&self) -> windows_core::Result<DWRITE_BITMAP_DATA_BGRA32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitmapData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteBitmapRenderTarget2 {}
unsafe impl Sync for IDWriteBitmapRenderTarget2 {}
#[repr(C)]
pub struct IDWriteBitmapRenderTarget2_Vtbl {
    pub base__: IDWriteBitmapRenderTarget1_Vtbl,
    pub GetBitmapData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_BITMAP_DATA_BGRA32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget3, IDWriteBitmapRenderTarget3_Vtbl, 0xaeec37db_c337_40f1_8e2a_9a41b167b238);
impl core::ops::Deref for IDWriteBitmapRenderTarget3 {
    type Target = IDWriteBitmapRenderTarget2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget3, windows_core::IUnknown, IDWriteBitmapRenderTarget, IDWriteBitmapRenderTarget1, IDWriteBitmapRenderTarget2);
impl IDWriteBitmapRenderTarget3 {
    pub unsafe fn GetPaintFeatureLevel(&self) -> DWRITE_PAINT_FEATURE_LEVEL {
        (windows_core::Interface::vtable(self).GetPaintFeatureLevel)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn DrawPaintGlyphRun<P0>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, textcolor: P0, colorpaletteindex: u32, blackboxrect: Option<*mut super::super::Foundation::RECT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::COLORREF>,
    {
        (windows_core::Interface::vtable(self).DrawPaintGlyphRun)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, glyphrun, glyphimageformat, textcolor.param().abi(), colorpaletteindex, core::mem::transmute(blackboxrect.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DrawGlyphRunWithColorSupport<P0, P1>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: P0, textcolor: P1, colorpaletteindex: u32, blackboxrect: Option<*mut super::super::Foundation::RECT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
        P1: windows_core::Param<super::super::Foundation::COLORREF>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRunWithColorSupport)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, glyphrun, renderingparams.param().abi(), textcolor.param().abi(), colorpaletteindex, core::mem::transmute(blackboxrect.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
unsafe impl Send for IDWriteBitmapRenderTarget3 {}
unsafe impl Sync for IDWriteBitmapRenderTarget3 {}
#[repr(C)]
pub struct IDWriteBitmapRenderTarget3_Vtbl {
    pub base__: IDWriteBitmapRenderTarget2_Vtbl,
    pub GetPaintFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PAINT_FEATURE_LEVEL,
    pub DrawPaintGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, DWRITE_GLYPH_IMAGE_FORMATS, super::super::Foundation::COLORREF, u32, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub DrawGlyphRunWithColorSupport: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, *mut core::ffi::c_void, super::super::Foundation::COLORREF, u32, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteColorGlyphRunEnumerator, IDWriteColorGlyphRunEnumerator_Vtbl, 0xd31fbe17_f157_41a2_8d24_cb779e0560e8);
impl core::ops::Deref for IDWriteColorGlyphRunEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator, windows_core::IUnknown);
impl IDWriteColorGlyphRunEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteColorGlyphRunEnumerator {}
unsafe impl Sync for IDWriteColorGlyphRunEnumerator {}
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrentRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DWRITE_COLOR_GLYPH_RUN) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteColorGlyphRunEnumerator1, IDWriteColorGlyphRunEnumerator1_Vtbl, 0x7c5f86da_c7a1_4f05_b8e1_55a179fe5a35);
impl core::ops::Deref for IDWriteColorGlyphRunEnumerator1 {
    type Target = IDWriteColorGlyphRunEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator1, windows_core::IUnknown, IDWriteColorGlyphRunEnumerator);
impl IDWriteColorGlyphRunEnumerator1 {
    pub unsafe fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteColorGlyphRunEnumerator1 {}
unsafe impl Sync for IDWriteColorGlyphRunEnumerator1 {}
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumerator1_Vtbl {
    pub base__: IDWriteColorGlyphRunEnumerator_Vtbl,
    pub GetCurrentRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory, IDWriteFactory_Vtbl, 0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48);
impl core::ops::Deref for IDWriteFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut Option<IDWriteFontCollection>, checkforupdates: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(fontcollection), checkforupdates.param().abi()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontCollection>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomFontCollection)(windows_core::Interface::as_raw(self), collectionloader.param().abi(), collectionkey, collectionkeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        (windows_core::Interface::vtable(self).RegisterFontCollectionLoader)(windows_core::Interface::as_raw(self), fontcollectionloader.param().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        (windows_core::Interface::vtable(self).UnregisterFontCollectionLoader)(windows_core::Interface::as_raw(self), fontcollectionloader.param().abi()).ok()
    }
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: Option<*const super::super::Foundation::FILETIME>) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFileReference)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(lastwritetime.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomFontFileReference)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), fontfacetype, fontfiles.len().try_into().unwrap(), core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRenderingParams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> windows_core::Result<IDWriteRenderingParams>
    where
        P0: windows_core::Param<super::Gdi::HMONITOR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMonitorRenderingParams)(windows_core::Interface::as_raw(self), monitor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        (windows_core::Interface::vtable(self).RegisterFontFileLoader)(windows_core::Interface::as_raw(self), fontfileloader.param().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        (windows_core::Interface::vtable(self).UnregisterFontFileLoader)(windows_core::Interface::as_raw(self), fontfileloader.param().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> windows_core::Result<IDWriteTextFormat>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTextFormat)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), fontcollection.param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTypography)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGdiInterop)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> windows_core::Result<IDWriteTextLayout>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), maxwidth, maxheight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> windows_core::Result<IDWriteTextLayout>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGdiCompatibleTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), layoutwidth, layoutheight, pixelsperdip, core::mem::transmute(transform.unwrap_or(std::ptr::null())), usegdinatural.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> windows_core::Result<IDWriteInlineObject>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEllipsisTrimmingSign)(windows_core::Interface::as_raw(self), textformat.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTextAnalyzer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNumberSubstitution)(windows_core::Interface::as_raw(self), substitutionmethod, localename.param().abi(), ignoreuseroverride.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(windows_core::Interface::as_raw(self), glyphrun, pixelsperdip, core::mem::transmute(transform.unwrap_or(std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory {}
unsafe impl Sync for IDWriteFactory {}
#[repr(C)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateCustomFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterFontCollectionLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterFontCollectionLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFileReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::FILETIME, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCustomFontFileReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FACE_TYPE, u32, *const *mut core::ffi::c_void, u32, DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateMonitorRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, super::Gdi::HMONITOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateMonitorRenderingParams: usize,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STYLE, DWRITE_FONT_STRETCH, f32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTypography: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGdiInterop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGdiCompatibleTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, f32, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextAnalyzer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_NUMBER_SUBSTITUTION_METHOD, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_GLYPH_RUN, f32, *const DWRITE_MATRIX, DWRITE_RENDERING_MODE, DWRITE_MEASURING_MODE, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory1, IDWriteFactory1_Vtbl, 0x30572f99_dac6_41db_a16e_0486307e606a);
impl core::ops::Deref for IDWriteFactory1 {
    type Target = IDWriteFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory1, windows_core::IUnknown, IDWriteFactory);
impl IDWriteFactory1 {
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut Option<IDWriteFontCollection>, checkforupdates: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetEudcFontCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(fontcollection), checkforupdates.param().abi()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory1 {}
unsafe impl Sync for IDWriteFactory1 {}
#[repr(C)]
pub struct IDWriteFactory1_Vtbl {
    pub base__: IDWriteFactory_Vtbl,
    pub GetEudcFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory2, IDWriteFactory2_Vtbl, 0x0439fc60_ca44_4994_8dee_3a9af7b732ec);
impl core::ops::Deref for IDWriteFactory2 {
    type Target = IDWriteFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory2, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1);
impl IDWriteFactory2 {
    pub unsafe fn GetSystemFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFallbackBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, glyphrun, core::mem::transmute(glyphrundescription.unwrap_or(std::ptr::null())), measuringmode, core::mem::transmute(worldtodevicetransform.unwrap_or(std::ptr::null())), colorpaletteindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(windows_core::Interface::as_raw(self), glyphrun, core::mem::transmute(transform.unwrap_or(std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory2 {}
unsafe impl Sync for IDWriteFactory2 {}
#[repr(C)]
pub struct IDWriteFactory2_Vtbl {
    pub base__: IDWriteFactory1_Vtbl,
    pub GetSystemFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFallbackBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TranslateColorGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_GLYPH_RUN, *const DWRITE_GLYPH_RUN_DESCRIPTION, DWRITE_MEASURING_MODE, *const DWRITE_MATRIX, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE, DWRITE_GRID_FIT_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_GLYPH_RUN, *const DWRITE_MATRIX, DWRITE_RENDERING_MODE, DWRITE_MEASURING_MODE, DWRITE_GRID_FIT_MODE, DWRITE_TEXT_ANTIALIAS_MODE, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory3, IDWriteFactory3_Vtbl, 0x9a1b41c3_d3bb_466a_87fc_fe67556a3b65);
impl core::ops::Deref for IDWriteFactory3 {
    type Target = IDWriteFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory3, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2);
impl IDWriteFactory3 {
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(windows_core::Interface::as_raw(self), glyphrun, core::mem::transmute(transform.unwrap_or(std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>
    where
        P0: windows_core::Param<IDWriteFontFile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFaceReference)(windows_core::Interface::as_raw(self), fontfile.param().abi(), faceindex, fontsimulations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFaceReference2)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(lastwritetime.unwrap_or(std::ptr::null())), faceindex, fontsimulations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSystemFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontSetBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> windows_core::Result<IDWriteFontCollection1>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontCollectionFromFontSet)(windows_core::Interface::as_raw(self), fontset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSystemFontCollection<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut Option<IDWriteFontCollection1>, checkforupdates: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), includedownloadablefonts.param().abi(), core::mem::transmute(fontcollection), checkforupdates.param().abi()).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> windows_core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontDownloadQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory3 {}
unsafe impl Sync for IDWriteFactory3 {}
#[repr(C)]
pub struct IDWriteFactory3_Vtbl {
    pub base__: IDWriteFactory2_Vtbl,
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_GLYPH_RUN, *const DWRITE_MATRIX, DWRITE_RENDERING_MODE1, DWRITE_MEASURING_MODE, DWRITE_GRID_FIT_MODE, DWRITE_TEXT_ANTIALIAS_MODE, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE1, DWRITE_GRID_FIT_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFaceReference2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::FILETIME, u32, DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontSetBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFontDownloadQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory4, IDWriteFactory4_Vtbl, 0x4b0b5bd3_0797_4549_8ac5_fe915cc53856);
impl core::ops::Deref for IDWriteFactory4 {
    type Target = IDWriteFactory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory4, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3);
impl IDWriteFactory4 {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, core::mem::transmute(glyphrundescription.unwrap_or(std::ptr::null())), desiredglyphimageformats, measuringmode, core::mem::transmute(worldanddpitransform.unwrap_or(std::ptr::null())), colorpaletteindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputeGlyphOrigins)(windows_core::Interface::as_raw(self), glyphrun, core::mem::transmute(baselineorigin), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: Option<*const DWRITE_MATRIX>) -> windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputeGlyphOrigins2)(windows_core::Interface::as_raw(self), glyphrun, measuringmode, core::mem::transmute(baselineorigin), core::mem::transmute(worldanddpitransform.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteFactory4 {}
unsafe impl Sync for IDWriteFactory4 {}
#[repr(C)]
pub struct IDWriteFactory4_Vtbl {
    pub base__: IDWriteFactory3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D_POINT_2F, *const DWRITE_GLYPH_RUN, *const DWRITE_GLYPH_RUN_DESCRIPTION, DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_MEASURING_MODE, *const DWRITE_MATRIX, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    TranslateColorGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub ComputeGlyphOrigins: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_GLYPH_RUN, super::Direct2D::Common::D2D_POINT_2F, *mut super::Direct2D::Common::D2D_POINT_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    ComputeGlyphOrigins: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub ComputeGlyphOrigins2: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_GLYPH_RUN, DWRITE_MEASURING_MODE, super::Direct2D::Common::D2D_POINT_2F, *const DWRITE_MATRIX, *mut super::Direct2D::Common::D2D_POINT_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    ComputeGlyphOrigins2: usize,
}
windows_core::imp::define_interface!(IDWriteFactory5, IDWriteFactory5_Vtbl, 0x958db99a_be2a_4f09_af7d_65189803d1d3);
impl core::ops::Deref for IDWriteFactory5 {
    type Target = IDWriteFactory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory5, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4);
impl IDWriteFactory5 {
    pub unsafe fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontSetBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> windows_core::Result<IDWriteInMemoryFontFileLoader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInMemoryFontFileLoader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> windows_core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateHttpFontFileLoader)(windows_core::Interface::as_raw(self), referrerurl.param().abi(), extraheaders.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        (windows_core::Interface::vtable(self).AnalyzeContainerType)(windows_core::Interface::as_raw(self), filedata, filedatasize)
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const core::ffi::c_void, filedatasize: u32) -> windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UnpackFontFile)(windows_core::Interface::as_raw(self), containertype, filedata, filedatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory5 {}
unsafe impl Sync for IDWriteFactory5 {}
#[repr(C)]
pub struct IDWriteFactory5_Vtbl {
    pub base__: IDWriteFactory4_Vtbl,
    pub CreateFontSetBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInMemoryFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateHttpFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeContainerType: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> DWRITE_CONTAINER_TYPE,
    pub UnpackFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_CONTAINER_TYPE, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory6, IDWriteFactory6_Vtbl, 0xf3744d80_21f7_42eb_b35d_995bc72fc223);
impl core::ops::Deref for IDWriteFactory6 {
    type Target = IDWriteFactory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory6, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5);
impl IDWriteFactory6 {
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontFaceReference1>
    where
        P0: windows_core::Param<IDWriteFontFile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFaceReference)(windows_core::Interface::as_raw(self), fontfile.param().abi(), faceindex, fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontResource<P0>(&self, fontfile: P0, faceindex: u32) -> windows_core::Result<IDWriteFontResource>
    where
        P0: windows_core::Param<IDWriteFontFile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontResource)(windows_core::Interface::as_raw(self), fontfile.param().abi(), faceindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSystemFontSet<P0>(&self, includedownloadablefonts: P0) -> windows_core::Result<IDWriteFontSet1>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemFontSet)(windows_core::Interface::as_raw(self), includedownloadablefonts.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSystemFontCollection<P0>(&self, includedownloadablefonts: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), includedownloadablefonts.param().abi(), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontCollectionFromFontSet)(windows_core::Interface::as_raw(self), fontset.param().abi(), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontSetBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontsize: f32, localename: P2) -> windows_core::Result<IDWriteTextFormat3>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTextFormat)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), fontcollection.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), fontsize, localename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory6 {}
unsafe impl Sync for IDWriteFactory6 {}
#[repr(C)]
pub struct IDWriteFactory6_Vtbl {
    pub base__: IDWriteFactory5_Vtbl,
    pub CreateFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontSetBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, f32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory7, IDWriteFactory7_Vtbl, 0x35d0e0b3_9076_4d2e_a016_a91b568a06b4);
impl core::ops::Deref for IDWriteFactory7 {
    type Target = IDWriteFactory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory7, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5, IDWriteFactory6);
impl IDWriteFactory7 {
    pub unsafe fn GetSystemFontSet<P0>(&self, includedownloadablefonts: P0) -> windows_core::Result<IDWriteFontSet2>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemFontSet)(windows_core::Interface::as_raw(self), includedownloadablefonts.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSystemFontCollection<P0>(&self, includedownloadablefonts: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection3>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), includedownloadablefonts.param().abi(), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory7 {}
unsafe impl Sync for IDWriteFactory7 {}
#[repr(C)]
pub struct IDWriteFactory7_Vtbl {
    pub base__: IDWriteFactory6_Vtbl,
    pub GetSystemFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFactory8, IDWriteFactory8_Vtbl, 0xee0a7fb5_def4_4c23_a454_c9c7dc878398);
impl core::ops::Deref for IDWriteFactory8 {
    type Target = IDWriteFactory7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory8, windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5, IDWriteFactory6, IDWriteFactory7);
impl IDWriteFactory8 {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), glyphrun, core::mem::transmute(glyphrundescription.unwrap_or(std::ptr::null())), desiredglyphimageformats, paintfeaturelevel, measuringmode, core::mem::transmute(worldanddpitransform.unwrap_or(std::ptr::null())), colorpaletteindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFactory8 {}
unsafe impl Sync for IDWriteFactory8 {}
#[repr(C)]
pub struct IDWriteFactory8_Vtbl {
    pub base__: IDWriteFactory7_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::Direct2D::Common::D2D_POINT_2F, *const DWRITE_GLYPH_RUN, *const DWRITE_GLYPH_RUN_DESCRIPTION, DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_PAINT_FEATURE_LEVEL, DWRITE_MEASURING_MODE, *const DWRITE_MATRIX, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    TranslateColorGlyphRun: usize,
}
windows_core::imp::define_interface!(IDWriteFont, IDWriteFont_Vtbl, 0xacd16696_8c14_4f5d_877e_fe3fc1d32737);
impl core::ops::Deref for IDWriteFont {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFont, windows_core::IUnknown);
impl IDWriteFont {
    pub unsafe fn GetFontFamily(&self) -> windows_core::Result<IDWriteFontFamily> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (windows_core::Interface::vtable(self).GetStretch)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsSymbolFont)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFaceNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFaceNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInformationalStrings)(windows_core::Interface::as_raw(self), informationalstringid, core::mem::transmute(informationalstrings), exists).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (windows_core::Interface::vtable(self).GetSimulations)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontmetrics)
    }
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasCharacter)(windows_core::Interface::as_raw(self), unicodevalue, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFont {}
unsafe impl Sync for IDWriteFont {}
#[repr(C)]
pub struct IDWriteFont_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub IsSymbolFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetFaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInformationalStrings: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_INFORMATIONAL_STRING_ID, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSimulations: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS),
    pub HasCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFont1, IDWriteFont1_Vtbl, 0xacd16696_8c14_4f5d_877e_fe3fc1d32738);
impl core::ops::Deref for IDWriteFont1 {
    type Target = IDWriteFont;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFont1, windows_core::IUnknown, IDWriteFont);
impl IDWriteFont1 {
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontmetrics)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPanose)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUnicodeRanges)(windows_core::Interface::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(unicoderanges.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsMonospacedFont)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFont1 {}
unsafe impl Sync for IDWriteFont1 {}
#[repr(C)]
pub struct IDWriteFont1_Vtbl {
    pub base__: IDWriteFont_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS1),
    pub GetPanose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PANOSE),
    pub GetUnicodeRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_UNICODE_RANGE, *mut u32) -> windows_core::HRESULT,
    pub IsMonospacedFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IDWriteFont2, IDWriteFont2_Vtbl, 0x29748ed6_8c9c_4a6a_be0b_d912e8538944);
impl core::ops::Deref for IDWriteFont2 {
    type Target = IDWriteFont1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFont2, windows_core::IUnknown, IDWriteFont, IDWriteFont1);
impl IDWriteFont2 {
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsColorFont)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFont2 {}
unsafe impl Sync for IDWriteFont2 {}
#[repr(C)]
pub struct IDWriteFont2_Vtbl {
    pub base__: IDWriteFont1_Vtbl,
    pub IsColorFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IDWriteFont3, IDWriteFont3_Vtbl, 0x29748ed6_8c9c_4a6a_be0b_d912e8538944);
impl core::ops::Deref for IDWriteFont3 {
    type Target = IDWriteFont2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFont3, windows_core::IUnknown, IDWriteFont, IDWriteFont1, IDWriteFont2);
impl IDWriteFont3 {
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Equals<P0>(&self, font: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<IDWriteFont>,
    {
        (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), font.param().abi())
    }
    pub unsafe fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasCharacter)(windows_core::Interface::as_raw(self), unicodevalue)
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        (windows_core::Interface::vtable(self).GetLocality)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFont3 {}
unsafe impl Sync for IDWriteFont3 {}
#[repr(C)]
pub struct IDWriteFont3_Vtbl {
    pub base__: IDWriteFont2_Vtbl,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::super::Foundation::BOOL,
    pub GetLocality: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_LOCALITY,
}
windows_core::imp::define_interface!(IDWriteFontCollection, IDWriteFontCollection_Vtbl, 0xa84cee02_3eea_4eee_a827_87c1a02a0fcc);
impl core::ops::Deref for IDWriteFontCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
impl IDWriteFontCollection {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontFamilyCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindFamilyName)(windows_core::Interface::as_raw(self), familyname.param().abi(), index, exists).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> windows_core::Result<IDWriteFont>
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFromFontFace)(windows_core::Interface::as_raw(self), fontface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontCollection {}
unsafe impl Sync for IDWriteFontCollection {}
#[repr(C)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFamilyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFontFromFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontCollection1, IDWriteFontCollection1_Vtbl, 0x53585141_d9f8_4095_8321_d73cf6bd116c);
impl core::ops::Deref for IDWriteFontCollection1 {
    type Target = IDWriteFontCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontCollection1, windows_core::IUnknown, IDWriteFontCollection);
impl IDWriteFontCollection1 {
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontCollection1 {}
unsafe impl Sync for IDWriteFontCollection1 {}
#[repr(C)]
pub struct IDWriteFontCollection1_Vtbl {
    pub base__: IDWriteFontCollection_Vtbl,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontCollection2, IDWriteFontCollection2_Vtbl, 0x514039c6_4617_4064_bf8b_92ea83e506e0);
impl core::ops::Deref for IDWriteFontCollection2 {
    type Target = IDWriteFontCollection1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontCollection2, windows_core::IUnknown, IDWriteFontCollection, IDWriteFontCollection1);
impl IDWriteFontCollection2 {
    pub unsafe fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontList2>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), familyname.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL {
        (windows_core::Interface::vtable(self).GetFontFamilyModel)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontCollection2 {}
unsafe impl Sync for IDWriteFontCollection2 {}
#[repr(C)]
pub struct IDWriteFontCollection2_Vtbl {
    pub base__: IDWriteFontCollection1_Vtbl,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamilyModel: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontCollection3, IDWriteFontCollection3_Vtbl, 0xa4d055a6_f9e3_4e25_93b7_9e309f3af8e9);
impl core::ops::Deref for IDWriteFontCollection3 {
    type Target = IDWriteFontCollection2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontCollection3, windows_core::IUnknown, IDWriteFontCollection, IDWriteFontCollection1, IDWriteFontCollection2);
impl IDWriteFontCollection3 {
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).GetExpirationEvent)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFontCollection3 {}
unsafe impl Sync for IDWriteFontCollection3 {}
#[repr(C)]
pub struct IDWriteFontCollection3_Vtbl {
    pub base__: IDWriteFontCollection2_Vtbl,
    pub GetExpirationEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
windows_core::imp::define_interface!(IDWriteFontCollectionLoader, IDWriteFontCollectionLoader_Vtbl, 0xcca920e4_52f0_492b_bfa8_29c72ee0a468);
impl core::ops::Deref for IDWriteFontCollectionLoader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontCollectionLoader, windows_core::IUnknown);
impl IDWriteFontCollectionLoader {
    pub unsafe fn CreateEnumeratorFromKey<P0>(&self, factory: P0, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontFileEnumerator>
    where
        P0: windows_core::Param<IDWriteFactory>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEnumeratorFromKey)(windows_core::Interface::as_raw(self), factory.param().abi(), collectionkey, collectionkeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontCollectionLoader {}
unsafe impl Sync for IDWriteFontCollectionLoader {}
#[repr(C)]
pub struct IDWriteFontCollectionLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateEnumeratorFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontDownloadListener, IDWriteFontDownloadListener_Vtbl, 0xb06fe5b9_43ec_4393_881b_dbe4dc72fda7);
impl core::ops::Deref for IDWriteFontDownloadListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontDownloadListener, windows_core::IUnknown);
impl IDWriteFontDownloadListener {
    pub unsafe fn DownloadCompleted<P0, P1>(&self, downloadqueue: P0, context: P1, downloadresult: windows_core::HRESULT)
    where
        P0: windows_core::Param<IDWriteFontDownloadQueue>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DownloadCompleted)(windows_core::Interface::as_raw(self), downloadqueue.param().abi(), context.param().abi(), downloadresult)
    }
}
unsafe impl Send for IDWriteFontDownloadListener {}
unsafe impl Sync for IDWriteFontDownloadListener {}
#[repr(C)]
pub struct IDWriteFontDownloadListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DownloadCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
}
windows_core::imp::define_interface!(IDWriteFontDownloadQueue, IDWriteFontDownloadQueue_Vtbl, 0xb71e6052_5aea_4fa3_832e_f60d431f7e91);
impl core::ops::Deref for IDWriteFontDownloadQueue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontDownloadQueue, windows_core::IUnknown);
impl IDWriteFontDownloadQueue {
    pub unsafe fn AddListener<P0>(&self, listener: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDWriteFontDownloadListener>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddListener)(windows_core::Interface::as_raw(self), listener.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveListener(&self, token: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveListener)(windows_core::Interface::as_raw(self), token).ok()
    }
    pub unsafe fn IsEmpty(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsEmpty)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn BeginDownload<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), context.param().abi()).ok()
    }
    pub unsafe fn CancelDownload(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelDownload)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetGenerationCount(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetGenerationCount)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFontDownloadQueue {}
unsafe impl Sync for IDWriteFontDownloadQueue {}
#[repr(C)]
pub struct IDWriteFontDownloadQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveListener: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelDownload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGenerationCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
}
windows_core::imp::define_interface!(IDWriteFontFace, IDWriteFontFace_Vtbl, 0x5f49804d_7024_4d43_bfa9_d25984f53849);
impl core::ops::Deref for IDWriteFontFace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace, windows_core::IUnknown);
impl IDWriteFontFace {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: Option<*mut Option<IDWriteFontFile>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFiles)(windows_core::Interface::as_raw(self), numberoffiles, core::mem::transmute(fontfiles.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (windows_core::Interface::vtable(self).GetSimulations)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsSymbolFont)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (windows_core::Interface::vtable(self).GetGlyphCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetDesignGlyphMetrics)(windows_core::Interface::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.param().abi()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGlyphIndices)(windows_core::Interface::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TryGetFontTable)(windows_core::Interface::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const core::ffi::c_void) {
        (windows_core::Interface::vtable(self).ReleaseFontTable)(windows_core::Interface::as_raw(self), tablecontext)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: Option<*const f32>, glyphoffsets: Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        (windows_core::Interface::vtable(self).GetGlyphRunOutline)(windows_core::Interface::as_raw(self), emsize, glyphindices, core::mem::transmute(glyphadvances.unwrap_or(std::ptr::null())), core::mem::transmute(glyphoffsets.unwrap_or(std::ptr::null())), glyphcount, issideways.param().abi(), isrighttoleft.param().abi(), geometrysink.param().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGdiCompatibleMetrics)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, core::mem::transmute(transform.unwrap_or(std::ptr::null())), fontfacemetrics).ok()
    }
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetGdiCompatibleGlyphMetrics)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, core::mem::transmute(transform.unwrap_or(std::ptr::null())), usegdinatural.param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteFontFace {}
unsafe impl Sync for IDWriteFontFace {}
#[repr(C)]
pub struct IDWriteFontFace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE,
    pub GetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSimulations: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub IsSymbolFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS),
    pub GetGlyphCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub GetDesignGlyphMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut DWRITE_GLYPH_METRICS, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32, *mut u16) -> windows_core::HRESULT,
    pub TryGetFontTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ReleaseFontTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGlyphRunOutline: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const u16, *const f32, *const DWRITE_GLYPH_OFFSET, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGlyphRunOutline: usize,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, DWRITE_MEASURING_MODE, *mut core::ffi::c_void, *mut DWRITE_RENDERING_MODE) -> windows_core::HRESULT,
    pub GetGdiCompatibleMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, *mut DWRITE_FONT_METRICS) -> windows_core::HRESULT,
    pub GetGdiCompatibleGlyphMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, *const u16, u32, *mut DWRITE_GLYPH_METRICS, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFace1, IDWriteFontFace1_Vtbl, 0xa71efdb4_9fdb_4838_ad90_cfc3be8c3daf);
impl core::ops::Deref for IDWriteFontFace1 {
    type Target = IDWriteFontFace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace1, windows_core::IUnknown, IDWriteFontFace);
impl IDWriteFontFace1 {
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontmetrics)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGdiCompatibleMetrics)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, core::mem::transmute(transform.unwrap_or(std::ptr::null())), fontmetrics).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCaretMetrics)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUnicodeRanges)(windows_core::Interface::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(unicoderanges.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsMonospacedFont)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetDesignGlyphAdvances)(windows_core::Interface::as_raw(self), glyphcount, glyphindices, glyphadvances, issideways.param().abi()).ok()
    }
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetGdiCompatibleGlyphAdvances)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, core::mem::transmute(transform.unwrap_or(std::ptr::null())), usegdinatural.param().abi(), issideways.param().abi(), glyphcount, glyphindices, glyphadvances).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetKerningPairAdjustments)(windows_core::Interface::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments).ok()
    }
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasKerningPairs)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), fontemsize, dpix, dpiy, core::mem::transmute(transform.unwrap_or(std::ptr::null())), issideways.param().abi(), outlinethreshold, measuringmode, &mut result__).map(|| result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVerticalGlyphVariants)(windows_core::Interface::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices).ok()
    }
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasVerticalGlyphVariants)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFontFace1 {}
unsafe impl Sync for IDWriteFontFace1 {}
#[repr(C)]
pub struct IDWriteFontFace1_Vtbl {
    pub base__: IDWriteFontFace_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS1),
    pub GetGdiCompatibleMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, *mut DWRITE_FONT_METRICS1) -> windows_core::HRESULT,
    pub GetCaretMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_CARET_METRICS),
    pub GetUnicodeRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_UNICODE_RANGE, *mut u32) -> windows_core::HRESULT,
    pub IsMonospacedFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetDesignGlyphAdvances: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut i32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetGdiCompatibleGlyphAdvances: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, super::super::Foundation::BOOL, u32, *const u16, *mut i32) -> windows_core::HRESULT,
    pub GetKerningPairAdjustments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut i32) -> windows_core::HRESULT,
    pub HasKerningPairs: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, DWRITE_OUTLINE_THRESHOLD, DWRITE_MEASURING_MODE, *mut DWRITE_RENDERING_MODE) -> windows_core::HRESULT,
    pub GetVerticalGlyphVariants: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut u16) -> windows_core::HRESULT,
    pub HasVerticalGlyphVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IDWriteFontFace2, IDWriteFontFace2_Vtbl, 0xd8b768ff_64bc_4e66_982b_ec8e87f693f7);
impl core::ops::Deref for IDWriteFontFace2 {
    type Target = IDWriteFontFace1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace2, windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1);
impl IDWriteFontFace2 {
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsColorFont)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetColorPaletteCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetPaletteEntryCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPaletteEntries)(windows_core::Interface::as_raw(self), colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), core::mem::transmute(paletteentries.as_ptr())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<IDWriteRenderingParams>,
    {
        (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), fontemsize, dpix, dpiy, core::mem::transmute(transform.unwrap_or(std::ptr::null())), issideways.param().abi(), outlinethreshold, measuringmode, renderingparams.param().abi(), renderingmode, gridfitmode).ok()
    }
}
unsafe impl Send for IDWriteFontFace2 {}
unsafe impl Sync for IDWriteFontFace2 {}
#[repr(C)]
pub struct IDWriteFontFace2_Vtbl {
    pub base__: IDWriteFontFace1_Vtbl,
    pub IsColorFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetColorPaletteCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPaletteEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut DWRITE_COLOR_F) -> windows_core::HRESULT,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, DWRITE_OUTLINE_THRESHOLD, DWRITE_MEASURING_MODE, *mut core::ffi::c_void, *mut DWRITE_RENDERING_MODE, *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFace3, IDWriteFontFace3_Vtbl, 0xd37d7598_09be_4222_a236_2081341cc1f2);
impl core::ops::Deref for IDWriteFontFace3 {
    type Target = IDWriteFontFace2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace3, windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2);
impl IDWriteFontFace3 {
    pub unsafe fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPanose)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (windows_core::Interface::vtable(self).GetStretch)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFamilyNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFaceNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFaceNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInformationalStrings)(windows_core::Interface::as_raw(self), informationalstringid, core::mem::transmute(informationalstrings), exists).ok()
    }
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasCharacter)(windows_core::Interface::as_raw(self), unicodevalue)
    }
    pub unsafe fn GetRecommendedRenderingMode<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<IDWriteRenderingParams>,
    {
        (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), fontemsize, dpix, dpiy, core::mem::transmute(transform.unwrap_or(std::ptr::null())), issideways.param().abi(), outlinethreshold, measuringmode, renderingparams.param().abi(), renderingmode, gridfitmode).ok()
    }
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsCharacterLocal)(windows_core::Interface::as_raw(self), unicodevalue)
    }
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsGlyphLocal)(windows_core::Interface::as_raw(self), glyphid)
    }
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AreCharactersLocal)(windows_core::Interface::as_raw(self), core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap(), enqueueifnotlocal.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AreGlyphsLocal)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap(), enqueueifnotlocal.param().abi(), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteFontFace3 {}
unsafe impl Sync for IDWriteFontFace3 {}
#[repr(C)]
pub struct IDWriteFontFace3_Vtbl {
    pub base__: IDWriteFontFace2_Vtbl,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPanose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PANOSE),
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInformationalStrings: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_INFORMATIONAL_STRING_ID, *mut *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub HasCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::super::Foundation::BOOL,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, DWRITE_OUTLINE_THRESHOLD, DWRITE_MEASURING_MODE, *mut core::ffi::c_void, *mut DWRITE_RENDERING_MODE1, *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT,
    pub IsCharacterLocal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::super::Foundation::BOOL,
    pub IsGlyphLocal: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> super::super::Foundation::BOOL,
    pub AreCharactersLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AreGlyphsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFace4, IDWriteFontFace4_Vtbl, 0x27f2a904_4eb8_441d_9678_0563f53e3e2f);
impl core::ops::Deref for IDWriteFontFace4 {
    type Target = IDWriteFontFace3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace4, windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3);
impl IDWriteFontFace4 {
    pub unsafe fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> windows_core::Result<DWRITE_GLYPH_IMAGE_FORMATS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlyphImageFormats)(windows_core::Interface::as_raw(self), glyphid, pixelsperemfirst, pixelsperemlast, &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS {
        (windows_core::Interface::vtable(self).GetGlyphImageFormats2)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGlyphImageData)(windows_core::Interface::as_raw(self), glyphid, pixelsperem, glyphimageformat, glyphdata, glyphdatacontext).ok()
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut core::ffi::c_void) {
        (windows_core::Interface::vtable(self).ReleaseGlyphImageData)(windows_core::Interface::as_raw(self), glyphdatacontext)
    }
}
unsafe impl Send for IDWriteFontFace4 {}
unsafe impl Sync for IDWriteFontFace4 {}
#[repr(C)]
pub struct IDWriteFontFace4_Vtbl {
    pub base__: IDWriteFontFace3_Vtbl,
    pub GetGlyphImageFormats: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, u32, *mut DWRITE_GLYPH_IMAGE_FORMATS) -> windows_core::HRESULT,
    pub GetGlyphImageFormats2: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGlyphImageData: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, DWRITE_GLYPH_IMAGE_FORMATS, *mut DWRITE_GLYPH_IMAGE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGlyphImageData: usize,
    pub ReleaseGlyphImageData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IDWriteFontFace5, IDWriteFontFace5_Vtbl, 0x98eff3a5_b667_479a_b145_e2fa5b9fdc29);
impl core::ops::Deref for IDWriteFontFace5 {
    type Target = IDWriteFontFace4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace5, windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4);
impl IDWriteFontFace5 {
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn HasVariations(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasVariations)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontResource(&self) -> windows_core::Result<IDWriteFontResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Equals<P0>(&self, fontface: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), fontface.param().abi())
    }
}
unsafe impl Send for IDWriteFontFace5 {}
unsafe impl Sync for IDWriteFontFace5 {}
#[repr(C)]
pub struct IDWriteFontFace5_Vtbl {
    pub base__: IDWriteFontFace4_Vtbl,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub HasVariations: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IDWriteFontFace6, IDWriteFontFace6_Vtbl, 0xc4b1fe1b_6e84_47d5_b54c_a597981b06ad);
impl core::ops::Deref for IDWriteFontFace6 {
    type Target = IDWriteFontFace5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace6, windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4, IDWriteFontFace5);
impl IDWriteFontFace6 {
    pub unsafe fn GetFamilyNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFamilyNames)(windows_core::Interface::as_raw(self), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFaceNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFaceNames)(windows_core::Interface::as_raw(self), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFace6 {}
unsafe impl Sync for IDWriteFontFace6 {}
#[repr(C)]
pub struct IDWriteFontFace6_Vtbl {
    pub base__: IDWriteFontFace5_Vtbl,
    pub GetFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFace7, IDWriteFontFace7_Vtbl, 0x3945b85b_bc95_40f7_b72c_8b73bfc7e13b);
impl core::ops::Deref for IDWriteFontFace7 {
    type Target = IDWriteFontFace6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace7, windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4, IDWriteFontFace5, IDWriteFontFace6);
impl IDWriteFontFace7 {
    pub unsafe fn GetPaintFeatureLevel(&self, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL {
        (windows_core::Interface::vtable(self).GetPaintFeatureLevel)(windows_core::Interface::as_raw(self), glyphimageformat)
    }
    pub unsafe fn CreatePaintReader(&self, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL) -> windows_core::Result<IDWritePaintReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePaintReader)(windows_core::Interface::as_raw(self), glyphimageformat, paintfeaturelevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFace7 {}
unsafe impl Sync for IDWriteFontFace7 {}
#[repr(C)]
pub struct IDWriteFontFace7_Vtbl {
    pub base__: IDWriteFontFace6_Vtbl,
    pub GetPaintFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL,
    pub CreatePaintReader: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_PAINT_FEATURE_LEVEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFaceReference, IDWriteFontFaceReference_Vtbl, 0x5e7fa7ca_dde3_424c_89f0_9fcd6fed58cd);
impl core::ops::Deref for IDWriteFontFaceReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFaceReference, windows_core::IUnknown);
impl IDWriteFontFaceReference {
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFaceWithSimulations)(windows_core::Interface::as_raw(self), fontfacesimulationflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Equals<P0>(&self, fontfacereference: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), fontfacereference.param().abi())
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontFaceIndex)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (windows_core::Interface::vtable(self).GetSimulations)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontFile(&self) -> windows_core::Result<IDWriteFontFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLocalFileSize(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetLocalFileSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFileSize(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFileTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        (windows_core::Interface::vtable(self).GetLocality)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn EnqueueFontDownloadRequest(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnqueueFontDownloadRequest)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnqueueCharacterDownloadRequest(&self, characters: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnqueueCharacterDownloadRequest)(windows_core::Interface::as_raw(self), core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap()).ok()
    }
    pub unsafe fn EnqueueGlyphDownloadRequest(&self, glyphindices: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnqueueGlyphDownloadRequest)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap()).ok()
    }
    pub unsafe fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnqueueFileFragmentDownloadRequest)(windows_core::Interface::as_raw(self), fileoffset, fragmentsize).ok()
    }
}
unsafe impl Send for IDWriteFontFaceReference {}
unsafe impl Sync for IDWriteFontFaceReference {}
#[repr(C)]
pub struct IDWriteFontFaceReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFaceWithSimulations: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub GetFontFaceIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSimulations: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub GetFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalFileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetLocality: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_LOCALITY,
    pub EnqueueFontDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnqueueCharacterDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub EnqueueGlyphDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32) -> windows_core::HRESULT,
    pub EnqueueFileFragmentDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFaceReference1, IDWriteFontFaceReference1_Vtbl, 0xc081fe77_2fd1_41ac_a5a3_34983c4ba61a);
impl core::ops::Deref for IDWriteFontFaceReference1 {
    type Target = IDWriteFontFaceReference;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFaceReference1, windows_core::IUnknown, IDWriteFontFaceReference);
impl IDWriteFontFaceReference1 {
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace5> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for IDWriteFontFaceReference1 {}
unsafe impl Sync for IDWriteFontFaceReference1 {}
#[repr(C)]
pub struct IDWriteFontFaceReference1_Vtbl {
    pub base__: IDWriteFontFaceReference_Vtbl,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFallback, IDWriteFontFallback_Vtbl, 0xefa008f9_f7a1_48bf_b05c_f224713cc0ff);
impl core::ops::Deref for IDWriteFontFallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFallback, windows_core::IUnknown);
impl IDWriteFontFallback {
    pub unsafe fn MapCharacters<P0, P1, P2>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P1, basefamilyname: P2, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut Option<IDWriteFont>, scale: *mut f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).MapCharacters)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, basefontcollection.param().abi(), basefamilyname.param().abi(), baseweight, basestyle, basestretch, mappedlength, core::mem::transmute(mappedfont), scale).ok()
    }
}
unsafe impl Send for IDWriteFontFallback {}
unsafe impl Sync for IDWriteFontFallback {}
#[repr(C)]
pub struct IDWriteFontFallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MapCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_FONT_WEIGHT, DWRITE_FONT_STYLE, DWRITE_FONT_STRETCH, *mut u32, *mut *mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFallback1, IDWriteFontFallback1_Vtbl, 0x2397599d_dd0d_4681_bd6a_f4f31eaade77);
impl core::ops::Deref for IDWriteFontFallback1 {
    type Target = IDWriteFontFallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFallback1, windows_core::IUnknown, IDWriteFontFallback);
impl IDWriteFontFallback1 {
    pub unsafe fn MapCharacters<P0, P1, P2>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P1, basefamilyname: P2, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut Option<IDWriteFontFace5>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).MapCharacters)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, basefontcollection.param().abi(), basefamilyname.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), mappedlength, scale, core::mem::transmute(mappedfontface)).ok()
    }
}
unsafe impl Send for IDWriteFontFallback1 {}
unsafe impl Sync for IDWriteFontFallback1 {}
#[repr(C)]
pub struct IDWriteFontFallback1_Vtbl {
    pub base__: IDWriteFontFallback_Vtbl,
    pub MapCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const DWRITE_FONT_AXIS_VALUE, u32, *mut u32, *mut f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFallbackBuilder, IDWriteFontFallbackBuilder_Vtbl, 0xfd882d06_8aba_4fb8_b849_8be8b73e14de);
impl core::ops::Deref for IDWriteFontFallbackBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFallbackBuilder, windows_core::IUnknown);
impl IDWriteFontFallbackBuilder {
    pub unsafe fn AddMapping<P0, P1, P2>(&self, ranges: &[DWRITE_UNICODE_RANGE], targetfamilynames: &[*const u16], fontcollection: P0, localename: P1, basefamilyname: P2, scale: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollection>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddMapping)(windows_core::Interface::as_raw(self), core::mem::transmute(ranges.as_ptr()), ranges.len().try_into().unwrap(), core::mem::transmute(targetfamilynames.as_ptr()), targetfamilynames.len().try_into().unwrap(), fontcollection.param().abi(), localename.param().abi(), basefamilyname.param().abi(), scale).ok()
    }
    pub unsafe fn AddMappings<P0>(&self, fontfallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        (windows_core::Interface::vtable(self).AddMappings)(windows_core::Interface::as_raw(self), fontfallback.param().abi()).ok()
    }
    pub unsafe fn CreateFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFallbackBuilder {}
unsafe impl Sync for IDWriteFontFallbackBuilder {}
#[repr(C)]
pub struct IDWriteFontFallbackBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_UNICODE_RANGE, u32, *const *const u16, u32, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, f32) -> windows_core::HRESULT,
    pub AddMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFamily, IDWriteFontFamily_Vtbl, 0xda20d8ef_812a_4c43_9802_62ec4abd7add);
impl core::ops::Deref for IDWriteFontFamily {
    type Target = IDWriteFontList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFamily, windows_core::IUnknown, IDWriteFontList);
impl IDWriteFontFamily {
    pub unsafe fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFamilyNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFont> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstMatchingFont)(windows_core::Interface::as_raw(self), weight, stretch, style, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), weight, stretch, style, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFamily {}
unsafe impl Sync for IDWriteFontFamily {}
#[repr(C)]
pub struct IDWriteFontFamily_Vtbl {
    pub base__: IDWriteFontList_Vtbl,
    pub GetFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstMatchingFont: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFamily1, IDWriteFontFamily1_Vtbl, 0xda20d8ef_812a_4c43_9802_62ec4abd7adf);
impl core::ops::Deref for IDWriteFontFamily1 {
    type Target = IDWriteFontFamily;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFamily1, windows_core::IUnknown, IDWriteFontList, IDWriteFontFamily);
impl IDWriteFontFamily1 {
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (windows_core::Interface::vtable(self).GetFontLocality)(windows_core::Interface::as_raw(self), listindex)
    }
    pub unsafe fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFamily1 {}
unsafe impl Sync for IDWriteFontFamily1 {}
#[repr(C)]
pub struct IDWriteFontFamily1_Vtbl {
    pub base__: IDWriteFontFamily_Vtbl,
    pub GetFontLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_LOCALITY,
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFamily2, IDWriteFontFamily2_Vtbl, 0x3ed49e77_a398_4261_b9cf_c126c2131ef3);
impl core::ops::Deref for IDWriteFontFamily2 {
    type Target = IDWriteFontFamily1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFamily2, windows_core::IUnknown, IDWriteFontList, IDWriteFontFamily, IDWriteFontFamily1);
impl IDWriteFontFamily2 {
    pub unsafe fn GetMatchingFonts(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontList2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFamily2 {}
unsafe impl Sync for IDWriteFontFamily2 {}
#[repr(C)]
pub struct IDWriteFontFamily2_Vtbl {
    pub base__: IDWriteFontFamily1_Vtbl,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFile, IDWriteFontFile_Vtbl, 0x739d886a_cef5_47dc_8769_1a8b41bebbb0);
impl core::ops::Deref for IDWriteFontFile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFile, windows_core::IUnknown);
impl IDWriteFontFile {
    pub unsafe fn GetReferenceKey(&self, fontfilereferencekey: *mut *mut core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReferenceKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize).ok()
    }
    pub unsafe fn GetLoader(&self) -> windows_core::Result<IDWriteFontFileLoader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLoader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Analyze(&self, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: Option<*mut DWRITE_FONT_FACE_TYPE>, numberoffaces: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Analyze)(windows_core::Interface::as_raw(self), issupportedfonttype, fontfiletype, core::mem::transmute(fontfacetype.unwrap_or(std::ptr::null_mut())), numberoffaces).ok()
    }
}
unsafe impl Send for IDWriteFontFile {}
unsafe impl Sync for IDWriteFontFile {}
#[repr(C)]
pub struct IDWriteFontFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetReferenceKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Analyze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut DWRITE_FONT_FILE_TYPE, *mut DWRITE_FONT_FACE_TYPE, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFileEnumerator, IDWriteFontFileEnumerator_Vtbl, 0x72755049_5ff7_435d_8348_4be97cfa6c7c);
impl core::ops::Deref for IDWriteFontFileEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFileEnumerator, windows_core::IUnknown);
impl IDWriteFontFileEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentFontFile(&self) -> windows_core::Result<IDWriteFontFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentFontFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFileEnumerator {}
unsafe impl Sync for IDWriteFontFileEnumerator {}
#[repr(C)]
pub struct IDWriteFontFileEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrentFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFileLoader, IDWriteFontFileLoader_Vtbl, 0x727cad4e_d6af_4c9e_8a08_d695b11caa49);
impl core::ops::Deref for IDWriteFontFileLoader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFileLoader, windows_core::IUnknown);
impl IDWriteFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStreamFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontFileLoader {}
unsafe impl Sync for IDWriteFontFileLoader {}
#[repr(C)]
pub struct IDWriteFontFileLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStreamFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontFileStream, IDWriteFontFileStream_Vtbl, 0x6d4865fe_0ab8_4d91_8f62_5dd6be34a3e0);
impl core::ops::Deref for IDWriteFontFileStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFileStream, windows_core::IUnknown);
impl IDWriteFontFileStream {
    pub unsafe fn ReadFileFragment(&self, fragmentstart: *mut *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadFileFragment)(windows_core::Interface::as_raw(self), fragmentstart, fileoffset, fragmentsize, fragmentcontext).ok()
    }
    pub unsafe fn ReleaseFileFragment(&self, fragmentcontext: *mut core::ffi::c_void) {
        (windows_core::Interface::vtable(self).ReleaseFileFragment)(windows_core::Interface::as_raw(self), fragmentcontext)
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastWriteTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastWriteTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteFontFileStream {}
unsafe impl Sync for IDWriteFontFileStream {}
#[repr(C)]
pub struct IDWriteFontFileStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadFileFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u64, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseFileFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetLastWriteTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontList, IDWriteFontList_Vtbl, 0x1a0d8438_1d97_4ec1_aef9_a2fb86ed6acb);
impl core::ops::Deref for IDWriteFontList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontList, windows_core::IUnknown);
impl IDWriteFontList {
    pub unsafe fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFont(&self, index: u32) -> windows_core::Result<IDWriteFont> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontList {}
unsafe impl Sync for IDWriteFontList {}
#[repr(C)]
pub struct IDWriteFontList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontList1, IDWriteFontList1_Vtbl, 0xda20d8ef_812a_4c43_9802_62ec4abd7ade);
impl core::ops::Deref for IDWriteFontList1 {
    type Target = IDWriteFontList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontList1, windows_core::IUnknown, IDWriteFontList);
impl IDWriteFontList1 {
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (windows_core::Interface::vtable(self).GetFontLocality)(windows_core::Interface::as_raw(self), listindex)
    }
    pub unsafe fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontList1 {}
unsafe impl Sync for IDWriteFontList1 {}
#[repr(C)]
pub struct IDWriteFontList1_Vtbl {
    pub base__: IDWriteFontList_Vtbl,
    pub GetFontLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_LOCALITY,
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontList2, IDWriteFontList2_Vtbl, 0xc0763a34_77af_445a_b735_08c37b0a5bf5);
impl core::ops::Deref for IDWriteFontList2 {
    type Target = IDWriteFontList1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontList2, windows_core::IUnknown, IDWriteFontList, IDWriteFontList1);
impl IDWriteFontList2 {
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontList2 {}
unsafe impl Sync for IDWriteFontList2 {}
#[repr(C)]
pub struct IDWriteFontList2_Vtbl {
    pub base__: IDWriteFontList1_Vtbl,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontResource, IDWriteFontResource_Vtbl, 0x1f803a76_6871_48e8_987f_b975551c50f2);
impl core::ops::Deref for IDWriteFontResource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontResource, windows_core::IUnknown);
impl IDWriteFontResource {
    pub unsafe fn GetFontFile(&self) -> windows_core::Result<IDWriteFontFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontFaceIndex)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontAxisCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontAxisCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDefaultFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisRanges)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFontAxisAttributes(&self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
        (windows_core::Interface::vtable(self).GetFontAxisAttributes)(windows_core::Interface::as_raw(self), axisindex)
    }
    pub unsafe fn GetAxisNames(&self, axisindex: u32) -> windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAxisNames)(windows_core::Interface::as_raw(self), axisindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAxisValueNameCount(&self, axisindex: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetAxisValueNameCount)(windows_core::Interface::as_raw(self), axisindex)
    }
    pub unsafe fn GetAxisValueNames(&self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut Option<IDWriteLocalizedStrings>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAxisValueNames)(windows_core::Interface::as_raw(self), axisindex, axisvalueindex, fontaxisrange, core::mem::transmute(names)).ok()
    }
    pub unsafe fn HasVariations(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).HasVariations)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CreateFontFace(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontFace5> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFaceReference(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFaceReference)(windows_core::Interface::as_raw(self), fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontResource {}
unsafe impl Sync for IDWriteFontResource {}
#[repr(C)]
pub struct IDWriteFontResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDefaultFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub GetFontAxisRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_RANGE, u32) -> windows_core::HRESULT,
    pub GetFontAxisAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_FONT_AXIS_ATTRIBUTES,
    pub GetAxisNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAxisValueNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetAxisValueNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DWRITE_FONT_AXIS_RANGE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasVariations: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontSet, IDWriteFontSet_Vtbl, 0x53585141_d9f8_4095_8321_d73cf6bd116b);
impl core::ops::Deref for IDWriteFontSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet, windows_core::IUnknown);
impl IDWriteFontSet {
    pub unsafe fn GetFontCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        (windows_core::Interface::vtable(self).FindFontFaceReference)(windows_core::Interface::as_raw(self), fontfacereference.param().abi(), listindex, exists).ok()
    }
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        (windows_core::Interface::vtable(self).FindFontFace)(windows_core::Interface::as_raw(self), fontface.param().abi(), listindex, exists).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> windows_core::Result<IDWriteStringList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyValues)(windows_core::Interface::as_raw(self), propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> windows_core::Result<IDWriteStringList>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyValues2)(windows_core::Interface::as_raw(self), propertyid, preferredlocalenames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut Option<IDWriteLocalizedStrings>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyValues3)(windows_core::Interface::as_raw(self), listindex, propertyid, exists, core::mem::transmute(values)).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyOccurrenceCount)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontSet>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), familyname.param().abi(), fontweight, fontstretch, fontstyle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> windows_core::Result<IDWriteFontSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts2)(windows_core::Interface::as_raw(self), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontSet {}
unsafe impl Sync for IDWriteFontSet {}
#[repr(C)]
pub struct IDWriteFontSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FindFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPropertyValues: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_PROPERTY_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValues2: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_PROPERTY_ID, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValues3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DWRITE_FONT_PROPERTY_ID, *mut super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyOccurrenceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, *mut u32) -> windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_FONT_WEIGHT, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMatchingFonts2: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontSet1, IDWriteFontSet1_Vtbl, 0x7e9fda85_6c92_4053_bc47_7ae3530db4d3);
impl core::ops::Deref for IDWriteFontSet1 {
    type Target = IDWriteFontSet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet1, windows_core::IUnknown, IDWriteFontSet);
impl IDWriteFontSet1 {
    pub unsafe fn GetMatchingFonts(&self, fontproperty: Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontSet1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(fontproperty.unwrap_or(std::ptr::null())), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFirstFontResources(&self) -> windows_core::Result<IDWriteFontSet1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFirstFontResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> windows_core::Result<IDWriteFontSet1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> windows_core::Result<IDWriteFontSet1>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredFonts2)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> windows_core::Result<IDWriteFontSet1>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredFonts3)(windows_core::Interface::as_raw(self), core::mem::transmute(properties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), selectanyproperty.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetFilteredFontIndices)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.param().abi(), core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount).ok()
    }
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetFilteredFontIndices2)(windows_core::Interface::as_raw(self), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), selectanyproperty.param().abi(), core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisRanges)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisRanges2)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount).ok()
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> windows_core::Result<IDWriteFontResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontResource)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> windows_core::Result<IDWriteFontFace5> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (windows_core::Interface::vtable(self).GetFontLocality)(windows_core::Interface::as_raw(self), listindex)
    }
}
unsafe impl Send for IDWriteFontSet1 {}
unsafe impl Sync for IDWriteFontSet1 {}
#[repr(C)]
pub struct IDWriteFontSet1_Vtbl {
    pub base__: IDWriteFontSet_Vtbl,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstFontResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFonts2: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_RANGE, u32, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFonts3: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFontIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_RANGE, u32, super::super::Foundation::BOOL, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFilteredFontIndices2: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32, super::super::Foundation::BOOL, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFontAxisRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_AXIS_RANGE, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFontAxisRanges2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_RANGE, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_LOCALITY,
}
windows_core::imp::define_interface!(IDWriteFontSet2, IDWriteFontSet2_Vtbl, 0xdc7ead19_e54c_43af_b2da_4e2b79ba3f7f);
impl core::ops::Deref for IDWriteFontSet2 {
    type Target = IDWriteFontSet1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet2, windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1);
impl IDWriteFontSet2 {
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).GetExpirationEvent)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteFontSet2 {}
unsafe impl Sync for IDWriteFontSet2 {}
#[repr(C)]
pub struct IDWriteFontSet2_Vtbl {
    pub base__: IDWriteFontSet1_Vtbl,
    pub GetExpirationEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
windows_core::imp::define_interface!(IDWriteFontSet3, IDWriteFontSet3_Vtbl, 0x7c073ef2_a7f4_4045_8c32_8ab8ae640f90);
impl core::ops::Deref for IDWriteFontSet3 {
    type Target = IDWriteFontSet2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet3, windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1, IDWriteFontSet2);
impl IDWriteFontSet3 {
    pub unsafe fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
        (windows_core::Interface::vtable(self).GetFontSourceType)(windows_core::Interface::as_raw(self), fontindex)
    }
    pub unsafe fn GetFontSourceNameLength(&self, listindex: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetFontSourceNameLength)(windows_core::Interface::as_raw(self), listindex)
    }
    pub unsafe fn GetFontSourceName(&self, listindex: u32, stringbuffer: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontSourceName)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for IDWriteFontSet3 {}
unsafe impl Sync for IDWriteFontSet3 {}
#[repr(C)]
pub struct IDWriteFontSet3_Vtbl {
    pub base__: IDWriteFontSet2_Vtbl,
    pub GetFontSourceType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_FONT_SOURCE_TYPE,
    pub GetFontSourceNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetFontSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontSet4, IDWriteFontSet4_Vtbl, 0xeec175fc_bea9_4c86_8b53_ccbdd7df0c82);
impl core::ops::Deref for IDWriteFontSet4 {
    type Target = IDWriteFontSet3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet4, windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1, IDWriteFontSet2, IDWriteFontSet3);
impl IDWriteFontSet4 {
    pub unsafe fn ConvertWeightStretchStyleToFontAxisValues(&self, inputaxisvalues: Option<&[DWRITE_FONT_AXIS_VALUE]>, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE; 5]) -> u32 {
        (windows_core::Interface::vtable(self).ConvertWeightStretchStyleToFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(inputaxisvalues.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), inputaxisvalues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), fontweight, fontstretch, fontstyle, fontsize, core::mem::transmute(outputaxisvalues.as_ptr()))
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], allowedsimulations: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontSet4>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), familyname.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), allowedsimulations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontSet4 {}
unsafe impl Sync for IDWriteFontSet4 {}
#[repr(C)]
pub struct IDWriteFontSet4_Vtbl {
    pub base__: IDWriteFontSet3_Vtbl,
    pub ConvertWeightStretchStyleToFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, DWRITE_FONT_WEIGHT, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, f32, *mut DWRITE_FONT_AXIS_VALUE) -> u32,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const DWRITE_FONT_AXIS_VALUE, u32, DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontSetBuilder, IDWriteFontSetBuilder_Vtbl, 0x2f642afe_9c68_4f40_b8be_457401afcb3d);
impl core::ops::Deref for IDWriteFontSetBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder, windows_core::IUnknown);
impl IDWriteFontSetBuilder {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        (windows_core::Interface::vtable(self).AddFontFaceReference)(windows_core::Interface::as_raw(self), fontfacereference.param().abi(), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap()).ok()
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        (windows_core::Interface::vtable(self).AddFontFaceReference2)(windows_core::Interface::as_raw(self), fontfacereference.param().abi()).ok()
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        (windows_core::Interface::vtable(self).AddFontSet)(windows_core::Interface::as_raw(self), fontset.param().abi()).ok()
    }
    pub unsafe fn CreateFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteFontSetBuilder {}
unsafe impl Sync for IDWriteFontSetBuilder {}
#[repr(C)]
pub struct IDWriteFontSetBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32) -> windows_core::HRESULT,
    pub AddFontFaceReference2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontSetBuilder1, IDWriteFontSetBuilder1_Vtbl, 0x3ff7715f_3cdc_4dc6_9b72_ec5621dccafd);
impl core::ops::Deref for IDWriteFontSetBuilder1 {
    type Target = IDWriteFontSetBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder1, windows_core::IUnknown, IDWriteFontSetBuilder);
impl IDWriteFontSetBuilder1 {
    pub unsafe fn AddFontFile<P0>(&self, fontfile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFile>,
    {
        (windows_core::Interface::vtable(self).AddFontFile)(windows_core::Interface::as_raw(self), fontfile.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteFontSetBuilder1 {}
unsafe impl Sync for IDWriteFontSetBuilder1 {}
#[repr(C)]
pub struct IDWriteFontSetBuilder1_Vtbl {
    pub base__: IDWriteFontSetBuilder_Vtbl,
    pub AddFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteFontSetBuilder2, IDWriteFontSetBuilder2_Vtbl, 0xee5ba612_b131_463c_8f4f_3189b9401e45);
impl core::ops::Deref for IDWriteFontSetBuilder2 {
    type Target = IDWriteFontSetBuilder1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder2, windows_core::IUnknown, IDWriteFontSetBuilder, IDWriteFontSetBuilder1);
impl IDWriteFontSetBuilder2 {
    pub unsafe fn AddFont<P0>(&self, fontfile: P0, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], properties: &[DWRITE_FONT_PROPERTY]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFile>,
    {
        (windows_core::Interface::vtable(self).AddFont)(windows_core::Interface::as_raw(self), fontfile.param().abi(), fontfaceindex, fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap()).ok()
    }
    pub unsafe fn AddFontFile<P0>(&self, filepath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddFontFile)(windows_core::Interface::as_raw(self), filepath.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteFontSetBuilder2 {}
unsafe impl Sync for IDWriteFontSetBuilder2 {}
#[repr(C)]
pub struct IDWriteFontSetBuilder2_Vtbl {
    pub base__: IDWriteFontSetBuilder1_Vtbl,
    pub AddFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *const DWRITE_FONT_AXIS_RANGE, u32, *const DWRITE_FONT_PROPERTY, u32) -> windows_core::HRESULT,
    pub AddFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteGdiInterop, IDWriteGdiInterop_Vtbl, 0x1edd9491_9853_4299_898f_6432983b6f3a);
impl core::ops::Deref for IDWriteGdiInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteGdiInterop, windows_core::IUnknown);
impl IDWriteGdiInterop {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW) -> windows_core::Result<IDWriteFont> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFromLOGFONT)(windows_core::Interface::as_raw(self), logfont, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ConvertFontToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFont>,
    {
        (windows_core::Interface::vtable(self).ConvertFontToLOGFONT)(windows_core::Interface::as_raw(self), font.param().abi(), logfont, issystemfont).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ConvertFontFaceToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        (windows_core::Interface::vtable(self).ConvertFontFaceToLOGFONT)(windows_core::Interface::as_raw(self), font.param().abi(), logfont).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFaceFromHdc<P0>(&self, hdc: P0) -> windows_core::Result<IDWriteFontFace>
    where
        P0: windows_core::Param<super::Gdi::HDC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFaceFromHdc)(windows_core::Interface::as_raw(self), hdc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapRenderTarget<P0>(&self, hdc: P0, width: u32, height: u32) -> windows_core::Result<IDWriteBitmapRenderTarget>
    where
        P0: windows_core::Param<super::Gdi::HDC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBitmapRenderTarget)(windows_core::Interface::as_raw(self), hdc.param().abi(), width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteGdiInterop {}
unsafe impl Sync for IDWriteGdiInterop {}
#[repr(C)]
pub struct IDWriteGdiInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFromLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::LOGFONTW, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFromLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ConvertFontToLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Gdi::LOGFONTW, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ConvertFontToLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ConvertFontFaceToLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Gdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ConvertFontFaceToLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFaceFromHdc: unsafe extern "system" fn(*mut core::ffi::c_void, super::Gdi::HDC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFaceFromHdc: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateBitmapRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::Gdi::HDC, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateBitmapRenderTarget: usize,
}
windows_core::imp::define_interface!(IDWriteGdiInterop1, IDWriteGdiInterop1_Vtbl, 0x4556be70_3abd_4f70_90be_421780a6f515);
impl core::ops::Deref for IDWriteGdiInterop1 {
    type Target = IDWriteGdiInterop;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteGdiInterop1, windows_core::IUnknown, IDWriteGdiInterop);
impl IDWriteGdiInterop1 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFromLOGFONT<P0>(&self, logfont: *const super::Gdi::LOGFONTW, fontcollection: P0) -> windows_core::Result<IDWriteFont>
    where
        P0: windows_core::Param<IDWriteFontCollection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFromLOGFONT)(windows_core::Interface::as_raw(self), logfont, fontcollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetFontSignature<P0>(&self, fontface: P0, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        (windows_core::Interface::vtable(self).GetFontSignature)(windows_core::Interface::as_raw(self), fontface.param().abi(), fontsignature).ok()
    }
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetFontSignature2<P0>(&self, font: P0, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFont>,
    {
        (windows_core::Interface::vtable(self).GetFontSignature2)(windows_core::Interface::as_raw(self), font.param().abi(), fontsignature).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetMatchingFontsByLOGFONT<P0>(&self, logfont: *const super::Gdi::LOGFONTA, fontset: P0) -> windows_core::Result<IDWriteFontSet>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMatchingFontsByLOGFONT)(windows_core::Interface::as_raw(self), logfont, fontset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteGdiInterop1 {}
unsafe impl Sync for IDWriteGdiInterop1 {}
#[repr(C)]
pub struct IDWriteGdiInterop1_Vtbl {
    pub base__: IDWriteGdiInterop_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFromLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::LOGFONTW, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFromLOGFONT: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetFontSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetFontSignature: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetFontSignature2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetFontSignature2: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetMatchingFontsByLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Gdi::LOGFONTA, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetMatchingFontsByLOGFONT: usize,
}
windows_core::imp::define_interface!(IDWriteGlyphRunAnalysis, IDWriteGlyphRunAnalysis_Vtbl, 0x7d97dbf7_e085_42d4_81e3_6a883bded118);
impl core::ops::Deref for IDWriteGlyphRunAnalysis {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteGlyphRunAnalysis, windows_core::IUnknown);
impl IDWriteGlyphRunAnalysis {
    pub unsafe fn GetAlphaTextureBounds(&self, texturetype: DWRITE_TEXTURE_TYPE) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlphaTextureBounds)(windows_core::Interface::as_raw(self), texturetype, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateAlphaTexture(&self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateAlphaTexture)(windows_core::Interface::as_raw(self), texturetype, texturebounds, core::mem::transmute(alphavalues.as_ptr()), alphavalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetAlphaBlendParams<P0>(&self, renderingparams: P0, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
    {
        (windows_core::Interface::vtable(self).GetAlphaBlendParams)(windows_core::Interface::as_raw(self), renderingparams.param().abi(), blendgamma, blendenhancedcontrast, blendcleartypelevel).ok()
    }
}
unsafe impl Send for IDWriteGlyphRunAnalysis {}
unsafe impl Sync for IDWriteGlyphRunAnalysis {}
#[repr(C)]
pub struct IDWriteGlyphRunAnalysis_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAlphaTextureBounds: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXTURE_TYPE, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub CreateAlphaTexture: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXTURE_TYPE, *const super::super::Foundation::RECT, *mut u8, u32) -> windows_core::HRESULT,
    pub GetAlphaBlendParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteInMemoryFontFileLoader, IDWriteInMemoryFontFileLoader_Vtbl, 0xdc102f47_a12d_4b1c_822d_9e117e33043f);
impl core::ops::Deref for IDWriteInMemoryFontFileLoader {
    type Target = IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteInMemoryFontFileLoader, windows_core::IUnknown, IDWriteFontFileLoader);
impl IDWriteInMemoryFontFileLoader {
    pub unsafe fn CreateInMemoryFontFileReference<P0, P1>(&self, factory: P0, fontdata: *const core::ffi::c_void, fontdatasize: u32, ownerobject: P1) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<IDWriteFactory>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInMemoryFontFileReference)(windows_core::Interface::as_raw(self), factory.param().abi(), fontdata, fontdatasize, ownerobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFileCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFileCount)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteInMemoryFontFileLoader {}
unsafe impl Sync for IDWriteInMemoryFontFileLoader {}
#[repr(C)]
pub struct IDWriteInMemoryFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub CreateInMemoryFontFileReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(IDWriteInlineObject, IDWriteInlineObject_Vtbl, 0x8339fde3_106f_47ab_8373_1c6295eb10b3);
impl core::ops::Deref for IDWriteInlineObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteInlineObject, windows_core::IUnknown);
impl IDWriteInlineObject {
    pub unsafe fn Draw<P0, P1, P2, P3>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, renderer: P0, originx: f32, originy: f32, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextRenderer>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), renderer.param().abi(), originx, originy, issideways.param().abi(), isrighttoleft.param().abi(), clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn GetMetrics(&self) -> windows_core::Result<DWRITE_INLINE_OBJECT_METRICS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOverhangMetrics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBreakConditions(&self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBreakConditions)(windows_core::Interface::as_raw(self), breakconditionbefore, breakconditionafter).ok()
    }
}
unsafe impl Send for IDWriteInlineObject {}
unsafe impl Sync for IDWriteInlineObject {}
#[repr(C)]
pub struct IDWriteInlineObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, f32, f32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_INLINE_OBJECT_METRICS) -> windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT,
    pub GetBreakConditions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_BREAK_CONDITION, *mut DWRITE_BREAK_CONDITION) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteLocalFontFileLoader, IDWriteLocalFontFileLoader_Vtbl, 0xb2d9f3ec_c9fe_4a11_a2ec_d86208f7c0a2);
impl core::ops::Deref for IDWriteLocalFontFileLoader {
    type Target = IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteLocalFontFileLoader, windows_core::IUnknown, IDWriteFontFileLoader);
impl IDWriteLocalFontFileLoader {
    pub unsafe fn GetFilePathLengthFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilePathLengthFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).map(|| result__)
    }
    pub unsafe fn GetFilePathFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepath: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilePathFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetLastWriteTimeFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastWriteTimeFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteLocalFontFileLoader {}
unsafe impl Sync for IDWriteLocalFontFileLoader {}
#[repr(C)]
pub struct IDWriteLocalFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub GetFilePathLengthFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFilePathFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetLastWriteTimeFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteLocalizedStrings, IDWriteLocalizedStrings_Vtbl, 0x08256209_099a_4b34_b86d_c22b110e7771);
impl core::ops::Deref for IDWriteLocalizedStrings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteLocalizedStrings, windows_core::IUnknown);
impl IDWriteLocalizedStrings {
    pub unsafe fn GetCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn FindLocaleName<P0>(&self, localename: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindLocaleName)(windows_core::Interface::as_raw(self), localename.param().abi(), index, exists).ok()
    }
    pub unsafe fn GetLocaleNameLength(&self, index: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetLocaleName(&self, index: u32, localename: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), index, core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetStringLength(&self, index: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetString(&self, index: u32, stringbuffer: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), index, core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for IDWriteLocalizedStrings {}
unsafe impl Sync for IDWriteLocalizedStrings {}
#[repr(C)]
pub struct IDWriteLocalizedStrings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub FindLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteNumberSubstitution, IDWriteNumberSubstitution_Vtbl, 0x14885cc9_bab0_4f90_b6ed_5c366a2cd03d);
impl core::ops::Deref for IDWriteNumberSubstitution {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteNumberSubstitution, windows_core::IUnknown);
impl IDWriteNumberSubstitution {}
unsafe impl Send for IDWriteNumberSubstitution {}
unsafe impl Sync for IDWriteNumberSubstitution {}
#[repr(C)]
pub struct IDWriteNumberSubstitution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IDWritePaintReader, IDWritePaintReader_Vtbl, 0x8128e912_3b97_42a5_ab6c_24aad3a86e54);
impl core::ops::Deref for IDWritePaintReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWritePaintReader, windows_core::IUnknown);
impl IDWritePaintReader {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCurrentGlyph(&self, glyphindex: u32, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32, clipbox: *mut super::Direct2D::Common::D2D_RECT_F, glyphattributes: Option<*mut DWRITE_PAINT_ATTRIBUTES>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentGlyph)(windows_core::Interface::as_raw(self), glyphindex, paintelement, structsize, clipbox, core::mem::transmute(glyphattributes.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTextColor(&self, textcolor: *const DWRITE_COLOR_F) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextColor)(windows_core::Interface::as_raw(self), textcolor).ok()
    }
    pub unsafe fn SetColorPaletteIndex(&self, colorpaletteindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorPaletteIndex)(windows_core::Interface::as_raw(self), colorpaletteindex).ok()
    }
    pub unsafe fn SetCustomColorPalette(&self, paletteentries: &[DWRITE_COLOR_F]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCustomColorPalette)(windows_core::Interface::as_raw(self), core::mem::transmute(paletteentries.as_ptr()), paletteentries.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn MoveToFirstChild(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MoveToFirstChild)(windows_core::Interface::as_raw(self), paintelement, structsize).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn MoveToNextSibling(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MoveToNextSibling)(windows_core::Interface::as_raw(self), paintelement, structsize).ok()
    }
    pub unsafe fn MoveToParent(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MoveToParent)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientStops(&self, firstgradientstopindex: u32, gradientstops: &mut [super::Direct2D::Common::D2D1_GRADIENT_STOP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGradientStops)(windows_core::Interface::as_raw(self), firstgradientstopindex, gradientstops.len().try_into().unwrap(), core::mem::transmute(gradientstops.as_ptr())).ok()
    }
    pub unsafe fn GetGradientStopColors(&self, firstgradientstopindex: u32, gradientstopcolors: &mut [DWRITE_PAINT_COLOR]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGradientStopColors)(windows_core::Interface::as_raw(self), firstgradientstopindex, gradientstopcolors.len().try_into().unwrap(), core::mem::transmute(gradientstopcolors.as_ptr())).ok()
    }
}
unsafe impl Send for IDWritePaintReader {}
unsafe impl Sync for IDWritePaintReader {}
#[repr(C)]
pub struct IDWritePaintReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCurrentGlyph: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_PAINT_ELEMENT, u32, *mut super::Direct2D::Common::D2D_RECT_F, *mut DWRITE_PAINT_ATTRIBUTES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCurrentGlyph: usize,
    pub SetTextColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_COLOR_F) -> windows_core::HRESULT,
    pub SetColorPaletteIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetCustomColorPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_COLOR_F, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub MoveToFirstChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PAINT_ELEMENT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    MoveToFirstChild: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub MoveToNextSibling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PAINT_ELEMENT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    MoveToNextSibling: usize,
    pub MoveToParent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetGradientStops: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::Direct2D::Common::D2D1_GRADIENT_STOP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetGradientStops: usize,
    pub GetGradientStopColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DWRITE_PAINT_COLOR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWritePixelSnapping, IDWritePixelSnapping_Vtbl, 0xeaf3a2da_ecf4_4d24_b644_b34f6842024b);
impl core::ops::Deref for IDWritePixelSnapping {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWritePixelSnapping, windows_core::IUnknown);
impl IDWritePixelSnapping {
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPixelSnappingDisabled)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), transform).ok()
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPixelsPerDip)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWritePixelSnapping {}
unsafe impl Sync for IDWritePixelSnapping {}
#[repr(C)]
pub struct IDWritePixelSnapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPixelSnappingDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetPixelsPerDip: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteRemoteFontFileLoader, IDWriteRemoteFontFileLoader_Vtbl, 0x68648c83_6ede_46c0_ab46_20083a887fde);
impl core::ops::Deref for IDWriteRemoteFontFileLoader {
    type Target = IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteRemoteFontFileLoader, windows_core::IUnknown, IDWriteFontFileLoader);
impl IDWriteRemoteFontFileLoader {
    pub unsafe fn CreateRemoteStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteRemoteFontFileStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRemoteStreamFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLocalityFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<DWRITE_LOCALITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalityFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateFontFileReferenceFromUrl<P0, P1, P2>(&self, factory: P0, baseurl: P1, fontfileurl: P2) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<IDWriteFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFontFileReferenceFromUrl)(windows_core::Interface::as_raw(self), factory.param().abi(), baseurl.param().abi(), fontfileurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteRemoteFontFileLoader {}
unsafe impl Sync for IDWriteRemoteFontFileLoader {}
#[repr(C)]
pub struct IDWriteRemoteFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub CreateRemoteStreamFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalityFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut DWRITE_LOCALITY) -> windows_core::HRESULT,
    pub CreateFontFileReferenceFromUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteRemoteFontFileStream, IDWriteRemoteFontFileStream_Vtbl, 0x4db3757a_2c72_4ed9_b2b6_1ababe1aff9c);
impl core::ops::Deref for IDWriteRemoteFontFileStream {
    type Target = IDWriteFontFileStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteRemoteFontFileStream, windows_core::IUnknown, IDWriteFontFileStream);
impl IDWriteRemoteFontFileStream {
    pub unsafe fn GetLocalFileSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFileFragmentLocality(&self, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileFragmentLocality)(windows_core::Interface::as_raw(self), fileoffset, fragmentsize, islocal, partialsize).ok()
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        (windows_core::Interface::vtable(self).GetLocality)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn BeginDownload(&self, downloadoperationid: *const windows_core::GUID, filefragments: &[DWRITE_FILE_FRAGMENT]) -> windows_core::Result<IDWriteAsyncResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), downloadoperationid, core::mem::transmute(filefragments.as_ptr()), filefragments.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteRemoteFontFileStream {}
unsafe impl Sync for IDWriteRemoteFontFileStream {}
#[repr(C)]
pub struct IDWriteRemoteFontFileStream_Vtbl {
    pub base__: IDWriteFontFileStream_Vtbl,
    pub GetLocalFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetFileFragmentLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, *mut super::super::Foundation::BOOL, *mut u64) -> windows_core::HRESULT,
    pub GetLocality: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_LOCALITY,
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DWRITE_FILE_FRAGMENT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteRenderingParams, IDWriteRenderingParams_Vtbl, 0x2f0da53a_2add_47cd_82ee_d9ec34688e75);
impl core::ops::Deref for IDWriteRenderingParams {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams, windows_core::IUnknown);
impl IDWriteRenderingParams {
    pub unsafe fn GetGamma(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetGamma)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetEnhancedContrast)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetClearTypeLevel)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        (windows_core::Interface::vtable(self).GetPixelGeometry)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        (windows_core::Interface::vtable(self).GetRenderingMode)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteRenderingParams {}
unsafe impl Sync for IDWriteRenderingParams {}
#[repr(C)]
pub struct IDWriteRenderingParams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGamma: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetEnhancedContrast: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetClearTypeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetPixelGeometry: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY,
    pub GetRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_RENDERING_MODE,
}
windows_core::imp::define_interface!(IDWriteRenderingParams1, IDWriteRenderingParams1_Vtbl, 0x94413cf4_a6fc_4248_8b50_6674348fcad3);
impl core::ops::Deref for IDWriteRenderingParams1 {
    type Target = IDWriteRenderingParams;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams1, windows_core::IUnknown, IDWriteRenderingParams);
impl IDWriteRenderingParams1 {
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetGrayscaleEnhancedContrast)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteRenderingParams1 {}
unsafe impl Sync for IDWriteRenderingParams1 {}
#[repr(C)]
pub struct IDWriteRenderingParams1_Vtbl {
    pub base__: IDWriteRenderingParams_Vtbl,
    pub GetGrayscaleEnhancedContrast: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
}
windows_core::imp::define_interface!(IDWriteRenderingParams2, IDWriteRenderingParams2_Vtbl, 0xf9d711c3_9777_40ae_87e8_3e5af9bf0948);
impl core::ops::Deref for IDWriteRenderingParams2 {
    type Target = IDWriteRenderingParams1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams2, windows_core::IUnknown, IDWriteRenderingParams, IDWriteRenderingParams1);
impl IDWriteRenderingParams2 {
    pub unsafe fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE {
        (windows_core::Interface::vtable(self).GetGridFitMode)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteRenderingParams2 {}
unsafe impl Sync for IDWriteRenderingParams2 {}
#[repr(C)]
pub struct IDWriteRenderingParams2_Vtbl {
    pub base__: IDWriteRenderingParams1_Vtbl,
    pub GetGridFitMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_GRID_FIT_MODE,
}
windows_core::imp::define_interface!(IDWriteRenderingParams3, IDWriteRenderingParams3_Vtbl, 0xb7924baa_391b_412a_8c5c_e44cc2d867dc);
impl core::ops::Deref for IDWriteRenderingParams3 {
    type Target = IDWriteRenderingParams2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams3, windows_core::IUnknown, IDWriteRenderingParams, IDWriteRenderingParams1, IDWriteRenderingParams2);
impl IDWriteRenderingParams3 {
    pub unsafe fn GetRenderingMode1(&self) -> DWRITE_RENDERING_MODE1 {
        (windows_core::Interface::vtable(self).GetRenderingMode1)(windows_core::Interface::as_raw(self))
    }
}
unsafe impl Send for IDWriteRenderingParams3 {}
unsafe impl Sync for IDWriteRenderingParams3 {}
#[repr(C)]
pub struct IDWriteRenderingParams3_Vtbl {
    pub base__: IDWriteRenderingParams2_Vtbl,
    pub GetRenderingMode1: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_RENDERING_MODE1,
}
windows_core::imp::define_interface!(IDWriteStringList, IDWriteStringList_Vtbl, 0xcfee3140_1157_47ca_8b85_31bfcf3f2d0e);
impl core::ops::Deref for IDWriteStringList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteStringList, windows_core::IUnknown);
impl IDWriteStringList {
    pub unsafe fn GetCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self, listindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self), listindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetLocaleName(&self, listindex: u32, localename: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetStringLength(&self, listindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), listindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetString(&self, listindex: u32, stringbuffer: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for IDWriteStringList {}
unsafe impl Sync for IDWriteStringList {}
#[repr(C)]
pub struct IDWriteStringList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalysisSink, IDWriteTextAnalysisSink_Vtbl, 0x5810cd44_0ca0_4701_b3fa_bec5182ae4f6);
impl core::ops::Deref for IDWriteTextAnalysisSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSink, windows_core::IUnknown);
impl IDWriteTextAnalysisSink {
    pub unsafe fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScriptAnalysis)(windows_core::Interface::as_raw(self), textposition, textlength, scriptanalysis).ok()
    }
    pub unsafe fn SetLineBreakpoints(&self, textposition: u32, linebreakpoints: &[DWRITE_LINE_BREAKPOINT]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineBreakpoints)(windows_core::Interface::as_raw(self), textposition, linebreakpoints.len().try_into().unwrap(), core::mem::transmute(linebreakpoints.as_ptr())).ok()
    }
    pub unsafe fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBidiLevel)(windows_core::Interface::as_raw(self), textposition, textlength, explicitlevel, resolvedlevel).ok()
    }
    pub unsafe fn SetNumberSubstitution<P0>(&self, textposition: u32, textlength: u32, numbersubstitution: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteNumberSubstitution>,
    {
        (windows_core::Interface::vtable(self).SetNumberSubstitution)(windows_core::Interface::as_raw(self), textposition, textlength, numbersubstitution.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteTextAnalysisSink {}
unsafe impl Sync for IDWriteTextAnalysisSink {}
#[repr(C)]
pub struct IDWriteTextAnalysisSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScriptAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::HRESULT,
    pub SetLineBreakpoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const DWRITE_LINE_BREAKPOINT) -> windows_core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u8, u8) -> windows_core::HRESULT,
    pub SetNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalysisSink1, IDWriteTextAnalysisSink1_Vtbl, 0xb0d941a0_85e7_4d8b_9fd3_5ced9934482a);
impl core::ops::Deref for IDWriteTextAnalysisSink1 {
    type Target = IDWriteTextAnalysisSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSink1, windows_core::IUnknown, IDWriteTextAnalysisSink);
impl IDWriteTextAnalysisSink1 {
    pub unsafe fn SetGlyphOrientation<P0, P1>(&self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: P0, isrighttoleft: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGlyphOrientation)(windows_core::Interface::as_raw(self), textposition, textlength, glyphorientationangle, adjustedbidilevel, issideways.param().abi(), isrighttoleft.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteTextAnalysisSink1 {}
unsafe impl Sync for IDWriteTextAnalysisSink1 {}
#[repr(C)]
pub struct IDWriteTextAnalysisSink1_Vtbl {
    pub base__: IDWriteTextAnalysisSink_Vtbl,
    pub SetGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, DWRITE_GLYPH_ORIENTATION_ANGLE, u8, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalysisSource, IDWriteTextAnalysisSource_Vtbl, 0x688e1a58_5094_47c8_adc8_fbcea60ae92b);
impl core::ops::Deref for IDWriteTextAnalysisSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSource, windows_core::IUnknown);
impl IDWriteTextAnalysisSource {
    pub unsafe fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTextAtPosition)(windows_core::Interface::as_raw(self), textposition, textstring, textlength).ok()
    }
    pub unsafe fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTextBeforePosition)(windows_core::Interface::as_raw(self), textposition, textstring, textlength).ok()
    }
    pub unsafe fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (windows_core::Interface::vtable(self).GetParagraphReadingDirection)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), textposition, textlength, localename).ok()
    }
    pub unsafe fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut Option<IDWriteNumberSubstitution>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNumberSubstitution)(windows_core::Interface::as_raw(self), textposition, textlength, core::mem::transmute(numbersubstitution)).ok()
    }
}
unsafe impl Send for IDWriteTextAnalysisSource {}
unsafe impl Sync for IDWriteTextAnalysisSource {}
#[repr(C)]
pub struct IDWriteTextAnalysisSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTextAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetTextBeforePosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetParagraphReadingDirection: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut u16) -> windows_core::HRESULT,
    pub GetNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalysisSource1, IDWriteTextAnalysisSource1_Vtbl, 0x639cfad8_0fb4_4b21_a58a_067920120009);
impl core::ops::Deref for IDWriteTextAnalysisSource1 {
    type Target = IDWriteTextAnalysisSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSource1, windows_core::IUnknown, IDWriteTextAnalysisSource);
impl IDWriteTextAnalysisSource1 {
    pub unsafe fn GetVerticalGlyphOrientation(&self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), textposition, textlength, glyphorientation, bidilevel).ok()
    }
}
unsafe impl Send for IDWriteTextAnalysisSource1 {}
unsafe impl Sync for IDWriteTextAnalysisSource1 {}
#[repr(C)]
pub struct IDWriteTextAnalysisSource1_Vtbl {
    pub base__: IDWriteTextAnalysisSource_Vtbl,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalyzer, IDWriteTextAnalyzer_Vtbl, 0xb7e6163e_7f46_43b4_84b3_e4e6249c365d);
impl core::ops::Deref for IDWriteTextAnalyzer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer, windows_core::IUnknown);
impl IDWriteTextAnalyzer {
    pub unsafe fn AnalyzeScript<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P1: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        (windows_core::Interface::vtable(self).AnalyzeScript)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()).ok()
    }
    pub unsafe fn AnalyzeBidi<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P1: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        (windows_core::Interface::vtable(self).AnalyzeBidi)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()).ok()
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P1: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        (windows_core::Interface::vtable(self).AnalyzeNumberSubstitution)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()).ok()
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P1: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        (windows_core::Interface::vtable(self).AnalyzeLineBreakpoints)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()).ok()
    }
    pub unsafe fn GetGlyphs<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, textlength: u32, fontface: P1, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, numbersubstitution: P5, features: Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontFace>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<IDWriteNumberSubstitution>,
    {
        (windows_core::Interface::vtable(self).GetGlyphs)(windows_core::Interface::as_raw(self), textstring.param().abi(), textlength, fontface.param().abi(), issideways.param().abi(), isrighttoleft.param().abi(), scriptanalysis, localename.param().abi(), numbersubstitution.param().abi(), core::mem::transmute(features.unwrap_or(std::ptr::null())), core::mem::transmute(featurerangelengths.unwrap_or(std::ptr::null())), featureranges, maxglyphcount, clustermap, textprops, glyphindices, glyphprops, actualglyphcount).ok()
    }
    pub unsafe fn GetGlyphPlacements<P0, P1, P2, P3, P4>(&self, textstring: P0, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, features: Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontFace>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetGlyphPlacements)(windows_core::Interface::as_raw(self), textstring.param().abi(), clustermap, textprops, textlength, glyphindices, glyphprops, glyphcount, fontface.param().abi(), fontemsize, issideways.param().abi(), isrighttoleft.param().abi(), scriptanalysis, localename.param().abi(), core::mem::transmute(features.unwrap_or(std::ptr::null())), core::mem::transmute(featurerangelengths.unwrap_or(std::ptr::null())), featureranges, glyphadvances, glyphoffsets).ok()
    }
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: P2, issideways: P3, isrighttoleft: P4, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P5, features: Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontFace>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<super::super::Foundation::BOOL>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetGdiCompatibleGlyphPlacements)(
            windows_core::Interface::as_raw(self),
            textstring.param().abi(),
            clustermap,
            textprops,
            textlength,
            glyphindices,
            glyphprops,
            glyphcount,
            fontface.param().abi(),
            fontemsize,
            pixelsperdip,
            core::mem::transmute(transform.unwrap_or(std::ptr::null())),
            usegdinatural.param().abi(),
            issideways.param().abi(),
            isrighttoleft.param().abi(),
            scriptanalysis,
            localename.param().abi(),
            core::mem::transmute(features.unwrap_or(std::ptr::null())),
            core::mem::transmute(featurerangelengths.unwrap_or(std::ptr::null())),
            featureranges,
            glyphadvances,
            glyphoffsets,
        )
        .ok()
    }
}
unsafe impl Send for IDWriteTextAnalyzer {}
unsafe impl Sync for IDWriteTextAnalyzer {}
#[repr(C)]
pub struct IDWriteTextAnalyzer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AnalyzeScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeBidi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeLineBreakpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *const DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *mut core::ffi::c_void, *const *const DWRITE_TYPOGRAPHIC_FEATURES, *const u32, u32, u32, *mut u16, *mut DWRITE_SHAPING_TEXT_PROPERTIES, *mut u16, *mut DWRITE_SHAPING_GLYPH_PROPERTIES, *mut u32) -> windows_core::HRESULT,
    pub GetGlyphPlacements: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u16, *mut DWRITE_SHAPING_TEXT_PROPERTIES, u32, *const u16, *const DWRITE_SHAPING_GLYPH_PROPERTIES, u32, *mut core::ffi::c_void, f32, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *const DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *const *const DWRITE_TYPOGRAPHIC_FEATURES, *const u32, u32, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
    pub GetGdiCompatibleGlyphPlacements: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u16, *const DWRITE_SHAPING_TEXT_PROPERTIES, u32, *const u16, *const DWRITE_SHAPING_GLYPH_PROPERTIES, u32, *mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, super::super::Foundation::BOOL, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *const DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *const *const DWRITE_TYPOGRAPHIC_FEATURES, *const u32, u32, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalyzer1, IDWriteTextAnalyzer1_Vtbl, 0x80dad800_e21f_4e83_96ce_bfcce500db7c);
impl core::ops::Deref for IDWriteTextAnalyzer1 {
    type Target = IDWriteTextAnalyzer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer1, windows_core::IUnknown, IDWriteTextAnalyzer);
impl IDWriteTextAnalyzer1 {
    pub unsafe fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, glyphcount: u32, clustermap: &[u16], glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyCharacterSpacing)(windows_core::Interface::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, clustermap.len().try_into().unwrap(), glyphcount, core::mem::transmute(clustermap.as_ptr()), glyphadvances, glyphoffsets, glyphproperties, modifiedglyphadvances, modifiedglyphoffsets).ok()
    }
    pub unsafe fn GetBaseline<P0, P1, P2, P3>(&self, fontface: P0, baseline: DWRITE_BASELINE, isvertical: P1, issimulationallowed: P2, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P3, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetBaseline)(windows_core::Interface::as_raw(self), fontface.param().abi(), baseline, isvertical.param().abi(), issimulationallowed.param().abi(), core::mem::transmute(scriptanalysis), localename.param().abi(), baselinecoordinate, exists).ok()
    }
    pub unsafe fn AnalyzeVerticalGlyphOrientation<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource1>,
        P1: windows_core::Param<IDWriteTextAnalysisSink1>,
    {
        (windows_core::Interface::vtable(self).AnalyzeVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()).ok()
    }
    pub unsafe fn GetGlyphOrientationTransform<P0>(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: P0, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetGlyphOrientationTransform)(windows_core::Interface::as_raw(self), glyphorientationangle, issideways.param().abi(), transform).ok()
    }
    pub unsafe fn GetScriptProperties(&self, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScriptProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(scriptanalysis), scriptproperties).ok()
    }
    pub unsafe fn GetTextComplexity<P0, P1>(&self, textstring: P0, textlength: u32, fontface: P1, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: Option<*mut u16>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontFace>,
    {
        (windows_core::Interface::vtable(self).GetTextComplexity)(windows_core::Interface::as_raw(self), textstring.param().abi(), textlength, fontface.param().abi(), istextsimple, textlengthread, core::mem::transmute(glyphindices.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetJustificationOpportunities<P0, P1>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: P1, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetJustificationOpportunities)(windows_core::Interface::as_raw(self), fontface.param().abi(), fontemsize, core::mem::transmute(scriptanalysis), textlength, glyphcount, textstring.param().abi(), clustermap, glyphproperties, justificationopportunities).ok()
    }
    pub unsafe fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: Option<*mut DWRITE_GLYPH_OFFSET>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).JustifyGlyphAdvances)(windows_core::Interface::as_raw(self), linewidth, glyphcount, justificationopportunities, glyphadvances, glyphoffsets, justifiedglyphadvances, core::mem::transmute(justifiedglyphoffsets.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetJustifiedGlyphs<P0>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: Option<*const u16>, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: Option<*mut u16>, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        (windows_core::Interface::vtable(self).GetJustifiedGlyphs)(windows_core::Interface::as_raw(self), fontface.param().abi(), fontemsize, core::mem::transmute(scriptanalysis), textlength, glyphcount, maxglyphcount, core::mem::transmute(clustermap.unwrap_or(std::ptr::null())), glyphindices, glyphadvances, justifiedglyphadvances, justifiedglyphoffsets, glyphproperties, actualglyphcount, core::mem::transmute(modifiedclustermap.unwrap_or(std::ptr::null_mut())), modifiedglyphindices, modifiedglyphadvances, modifiedglyphoffsets).ok()
    }
}
unsafe impl Send for IDWriteTextAnalyzer1 {}
unsafe impl Sync for IDWriteTextAnalyzer1 {}
#[repr(C)]
pub struct IDWriteTextAnalyzer1_Vtbl {
    pub base__: IDWriteTextAnalyzer_Vtbl,
    pub ApplyCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32, u32, *const u16, *const f32, *const DWRITE_GLYPH_OFFSET, *const DWRITE_SHAPING_GLYPH_PROPERTIES, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
    pub GetBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_BASELINE, super::super::Foundation::BOOL, super::super::Foundation::BOOL, DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *mut i32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AnalyzeVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_GLYPH_ORIENTATION_ANGLE, super::super::Foundation::BOOL, *mut DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetScriptProperties: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_SCRIPT_ANALYSIS, *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::HRESULT,
    pub GetTextComplexity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut u32, *mut u16) -> windows_core::HRESULT,
    pub GetJustificationOpportunities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, DWRITE_SCRIPT_ANALYSIS, u32, u32, windows_core::PCWSTR, *const u16, *const DWRITE_SHAPING_GLYPH_PROPERTIES, *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::HRESULT,
    pub JustifyGlyphAdvances: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32, *const DWRITE_JUSTIFICATION_OPPORTUNITY, *const f32, *const DWRITE_GLYPH_OFFSET, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
    pub GetJustifiedGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, DWRITE_SCRIPT_ANALYSIS, u32, u32, u32, *const u16, *const u16, *const f32, *const f32, *const DWRITE_GLYPH_OFFSET, *const DWRITE_SHAPING_GLYPH_PROPERTIES, *mut u32, *mut u16, *mut u16, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextAnalyzer2, IDWriteTextAnalyzer2_Vtbl, 0x553a9ff3_5693_4df7_b52b_74806f7f2eb9);
impl core::ops::Deref for IDWriteTextAnalyzer2 {
    type Target = IDWriteTextAnalyzer1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer2, windows_core::IUnknown, IDWriteTextAnalyzer, IDWriteTextAnalyzer1);
impl IDWriteTextAnalyzer2 {
    pub unsafe fn GetGlyphOrientationTransform<P0>(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: P0, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetGlyphOrientationTransform)(windows_core::Interface::as_raw(self), glyphorientationangle, issideways.param().abi(), originx, originy, transform).ok()
    }
    pub unsafe fn GetTypographicFeatures<P0, P1>(&self, fontface: P0, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P1, actualtagcount: *mut u32, tags: &mut [DWRITE_FONT_FEATURE_TAG]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetTypographicFeatures)(windows_core::Interface::as_raw(self), fontface.param().abi(), core::mem::transmute(scriptanalysis), localename.param().abi(), tags.len().try_into().unwrap(), actualtagcount, core::mem::transmute(tags.as_ptr())).ok()
    }
    pub unsafe fn CheckTypographicFeature<P0, P1>(&self, fontface: P0, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P1, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFace>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CheckTypographicFeature)(windows_core::Interface::as_raw(self), fontface.param().abi(), core::mem::transmute(scriptanalysis), localename.param().abi(), featuretag, glyphcount, glyphindices, featureapplies).ok()
    }
}
unsafe impl Send for IDWriteTextAnalyzer2 {}
unsafe impl Sync for IDWriteTextAnalyzer2 {}
#[repr(C)]
pub struct IDWriteTextAnalyzer2_Vtbl {
    pub base__: IDWriteTextAnalyzer1_Vtbl,
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_GLYPH_ORIENTATION_ANGLE, super::super::Foundation::BOOL, f32, f32, *mut DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetTypographicFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, u32, *mut u32, *mut DWRITE_FONT_FEATURE_TAG) -> windows_core::HRESULT,
    pub CheckTypographicFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, DWRITE_FONT_FEATURE_TAG, u32, *const u16, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextFormat, IDWriteTextFormat_Vtbl, 0x9c906818_31d7_4fd3_a151_7c5e225db55a);
impl core::ops::Deref for IDWriteTextFormat {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
impl IDWriteTextFormat {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextAlignment)(windows_core::Interface::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParagraphAlignment)(windows_core::Interface::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWordWrapping)(windows_core::Interface::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReadingDirection)(windows_core::Interface::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlowDirection)(windows_core::Interface::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIncrementalTabStop)(windows_core::Interface::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteInlineObject>,
    {
        (windows_core::Interface::vtable(self).SetTrimming)(windows_core::Interface::as_raw(self), trimmingoptions, trimmingsign.param().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (windows_core::Interface::vtable(self).GetTextAlignment)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (windows_core::Interface::vtable(self).GetParagraphAlignment)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (windows_core::Interface::vtable(self).GetWordWrapping)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (windows_core::Interface::vtable(self).GetReadingDirection)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (windows_core::Interface::vtable(self).GetFlowDirection)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetIncrementalTabStop)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut Option<IDWriteInlineObject>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTrimming)(windows_core::Interface::as_raw(self), trimmingoptions, core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontFamilyName)(windows_core::Interface::as_raw(self), core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (windows_core::Interface::vtable(self).GetFontWeight)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (windows_core::Interface::vtable(self).GetFontStretch)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap()).ok()
    }
}
unsafe impl Send for IDWriteTextFormat {}
unsafe impl Sync for IDWriteTextFormat {}
#[repr(C)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXT_ALIGNMENT) -> windows_core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_WORD_WRAPPING) -> windows_core::HRESULT,
    pub SetReadingDirection: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_READING_DIRECTION) -> windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FLOW_DIRECTION) -> windows_core::HRESULT,
    pub SetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTrimming: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_TRIMMING, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_LINE_SPACING_METHOD, f32, f32) -> windows_core::HRESULT,
    pub GetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT,
    pub GetParagraphAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT,
    pub GetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_WORD_WRAPPING,
    pub GetReadingDirection: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetFlowDirection: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FLOW_DIRECTION,
    pub GetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetTrimming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TRIMMING, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_SPACING_METHOD, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextFormat1, IDWriteTextFormat1_Vtbl, 0x5f174b49_0d8b_4cfb_8bca_f1cce9d06c67);
impl core::ops::Deref for IDWriteTextFormat1 {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextFormat1, windows_core::IUnknown, IDWriteTextFormat);
impl IDWriteTextFormat1 {
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), glyphorientation).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        (windows_core::Interface::vtable(self).GetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLastLineWrapping)(windows_core::Interface::as_raw(self), islastlinewrappingenabled.param().abi()).ok()
    }
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetLastLineWrapping)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpticalAlignment)(windows_core::Interface::as_raw(self), opticalalignment).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        (windows_core::Interface::vtable(self).GetOpticalAlignment)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        (windows_core::Interface::vtable(self).SetFontFallback)(windows_core::Interface::as_raw(self), fontfallback.param().abi()).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteTextFormat1 {}
unsafe impl Sync for IDWriteTextFormat1 {}
#[repr(C)]
pub struct IDWriteTextFormat1_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    pub SetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub SetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextFormat2, IDWriteTextFormat2_Vtbl, 0xf67e0edd_9e3d_4ecc_8c32_4183253dfe70);
impl core::ops::Deref for IDWriteTextFormat2 {
    type Target = IDWriteTextFormat1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextFormat2, windows_core::IUnknown, IDWriteTextFormat, IDWriteTextFormat1);
impl IDWriteTextFormat2 {
    pub unsafe fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions).ok()
    }
}
unsafe impl Send for IDWriteTextFormat2 {}
unsafe impl Sync for IDWriteTextFormat2 {}
#[repr(C)]
pub struct IDWriteTextFormat2_Vtbl {
    pub base__: IDWriteTextFormat1_Vtbl,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_LINE_SPACING) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextFormat3, IDWriteTextFormat3_Vtbl, 0x6d3b5641_e550_430d_a85b_b7bf48a93427);
impl core::ops::Deref for IDWriteTextFormat3 {
    type Target = IDWriteTextFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextFormat3, windows_core::IUnknown, IDWriteTextFormat, IDWriteTextFormat1, IDWriteTextFormat2);
impl IDWriteTextFormat3 {
    pub unsafe fn SetFontAxisValues(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES {
        (windows_core::Interface::vtable(self).GetAutomaticFontAxes)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutomaticFontAxes)(windows_core::Interface::as_raw(self), automaticfontaxes).ok()
    }
}
unsafe impl Send for IDWriteTextFormat3 {}
unsafe impl Sync for IDWriteTextFormat3 {}
#[repr(C)]
pub struct IDWriteTextFormat3_Vtbl {
    pub base__: IDWriteTextFormat2_Vtbl,
    pub SetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextLayout, IDWriteTextLayout_Vtbl, 0x53737037_6d14_410b_9bfe_0b182bb70961);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout, windows_core::IUnknown, IDWriteTextFormat);
impl IDWriteTextLayout {
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxWidth)(windows_core::Interface::as_raw(self), maxwidth).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxHeight)(windows_core::Interface::as_raw(self), maxheight).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollection>,
    {
        (windows_core::Interface::vtable(self).SetFontCollection)(windows_core::Interface::as_raw(self), fontcollection.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFontFamilyName)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontWeight)(windows_core::Interface::as_raw(self), fontweight, core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontStyle)(windows_core::Interface::as_raw(self), fontstyle, core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontStretch)(windows_core::Interface::as_raw(self), fontstretch, core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontSize)(windows_core::Interface::as_raw(self), fontsize, core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUnderline)(windows_core::Interface::as_raw(self), hasunderline.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStrikethrough)(windows_core::Interface::as_raw(self), hasstrikethrough.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetDrawingEffect)(windows_core::Interface::as_raw(self), drawingeffect.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteInlineObject>,
    {
        (windows_core::Interface::vtable(self).SetInlineObject)(windows_core::Interface::as_raw(self), inlineobject.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTypography>,
    {
        (windows_core::Interface::vtable(self).SetTypography)(windows_core::Interface::as_raw(self), typography.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocaleName)(windows_core::Interface::as_raw(self), localename.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetMaxWidth)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        (windows_core::Interface::vtable(self).GetMaxHeight)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontCollection(&self, currentposition: u32, fontcollection: *mut Option<IDWriteFontCollection>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(fontcollection), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(windows_core::Interface::as_raw(self), currentposition, namelength, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyName(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontFamilyName)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontWeight(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontWeight)(windows_core::Interface::as_raw(self), currentposition, fontweight, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStyle(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(self), currentposition, fontstyle, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStretch(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontStretch)(windows_core::Interface::as_raw(self), currentposition, fontstretch, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontSize(&self, currentposition: u32, fontsize: *mut f32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(self), currentposition, fontsize, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUnderline)(windows_core::Interface::as_raw(self), currentposition, hasunderline, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStrikethrough)(windows_core::Interface::as_raw(self), currentposition, hasstrikethrough, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut Option<windows_core::IUnknown>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDrawingEffect)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(drawingeffect), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut Option<IDWriteInlineObject>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInlineObject)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(inlineobject), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut Option<IDWriteTypography>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTypography)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(typography), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self), currentposition, namelength, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleName(&self, currentposition: u32, localename: &mut [u16], textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTextRenderer>,
    {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), renderer.param().abi(), originx, originy).ok()
    }
    pub unsafe fn GetLineMetrics(&self, linemetrics: Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLineMetrics)(windows_core::Interface::as_raw(self), core::mem::transmute(linemetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOverhangMetrics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClusterMetrics)(windows_core::Interface::as_raw(self), core::mem::transmute(clustermetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DetermineMinWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HitTestPoint)(windows_core::Interface::as_raw(self), pointx, pointy, istrailinghit, isinside, hittestmetrics).ok()
    }
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).HitTestTextPosition)(windows_core::Interface::as_raw(self), textposition, istrailinghit.param().abi(), pointx, pointy, hittestmetrics).ok()
    }
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HitTestTextRange)(windows_core::Interface::as_raw(self), textposition, textlength, originx, originy, core::mem::transmute(hittestmetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount).ok()
    }
}
unsafe impl Send for IDWriteTextLayout {}
unsafe impl Sync for IDWriteTextLayout {}
#[repr(C)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_STYLE, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_STRETCH, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetDrawingEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_WEIGHT, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_STYLE, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_STRETCH, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetDrawingEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_METRICS, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT,
    pub GetClusterMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_CLUSTER_METRICS, u32, *mut u32) -> windows_core::HRESULT,
    pub DetermineMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub HitTestPoint: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL, *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT,
    pub HitTestTextPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL, *mut f32, *mut f32, *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT,
    pub HitTestTextRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, f32, f32, *mut DWRITE_HIT_TEST_METRICS, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextLayout1, IDWriteTextLayout1_Vtbl, 0x9064d822_80a7_465c_a986_df65f78b8feb);
impl core::ops::Deref for IDWriteTextLayout1 {
    type Target = IDWriteTextLayout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout1, windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout);
impl IDWriteTextLayout1 {
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPairKerning)(windows_core::Interface::as_raw(self), ispairkerningenabled.param().abi(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPairKerning)(windows_core::Interface::as_raw(self), currentposition, ispairkerningenabled, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCharacterSpacing)(windows_core::Interface::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCharacterSpacing)(windows_core::Interface::as_raw(self), currentposition, leadingspacing, trailingspacing, minimumadvancewidth, core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
unsafe impl Send for IDWriteTextLayout1 {}
unsafe impl Sync for IDWriteTextLayout1 {}
#[repr(C)]
pub struct IDWriteTextLayout1_Vtbl {
    pub base__: IDWriteTextLayout_Vtbl,
    pub SetPairKerning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetPairKerning: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextLayout2, IDWriteTextLayout2_Vtbl, 0x1093c18f_8d5e_43f0_b064_0917311b525e);
impl core::ops::Deref for IDWriteTextLayout2 {
    type Target = IDWriteTextLayout1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout2, windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout, IDWriteTextLayout1);
impl IDWriteTextLayout2 {
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), glyphorientation).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        (windows_core::Interface::vtable(self).GetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLastLineWrapping)(windows_core::Interface::as_raw(self), islastlinewrappingenabled.param().abi()).ok()
    }
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetLastLineWrapping)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOpticalAlignment)(windows_core::Interface::as_raw(self), opticalalignment).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        (windows_core::Interface::vtable(self).GetOpticalAlignment)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        (windows_core::Interface::vtable(self).SetFontFallback)(windows_core::Interface::as_raw(self), fontfallback.param().abi()).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFallback)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IDWriteTextLayout2 {}
unsafe impl Sync for IDWriteTextLayout2 {}
#[repr(C)]
pub struct IDWriteTextLayout2_Vtbl {
    pub base__: IDWriteTextLayout1_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TEXT_METRICS1) -> windows_core::HRESULT,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    pub SetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLastLineWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub SetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextLayout3, IDWriteTextLayout3_Vtbl, 0x07ddcd52_020e_4de8_ac33_6c953d83f92d);
impl core::ops::Deref for IDWriteTextLayout3 {
    type Target = IDWriteTextLayout2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout3, windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout, IDWriteTextLayout1, IDWriteTextLayout2);
impl IDWriteTextLayout3 {
    pub unsafe fn InvalidateLayout(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvalidateLayout)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions).ok()
    }
    pub unsafe fn GetLineMetrics(&self, linemetrics: Option<&mut [DWRITE_LINE_METRICS1]>, actuallinecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLineMetrics)(windows_core::Interface::as_raw(self), core::mem::transmute(linemetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount).ok()
    }
}
unsafe impl Send for IDWriteTextLayout3 {}
unsafe impl Sync for IDWriteTextLayout3 {}
#[repr(C)]
pub struct IDWriteTextLayout3_Vtbl {
    pub base__: IDWriteTextLayout2_Vtbl,
    pub InvalidateLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_LINE_SPACING) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_METRICS1, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextLayout4, IDWriteTextLayout4_Vtbl, 0x05a9bf42_223f_4441_b5fb_8263685f55e9);
impl core::ops::Deref for IDWriteTextLayout4 {
    type Target = IDWriteTextLayout3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout4, windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout, IDWriteTextLayout1, IDWriteTextLayout2, IDWriteTextLayout3);
impl IDWriteTextLayout4 {
    pub unsafe fn SetFontAxisValues(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetFontAxisValueCount(&self, currentposition: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self), currentposition)
    }
    pub unsafe fn GetFontAxisValues(&self, currentposition: u32, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE], textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), core::mem::transmute(textrange.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES {
        (windows_core::Interface::vtable(self).GetAutomaticFontAxes)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutomaticFontAxes)(windows_core::Interface::as_raw(self), automaticfontaxes).ok()
    }
}
unsafe impl Send for IDWriteTextLayout4 {}
unsafe impl Sync for IDWriteTextLayout4 {}
#[repr(C)]
pub struct IDWriteTextLayout4_Vtbl {
    pub base__: IDWriteTextLayout3_Vtbl,
    pub SetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_AXIS_VALUE, u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextRenderer, IDWriteTextRenderer_Vtbl, 0xef8a8135_5cc6_45fe_8825_c5a0724eb819);
impl core::ops::Deref for IDWriteTextRenderer {
    type Target = IDWritePixelSnapping;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextRenderer, windows_core::IUnknown, IDWritePixelSnapping);
impl IDWriteTextRenderer {
    pub unsafe fn DrawGlyphRun<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), baselineoriginx, baselineoriginy, measuringmode, glyphrun, glyphrundescription, clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn DrawUnderline<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawUnderline)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), baselineoriginx, baselineoriginy, underline, clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn DrawStrikethrough<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawStrikethrough)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), baselineoriginx, baselineoriginy, strikethrough, clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn DrawInlineObject<P0, P1, P2, P3>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, originx: f32, originy: f32, inlineobject: P0, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteInlineObject>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawInlineObject)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), originx, originy, inlineobject.param().abi(), issideways.param().abi(), isrighttoleft.param().abi(), clientdrawingeffect.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteTextRenderer {}
unsafe impl Sync for IDWriteTextRenderer {}
#[repr(C)]
pub struct IDWriteTextRenderer_Vtbl {
    pub base__: IDWritePixelSnapping_Vtbl,
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, *const DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, *const DWRITE_UNDERLINE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, *const DWRITE_STRIKETHROUGH, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTextRenderer1, IDWriteTextRenderer1_Vtbl, 0xd3e0e934_22a0_427e_aae4_7d9574b59db1);
impl core::ops::Deref for IDWriteTextRenderer1 {
    type Target = IDWriteTextRenderer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextRenderer1, windows_core::IUnknown, IDWritePixelSnapping, IDWriteTextRenderer);
impl IDWriteTextRenderer1 {
    pub unsafe fn DrawGlyphRun<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), baselineoriginx, baselineoriginy, orientationangle, measuringmode, glyphrun, glyphrundescription, clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn DrawUnderline<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawUnderline)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), baselineoriginx, baselineoriginy, orientationangle, underline, clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn DrawStrikethrough<P0>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawStrikethrough)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), baselineoriginx, baselineoriginy, orientationangle, strikethrough, clientdrawingeffect.param().abi()).ok()
    }
    pub unsafe fn DrawInlineObject<P0, P1, P2, P3>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: P0, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteInlineObject>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DrawInlineObject)(windows_core::Interface::as_raw(self), core::mem::transmute(clientdrawingcontext.unwrap_or(std::ptr::null())), originx, originy, orientationangle, inlineobject.param().abi(), issideways.param().abi(), isrighttoleft.param().abi(), clientdrawingeffect.param().abi()).ok()
    }
}
unsafe impl Send for IDWriteTextRenderer1 {}
unsafe impl Sync for IDWriteTextRenderer1 {}
#[repr(C)]
pub struct IDWriteTextRenderer1_Vtbl {
    pub base__: IDWriteTextRenderer_Vtbl,
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, DWRITE_GLYPH_ORIENTATION_ANGLE, DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, *const DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, DWRITE_GLYPH_ORIENTATION_ANGLE, *const DWRITE_UNDERLINE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, DWRITE_GLYPH_ORIENTATION_ANGLE, *const DWRITE_STRIKETHROUGH, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, DWRITE_GLYPH_ORIENTATION_ANGLE, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDWriteTypography, IDWriteTypography_Vtbl, 0x55f1112b_1dc2_4b3c_9541_f46894ed85b6);
impl core::ops::Deref for IDWriteTypography {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTypography, windows_core::IUnknown);
impl IDWriteTypography {
    pub unsafe fn AddFontFeature(&self, fontfeature: DWRITE_FONT_FEATURE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddFontFeature)(windows_core::Interface::as_raw(self), core::mem::transmute(fontfeature)).ok()
    }
    pub unsafe fn GetFontFeatureCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetFontFeatureCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetFontFeature(&self, fontfeatureindex: u32) -> windows_core::Result<DWRITE_FONT_FEATURE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFontFeature)(windows_core::Interface::as_raw(self), fontfeatureindex, &mut result__).map(|| result__)
    }
}
unsafe impl Send for IDWriteTypography {}
unsafe impl Sync for IDWriteTypography {}
#[repr(C)]
pub struct IDWriteTypography_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFontFeature: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FEATURE) -> windows_core::HRESULT,
    pub GetFontFeatureCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFeature: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_FEATURE) -> windows_core::HRESULT,
}
pub const DWRITE_ALPHA_MAX: u32 = 255u32;
pub const DWRITE_AUTOMATIC_FONT_AXES_NONE: DWRITE_AUTOMATIC_FONT_AXES = DWRITE_AUTOMATIC_FONT_AXES(0i32);
pub const DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE: DWRITE_AUTOMATIC_FONT_AXES = DWRITE_AUTOMATIC_FONT_AXES(1i32);
pub const DWRITE_BASELINE_CENTRAL: DWRITE_BASELINE = DWRITE_BASELINE(2i32);
pub const DWRITE_BASELINE_DEFAULT: DWRITE_BASELINE = DWRITE_BASELINE(0i32);
pub const DWRITE_BASELINE_HANGING: DWRITE_BASELINE = DWRITE_BASELINE(4i32);
pub const DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM: DWRITE_BASELINE = DWRITE_BASELINE(5i32);
pub const DWRITE_BASELINE_IDEOGRAPHIC_TOP: DWRITE_BASELINE = DWRITE_BASELINE(6i32);
pub const DWRITE_BASELINE_MATH: DWRITE_BASELINE = DWRITE_BASELINE(3i32);
pub const DWRITE_BASELINE_MAXIMUM: DWRITE_BASELINE = DWRITE_BASELINE(8i32);
pub const DWRITE_BASELINE_MINIMUM: DWRITE_BASELINE = DWRITE_BASELINE(7i32);
pub const DWRITE_BASELINE_ROMAN: DWRITE_BASELINE = DWRITE_BASELINE(1i32);
pub const DWRITE_BREAK_CONDITION_CAN_BREAK: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(1i32);
pub const DWRITE_BREAK_CONDITION_MAY_NOT_BREAK: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(2i32);
pub const DWRITE_BREAK_CONDITION_MUST_BREAK: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(3i32);
pub const DWRITE_BREAK_CONDITION_NEUTRAL: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(0i32);
pub const DWRITE_COLOR_COMPOSITE_CLEAR: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(0i32);
pub const DWRITE_COLOR_COMPOSITE_COLOR_BURN: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(18i32);
pub const DWRITE_COLOR_COMPOSITE_COLOR_DODGE: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(17i32);
pub const DWRITE_COLOR_COMPOSITE_DARKEN: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(15i32);
pub const DWRITE_COLOR_COMPOSITE_DEST: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(2i32);
pub const DWRITE_COLOR_COMPOSITE_DEST_ATOP: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(10i32);
pub const DWRITE_COLOR_COMPOSITE_DEST_IN: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(6i32);
pub const DWRITE_COLOR_COMPOSITE_DEST_OUT: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(8i32);
pub const DWRITE_COLOR_COMPOSITE_DEST_OVER: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(4i32);
pub const DWRITE_COLOR_COMPOSITE_DIFFERENCE: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(21i32);
pub const DWRITE_COLOR_COMPOSITE_EXCLUSION: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(22i32);
pub const DWRITE_COLOR_COMPOSITE_HARD_LIGHT: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(19i32);
pub const DWRITE_COLOR_COMPOSITE_HSL_COLOR: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(26i32);
pub const DWRITE_COLOR_COMPOSITE_HSL_HUE: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(24i32);
pub const DWRITE_COLOR_COMPOSITE_HSL_LUMINOSITY: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(27i32);
pub const DWRITE_COLOR_COMPOSITE_HSL_SATURATION: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(25i32);
pub const DWRITE_COLOR_COMPOSITE_LIGHTEN: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(16i32);
pub const DWRITE_COLOR_COMPOSITE_MULTIPLY: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(23i32);
pub const DWRITE_COLOR_COMPOSITE_OVERLAY: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(14i32);
pub const DWRITE_COLOR_COMPOSITE_PLUS: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(12i32);
pub const DWRITE_COLOR_COMPOSITE_SCREEN: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(13i32);
pub const DWRITE_COLOR_COMPOSITE_SOFT_LIGHT: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(20i32);
pub const DWRITE_COLOR_COMPOSITE_SRC: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(1i32);
pub const DWRITE_COLOR_COMPOSITE_SRC_ATOP: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(9i32);
pub const DWRITE_COLOR_COMPOSITE_SRC_IN: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(5i32);
pub const DWRITE_COLOR_COMPOSITE_SRC_OUT: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(7i32);
pub const DWRITE_COLOR_COMPOSITE_SRC_OVER: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(3i32);
pub const DWRITE_COLOR_COMPOSITE_XOR: DWRITE_COLOR_COMPOSITE_MODE = DWRITE_COLOR_COMPOSITE_MODE(11i32);
pub const DWRITE_CONTAINER_TYPE_UNKNOWN: DWRITE_CONTAINER_TYPE = DWRITE_CONTAINER_TYPE(0i32);
pub const DWRITE_CONTAINER_TYPE_WOFF: DWRITE_CONTAINER_TYPE = DWRITE_CONTAINER_TYPE(1i32);
pub const DWRITE_CONTAINER_TYPE_WOFF2: DWRITE_CONTAINER_TYPE = DWRITE_CONTAINER_TYPE(2i32);
pub const DWRITE_ERR_BASE: u32 = 20480u32;
pub const DWRITE_E_DOWNLOADCANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x8898500E_u32 as _);
pub const DWRITE_E_DOWNLOADFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8898500F_u32 as _);
pub const DWRITE_E_REMOTEFONT: windows_core::HRESULT = windows_core::HRESULT(0x8898500D_u32 as _);
pub const DWRITE_E_TOOMANYDOWNLOADS: windows_core::HRESULT = windows_core::HRESULT(0x88985010_u32 as _);
pub const DWRITE_FACTORY_TYPE_ISOLATED: DWRITE_FACTORY_TYPE = DWRITE_FACTORY_TYPE(1i32);
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = DWRITE_FACTORY_TYPE(0i32);
pub const DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(1i32);
pub const DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(2i32);
pub const DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(3i32);
pub const DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(0i32);
pub const DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN: DWRITE_FONT_AXIS_ATTRIBUTES = DWRITE_FONT_AXIS_ATTRIBUTES(2i32);
pub const DWRITE_FONT_AXIS_ATTRIBUTES_NONE: DWRITE_FONT_AXIS_ATTRIBUTES = DWRITE_FONT_AXIS_ATTRIBUTES(0i32);
pub const DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE: DWRITE_FONT_AXIS_ATTRIBUTES = DWRITE_FONT_AXIS_ATTRIBUTES(1i32);
pub const DWRITE_FONT_AXIS_TAG_ITALIC: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1818326121u32);
pub const DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(2054385775u32);
pub const DWRITE_FONT_AXIS_TAG_SLANT: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1953393779u32);
pub const DWRITE_FONT_AXIS_TAG_WEIGHT: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1952999287u32);
pub const DWRITE_FONT_AXIS_TAG_WIDTH: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1752458359u32);
pub const DWRITE_FONT_FACE_TYPE_BITMAP: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(5i32);
pub const DWRITE_FONT_FACE_TYPE_CFF: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(0i32);
pub const DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(2i32);
pub const DWRITE_FONT_FACE_TYPE_RAW_CFF: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(7i32);
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(1i32);
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(2i32);
pub const DWRITE_FONT_FACE_TYPE_TYPE1: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(3i32);
pub const DWRITE_FONT_FACE_TYPE_UNKNOWN: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(6i32);
pub const DWRITE_FONT_FACE_TYPE_VECTOR: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(4i32);
pub const DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC: DWRITE_FONT_FAMILY_MODEL = DWRITE_FONT_FAMILY_MODEL(0i32);
pub const DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE: DWRITE_FONT_FAMILY_MODEL = DWRITE_FONT_FAMILY_MODEL(1i32);
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259886u32);
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259880u32);
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1668441697u32);
pub const DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1886613603u32);
pub const DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1702060387u32);
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259875u32);
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962275u32);
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1752658787u32);
pub const DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1936880995u32);
pub const DWRITE_FONT_FEATURE_TAG_DEFAULT: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953261156u32);
pub const DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962276u32);
pub const DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953527909u32);
pub const DWRITE_FONT_FEATURE_TAG_FRACTIONS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1667330662u32);
pub const DWRITE_FONT_FEATURE_TAG_FULL_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633446u32);
pub const DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1886217059u32);
pub const DWRITE_FONT_FEATURE_TAG_HALANT_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1852596584u32);
pub const DWRITE_FONT_FEATURE_TAG_HALF_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1718378856u32);
pub const DWRITE_FONT_FEATURE_TAG_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633448u32);
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953720680u32);
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962280u32);
pub const DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1869246312u32);
pub const DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1634626408u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS04_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(875589738u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS78_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(943157354u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS83_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(859336810u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS90_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(809070698u32);
pub const DWRITE_FONT_FEATURE_TAG_KERNING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1852990827u32);
pub const DWRITE_FONT_FEATURE_TAG_LINING_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412524u32);
pub const DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1818455916u32);
pub const DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1802658157u32);
pub const DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1802333037u32);
pub const DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1802659693u32);
pub const DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1801677934u32);
pub const DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412527u32);
pub const DWRITE_FONT_FEATURE_TAG_ORDINALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1852076655u32);
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1885430640u32);
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1668297315u32);
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259888u32);
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412528u32);
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633456u32);
pub const DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633457u32);
pub const DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962290u32);
pub const DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(2036495730u32);
pub const DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1718511987u32);
pub const DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1819307379u32);
pub const DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1869768058u32);
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1885564275u32);
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1668493923u32);
pub const DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1634167148u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259891u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(825258867u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(808547187u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(825324403u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(842101619u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(858878835u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(875656051u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(892433267u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(909210483u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(925987699u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(942764915u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(959542131u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(842036083u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(808612723u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(858813299u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(875590515u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(892367731u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(909144947u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(925922163u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(942699379u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(959476595u32);
pub const DWRITE_FONT_FEATURE_TAG_SUBSCRIPT: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1935832435u32);
pub const DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1936749939u32);
pub const DWRITE_FONT_FEATURE_TAG_SWASH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1752397683u32);
pub const DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412532u32);
pub const DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633460u32);
pub const DWRITE_FONT_FEATURE_TAG_TITLING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1819568500u32);
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684107892u32);
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1835101812u32);
pub const DWRITE_FONT_FEATURE_TAG_UNICASE: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1667853941u32);
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(846492278u32);
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953654134u32);
pub const DWRITE_FONT_FILE_TYPE_BITMAP: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(7i32);
pub const DWRITE_FONT_FILE_TYPE_CFF: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(1i32);
pub const DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(3i32);
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(2i32);
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(3i32);
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFB: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(5i32);
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFM: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(4i32);
pub const DWRITE_FONT_FILE_TYPE_UNKNOWN: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(0i32);
pub const DWRITE_FONT_FILE_TYPE_VECTOR: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(6i32);
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: DWRITE_FONT_LINE_GAP_USAGE = DWRITE_FONT_LINE_GAP_USAGE(0i32);
pub const DWRITE_FONT_LINE_GAP_USAGE_DISABLED: DWRITE_FONT_LINE_GAP_USAGE = DWRITE_FONT_LINE_GAP_USAGE(1i32);
pub const DWRITE_FONT_LINE_GAP_USAGE_ENABLED: DWRITE_FONT_LINE_GAP_USAGE = DWRITE_FONT_LINE_GAP_USAGE(2i32);
pub const DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(7i32);
pub const DWRITE_FONT_PROPERTY_ID_FACE_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(3i32);
pub const DWRITE_FONT_PROPERTY_ID_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(1i32);
pub const DWRITE_FONT_PROPERTY_ID_FULL_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(4i32);
pub const DWRITE_FONT_PROPERTY_ID_NONE: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(0i32);
pub const DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(6i32);
pub const DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(2i32);
pub const DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(9i32);
pub const DWRITE_FONT_PROPERTY_ID_STRETCH: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(11i32);
pub const DWRITE_FONT_PROPERTY_ID_STYLE: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(12i32);
pub const DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(8i32);
pub const DWRITE_FONT_PROPERTY_ID_TOTAL: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(13i32);
pub const DWRITE_FONT_PROPERTY_ID_TOTAL_RS3: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(14i32);
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(13i32);
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(2i32);
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(10i32);
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(3i32);
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(1i32);
pub const DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(5i32);
pub const DWRITE_FONT_SIMULATIONS_BOLD: DWRITE_FONT_SIMULATIONS = DWRITE_FONT_SIMULATIONS(1i32);
pub const DWRITE_FONT_SIMULATIONS_NONE: DWRITE_FONT_SIMULATIONS = DWRITE_FONT_SIMULATIONS(0i32);
pub const DWRITE_FONT_SIMULATIONS_OBLIQUE: DWRITE_FONT_SIMULATIONS = DWRITE_FONT_SIMULATIONS(2i32);
pub const DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(3i32);
pub const DWRITE_FONT_SOURCE_TYPE_PER_MACHINE: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(1i32);
pub const DWRITE_FONT_SOURCE_TYPE_PER_USER: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(2i32);
pub const DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(4i32);
pub const DWRITE_FONT_SOURCE_TYPE_UNKNOWN: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(0i32);
pub const DWRITE_FONT_STRETCH_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(3i32);
pub const DWRITE_FONT_STRETCH_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(7i32);
pub const DWRITE_FONT_STRETCH_EXTRA_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(2i32);
pub const DWRITE_FONT_STRETCH_EXTRA_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(8i32);
pub const DWRITE_FONT_STRETCH_MEDIUM: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(5i32);
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(5i32);
pub const DWRITE_FONT_STRETCH_SEMI_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(4i32);
pub const DWRITE_FONT_STRETCH_SEMI_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(6i32);
pub const DWRITE_FONT_STRETCH_ULTRA_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(1i32);
pub const DWRITE_FONT_STRETCH_ULTRA_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(9i32);
pub const DWRITE_FONT_STRETCH_UNDEFINED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(0i32);
pub const DWRITE_FONT_STYLE_ITALIC: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(2i32);
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(0i32);
pub const DWRITE_FONT_STYLE_OBLIQUE: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(1i32);
pub const DWRITE_FONT_WEIGHT_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(900i32);
pub const DWRITE_FONT_WEIGHT_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(700i32);
pub const DWRITE_FONT_WEIGHT_DEMI_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(600i32);
pub const DWRITE_FONT_WEIGHT_EXTRA_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(950i32);
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(800i32);
pub const DWRITE_FONT_WEIGHT_EXTRA_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(200i32);
pub const DWRITE_FONT_WEIGHT_HEAVY: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(900i32);
pub const DWRITE_FONT_WEIGHT_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(300i32);
pub const DWRITE_FONT_WEIGHT_MEDIUM: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(500i32);
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(400i32);
pub const DWRITE_FONT_WEIGHT_REGULAR: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(400i32);
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(600i32);
pub const DWRITE_FONT_WEIGHT_SEMI_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(350i32);
pub const DWRITE_FONT_WEIGHT_THIN: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(100i32);
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(950i32);
pub const DWRITE_FONT_WEIGHT_ULTRA_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(800i32);
pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(200i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_CFF: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(2i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(4i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR_PAINT_TREE: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(256i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_JPEG: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(32i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_NONE: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(0i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_PNG: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(16i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(128i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_SVG: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(8i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_TIFF: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(64i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(1i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(0i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(2i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(3i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(1i32);
pub const DWRITE_GRID_FIT_MODE_DEFAULT: DWRITE_GRID_FIT_MODE = DWRITE_GRID_FIT_MODE(0i32);
pub const DWRITE_GRID_FIT_MODE_DISABLED: DWRITE_GRID_FIT_MODE = DWRITE_GRID_FIT_MODE(1i32);
pub const DWRITE_GRID_FIT_MODE_ENABLED: DWRITE_GRID_FIT_MODE = DWRITE_GRID_FIT_MODE(2i32);
pub const DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(1i32);
pub const DWRITE_INFORMATIONAL_STRING_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(7i32);
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(5i32);
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER_URL: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(6i32);
pub const DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(20i32);
pub const DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(8i32);
pub const DWRITE_INFORMATIONAL_STRING_FULL_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(16i32);
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(9i32);
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(10i32);
pub const DWRITE_INFORMATIONAL_STRING_MANUFACTURER: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(4i32);
pub const DWRITE_INFORMATIONAL_STRING_NONE: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(0i32);
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(18i32);
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(17i32);
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(13i32);
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(14i32);
pub const DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(15i32);
pub const DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(21i32);
pub const DWRITE_INFORMATIONAL_STRING_TRADEMARK: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(3i32);
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(13i32);
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(14i32);
pub const DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(2i32);
pub const DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(19i32);
pub const DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(11i32);
pub const DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(12i32);
pub const DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(19i32);
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: DWRITE_LINE_SPACING_METHOD = DWRITE_LINE_SPACING_METHOD(0i32);
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: DWRITE_LINE_SPACING_METHOD = DWRITE_LINE_SPACING_METHOD(2i32);
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: DWRITE_LINE_SPACING_METHOD = DWRITE_LINE_SPACING_METHOD(1i32);
pub const DWRITE_LOCALITY_LOCAL: DWRITE_LOCALITY = DWRITE_LOCALITY(2i32);
pub const DWRITE_LOCALITY_PARTIAL: DWRITE_LOCALITY = DWRITE_LOCALITY(1i32);
pub const DWRITE_LOCALITY_REMOTE: DWRITE_LOCALITY = DWRITE_LOCALITY(0i32);
pub const DWRITE_MEASURING_MODE_GDI_CLASSIC: DWRITE_MEASURING_MODE = DWRITE_MEASURING_MODE(1i32);
pub const DWRITE_MEASURING_MODE_GDI_NATURAL: DWRITE_MEASURING_MODE = DWRITE_MEASURING_MODE(2i32);
pub const DWRITE_MEASURING_MODE_NATURAL: DWRITE_MEASURING_MODE = DWRITE_MEASURING_MODE(0i32);
pub const DWRITE_NO_PALETTE_INDEX: u32 = 65535u32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(1i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(0i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(3i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(2i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(4i32);
pub const DWRITE_OPTICAL_ALIGNMENT_NONE: DWRITE_OPTICAL_ALIGNMENT = DWRITE_OPTICAL_ALIGNMENT(0i32);
pub const DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS: DWRITE_OPTICAL_ALIGNMENT = DWRITE_OPTICAL_ALIGNMENT(1i32);
pub const DWRITE_OUTLINE_THRESHOLD_ALIASED: DWRITE_OUTLINE_THRESHOLD = DWRITE_OUTLINE_THRESHOLD(1i32);
pub const DWRITE_OUTLINE_THRESHOLD_ANTIALIASED: DWRITE_OUTLINE_THRESHOLD = DWRITE_OUTLINE_THRESHOLD(0i32);
pub const DWRITE_PAINT_ATTRIBUTES_NONE: DWRITE_PAINT_ATTRIBUTES = DWRITE_PAINT_ATTRIBUTES(0i32);
pub const DWRITE_PAINT_ATTRIBUTES_USES_PALETTE: DWRITE_PAINT_ATTRIBUTES = DWRITE_PAINT_ATTRIBUTES(1i32);
pub const DWRITE_PAINT_ATTRIBUTES_USES_TEXT_COLOR: DWRITE_PAINT_ATTRIBUTES = DWRITE_PAINT_ATTRIBUTES(2i32);
pub const DWRITE_PAINT_FEATURE_LEVEL_COLR_V0: DWRITE_PAINT_FEATURE_LEVEL = DWRITE_PAINT_FEATURE_LEVEL(1i32);
pub const DWRITE_PAINT_FEATURE_LEVEL_COLR_V1: DWRITE_PAINT_FEATURE_LEVEL = DWRITE_PAINT_FEATURE_LEVEL(2i32);
pub const DWRITE_PAINT_FEATURE_LEVEL_NONE: DWRITE_PAINT_FEATURE_LEVEL = DWRITE_PAINT_FEATURE_LEVEL(0i32);
pub const DWRITE_PAINT_TYPE_COLOR_GLYPH: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(8i32);
pub const DWRITE_PAINT_TYPE_COMPOSITE: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(10i32);
pub const DWRITE_PAINT_TYPE_GLYPH: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(7i32);
pub const DWRITE_PAINT_TYPE_LAYERS: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(1i32);
pub const DWRITE_PAINT_TYPE_LINEAR_GRADIENT: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(4i32);
pub const DWRITE_PAINT_TYPE_NONE: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(0i32);
pub const DWRITE_PAINT_TYPE_RADIAL_GRADIENT: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(5i32);
pub const DWRITE_PAINT_TYPE_SOLID: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(3i32);
pub const DWRITE_PAINT_TYPE_SOLID_GLYPH: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(2i32);
pub const DWRITE_PAINT_TYPE_SWEEP_GRADIENT: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(6i32);
pub const DWRITE_PAINT_TYPE_TRANSFORM: DWRITE_PAINT_TYPE = DWRITE_PAINT_TYPE(9i32);
pub const DWRITE_PANOSE_ARM_STYLE_ANY: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(0i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(11i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(7i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(10i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(9i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(8i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(11i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(7i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(10i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(9i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(8i32);
pub const DWRITE_PANOSE_ARM_STYLE_NO_FIT: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(1i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(6i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(2i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(2i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(5i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(4i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(4i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(3i32);
pub const DWRITE_PANOSE_ASPECT_ANY: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(0i32);
pub const DWRITE_PANOSE_ASPECT_CONDENSED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(4i32);
pub const DWRITE_PANOSE_ASPECT_EXTENDED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(6i32);
pub const DWRITE_PANOSE_ASPECT_MONOSPACED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(9i32);
pub const DWRITE_PANOSE_ASPECT_NORMAL: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(5i32);
pub const DWRITE_PANOSE_ASPECT_NO_FIT: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(1i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_ANY: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(0i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(3i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(5i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(4i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(1i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(2i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(6i32);
pub const DWRITE_PANOSE_ASPECT_SUPER_CONDENSED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(2i32);
pub const DWRITE_PANOSE_ASPECT_SUPER_EXTENDED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(8i32);
pub const DWRITE_PANOSE_ASPECT_VERY_CONDENSED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(3i32);
pub const DWRITE_PANOSE_ASPECT_VERY_EXTENDED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(7i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_ANY: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(0i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(2i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_LITERALS: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(3i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(1i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(4i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(5i32);
pub const DWRITE_PANOSE_CONTRAST_ANY: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(0i32);
pub const DWRITE_PANOSE_CONTRAST_BROKEN: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(13i32);
pub const DWRITE_PANOSE_CONTRAST_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(8i32);
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(12i32);
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(10i32);
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(11i32);
pub const DWRITE_PANOSE_CONTRAST_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(4i32);
pub const DWRITE_PANOSE_CONTRAST_MEDIUM: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(6i32);
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(7i32);
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(5i32);
pub const DWRITE_PANOSE_CONTRAST_NONE: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(2i32);
pub const DWRITE_PANOSE_CONTRAST_NO_FIT: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(1i32);
pub const DWRITE_PANOSE_CONTRAST_VERY_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(9i32);
pub const DWRITE_PANOSE_CONTRAST_VERY_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(3i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ANY: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(0i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(7i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(11i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(2i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(6i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(12i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(5i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(4i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(3i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(1i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(9i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(8i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(10i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(0i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(5i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(14i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(13i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(7i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(8i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(12i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(11i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(9i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(4i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(1i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(3i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(2i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(15i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(6i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(10i32);
pub const DWRITE_PANOSE_FAMILY_ANY: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(0i32);
pub const DWRITE_PANOSE_FAMILY_DECORATIVE: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(4i32);
pub const DWRITE_PANOSE_FAMILY_NO_FIT: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(1i32);
pub const DWRITE_PANOSE_FAMILY_PICTORIAL: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(5i32);
pub const DWRITE_PANOSE_FAMILY_SCRIPT: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(3i32);
pub const DWRITE_PANOSE_FAMILY_SYMBOL: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(5i32);
pub const DWRITE_PANOSE_FAMILY_TEXT_DISPLAY: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(2i32);
pub const DWRITE_PANOSE_FILL_ANY: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(0i32);
pub const DWRITE_PANOSE_FILL_COMPLEX_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(5i32);
pub const DWRITE_PANOSE_FILL_DRAWN_DISTRESSED: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(7i32);
pub const DWRITE_PANOSE_FILL_NO_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(3i32);
pub const DWRITE_PANOSE_FILL_NO_FIT: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(1i32);
pub const DWRITE_PANOSE_FILL_PATTERNED_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(4i32);
pub const DWRITE_PANOSE_FILL_SHAPED_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(6i32);
pub const DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(2i32);
pub const DWRITE_PANOSE_FINIALS_ANY: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(0i32);
pub const DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(3i32);
pub const DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(2i32);
pub const DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(4i32);
pub const DWRITE_PANOSE_FINIALS_NO_FIT: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(1i32);
pub const DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(12i32);
pub const DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(11i32);
pub const DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(13i32);
pub const DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(6i32);
pub const DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(5i32);
pub const DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(7i32);
pub const DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(9i32);
pub const DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(8i32);
pub const DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(10i32);
pub const DWRITE_PANOSE_LETTERFORM_ANY: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(0i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(4i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(2i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(5i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(7i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(6i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(8i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(3i32);
pub const DWRITE_PANOSE_LETTERFORM_NO_FIT: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(1i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(11i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(9i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(12i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(14i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(13i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(15i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(10i32);
pub const DWRITE_PANOSE_LINING_ANY: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(0i32);
pub const DWRITE_PANOSE_LINING_BACKDROP: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(8i32);
pub const DWRITE_PANOSE_LINING_ENGRAVED: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(5i32);
pub const DWRITE_PANOSE_LINING_INLINE: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(3i32);
pub const DWRITE_PANOSE_LINING_NONE: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(2i32);
pub const DWRITE_PANOSE_LINING_NO_FIT: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(1i32);
pub const DWRITE_PANOSE_LINING_OUTLINE: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(4i32);
pub const DWRITE_PANOSE_LINING_RELIEF: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(7i32);
pub const DWRITE_PANOSE_LINING_SHADOW: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(6i32);
pub const DWRITE_PANOSE_MIDLINE_ANY: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(0i32);
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(9i32);
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(10i32);
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(8i32);
pub const DWRITE_PANOSE_MIDLINE_HIGH_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(6i32);
pub const DWRITE_PANOSE_MIDLINE_HIGH_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(7i32);
pub const DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(5i32);
pub const DWRITE_PANOSE_MIDLINE_LOW_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(12i32);
pub const DWRITE_PANOSE_MIDLINE_LOW_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(13i32);
pub const DWRITE_PANOSE_MIDLINE_LOW_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(11i32);
pub const DWRITE_PANOSE_MIDLINE_NO_FIT: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(1i32);
pub const DWRITE_PANOSE_MIDLINE_STANDARD_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(3i32);
pub const DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(4i32);
pub const DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(2i32);
pub const DWRITE_PANOSE_PROPORTION_ANY: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(0i32);
pub const DWRITE_PANOSE_PROPORTION_CONDENSED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(6i32);
pub const DWRITE_PANOSE_PROPORTION_EVEN_WIDTH: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(4i32);
pub const DWRITE_PANOSE_PROPORTION_EXPANDED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(5i32);
pub const DWRITE_PANOSE_PROPORTION_MODERN: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(3i32);
pub const DWRITE_PANOSE_PROPORTION_MONOSPACED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(9i32);
pub const DWRITE_PANOSE_PROPORTION_NO_FIT: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(1i32);
pub const DWRITE_PANOSE_PROPORTION_OLD_STYLE: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(2i32);
pub const DWRITE_PANOSE_PROPORTION_VERY_CONDENSED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(8i32);
pub const DWRITE_PANOSE_PROPORTION_VERY_EXPANDED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(7i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_ANY: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(0i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(13i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(12i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(10i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(11i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_NO_FIT: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(1i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(9i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(8i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(6i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(7i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(5i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(4i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(2i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(3i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(0i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(10i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(8i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(9i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(7i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(5i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(6i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(1i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(4i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(2i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(3i32);
pub const DWRITE_PANOSE_SERIF_STYLE_ANY: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(0i32);
pub const DWRITE_PANOSE_SERIF_STYLE_BONE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(8i32);
pub const DWRITE_PANOSE_SERIF_STYLE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(2i32);
pub const DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(9i32);
pub const DWRITE_PANOSE_SERIF_STYLE_FLARED: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(14i32);
pub const DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(11i32);
pub const DWRITE_PANOSE_SERIF_STYLE_NO_FIT: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(1i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(3i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(12i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(5i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OVAL: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(8i32);
pub const DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(13i32);
pub const DWRITE_PANOSE_SERIF_STYLE_PERP_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(13i32);
pub const DWRITE_PANOSE_SERIF_STYLE_ROUNDED: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(15i32);
pub const DWRITE_PANOSE_SERIF_STYLE_SCRIPT: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(16i32);
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(6i32);
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(4i32);
pub const DWRITE_PANOSE_SERIF_STYLE_THIN: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(7i32);
pub const DWRITE_PANOSE_SERIF_STYLE_TRIANGLE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(10i32);
pub const DWRITE_PANOSE_SPACING_ANY: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(0i32);
pub const DWRITE_PANOSE_SPACING_MONOSPACED: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(3i32);
pub const DWRITE_PANOSE_SPACING_NO_FIT: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(1i32);
pub const DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(2i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_ANY: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(0i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(3i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(6i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(4i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(5i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(10i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(9i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_FIT: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(1i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(2i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(8i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(7i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(0i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(3i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(8i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(7i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(1i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(2i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(4i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(9i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(5i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(6i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_ANY: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(0i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_BOARDERS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(9i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_EXPERT: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(7i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_ICONS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(10i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(12i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_LOGOS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(11i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_MONTAGES: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(2i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_MUSIC: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(6i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_NO_FIT: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(1i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_PATTERNS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(8i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_PICTURES: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(3i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(5i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_SHAPES: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(4i32);
pub const DWRITE_PANOSE_TOOL_KIND_ANY: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(0i32);
pub const DWRITE_PANOSE_TOOL_KIND_BALL: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(5i32);
pub const DWRITE_PANOSE_TOOL_KIND_BRUSH: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(6i32);
pub const DWRITE_PANOSE_TOOL_KIND_ENGRAVED: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(4i32);
pub const DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(8i32);
pub const DWRITE_PANOSE_TOOL_KIND_FLAT_NIB: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(2i32);
pub const DWRITE_PANOSE_TOOL_KIND_NO_FIT: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(1i32);
pub const DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(3i32);
pub const DWRITE_PANOSE_TOOL_KIND_ROUGH: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(7i32);
pub const DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(9i32);
pub const DWRITE_PANOSE_WEIGHT_ANY: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(0i32);
pub const DWRITE_PANOSE_WEIGHT_BLACK: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(10i32);
pub const DWRITE_PANOSE_WEIGHT_BOLD: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(8i32);
pub const DWRITE_PANOSE_WEIGHT_BOOK: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(5i32);
pub const DWRITE_PANOSE_WEIGHT_DEMI: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(7i32);
pub const DWRITE_PANOSE_WEIGHT_EXTRA_BLACK: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(11i32);
pub const DWRITE_PANOSE_WEIGHT_HEAVY: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(9i32);
pub const DWRITE_PANOSE_WEIGHT_LIGHT: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(3i32);
pub const DWRITE_PANOSE_WEIGHT_MEDIUM: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(6i32);
pub const DWRITE_PANOSE_WEIGHT_NORD: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(11i32);
pub const DWRITE_PANOSE_WEIGHT_NO_FIT: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(1i32);
pub const DWRITE_PANOSE_WEIGHT_THIN: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(4i32);
pub const DWRITE_PANOSE_WEIGHT_VERY_LIGHT: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(2i32);
pub const DWRITE_PANOSE_XASCENT_ANY: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(0i32);
pub const DWRITE_PANOSE_XASCENT_HIGH: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(5i32);
pub const DWRITE_PANOSE_XASCENT_LOW: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(3i32);
pub const DWRITE_PANOSE_XASCENT_MEDIUM: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(4i32);
pub const DWRITE_PANOSE_XASCENT_NO_FIT: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(1i32);
pub const DWRITE_PANOSE_XASCENT_VERY_HIGH: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(6i32);
pub const DWRITE_PANOSE_XASCENT_VERY_LOW: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(2i32);
pub const DWRITE_PANOSE_XHEIGHT_ANY: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(0i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(4i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(2i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(3i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(3i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(7i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(5i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(6i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(6i32);
pub const DWRITE_PANOSE_XHEIGHT_NO_FIT: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(1i32);
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(2i32);
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(1i32);
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(0i32);
pub const DWRITE_PIXEL_GEOMETRY_BGR: DWRITE_PIXEL_GEOMETRY = DWRITE_PIXEL_GEOMETRY(2i32);
pub const DWRITE_PIXEL_GEOMETRY_FLAT: DWRITE_PIXEL_GEOMETRY = DWRITE_PIXEL_GEOMETRY(0i32);
pub const DWRITE_PIXEL_GEOMETRY_RGB: DWRITE_PIXEL_GEOMETRY = DWRITE_PIXEL_GEOMETRY(1i32);
pub const DWRITE_READING_DIRECTION_BOTTOM_TO_TOP: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(3i32);
pub const DWRITE_READING_DIRECTION_LEFT_TO_RIGHT: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(0i32);
pub const DWRITE_READING_DIRECTION_RIGHT_TO_LEFT: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(1i32);
pub const DWRITE_READING_DIRECTION_TOP_TO_BOTTOM: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(2i32);
pub const DWRITE_RENDERING_MODE1_ALIASED: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(1i32);
pub const DWRITE_RENDERING_MODE1_DEFAULT: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(0i32);
pub const DWRITE_RENDERING_MODE1_GDI_CLASSIC: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(2i32);
pub const DWRITE_RENDERING_MODE1_GDI_NATURAL: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(3i32);
pub const DWRITE_RENDERING_MODE1_NATURAL: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(4i32);
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(5i32);
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(7i32);
pub const DWRITE_RENDERING_MODE1_OUTLINE: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(6i32);
pub const DWRITE_RENDERING_MODE_ALIASED: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(1i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(2i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(3i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(4i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(5i32);
pub const DWRITE_RENDERING_MODE_DEFAULT: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(0i32);
pub const DWRITE_RENDERING_MODE_GDI_CLASSIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(2i32);
pub const DWRITE_RENDERING_MODE_GDI_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(3i32);
pub const DWRITE_RENDERING_MODE_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(4i32);
pub const DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(5i32);
pub const DWRITE_RENDERING_MODE_OUTLINE: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(6i32);
pub const DWRITE_SCRIPT_SHAPES_DEFAULT: DWRITE_SCRIPT_SHAPES = DWRITE_SCRIPT_SHAPES(0i32);
pub const DWRITE_SCRIPT_SHAPES_NO_VISUAL: DWRITE_SCRIPT_SHAPES = DWRITE_SCRIPT_SHAPES(1i32);
pub const DWRITE_STANDARD_FONT_AXIS_COUNT: u32 = 5u32;
pub const DWRITE_TEXTURE_ALIASED_1x1: DWRITE_TEXTURE_TYPE = DWRITE_TEXTURE_TYPE(0i32);
pub const DWRITE_TEXTURE_CLEARTYPE_3x1: DWRITE_TEXTURE_TYPE = DWRITE_TEXTURE_TYPE(1i32);
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(2i32);
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(3i32);
pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(0i32);
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(1i32);
pub const DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE: DWRITE_TEXT_ANTIALIAS_MODE = DWRITE_TEXT_ANTIALIAS_MODE(0i32);
pub const DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE: DWRITE_TEXT_ANTIALIAS_MODE = DWRITE_TEXT_ANTIALIAS_MODE(1i32);
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(1i32);
pub const DWRITE_TRIMMING_GRANULARITY_NONE: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(0i32);
pub const DWRITE_TRIMMING_GRANULARITY_WORD: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(2i32);
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT: DWRITE_VERTICAL_GLYPH_ORIENTATION = DWRITE_VERTICAL_GLYPH_ORIENTATION(0i32);
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED: DWRITE_VERTICAL_GLYPH_ORIENTATION = DWRITE_VERTICAL_GLYPH_ORIENTATION(1i32);
pub const DWRITE_WORD_WRAPPING_CHARACTER: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(4i32);
pub const DWRITE_WORD_WRAPPING_EMERGENCY_BREAK: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(2i32);
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(1i32);
pub const DWRITE_WORD_WRAPPING_WHOLE_WORD: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(3i32);
pub const DWRITE_WORD_WRAPPING_WRAP: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(0i32);
pub const FACILITY_DWRITE: u32 = 2200u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_AUTOMATIC_FONT_AXES(pub i32);
impl windows_core::TypeKind for DWRITE_AUTOMATIC_FONT_AXES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_AUTOMATIC_FONT_AXES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_AUTOMATIC_FONT_AXES").field(&self.0).finish()
    }
}
impl DWRITE_AUTOMATIC_FONT_AXES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWRITE_AUTOMATIC_FONT_AXES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWRITE_AUTOMATIC_FONT_AXES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_BASELINE(pub i32);
impl windows_core::TypeKind for DWRITE_BASELINE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_BASELINE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_BASELINE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_BREAK_CONDITION(pub i32);
impl windows_core::TypeKind for DWRITE_BREAK_CONDITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_BREAK_CONDITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_BREAK_CONDITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_COLOR_COMPOSITE_MODE(pub i32);
impl windows_core::TypeKind for DWRITE_COLOR_COMPOSITE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_COLOR_COMPOSITE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_COLOR_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_CONTAINER_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_CONTAINER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_CONTAINER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_CONTAINER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FACTORY_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_FACTORY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FACTORY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FACTORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FLOW_DIRECTION(pub i32);
impl windows_core::TypeKind for DWRITE_FLOW_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FLOW_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FLOW_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_AXIS_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_AXIS_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_AXIS_ATTRIBUTES").field(&self.0).finish()
    }
}
impl DWRITE_FONT_AXIS_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_AXIS_TAG(pub u32);
impl windows_core::TypeKind for DWRITE_FONT_AXIS_TAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_AXIS_TAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_AXIS_TAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_FACE_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_FACE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_FACE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FACE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_FAMILY_MODEL(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_FAMILY_MODEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_FAMILY_MODEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FAMILY_MODEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_FEATURE_TAG(pub u32);
impl windows_core::TypeKind for DWRITE_FONT_FEATURE_TAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_FEATURE_TAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FEATURE_TAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_FILE_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_LINE_GAP_USAGE(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_LINE_GAP_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_LINE_GAP_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_LINE_GAP_USAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_SIMULATIONS(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_SIMULATIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_SIMULATIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_SIMULATIONS").field(&self.0).finish()
    }
}
impl DWRITE_FONT_SIMULATIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWRITE_FONT_SIMULATIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWRITE_FONT_SIMULATIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_SOURCE_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_SOURCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_SOURCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_STRETCH(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_STRETCH {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_STRETCH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_STRETCH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_STYLE(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_FONT_WEIGHT(pub i32);
impl windows_core::TypeKind for DWRITE_FONT_WEIGHT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_FONT_WEIGHT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_WEIGHT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_GLYPH_IMAGE_FORMATS(pub i32);
impl windows_core::TypeKind for DWRITE_GLYPH_IMAGE_FORMATS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_GLYPH_IMAGE_FORMATS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_GLYPH_IMAGE_FORMATS").field(&self.0).finish()
    }
}
impl DWRITE_GLYPH_IMAGE_FORMATS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWRITE_GLYPH_IMAGE_FORMATS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWRITE_GLYPH_IMAGE_FORMATS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_GLYPH_ORIENTATION_ANGLE(pub i32);
impl windows_core::TypeKind for DWRITE_GLYPH_ORIENTATION_ANGLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_GLYPH_ORIENTATION_ANGLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_GLYPH_ORIENTATION_ANGLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_GRID_FIT_MODE(pub i32);
impl windows_core::TypeKind for DWRITE_GRID_FIT_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_GRID_FIT_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_GRID_FIT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_INFORMATIONAL_STRING_ID(pub i32);
impl windows_core::TypeKind for DWRITE_INFORMATIONAL_STRING_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_INFORMATIONAL_STRING_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_INFORMATIONAL_STRING_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_LINE_SPACING_METHOD(pub i32);
impl windows_core::TypeKind for DWRITE_LINE_SPACING_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_LINE_SPACING_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_LINE_SPACING_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_LOCALITY(pub i32);
impl windows_core::TypeKind for DWRITE_LOCALITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_LOCALITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_LOCALITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_MEASURING_MODE(pub i32);
impl windows_core::TypeKind for DWRITE_MEASURING_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_MEASURING_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_MEASURING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_NUMBER_SUBSTITUTION_METHOD(pub i32);
impl windows_core::TypeKind for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_NUMBER_SUBSTITUTION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_OPTICAL_ALIGNMENT(pub i32);
impl windows_core::TypeKind for DWRITE_OPTICAL_ALIGNMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_OPTICAL_ALIGNMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_OPTICAL_ALIGNMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_OUTLINE_THRESHOLD(pub i32);
impl windows_core::TypeKind for DWRITE_OUTLINE_THRESHOLD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_OUTLINE_THRESHOLD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_OUTLINE_THRESHOLD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PAINT_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for DWRITE_PAINT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PAINT_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PAINT_ATTRIBUTES").field(&self.0).finish()
    }
}
impl DWRITE_PAINT_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWRITE_PAINT_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWRITE_PAINT_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWRITE_PAINT_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWRITE_PAINT_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWRITE_PAINT_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PAINT_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_PAINT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PAINT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PAINT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_ARM_STYLE(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_ARM_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_ARM_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ARM_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_ASPECT(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_ASPECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_ASPECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ASPECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_ASPECT_RATIO(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_ASPECT_RATIO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_ASPECT_RATIO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ASPECT_RATIO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_CHARACTER_RANGES(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_CHARACTER_RANGES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_CHARACTER_RANGES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_CHARACTER_RANGES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_CONTRAST(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_CONTRAST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_CONTRAST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_CONTRAST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_DECORATIVE_CLASS(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_DECORATIVE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_DECORATIVE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_DECORATIVE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_DECORATIVE_TOPOLOGY(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_DECORATIVE_TOPOLOGY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_FAMILY(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_FAMILY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_FAMILY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FAMILY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_FILL(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_FILL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_FILL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FILL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_FINIALS(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_FINIALS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_FINIALS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FINIALS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_LETTERFORM(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_LETTERFORM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_LETTERFORM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_LETTERFORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_LINING(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_LINING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_LINING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_LINING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_MIDLINE(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_MIDLINE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_MIDLINE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_MIDLINE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_PROPORTION(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_PROPORTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_PROPORTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_PROPORTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_SCRIPT_FORM(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_SCRIPT_FORM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_SCRIPT_FORM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SCRIPT_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_SCRIPT_TOPOLOGY(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SCRIPT_TOPOLOGY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_SERIF_STYLE(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_SERIF_STYLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_SERIF_STYLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SERIF_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_SPACING(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_SPACING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_SPACING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SPACING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_STROKE_VARIATION(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_STROKE_VARIATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_STROKE_VARIATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_STROKE_VARIATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SYMBOL_ASPECT_RATIO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_SYMBOL_KIND(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_SYMBOL_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_SYMBOL_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SYMBOL_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_TOOL_KIND(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_TOOL_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_TOOL_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_TOOL_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_WEIGHT(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_WEIGHT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_WEIGHT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_WEIGHT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_XASCENT(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_XASCENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_XASCENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_XASCENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PANOSE_XHEIGHT(pub i32);
impl windows_core::TypeKind for DWRITE_PANOSE_XHEIGHT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PANOSE_XHEIGHT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_XHEIGHT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PARAGRAPH_ALIGNMENT(pub i32);
impl windows_core::TypeKind for DWRITE_PARAGRAPH_ALIGNMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PARAGRAPH_ALIGNMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PARAGRAPH_ALIGNMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_PIXEL_GEOMETRY(pub i32);
impl windows_core::TypeKind for DWRITE_PIXEL_GEOMETRY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_PIXEL_GEOMETRY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_PIXEL_GEOMETRY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_READING_DIRECTION(pub i32);
impl windows_core::TypeKind for DWRITE_READING_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_READING_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_READING_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_RENDERING_MODE(pub i32);
impl windows_core::TypeKind for DWRITE_RENDERING_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_RENDERING_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_RENDERING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_RENDERING_MODE1(pub i32);
impl windows_core::TypeKind for DWRITE_RENDERING_MODE1 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_RENDERING_MODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_RENDERING_MODE1").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_SCRIPT_SHAPES(pub i32);
impl windows_core::TypeKind for DWRITE_SCRIPT_SHAPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_SCRIPT_SHAPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_SCRIPT_SHAPES").field(&self.0).finish()
    }
}
impl DWRITE_SCRIPT_SHAPES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWRITE_SCRIPT_SHAPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWRITE_SCRIPT_SHAPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_TEXTURE_TYPE(pub i32);
impl windows_core::TypeKind for DWRITE_TEXTURE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_TEXTURE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_TEXTURE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_TEXT_ALIGNMENT(pub i32);
impl windows_core::TypeKind for DWRITE_TEXT_ALIGNMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_TEXT_ALIGNMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_TEXT_ALIGNMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_TEXT_ANTIALIAS_MODE(pub i32);
impl windows_core::TypeKind for DWRITE_TEXT_ANTIALIAS_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_TEXT_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_TEXT_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_TRIMMING_GRANULARITY(pub i32);
impl windows_core::TypeKind for DWRITE_TRIMMING_GRANULARITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_TRIMMING_GRANULARITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_TRIMMING_GRANULARITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_VERTICAL_GLYPH_ORIENTATION(pub i32);
impl windows_core::TypeKind for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_VERTICAL_GLYPH_ORIENTATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWRITE_WORD_WRAPPING(pub i32);
impl windows_core::TypeKind for DWRITE_WORD_WRAPPING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWRITE_WORD_WRAPPING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWRITE_WORD_WRAPPING").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_BITMAP_DATA_BGRA32 {
    pub width: u32,
    pub height: u32,
    pub pixels: *mut u32,
}
impl windows_core::TypeKind for DWRITE_BITMAP_DATA_BGRA32 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_BITMAP_DATA_BGRA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_CARET_METRICS {
    pub slopeRise: i16,
    pub slopeRun: i16,
    pub offset: i16,
}
impl windows_core::TypeKind for DWRITE_CARET_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_CARET_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
impl windows_core::TypeKind for DWRITE_CLUSTER_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_CLUSTER_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl windows_core::TypeKind for DWRITE_COLOR_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_COLOR_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: f32,
    pub baselineOriginY: f32,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: u16,
}
impl Clone for DWRITE_COLOR_GLYPH_RUN {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for DWRITE_COLOR_GLYPH_RUN {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_COLOR_GLYPH_RUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DWRITE_COLOR_GLYPH_RUN1 {
    pub Base: DWRITE_COLOR_GLYPH_RUN,
    pub glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl Clone for DWRITE_COLOR_GLYPH_RUN1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for DWRITE_COLOR_GLYPH_RUN1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_COLOR_GLYPH_RUN1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_FILE_FRAGMENT {
    pub fileOffset: u64,
    pub fragmentSize: u64,
}
impl windows_core::TypeKind for DWRITE_FILE_FRAGMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FILE_FRAGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_FONT_AXIS_RANGE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub minValue: f32,
    pub maxValue: f32,
}
impl windows_core::TypeKind for DWRITE_FONT_AXIS_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FONT_AXIS_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_FONT_AXIS_VALUE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub value: f32,
}
impl windows_core::TypeKind for DWRITE_FONT_AXIS_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FONT_AXIS_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: u32,
}
impl windows_core::TypeKind for DWRITE_FONT_FEATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FONT_FEATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: u16,
    pub ascent: u16,
    pub descent: u16,
    pub lineGap: i16,
    pub capHeight: u16,
    pub xHeight: u16,
    pub underlinePosition: i16,
    pub underlineThickness: u16,
    pub strikethroughPosition: i16,
    pub strikethroughThickness: u16,
}
impl windows_core::TypeKind for DWRITE_FONT_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FONT_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_FONT_METRICS1 {
    pub Base: DWRITE_FONT_METRICS,
    pub glyphBoxLeft: i16,
    pub glyphBoxTop: i16,
    pub glyphBoxRight: i16,
    pub glyphBoxBottom: i16,
    pub subscriptPositionX: i16,
    pub subscriptPositionY: i16,
    pub subscriptSizeX: i16,
    pub subscriptSizeY: i16,
    pub superscriptPositionX: i16,
    pub superscriptPositionY: i16,
    pub superscriptSizeX: i16,
    pub superscriptSizeY: i16,
    pub hasTypographicMetrics: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DWRITE_FONT_METRICS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FONT_METRICS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_FONT_PROPERTY {
    pub propertyId: DWRITE_FONT_PROPERTY_ID,
    pub propertyValue: windows_core::PCWSTR,
    pub localeName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for DWRITE_FONT_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_FONT_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_GLYPH_IMAGE_DATA {
    pub imageData: *const core::ffi::c_void,
    pub imageDataSize: u32,
    pub uniqueDataId: u32,
    pub pixelsPerEm: u32,
    pub pixelSize: super::Direct2D::Common::D2D_SIZE_U,
    pub horizontalLeftOrigin: super::super::Foundation::POINT,
    pub horizontalRightOrigin: super::super::Foundation::POINT,
    pub verticalTopOrigin: super::super::Foundation::POINT,
    pub verticalBottomOrigin: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_GLYPH_IMAGE_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_GLYPH_IMAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: i32,
    pub advanceWidth: u32,
    pub rightSideBearing: i32,
    pub topSideBearing: i32,
    pub advanceHeight: u32,
    pub bottomSideBearing: i32,
    pub verticalOriginY: i32,
}
impl windows_core::TypeKind for DWRITE_GLYPH_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_GLYPH_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
impl windows_core::TypeKind for DWRITE_GLYPH_OFFSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_GLYPH_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: core::mem::ManuallyDrop<Option<IDWriteFontFace>>,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *const u16,
    pub glyphAdvances: *const f32,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: super::super::Foundation::BOOL,
    pub bidiLevel: u32,
}
impl Clone for DWRITE_GLYPH_RUN {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for DWRITE_GLYPH_RUN {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_GLYPH_RUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: windows_core::PCWSTR,
    pub string: windows_core::PCWSTR,
    pub stringLength: u32,
    pub clusterMap: *const u16,
    pub textPosition: u32,
}
impl windows_core::TypeKind for DWRITE_GLYPH_RUN_DESCRIPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: super::super::Foundation::BOOL,
    pub isTrimmed: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DWRITE_HIT_TEST_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_HIT_TEST_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supportsSideways: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DWRITE_INLINE_OBJECT_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_INLINE_OBJECT_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_JUSTIFICATION_OPPORTUNITY {
    pub expansionMinimum: f32,
    pub expansionMaximum: f32,
    pub compressionMaximum: f32,
    pub _bitfield: u32,
}
impl windows_core::TypeKind for DWRITE_JUSTIFICATION_OPPORTUNITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_LINE_BREAKPOINT {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DWRITE_LINE_BREAKPOINT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_LINE_BREAKPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DWRITE_LINE_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_LINE_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_LINE_METRICS1 {
    pub Base: DWRITE_LINE_METRICS,
    pub leadingBefore: f32,
    pub leadingAfter: f32,
}
impl windows_core::TypeKind for DWRITE_LINE_METRICS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_LINE_METRICS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_LINE_SPACING {
    pub method: DWRITE_LINE_SPACING_METHOD,
    pub height: f32,
    pub baseline: f32,
    pub leadingBefore: f32,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
impl windows_core::TypeKind for DWRITE_LINE_SPACING {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_LINE_SPACING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl windows_core::TypeKind for DWRITE_MATRIX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_MATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl windows_core::TypeKind for DWRITE_OVERHANG_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_OVERHANG_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_PAINT_COLOR {
    pub value: DWRITE_COLOR_F,
    pub paletteEntryIndex: u16,
    pub alphaMultiplier: f32,
    pub colorAttributes: DWRITE_PAINT_ATTRIBUTES,
}
impl windows_core::TypeKind for DWRITE_PAINT_COLOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_PAINT_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy)]
pub struct DWRITE_PAINT_ELEMENT {
    pub paintType: DWRITE_PAINT_TYPE,
    pub paint: DWRITE_PAINT_ELEMENT_0,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy)]
pub union DWRITE_PAINT_ELEMENT_0 {
    pub layers: DWRITE_PAINT_ELEMENT_0_3,
    pub solidGlyph: DWRITE_PAINT_ELEMENT_0_6,
    pub solid: DWRITE_PAINT_COLOR,
    pub linearGradient: DWRITE_PAINT_ELEMENT_0_4,
    pub radialGradient: DWRITE_PAINT_ELEMENT_0_5,
    pub sweepGradient: DWRITE_PAINT_ELEMENT_0_7,
    pub glyph: DWRITE_PAINT_ELEMENT_0_2,
    pub colorGlyph: DWRITE_PAINT_ELEMENT_0_0,
    pub transform: DWRITE_MATRIX,
    pub composite: DWRITE_PAINT_ELEMENT_0_1,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_0 {
    pub glyphIndex: u32,
    pub clipBox: super::Direct2D::Common::D2D_RECT_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_1 {
    pub mode: DWRITE_COLOR_COMPOSITE_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_2 {
    pub glyphIndex: u32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_3 {
    pub childCount: u32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_4 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_5 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub x0: f32,
    pub y0: f32,
    pub radius0: f32,
    pub x1: f32,
    pub y1: f32,
    pub radius1: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_6 {
    pub glyphIndex: u32,
    pub color: DWRITE_PAINT_COLOR,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_7 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub centerX: f32,
    pub centerY: f32,
    pub startAngle: f32,
    pub endAngle: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for DWRITE_PAINT_ELEMENT_0_7 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for DWRITE_PAINT_ELEMENT_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DWRITE_PANOSE {
    pub values: [u8; 10],
    pub familyKind: u8,
    pub text: DWRITE_PANOSE_3,
    pub script: DWRITE_PANOSE_1,
    pub decorative: DWRITE_PANOSE_0,
    pub symbol: DWRITE_PANOSE_2,
}
impl windows_core::TypeKind for DWRITE_PANOSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_PANOSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PANOSE_0 {
    pub familyKind: u8,
    pub decorativeClass: u8,
    pub weight: u8,
    pub aspect: u8,
    pub contrast: u8,
    pub serifVariant: u8,
    pub fill: u8,
    pub lining: u8,
    pub decorativeTopology: u8,
    pub characterRange: u8,
}
impl windows_core::TypeKind for DWRITE_PANOSE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_PANOSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PANOSE_1 {
    pub familyKind: u8,
    pub toolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatio: u8,
    pub contrast: u8,
    pub scriptTopology: u8,
    pub scriptForm: u8,
    pub finials: u8,
    pub xAscent: u8,
}
impl windows_core::TypeKind for DWRITE_PANOSE_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_PANOSE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PANOSE_2 {
    pub familyKind: u8,
    pub symbolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatioAndContrast: u8,
    pub aspectRatio94: u8,
    pub aspectRatio119: u8,
    pub aspectRatio157: u8,
    pub aspectRatio163: u8,
    pub aspectRatio211: u8,
}
impl windows_core::TypeKind for DWRITE_PANOSE_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_PANOSE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_PANOSE_3 {
    pub familyKind: u8,
    pub serifStyle: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub strokeVariation: u8,
    pub armStyle: u8,
    pub letterform: u8,
    pub midline: u8,
    pub xHeight: u8,
}
impl windows_core::TypeKind for DWRITE_PANOSE_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_PANOSE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: u16,
    pub shapes: DWRITE_SCRIPT_SHAPES,
}
impl windows_core::TypeKind for DWRITE_SCRIPT_ANALYSIS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_SCRIPT_ANALYSIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_SCRIPT_PROPERTIES {
    pub isoScriptCode: u32,
    pub isoScriptNumber: u32,
    pub clusterLookahead: u32,
    pub justificationCharacter: u32,
    pub _bitfield: u32,
}
impl windows_core::TypeKind for DWRITE_SCRIPT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_SCRIPT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    pub _bitfield: u16,
}
impl windows_core::TypeKind for DWRITE_SHAPING_GLYPH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    pub _bitfield: u16,
}
impl windows_core::TypeKind for DWRITE_SHAPING_TEXT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_STRIKETHROUGH {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: windows_core::PCWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl windows_core::TypeKind for DWRITE_STRIKETHROUGH {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_STRIKETHROUGH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}
impl windows_core::TypeKind for DWRITE_TEXT_METRICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_TEXT_METRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_TEXT_METRICS1 {
    pub Base: DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: f32,
}
impl windows_core::TypeKind for DWRITE_TEXT_METRICS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_TEXT_METRICS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
impl windows_core::TypeKind for DWRITE_TEXT_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_TEXT_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: u32,
    pub delimiterCount: u32,
}
impl windows_core::TypeKind for DWRITE_TRIMMING {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_TRIMMING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_TYPOGRAPHIC_FEATURES {
    pub features: *mut DWRITE_FONT_FEATURE,
    pub featureCount: u32,
}
impl windows_core::TypeKind for DWRITE_TYPOGRAPHIC_FEATURES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_TYPOGRAPHIC_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_UNDERLINE {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: windows_core::PCWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl windows_core::TypeKind for DWRITE_UNDERLINE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_UNDERLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWRITE_UNICODE_RANGE {
    pub first: u32,
    pub last: u32,
}
impl windows_core::TypeKind for DWRITE_UNICODE_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWRITE_UNICODE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

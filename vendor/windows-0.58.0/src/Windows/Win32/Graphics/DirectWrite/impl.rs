pub trait IDWriteAsyncResult_Impl: Sized {
    fn GetWaitHandle(&self) -> super::super::Foundation::HANDLE;
    fn GetResult(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteAsyncResult {}
impl IDWriteAsyncResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteAsyncResult_Vtbl
    where
        Identity: IDWriteAsyncResult_Impl,
    {
        unsafe extern "system" fn GetWaitHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE
        where
            Identity: IDWriteAsyncResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteAsyncResult_Impl::GetWaitHandle(this)
        }
        unsafe extern "system" fn GetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteAsyncResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteAsyncResult_Impl::GetResult(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWaitHandle: GetWaitHandle::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteAsyncResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteBitmapRenderTarget_Impl: Sized {
    fn DrawGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: Option<&IDWriteRenderingParams>, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetMemoryDC(&self) -> super::Gdi::HDC;
    fn GetPixelsPerDip(&self) -> f32;
    fn SetPixelsPerDip(&self, pixelsperdip: f32) -> windows_core::Result<()>;
    fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>;
    fn SetCurrentTransform(&self, transform: *const DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn Resize(&self, width: u32, height: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteBitmapRenderTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget_Vtbl
    where
        Identity: IDWriteBitmapRenderTarget_Impl,
    {
        unsafe extern "system" fn DrawGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut core::ffi::c_void, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::DrawGlyphRun(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), windows_core::from_raw_borrowed(&renderingparams), core::mem::transmute_copy(&textcolor), core::mem::transmute_copy(&blackboxrect)).into()
        }
        unsafe extern "system" fn GetMemoryDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::Gdi::HDC
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::GetMemoryDC(this)
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::GetPixelsPerDip(this)
        }
        unsafe extern "system" fn SetPixelsPerDip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelsperdip: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::SetPixelsPerDip(this, core::mem::transmute_copy(&pixelsperdip)).into()
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::GetCurrentTransform(this, core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn SetCurrentTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const DWRITE_MATRIX) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::SetCurrentTransform(this, core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteBitmapRenderTarget_Impl::GetSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget_Impl::Resize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            GetMemoryDC: GetMemoryDC::<Identity, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, OFFSET>,
            SetPixelsPerDip: SetPixelsPerDip::<Identity, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, OFFSET>,
            SetCurrentTransform: SetCurrentTransform::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteBitmapRenderTarget1_Impl: Sized + IDWriteBitmapRenderTarget_Impl {
    fn GetTextAntialiasMode(&self) -> DWRITE_TEXT_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(&self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget1 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteBitmapRenderTarget1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget1_Vtbl
    where
        Identity: IDWriteBitmapRenderTarget1_Impl,
    {
        unsafe extern "system" fn GetTextAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE
        where
            Identity: IDWriteBitmapRenderTarget1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget1_Impl::GetTextAntialiasMode(this)
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget1_Impl::SetTextAntialiasMode(this, core::mem::transmute_copy(&antialiasmode)).into()
        }
        Self {
            base__: IDWriteBitmapRenderTarget_Vtbl::new::<Identity, OFFSET>(),
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget1 as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteBitmapRenderTarget2_Impl: Sized + IDWriteBitmapRenderTarget1_Impl {
    fn GetBitmapData(&self) -> windows_core::Result<DWRITE_BITMAP_DATA_BGRA32>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteBitmapRenderTarget2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget2_Vtbl
    where
        Identity: IDWriteBitmapRenderTarget2_Impl,
    {
        unsafe extern "system" fn GetBitmapData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapdata: *mut DWRITE_BITMAP_DATA_BGRA32) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteBitmapRenderTarget2_Impl::GetBitmapData(this) {
                Ok(ok__) => {
                    bitmapdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDWriteBitmapRenderTarget1_Vtbl::new::<Identity, OFFSET>(), GetBitmapData: GetBitmapData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget2 as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteBitmapRenderTarget3_Impl: Sized + IDWriteBitmapRenderTarget2_Impl {
    fn GetPaintFeatureLevel(&self) -> DWRITE_PAINT_FEATURE_LEVEL;
    fn DrawPaintGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, textcolor: super::super::Foundation::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn DrawGlyphRunWithColorSupport(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: Option<&IDWriteRenderingParams>, textcolor: super::super::Foundation::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget3 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteBitmapRenderTarget3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteBitmapRenderTarget3_Vtbl
    where
        Identity: IDWriteBitmapRenderTarget3_Impl,
    {
        unsafe extern "system" fn GetPaintFeatureLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_PAINT_FEATURE_LEVEL
        where
            Identity: IDWriteBitmapRenderTarget3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget3_Impl::GetPaintFeatureLevel(this)
        }
        unsafe extern "system" fn DrawPaintGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, textcolor: super::super::Foundation::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget3_Impl::DrawPaintGlyphRun(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphimageformat), core::mem::transmute_copy(&textcolor), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&blackboxrect)).into()
        }
        unsafe extern "system" fn DrawGlyphRunWithColorSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut core::ffi::c_void, textcolor: super::super::Foundation::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDWriteBitmapRenderTarget3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteBitmapRenderTarget3_Impl::DrawGlyphRunWithColorSupport(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), windows_core::from_raw_borrowed(&renderingparams), core::mem::transmute_copy(&textcolor), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&blackboxrect)).into()
        }
        Self {
            base__: IDWriteBitmapRenderTarget2_Vtbl::new::<Identity, OFFSET>(),
            GetPaintFeatureLevel: GetPaintFeatureLevel::<Identity, OFFSET>,
            DrawPaintGlyphRun: DrawPaintGlyphRun::<Identity, OFFSET>,
            DrawGlyphRunWithColorSupport: DrawGlyphRunWithColorSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget3 as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget1 as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteColorGlyphRunEnumerator_Impl: Sized {
    fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN>;
}
impl windows_core::RuntimeName for IDWriteColorGlyphRunEnumerator {}
impl IDWriteColorGlyphRunEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteColorGlyphRunEnumerator_Vtbl
    where
        Identity: IDWriteColorGlyphRunEnumerator_Impl,
    {
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasrun: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteColorGlyphRunEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteColorGlyphRunEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasrun.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> windows_core::HRESULT
        where
            Identity: IDWriteColorGlyphRunEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteColorGlyphRunEnumerator_Impl::GetCurrentRun(this) {
                Ok(ok__) => {
                    colorglyphrun.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            GetCurrentRun: GetCurrentRun::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator as windows_core::Interface>::IID
    }
}
pub trait IDWriteColorGlyphRunEnumerator1_Impl: Sized + IDWriteColorGlyphRunEnumerator_Impl {
    fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN1>;
}
impl windows_core::RuntimeName for IDWriteColorGlyphRunEnumerator1 {}
impl IDWriteColorGlyphRunEnumerator1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteColorGlyphRunEnumerator1_Vtbl
    where
        Identity: IDWriteColorGlyphRunEnumerator1_Impl,
    {
        unsafe extern "system" fn GetCurrentRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> windows_core::HRESULT
        where
            Identity: IDWriteColorGlyphRunEnumerator1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteColorGlyphRunEnumerator1_Impl::GetCurrentRun(this) {
                Ok(ok__) => {
                    colorglyphrun.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDWriteColorGlyphRunEnumerator_Vtbl::new::<Identity, OFFSET>(), GetCurrentRun: GetCurrentRun::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator1 as windows_core::Interface>::IID || iid == &<IDWriteColorGlyphRunEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteFactory_Impl: Sized {
    fn GetSystemFontCollection(&self, fontcollection: *mut Option<IDWriteFontCollection>, checkforupdates: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CreateCustomFontCollection(&self, collectionloader: Option<&IDWriteFontCollectionLoader>, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontCollection>;
    fn RegisterFontCollectionLoader(&self, fontcollectionloader: Option<&IDWriteFontCollectionLoader>) -> windows_core::Result<()>;
    fn UnregisterFontCollectionLoader(&self, fontcollectionloader: Option<&IDWriteFontCollectionLoader>) -> windows_core::Result<()>;
    fn CreateFontFileReference(&self, filepath: &windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME) -> windows_core::Result<IDWriteFontFile>;
    fn CreateCustomFontFileReference(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: Option<&IDWriteFontFileLoader>) -> windows_core::Result<IDWriteFontFile>;
    fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const Option<IDWriteFontFile>, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace>;
    fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateMonitorRenderingParams(&self, monitor: super::Gdi::HMONITOR) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams>;
    fn RegisterFontFileLoader(&self, fontfileloader: Option<&IDWriteFontFileLoader>) -> windows_core::Result<()>;
    fn UnregisterFontFileLoader(&self, fontfileloader: Option<&IDWriteFontFileLoader>) -> windows_core::Result<()>;
    fn CreateTextFormat(&self, fontfamilyname: &windows_core::PCWSTR, fontcollection: Option<&IDWriteFontCollection>, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: &windows_core::PCWSTR) -> windows_core::Result<IDWriteTextFormat>;
    fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography>;
    fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop>;
    fn CreateTextLayout(&self, string: &windows_core::PCWSTR, stringlength: u32, textformat: Option<&IDWriteTextFormat>, maxwidth: f32, maxheight: f32) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateGdiCompatibleTextLayout(&self, string: &windows_core::PCWSTR, stringlength: u32, textformat: Option<&IDWriteTextFormat>, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateEllipsisTrimmingSign(&self, textformat: Option<&IDWriteTextFormat>) -> windows_core::Result<IDWriteInlineObject>;
    fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer>;
    fn CreateNumberSubstitution(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: &windows_core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL) -> windows_core::Result<IDWriteNumberSubstitution>;
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteFactory {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory_Vtbl
    where
        Identity: IDWriteFactory_Impl,
    {
        unsafe extern "system" fn GetSystemFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&checkforupdates)).into()
        }
        unsafe extern "system" fn CreateCustomFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectionloader: *mut core::ffi::c_void, collectionkey: *const core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateCustomFontCollection(this, windows_core::from_raw_borrowed(&collectionloader), core::mem::transmute_copy(&collectionkey), core::mem::transmute_copy(&collectionkeysize)) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollectionloader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory_Impl::RegisterFontCollectionLoader(this, windows_core::from_raw_borrowed(&fontcollectionloader)).into()
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollectionloader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory_Impl::UnregisterFontCollectionLoader(this, windows_core::from_raw_borrowed(&fontcollectionloader)).into()
        }
        unsafe extern "system" fn CreateFontFileReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateFontFileReference(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&lastwritetime)) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomFontFileReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateCustomFontFileReference(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize), windows_core::from_raw_borrowed(&fontfileloader)) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const *mut core::ffi::c_void, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateFontFace(this, core::mem::transmute_copy(&fontfacetype), core::mem::transmute_copy(&numberoffiles), core::mem::transmute_copy(&fontfiles), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontfacesimulationflags)) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateRenderingParams(this) {
                Ok(ok__) => {
                    renderingparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: super::Gdi::HMONITOR, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateMonitorRenderingParams(this, core::mem::transmute_copy(&monitor)) {
                Ok(ok__) => {
                    renderingparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode)) {
                Ok(ok__) => {
                    renderingparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterFontFileLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfileloader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory_Impl::RegisterFontFileLoader(this, windows_core::from_raw_borrowed(&fontfileloader)).into()
        }
        unsafe extern "system" fn UnregisterFontFileLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfileloader: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory_Impl::UnregisterFontFileLoader(this, windows_core::from_raw_borrowed(&fontfileloader)).into()
        }
        unsafe extern "system" fn CreateTextFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PCWSTR, fontcollection: *mut core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: windows_core::PCWSTR, textformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateTextFormat(this, core::mem::transmute(&fontfamilyname), windows_core::from_raw_borrowed(&fontcollection), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&fontstyle), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&fontsize), core::mem::transmute(&localename)) {
                Ok(ok__) => {
                    textformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTypography<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, typography: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateTypography(this) {
                Ok(ok__) => {
                    typography.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiInterop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdiinterop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::GetGdiInterop(this) {
                Ok(ok__) => {
                    gdiinterop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: windows_core::PCWSTR, stringlength: u32, textformat: *mut core::ffi::c_void, maxwidth: f32, maxheight: f32, textlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateTextLayout(this, core::mem::transmute(&string), core::mem::transmute_copy(&stringlength), windows_core::from_raw_borrowed(&textformat), core::mem::transmute_copy(&maxwidth), core::mem::transmute_copy(&maxheight)) {
                Ok(ok__) => {
                    textlayout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: windows_core::PCWSTR, stringlength: u32, textformat: *mut core::ffi::c_void, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateGdiCompatibleTextLayout(this, core::mem::transmute(&string), core::mem::transmute_copy(&stringlength), windows_core::from_raw_borrowed(&textformat), core::mem::transmute_copy(&layoutwidth), core::mem::transmute_copy(&layoutheight), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&usegdinatural)) {
                Ok(ok__) => {
                    textlayout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textformat: *mut core::ffi::c_void, trimmingsign: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateEllipsisTrimmingSign(this, windows_core::from_raw_borrowed(&textformat)) {
                Ok(ok__) => {
                    trimmingsign.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextAnalyzer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textanalyzer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateTextAnalyzer(this) {
                Ok(ok__) => {
                    textanalyzer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNumberSubstitution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: windows_core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateNumberSubstitution(this, core::mem::transmute_copy(&substitutionmethod), core::mem::transmute(&localename), core::mem::transmute_copy(&ignoreuseroverride)) {
                Ok(ok__) => {
                    numbersubstitution.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory_Impl::CreateGlyphRunAnalysis(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy)) {
                Ok(ok__) => {
                    glyphrunanalysis.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            CreateCustomFontCollection: CreateCustomFontCollection::<Identity, OFFSET>,
            RegisterFontCollectionLoader: RegisterFontCollectionLoader::<Identity, OFFSET>,
            UnregisterFontCollectionLoader: UnregisterFontCollectionLoader::<Identity, OFFSET>,
            CreateFontFileReference: CreateFontFileReference::<Identity, OFFSET>,
            CreateCustomFontFileReference: CreateCustomFontFileReference::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateRenderingParams: CreateRenderingParams::<Identity, OFFSET>,
            CreateMonitorRenderingParams: CreateMonitorRenderingParams::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            RegisterFontFileLoader: RegisterFontFileLoader::<Identity, OFFSET>,
            UnregisterFontFileLoader: UnregisterFontFileLoader::<Identity, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, OFFSET>,
            CreateTypography: CreateTypography::<Identity, OFFSET>,
            GetGdiInterop: GetGdiInterop::<Identity, OFFSET>,
            CreateTextLayout: CreateTextLayout::<Identity, OFFSET>,
            CreateGdiCompatibleTextLayout: CreateGdiCompatibleTextLayout::<Identity, OFFSET>,
            CreateEllipsisTrimmingSign: CreateEllipsisTrimmingSign::<Identity, OFFSET>,
            CreateTextAnalyzer: CreateTextAnalyzer::<Identity, OFFSET>,
            CreateNumberSubstitution: CreateNumberSubstitution::<Identity, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteFactory1_Impl: Sized + IDWriteFactory_Impl {
    fn GetEudcFontCollection(&self, fontcollection: *mut Option<IDWriteFontCollection>, checkforupdates: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams1>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteFactory1 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteFactory1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory1_Vtbl
    where
        Identity: IDWriteFactory1_Impl,
    {
        unsafe extern "system" fn GetEudcFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory1_Impl::GetEudcFontCollection(this, core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&checkforupdates)).into()
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory1_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&enhancedcontrastgrayscale), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode)) {
                Ok(ok__) => {
                    renderingparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory_Vtbl::new::<Identity, OFFSET>(),
            GetEudcFontCollection: GetEudcFontCollection::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteFactory2_Impl: Sized + IDWriteFactory1_Impl {
    fn GetSystemFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
    fn CreateFontFallbackBuilder(&self) -> windows_core::Result<IDWriteFontFallbackBuilder>;
    fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams2>;
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteFactory2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory2_Vtbl
    where
        Identity: IDWriteFactory2_Impl,
    {
        unsafe extern "system" fn GetSystemFontFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory2_Impl::GetSystemFontFallback(this) {
                Ok(ok__) => {
                    fontfallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFallbackBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallbackbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory2_Impl::CreateFontFallbackBuilder(this) {
                Ok(ok__) => {
                    fontfallbackbuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory2_Impl::TranslateColorGlyphRun(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&worldtodevicetransform), core::mem::transmute_copy(&colorpaletteindex)) {
                Ok(ok__) => {
                    colorlayers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory2_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&grayscaleenhancedcontrast), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)) {
                Ok(ok__) => {
                    renderingparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory2_Impl::CreateGlyphRunAnalysis(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&gridfitmode), core::mem::transmute_copy(&antialiasmode), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy)) {
                Ok(ok__) => {
                    glyphrunanalysis.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory1_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontFallback: GetSystemFontFallback::<Identity, OFFSET>,
            CreateFontFallbackBuilder: CreateFontFallbackBuilder::<Identity, OFFSET>,
            TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteFactory3_Impl: Sized + IDWriteFactory2_Impl {
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams3>;
    fn CreateFontFaceReference(&self, fontfile: Option<&IDWriteFontFile>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>;
    fn CreateFontFaceReference2(&self, filepath: &windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>;
    fn GetSystemFontSet(&self) -> windows_core::Result<IDWriteFontSet>;
    fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder>;
    fn CreateFontCollectionFromFontSet(&self, fontset: Option<&IDWriteFontSet>) -> windows_core::Result<IDWriteFontCollection1>;
    fn GetSystemFontCollection(&self, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut Option<IDWriteFontCollection1>, checkforupdates: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFontDownloadQueue(&self) -> windows_core::Result<IDWriteFontDownloadQueue>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteFactory3 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteFactory3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory3_Vtbl
    where
        Identity: IDWriteFactory3_Impl,
    {
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::CreateGlyphRunAnalysis(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&gridfitmode), core::mem::transmute_copy(&antialiasmode), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy)) {
                Ok(ok__) => {
                    glyphrunanalysis.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&grayscaleenhancedcontrast), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)) {
                Ok(ok__) => {
                    renderingparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::CreateFontFaceReference(this, windows_core::from_raw_borrowed(&fontfile), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontsimulations)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::CreateFontFaceReference2(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&lastwritetime), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontsimulations)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::GetSystemFontSet(this) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsetbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::CreateFontSetBuilder(this) {
                Ok(ok__) => {
                    fontsetbuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::CreateFontCollectionFromFontSet(this, windows_core::from_raw_borrowed(&fontset)) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut *mut core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory3_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&includedownloadablefonts), core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&checkforupdates)).into()
        }
        unsafe extern "system" fn GetFontDownloadQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontdownloadqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory3_Impl::GetFontDownloadQueue(this) {
                Ok(ok__) => {
                    fontdownloadqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory2_Vtbl::new::<Identity, OFFSET>(),
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, OFFSET>,
            CreateFontFaceReference2: CreateFontFaceReference2::<Identity, OFFSET>,
            GetSystemFontSet: GetSystemFontSet::<Identity, OFFSET>,
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, OFFSET>,
            CreateFontCollectionFromFontSet: CreateFontCollectionFromFontSet::<Identity, OFFSET>,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            GetFontDownloadQueue: GetFontDownloadQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory4_Impl: Sized + IDWriteFactory3_Impl {
    fn TranslateColorGlyphRun(&self, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1>;
    fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F) -> windows_core::Result<super::Direct2D::Common::D2D_POINT_2F>;
    fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX) -> windows_core::Result<super::Direct2D::Common::D2D_POINT_2F>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDWriteFactory4 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory4_Vtbl
    where
        Identity: IDWriteFactory4_Impl,
    {
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory4_Impl::TranslateColorGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&desiredglyphimageformats), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&worldanddpitransform), core::mem::transmute_copy(&colorpaletteindex)) {
                Ok(ok__) => {
                    colorlayers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory4_Impl::ComputeGlyphOrigins(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute(&baselineorigin)) {
                Ok(ok__) => {
                    glyphorigins.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory4_Impl::ComputeGlyphOrigins2(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&measuringmode), core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&worldanddpitransform)) {
                Ok(ok__) => {
                    glyphorigins.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory3_Vtbl::new::<Identity, OFFSET>(),
            TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, OFFSET>,
            ComputeGlyphOrigins: ComputeGlyphOrigins::<Identity, OFFSET>,
            ComputeGlyphOrigins2: ComputeGlyphOrigins2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory5_Impl: Sized + IDWriteFactory4_Impl {
    fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder1>;
    fn CreateInMemoryFontFileLoader(&self) -> windows_core::Result<IDWriteInMemoryFontFileLoader>;
    fn CreateHttpFontFileLoader(&self, referrerurl: &windows_core::PCWSTR, extraheaders: &windows_core::PCWSTR) -> windows_core::Result<IDWriteRemoteFontFileLoader>;
    fn AnalyzeContainerType(&self, filedata: *const core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE;
    fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const core::ffi::c_void, filedatasize: u32) -> windows_core::Result<IDWriteFontFileStream>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDWriteFactory5 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory5_Vtbl
    where
        Identity: IDWriteFactory5_Impl,
    {
        unsafe extern "system" fn CreateFontSetBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsetbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory5_Impl::CreateFontSetBuilder(this) {
                Ok(ok__) => {
                    fontsetbuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInMemoryFontFileLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newloader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory5_Impl::CreateInMemoryFontFileLoader(this) {
                Ok(ok__) => {
                    newloader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHttpFontFileLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, referrerurl: windows_core::PCWSTR, extraheaders: windows_core::PCWSTR, newloader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory5_Impl::CreateHttpFontFileLoader(this, core::mem::transmute(&referrerurl), core::mem::transmute(&extraheaders)) {
                Ok(ok__) => {
                    newloader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnalyzeContainerType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filedata: *const core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE
        where
            Identity: IDWriteFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFactory5_Impl::AnalyzeContainerType(this, core::mem::transmute_copy(&filedata), core::mem::transmute_copy(&filedatasize))
        }
        unsafe extern "system" fn UnpackFontFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory5_Impl::UnpackFontFile(this, core::mem::transmute_copy(&containertype), core::mem::transmute_copy(&filedata), core::mem::transmute_copy(&filedatasize)) {
                Ok(ok__) => {
                    unpackedfontstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory4_Vtbl::new::<Identity, OFFSET>(),
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, OFFSET>,
            CreateInMemoryFontFileLoader: CreateInMemoryFontFileLoader::<Identity, OFFSET>,
            CreateHttpFontFileLoader: CreateHttpFontFileLoader::<Identity, OFFSET>,
            AnalyzeContainerType: AnalyzeContainerType::<Identity, OFFSET>,
            UnpackFontFile: UnpackFontFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory5 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory6_Impl: Sized + IDWriteFactory5_Impl {
    fn CreateFontFaceReference(&self, fontfile: Option<&IDWriteFontFile>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(&self, fontfile: Option<&IDWriteFontFile>, faceindex: u32) -> windows_core::Result<IDWriteFontResource>;
    fn GetSystemFontSet(&self, includedownloadablefonts: super::super::Foundation::BOOL) -> windows_core::Result<IDWriteFontSet1>;
    fn GetSystemFontCollection(&self, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>;
    fn CreateFontCollectionFromFontSet(&self, fontset: Option<&IDWriteFontSet>, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>;
    fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder2>;
    fn CreateTextFormat(&self, fontfamilyname: &windows_core::PCWSTR, fontcollection: Option<&IDWriteFontCollection>, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: &windows_core::PCWSTR) -> windows_core::Result<IDWriteTextFormat3>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDWriteFactory6 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory6_Vtbl
    where
        Identity: IDWriteFactory6_Impl,
    {
        unsafe extern "system" fn CreateFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::CreateFontFaceReference(this, windows_core::from_raw_borrowed(&fontfile), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, faceindex: u32, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::CreateFontResource(this, windows_core::from_raw_borrowed(&fontfile), core::mem::transmute_copy(&faceindex)) {
                Ok(ok__) => {
                    fontresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::GetSystemFontSet(this, core::mem::transmute_copy(&includedownloadablefonts)) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&includedownloadablefonts), core::mem::transmute_copy(&fontfamilymodel)) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::CreateFontCollectionFromFontSet(this, windows_core::from_raw_borrowed(&fontset), core::mem::transmute_copy(&fontfamilymodel)) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsetbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::CreateFontSetBuilder(this) {
                Ok(ok__) => {
                    fontsetbuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTextFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PCWSTR, fontcollection: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: windows_core::PCWSTR, textformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory6_Impl::CreateTextFormat(this, core::mem::transmute(&fontfamilyname), windows_core::from_raw_borrowed(&fontcollection), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&fontsize), core::mem::transmute(&localename)) {
                Ok(ok__) => {
                    textformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory5_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFaceReference: CreateFontFaceReference::<Identity, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, OFFSET>,
            GetSystemFontSet: GetSystemFontSet::<Identity, OFFSET>,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            CreateFontCollectionFromFontSet: CreateFontCollectionFromFontSet::<Identity, OFFSET>,
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory6 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory7_Impl: Sized + IDWriteFactory6_Impl {
    fn GetSystemFontSet(&self, includedownloadablefonts: super::super::Foundation::BOOL) -> windows_core::Result<IDWriteFontSet2>;
    fn GetSystemFontCollection(&self, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection3>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDWriteFactory7 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory7_Vtbl
    where
        Identity: IDWriteFactory7_Impl,
    {
        unsafe extern "system" fn GetSystemFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory7_Impl::GetSystemFontSet(this, core::mem::transmute_copy(&includedownloadablefonts)) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory7_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&includedownloadablefonts), core::mem::transmute_copy(&fontfamilymodel)) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFactory6_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontSet: GetSystemFontSet::<Identity, OFFSET>,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory7 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory5 as windows_core::Interface>::IID || iid == &<IDWriteFactory6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory8_Impl: Sized + IDWriteFactory7_Impl {
    fn TranslateColorGlyphRun(&self, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDWriteFactory8 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl IDWriteFactory8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFactory8_Vtbl
    where
        Identity: IDWriteFactory8_Impl,
    {
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFactory8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFactory8_Impl::TranslateColorGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&desiredglyphimageformats), core::mem::transmute_copy(&paintfeaturelevel), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&worldanddpitransform), core::mem::transmute_copy(&colorpaletteindex)) {
                Ok(ok__) => {
                    colorenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDWriteFactory7_Vtbl::new::<Identity, OFFSET>(), TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory8 as windows_core::Interface>::IID || iid == &<IDWriteFactory as windows_core::Interface>::IID || iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory5 as windows_core::Interface>::IID || iid == &<IDWriteFactory6 as windows_core::Interface>::IID || iid == &<IDWriteFactory7 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFont_Impl: Sized {
    fn GetFontFamily(&self) -> windows_core::Result<IDWriteFontFamily>;
    fn GetWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetStyle(&self) -> DWRITE_FONT_STYLE;
    fn IsSymbolFont(&self) -> super::super::Foundation::BOOL;
    fn GetFaceNames(&self) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS);
    fn HasCharacter(&self, unicodevalue: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace>;
}
impl windows_core::RuntimeName for IDWriteFont {}
impl IDWriteFont_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFont_Vtbl
    where
        Identity: IDWriteFont_Impl,
    {
        unsafe extern "system" fn GetFontFamily<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFont_Impl::GetFontFamily(this) {
                Ok(ok__) => {
                    fontfamily.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::GetWeight(this)
        }
        unsafe extern "system" fn GetStretch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STRETCH
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::GetStretch(this)
        }
        unsafe extern "system" fn GetStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STYLE
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::GetStyle(this)
        }
        unsafe extern "system" fn IsSymbolFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::IsSymbolFont(this)
        }
        unsafe extern "system" fn GetFaceNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFont_Impl::GetFaceNames(this) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::GetInformationalStrings(this, core::mem::transmute_copy(&informationalstringid), core::mem::transmute_copy(&informationalstrings), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetSimulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::GetSimulations(this)
        }
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS)
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont_Impl::GetMetrics(this, core::mem::transmute_copy(&fontmetrics))
        }
        unsafe extern "system" fn HasCharacter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFont_Impl::HasCharacter(this, core::mem::transmute_copy(&unicodevalue)) {
                Ok(ok__) => {
                    exists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFont_Impl::CreateFontFace(this) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            GetWeight: GetWeight::<Identity, OFFSET>,
            GetStretch: GetStretch::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, OFFSET>,
            GetSimulations: GetSimulations::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            HasCharacter: HasCharacter::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont as windows_core::Interface>::IID
    }
}
pub trait IDWriteFont1_Impl: Sized + IDWriteFont_Impl {
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetPanose(&self, panose: *mut DWRITE_PANOSE);
    fn GetUnicodeRanges(&self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::Result<()>;
    fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IDWriteFont1 {}
impl IDWriteFont1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFont1_Vtbl
    where
        Identity: IDWriteFont1_Impl,
    {
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1)
        where
            Identity: IDWriteFont1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont1_Impl::GetMetrics(this, core::mem::transmute_copy(&fontmetrics))
        }
        unsafe extern "system" fn GetPanose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, panose: *mut DWRITE_PANOSE)
        where
            Identity: IDWriteFont1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont1_Impl::GetPanose(this, core::mem::transmute_copy(&panose))
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFont1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont1_Impl::GetUnicodeRanges(this, core::mem::transmute_copy(&maxrangecount), core::mem::transmute_copy(&unicoderanges), core::mem::transmute_copy(&actualrangecount)).into()
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFont1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont1_Impl::IsMonospacedFont(this)
        }
        Self {
            base__: IDWriteFont_Vtbl::new::<Identity, OFFSET>(),
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetPanose: GetPanose::<Identity, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont1 as windows_core::Interface>::IID || iid == &<IDWriteFont as windows_core::Interface>::IID
    }
}
pub trait IDWriteFont2_Impl: Sized + IDWriteFont1_Impl {
    fn IsColorFont(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IDWriteFont2 {}
impl IDWriteFont2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFont2_Vtbl
    where
        Identity: IDWriteFont2_Impl,
    {
        unsafe extern "system" fn IsColorFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFont2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont2_Impl::IsColorFont(this)
        }
        Self { base__: IDWriteFont1_Vtbl::new::<Identity, OFFSET>(), IsColorFont: IsColorFont::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont2 as windows_core::Interface>::IID || iid == &<IDWriteFont as windows_core::Interface>::IID || iid == &<IDWriteFont1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFont3_Impl: Sized + IDWriteFont2_Impl {
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3>;
    fn Equals(&self, font: Option<&IDWriteFont>) -> super::super::Foundation::BOOL;
    fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference>;
    fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
}
impl windows_core::RuntimeName for IDWriteFont3 {}
impl IDWriteFont3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFont3_Vtbl
    where
        Identity: IDWriteFont3_Impl,
    {
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFont3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFont3_Impl::CreateFontFace(this) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFont3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont3_Impl::Equals(this, windows_core::from_raw_borrowed(&font))
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFont3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFont3_Impl::GetFontFaceReference(this) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCharacter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFont3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont3_Impl::HasCharacter(this, core::mem::transmute_copy(&unicodevalue))
        }
        unsafe extern "system" fn GetLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_LOCALITY
        where
            Identity: IDWriteFont3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFont3_Impl::GetLocality(this)
        }
        Self {
            base__: IDWriteFont2_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            Equals: Equals::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            HasCharacter: HasCharacter::<Identity, OFFSET>,
            GetLocality: GetLocality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont3 as windows_core::Interface>::IID || iid == &<IDWriteFont as windows_core::Interface>::IID || iid == &<IDWriteFont1 as windows_core::Interface>::IID || iid == &<IDWriteFont2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontCollection_Impl: Sized {
    fn GetFontFamilyCount(&self) -> u32;
    fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily>;
    fn FindFamilyName(&self, familyname: &windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFontFromFontFace(&self, fontface: Option<&IDWriteFontFace>) -> windows_core::Result<IDWriteFont>;
}
impl windows_core::RuntimeName for IDWriteFontCollection {}
impl IDWriteFontCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontCollection_Vtbl
    where
        Identity: IDWriteFontCollection_Impl,
    {
        unsafe extern "system" fn GetFontFamilyCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontCollection_Impl::GetFontFamilyCount(this)
        }
        unsafe extern "system" fn GetFontFamily<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection_Impl::GetFontFamily(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    fontfamily.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontCollection_Impl::FindFamilyName(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&index), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetFontFromFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection_Impl::GetFontFromFontFace(this, windows_core::from_raw_borrowed(&fontface)) {
                Ok(ok__) => {
                    font.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamilyCount: GetFontFamilyCount::<Identity, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            FindFamilyName: FindFamilyName::<Identity, OFFSET>,
            GetFontFromFontFace: GetFontFromFontFace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontCollection1_Impl: Sized + IDWriteFontCollection_Impl {
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet>;
    fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily1>;
}
impl windows_core::RuntimeName for IDWriteFontCollection1 {}
impl IDWriteFontCollection1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontCollection1_Vtbl
    where
        Identity: IDWriteFontCollection1_Impl,
    {
        unsafe extern "system" fn GetFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection1_Impl::GetFontSet(this) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamily<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection1_Impl::GetFontFamily(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    fontfamily.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontCollection_Vtbl::new::<Identity, OFFSET>(),
            GetFontSet: GetFontSet::<Identity, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection1 as windows_core::Interface>::IID || iid == &<IDWriteFontCollection as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontCollection2_Impl: Sized + IDWriteFontCollection1_Impl {
    fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily2>;
    fn GetMatchingFonts(&self, familyname: &windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontList2>;
    fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL;
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1>;
}
impl windows_core::RuntimeName for IDWriteFontCollection2 {}
impl IDWriteFontCollection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontCollection2_Vtbl
    where
        Identity: IDWriteFontCollection2_Impl,
    {
        unsafe extern "system" fn GetFontFamily<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection2_Impl::GetFontFamily(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    fontfamily.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection2_Impl::GetMatchingFonts(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                Ok(ok__) => {
                    fontlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyModel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL
        where
            Identity: IDWriteFontCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontCollection2_Impl::GetFontFamilyModel(this)
        }
        unsafe extern "system" fn GetFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollection2_Impl::GetFontSet(this) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontCollection1_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetFontFamilyModel: GetFontFamilyModel::<Identity, OFFSET>,
            GetFontSet: GetFontSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection2 as windows_core::Interface>::IID || iid == &<IDWriteFontCollection as windows_core::Interface>::IID || iid == &<IDWriteFontCollection1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontCollection3_Impl: Sized + IDWriteFontCollection2_Impl {
    fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for IDWriteFontCollection3 {}
impl IDWriteFontCollection3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontCollection3_Vtbl
    where
        Identity: IDWriteFontCollection3_Impl,
    {
        unsafe extern "system" fn GetExpirationEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE
        where
            Identity: IDWriteFontCollection3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontCollection3_Impl::GetExpirationEvent(this)
        }
        Self { base__: IDWriteFontCollection2_Vtbl::new::<Identity, OFFSET>(), GetExpirationEvent: GetExpirationEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection3 as windows_core::Interface>::IID || iid == &<IDWriteFontCollection as windows_core::Interface>::IID || iid == &<IDWriteFontCollection1 as windows_core::Interface>::IID || iid == &<IDWriteFontCollection2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontCollectionLoader_Impl: Sized {
    fn CreateEnumeratorFromKey(&self, factory: Option<&IDWriteFactory>, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontFileEnumerator>;
}
impl windows_core::RuntimeName for IDWriteFontCollectionLoader {}
impl IDWriteFontCollectionLoader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontCollectionLoader_Vtbl
    where
        Identity: IDWriteFontCollectionLoader_Impl,
    {
        unsafe extern "system" fn CreateEnumeratorFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, collectionkey: *const core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontCollectionLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontCollectionLoader_Impl::CreateEnumeratorFromKey(this, windows_core::from_raw_borrowed(&factory), core::mem::transmute_copy(&collectionkey), core::mem::transmute_copy(&collectionkeysize)) {
                Ok(ok__) => {
                    fontfileenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateEnumeratorFromKey: CreateEnumeratorFromKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollectionLoader as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontDownloadListener_Impl: Sized {
    fn DownloadCompleted(&self, downloadqueue: Option<&IDWriteFontDownloadQueue>, context: Option<&windows_core::IUnknown>, downloadresult: windows_core::HRESULT);
}
impl windows_core::RuntimeName for IDWriteFontDownloadListener {}
impl IDWriteFontDownloadListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontDownloadListener_Vtbl
    where
        Identity: IDWriteFontDownloadListener_Impl,
    {
        unsafe extern "system" fn DownloadCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadqueue: *mut core::ffi::c_void, context: *mut core::ffi::c_void, downloadresult: windows_core::HRESULT)
        where
            Identity: IDWriteFontDownloadListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontDownloadListener_Impl::DownloadCompleted(this, windows_core::from_raw_borrowed(&downloadqueue), windows_core::from_raw_borrowed(&context), core::mem::transmute_copy(&downloadresult))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DownloadCompleted: DownloadCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontDownloadListener as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontDownloadQueue_Impl: Sized {
    fn AddListener(&self, listener: Option<&IDWriteFontDownloadListener>) -> windows_core::Result<u32>;
    fn RemoveListener(&self, token: u32) -> windows_core::Result<()>;
    fn IsEmpty(&self) -> super::super::Foundation::BOOL;
    fn BeginDownload(&self, context: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CancelDownload(&self) -> windows_core::Result<()>;
    fn GetGenerationCount(&self) -> u64;
}
impl windows_core::RuntimeName for IDWriteFontDownloadQueue {}
impl IDWriteFontDownloadQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontDownloadQueue_Vtbl
    where
        Identity: IDWriteFontDownloadQueue_Impl,
    {
        unsafe extern "system" fn AddListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listener: *mut core::ffi::c_void, token: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontDownloadQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontDownloadQueue_Impl::AddListener(this, windows_core::from_raw_borrowed(&listener)) {
                Ok(ok__) => {
                    token.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontDownloadQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontDownloadQueue_Impl::RemoveListener(this, core::mem::transmute_copy(&token)).into()
        }
        unsafe extern "system" fn IsEmpty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontDownloadQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontDownloadQueue_Impl::IsEmpty(this)
        }
        unsafe extern "system" fn BeginDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontDownloadQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontDownloadQueue_Impl::BeginDownload(this, windows_core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn CancelDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontDownloadQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontDownloadQueue_Impl::CancelDownload(this).into()
        }
        unsafe extern "system" fn GetGenerationCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: IDWriteFontDownloadQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontDownloadQueue_Impl::GetGenerationCount(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddListener: AddListener::<Identity, OFFSET>,
            RemoveListener: RemoveListener::<Identity, OFFSET>,
            IsEmpty: IsEmpty::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
            CancelDownload: CancelDownload::<Identity, OFFSET>,
            GetGenerationCount: GetGenerationCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontDownloadQueue as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace_Impl: Sized {
    fn GetType(&self) -> DWRITE_FONT_FACE_TYPE;
    fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: *mut Option<IDWriteFontFile>) -> windows_core::Result<()>;
    fn GetIndex(&self) -> u32;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn IsSymbolFont(&self) -> super::super::Foundation::BOOL;
    fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS);
    fn GetGlyphCount(&self) -> u16;
    fn GetDesignGlyphMetrics(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> windows_core::Result<()>;
    fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReleaseFontTable(&self, tablecontext: *const core::ffi::c_void);
    fn GetGlyphRunOutline(&self, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: Option<&super::Direct2D::Common::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn GetRecommendedRenderingMode(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: Option<&IDWriteRenderingParams>) -> windows_core::Result<DWRITE_RENDERING_MODE>;
    fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> windows_core::Result<()>;
    fn GetGdiCompatibleGlyphMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace_Vtbl
    where
        Identity: IDWriteFontFace_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetType(this)
        }
        unsafe extern "system" fn GetFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetFiles(this, core::mem::transmute_copy(&numberoffiles), core::mem::transmute_copy(&fontfiles)).into()
        }
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetIndex(this)
        }
        unsafe extern "system" fn GetSimulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetSimulations(this)
        }
        unsafe extern "system" fn IsSymbolFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::IsSymbolFont(this)
        }
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS)
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetMetrics(this, core::mem::transmute_copy(&fontfacemetrics))
        }
        unsafe extern "system" fn GetGlyphCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u16
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetGlyphCount(this)
        }
        unsafe extern "system" fn GetDesignGlyphMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetDesignGlyphMetrics(this, core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphmetrics), core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&codepoints), core::mem::transmute_copy(&codepointcount), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn TryGetFontTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::TryGetFontTable(this, core::mem::transmute_copy(&opentypetabletag), core::mem::transmute_copy(&tabledata), core::mem::transmute_copy(&tablesize), core::mem::transmute_copy(&tablecontext), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn ReleaseFontTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablecontext: *const core::ffi::c_void)
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::ReleaseFontTable(this, core::mem::transmute_copy(&tablecontext))
        }
        unsafe extern "system" fn GetGlyphRunOutline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetGlyphRunOutline(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvances), core::mem::transmute_copy(&glyphoffsets), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), windows_core::from_raw_borrowed(&geometrysink)).into()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&measuringmode), windows_core::from_raw_borrowed(&renderingparams)) {
                Ok(ok__) => {
                    renderingmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetGdiCompatibleMetrics(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&fontfacemetrics)).into()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace_Impl::GetGdiCompatibleGlyphMetrics(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&usegdinatural), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphmetrics), core::mem::transmute_copy(&issideways)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetFiles: GetFiles::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetSimulations: GetSimulations::<Identity, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetGlyphCount: GetGlyphCount::<Identity, OFFSET>,
            GetDesignGlyphMetrics: GetDesignGlyphMetrics::<Identity, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, OFFSET>,
            TryGetFontTable: TryGetFontTable::<Identity, OFFSET>,
            ReleaseFontTable: ReleaseFontTable::<Identity, OFFSET>,
            GetGlyphRunOutline: GetGlyphRunOutline::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
            GetGdiCompatibleMetrics: GetGdiCompatibleMetrics::<Identity, OFFSET>,
            GetGdiCompatibleGlyphMetrics: GetGdiCompatibleGlyphMetrics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace1_Impl: Sized + IDWriteFontFace_Impl {
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> windows_core::Result<()>;
    fn GetCaretMetrics(&self, caretmetrics: *mut DWRITE_CARET_METRICS);
    fn GetUnicodeRanges(&self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::Result<()>;
    fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL;
    fn GetDesignGlyphAdvances(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetGdiCompatibleGlyphAdvances(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> windows_core::Result<()>;
    fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> windows_core::Result<()>;
    fn HasKerningPairs(&self) -> super::super::Foundation::BOOL;
    fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> windows_core::Result<DWRITE_RENDERING_MODE>;
    fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> windows_core::Result<()>;
    fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace1 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace1_Vtbl
    where
        Identity: IDWriteFontFace1_Impl,
    {
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1)
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetMetrics(this, core::mem::transmute_copy(&fontmetrics))
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetGdiCompatibleMetrics(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&fontmetrics)).into()
        }
        unsafe extern "system" fn GetCaretMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS)
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetCaretMetrics(this, core::mem::transmute_copy(&caretmetrics))
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetUnicodeRanges(this, core::mem::transmute_copy(&maxrangecount), core::mem::transmute_copy(&unicoderanges), core::mem::transmute_copy(&actualrangecount)).into()
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::IsMonospacedFont(this)
        }
        unsafe extern "system" fn GetDesignGlyphAdvances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetDesignGlyphAdvances(this, core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvances), core::mem::transmute_copy(&issideways)).into()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphAdvances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetGdiCompatibleGlyphAdvances(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&usegdinatural), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvances)).into()
        }
        unsafe extern "system" fn GetKerningPairAdjustments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetKerningPairAdjustments(this, core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvanceadjustments)).into()
        }
        unsafe extern "system" fn HasKerningPairs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::HasKerningPairs(this)
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace1_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&outlinethreshold), core::mem::transmute_copy(&measuringmode)) {
                Ok(ok__) => {
                    renderingmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalGlyphVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::GetVerticalGlyphVariants(this, core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&nominalglyphindices), core::mem::transmute_copy(&verticalglyphindices)).into()
        }
        unsafe extern "system" fn HasVerticalGlyphVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace1_Impl::HasVerticalGlyphVariants(this)
        }
        Self {
            base__: IDWriteFontFace_Vtbl::new::<Identity, OFFSET>(),
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetGdiCompatibleMetrics: GetGdiCompatibleMetrics::<Identity, OFFSET>,
            GetCaretMetrics: GetCaretMetrics::<Identity, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, OFFSET>,
            GetDesignGlyphAdvances: GetDesignGlyphAdvances::<Identity, OFFSET>,
            GetGdiCompatibleGlyphAdvances: GetGdiCompatibleGlyphAdvances::<Identity, OFFSET>,
            GetKerningPairAdjustments: GetKerningPairAdjustments::<Identity, OFFSET>,
            HasKerningPairs: HasKerningPairs::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
            GetVerticalGlyphVariants: GetVerticalGlyphVariants::<Identity, OFFSET>,
            HasVerticalGlyphVariants: HasVerticalGlyphVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace2_Impl: Sized + IDWriteFontFace1_Impl {
    fn IsColorFont(&self) -> super::super::Foundation::BOOL;
    fn GetColorPaletteCount(&self) -> u32;
    fn GetPaletteEntryCount(&self) -> u32;
    fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> windows_core::Result<()>;
    fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: Option<&IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace2 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace2_Vtbl
    where
        Identity: IDWriteFontFace2_Impl,
    {
        unsafe extern "system" fn IsColorFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace2_Impl::IsColorFont(this)
        }
        unsafe extern "system" fn GetColorPaletteCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontFace2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace2_Impl::GetColorPaletteCount(this)
        }
        unsafe extern "system" fn GetPaletteEntryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontFace2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace2_Impl::GetPaletteEntryCount(this)
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace2_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&firstentryindex), core::mem::transmute_copy(&entrycount), core::mem::transmute_copy(&paletteentries)).into()
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace2_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&outlinethreshold), core::mem::transmute_copy(&measuringmode), windows_core::from_raw_borrowed(&renderingparams), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)).into()
        }
        Self {
            base__: IDWriteFontFace1_Vtbl::new::<Identity, OFFSET>(),
            IsColorFont: IsColorFont::<Identity, OFFSET>,
            GetColorPaletteCount: GetColorPaletteCount::<Identity, OFFSET>,
            GetPaletteEntryCount: GetPaletteEntryCount::<Identity, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID || iid == &<IDWriteFontFace1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace3_Impl: Sized + IDWriteFontFace2_Impl {
    fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference>;
    fn GetPanose(&self, panose: *mut DWRITE_PANOSE);
    fn GetWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetStyle(&self) -> DWRITE_FONT_STYLE;
    fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFaceNames(&self) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: Option<&IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::Result<()>;
    fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL;
    fn AreCharactersLocal(&self, characters: &windows_core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn AreGlyphsLocal(&self, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace3 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace3_Vtbl
    where
        Identity: IDWriteFontFace3_Impl,
    {
        unsafe extern "system" fn GetFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace3_Impl::GetFontFaceReference(this) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPanose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, panose: *mut DWRITE_PANOSE)
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::GetPanose(this, core::mem::transmute_copy(&panose))
        }
        unsafe extern "system" fn GetWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::GetWeight(this)
        }
        unsafe extern "system" fn GetStretch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STRETCH
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::GetStretch(this)
        }
        unsafe extern "system" fn GetStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STYLE
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::GetStyle(this)
        }
        unsafe extern "system" fn GetFamilyNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace3_Impl::GetFamilyNames(this) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace3_Impl::GetFaceNames(this) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::GetInformationalStrings(this, core::mem::transmute_copy(&informationalstringid), core::mem::transmute_copy(&informationalstrings), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn HasCharacter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::HasCharacter(this, core::mem::transmute_copy(&unicodevalue))
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&outlinethreshold), core::mem::transmute_copy(&measuringmode), windows_core::from_raw_borrowed(&renderingparams), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)).into()
        }
        unsafe extern "system" fn IsCharacterLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::IsCharacterLocal(this, core::mem::transmute_copy(&unicodevalue))
        }
        unsafe extern "system" fn IsGlyphLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphid: u16) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace3_Impl::IsGlyphLocal(this, core::mem::transmute_copy(&glyphid))
        }
        unsafe extern "system" fn AreCharactersLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, characters: windows_core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace3_Impl::AreCharactersLocal(this, core::mem::transmute(&characters), core::mem::transmute_copy(&charactercount), core::mem::transmute_copy(&enqueueifnotlocal)) {
                Ok(ok__) => {
                    islocal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreGlyphsLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace3_Impl::AreGlyphsLocal(this, core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&enqueueifnotlocal)) {
                Ok(ok__) => {
                    islocal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFace2_Vtbl::new::<Identity, OFFSET>(),
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            GetPanose: GetPanose::<Identity, OFFSET>,
            GetWeight: GetWeight::<Identity, OFFSET>,
            GetStretch: GetStretch::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            GetFamilyNames: GetFamilyNames::<Identity, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, OFFSET>,
            HasCharacter: HasCharacter::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
            IsCharacterLocal: IsCharacterLocal::<Identity, OFFSET>,
            IsGlyphLocal: IsGlyphLocal::<Identity, OFFSET>,
            AreCharactersLocal: AreCharactersLocal::<Identity, OFFSET>,
            AreGlyphsLocal: AreGlyphsLocal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID || iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<IDWriteFontFace2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace4_Impl: Sized + IDWriteFontFace3_Impl {
    fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> windows_core::Result<DWRITE_GLYPH_IMAGE_FORMATS>;
    fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS;
    fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut core::ffi::c_void);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace4 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace4_Vtbl
    where
        Identity: IDWriteFontFace4_Impl,
    {
        unsafe extern "system" fn GetGlyphImageFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace4_Impl::GetGlyphImageFormats(this, core::mem::transmute_copy(&glyphid), core::mem::transmute_copy(&pixelsperemfirst), core::mem::transmute_copy(&pixelsperemlast)) {
                Ok(ok__) => {
                    glyphimageformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphImageFormats2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS
        where
            Identity: IDWriteFontFace4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace4_Impl::GetGlyphImageFormats2(this)
        }
        unsafe extern "system" fn GetGlyphImageData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace4_Impl::GetGlyphImageData(this, core::mem::transmute_copy(&glyphid), core::mem::transmute_copy(&pixelsperem), core::mem::transmute_copy(&glyphimageformat), core::mem::transmute_copy(&glyphdata), core::mem::transmute_copy(&glyphdatacontext)).into()
        }
        unsafe extern "system" fn ReleaseGlyphImageData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphdatacontext: *mut core::ffi::c_void)
        where
            Identity: IDWriteFontFace4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace4_Impl::ReleaseGlyphImageData(this, core::mem::transmute_copy(&glyphdatacontext))
        }
        Self {
            base__: IDWriteFontFace3_Vtbl::new::<Identity, OFFSET>(),
            GetGlyphImageFormats: GetGlyphImageFormats::<Identity, OFFSET>,
            GetGlyphImageFormats2: GetGlyphImageFormats2::<Identity, OFFSET>,
            GetGlyphImageData: GetGlyphImageData::<Identity, OFFSET>,
            ReleaseGlyphImageData: ReleaseGlyphImageData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace4 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID || iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace5_Impl: Sized + IDWriteFontFace4_Impl {
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn HasVariations(&self) -> super::super::Foundation::BOOL;
    fn GetFontResource(&self) -> windows_core::Result<IDWriteFontResource>;
    fn Equals(&self, fontface: Option<&IDWriteFontFace>) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace5 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace5_Vtbl
    where
        Identity: IDWriteFontFace5_Impl,
    {
        unsafe extern "system" fn GetFontAxisValueCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontFace5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace5_Impl::GetFontAxisValueCount(this)
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace5_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn HasVariations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace5_Impl::HasVariations(this)
        }
        unsafe extern "system" fn GetFontResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace5_Impl::GetFontResource(this) {
                Ok(ok__) => {
                    fontresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFace5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace5_Impl::Equals(this, windows_core::from_raw_borrowed(&fontface))
        }
        Self {
            base__: IDWriteFontFace4_Vtbl::new::<Identity, OFFSET>(),
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
            HasVariations: HasVariations::<Identity, OFFSET>,
            GetFontResource: GetFontResource::<Identity, OFFSET>,
            Equals: Equals::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace5 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID || iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace6_Impl: Sized + IDWriteFontFace5_Impl {
    fn GetFamilyNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFaceNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteLocalizedStrings>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace6 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace6_Vtbl
    where
        Identity: IDWriteFontFace6_Impl,
    {
        unsafe extern "system" fn GetFamilyNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace6_Impl::GetFamilyNames(this, core::mem::transmute_copy(&fontfamilymodel)) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaceNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace6_Impl::GetFaceNames(this, core::mem::transmute_copy(&fontfamilymodel)) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFace5_Vtbl::new::<Identity, OFFSET>(),
            GetFamilyNames: GetFamilyNames::<Identity, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace6 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID || iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace4 as windows_core::Interface>::IID || iid == &<IDWriteFontFace5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWriteFontFace7_Impl: Sized + IDWriteFontFace6_Impl {
    fn GetPaintFeatureLevel(&self, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL;
    fn CreatePaintReader(&self, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL) -> windows_core::Result<IDWritePaintReader>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWriteFontFace7 {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWriteFontFace7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFace7_Vtbl
    where
        Identity: IDWriteFontFace7_Impl,
    {
        unsafe extern "system" fn GetPaintFeatureLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL
        where
            Identity: IDWriteFontFace7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFace7_Impl::GetPaintFeatureLevel(this, core::mem::transmute_copy(&glyphimageformat))
        }
        unsafe extern "system" fn CreatePaintReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, paintreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFace7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFace7_Impl::CreatePaintReader(this, core::mem::transmute_copy(&glyphimageformat), core::mem::transmute_copy(&paintfeaturelevel)) {
                Ok(ok__) => {
                    paintreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFace6_Vtbl::new::<Identity, OFFSET>(),
            GetPaintFeatureLevel: GetPaintFeatureLevel::<Identity, OFFSET>,
            CreatePaintReader: CreatePaintReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace7 as windows_core::Interface>::IID || iid == &<IDWriteFontFace as windows_core::Interface>::IID || iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace4 as windows_core::Interface>::IID || iid == &<IDWriteFontFace5 as windows_core::Interface>::IID || iid == &<IDWriteFontFace6 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFaceReference_Impl: Sized {
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3>;
    fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace3>;
    fn Equals(&self, fontfacereference: Option<&IDWriteFontFaceReference>) -> super::super::Foundation::BOOL;
    fn GetFontFaceIndex(&self) -> u32;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn GetFontFile(&self) -> windows_core::Result<IDWriteFontFile>;
    fn GetLocalFileSize(&self) -> u64;
    fn GetFileSize(&self) -> u64;
    fn GetFileTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
    fn EnqueueFontDownloadRequest(&self) -> windows_core::Result<()>;
    fn EnqueueCharacterDownloadRequest(&self, characters: &windows_core::PCWSTR, charactercount: u32) -> windows_core::Result<()>;
    fn EnqueueGlyphDownloadRequest(&self, glyphindices: *const u16, glyphcount: u32) -> windows_core::Result<()>;
    fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontFaceReference {}
impl IDWriteFontFaceReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFaceReference_Vtbl
    where
        Identity: IDWriteFontFaceReference_Impl,
    {
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFaceReference_Impl::CreateFontFace(this) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceWithSimulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFaceReference_Impl::CreateFontFaceWithSimulations(this, core::mem::transmute_copy(&fontfacesimulationflags)) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::Equals(this, windows_core::from_raw_borrowed(&fontfacereference))
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::GetFontFaceIndex(this)
        }
        unsafe extern "system" fn GetSimulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::GetSimulations(this)
        }
        unsafe extern "system" fn GetFontFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFaceReference_Impl::GetFontFile(this) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::GetLocalFileSize(this)
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::GetFileSize(this)
        }
        unsafe extern "system" fn GetFileTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastwritetime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFaceReference_Impl::GetFileTime(this) {
                Ok(ok__) => {
                    lastwritetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_LOCALITY
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::GetLocality(this)
        }
        unsafe extern "system" fn EnqueueFontDownloadRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::EnqueueFontDownloadRequest(this).into()
        }
        unsafe extern "system" fn EnqueueCharacterDownloadRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, characters: windows_core::PCWSTR, charactercount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::EnqueueCharacterDownloadRequest(this, core::mem::transmute(&characters), core::mem::transmute_copy(&charactercount)).into()
        }
        unsafe extern "system" fn EnqueueGlyphDownloadRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::EnqueueGlyphDownloadRequest(this, core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount)).into()
        }
        unsafe extern "system" fn EnqueueFileFragmentDownloadRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference_Impl::EnqueueFileFragmentDownloadRequest(this, core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&fragmentsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateFontFaceWithSimulations: CreateFontFaceWithSimulations::<Identity, OFFSET>,
            Equals: Equals::<Identity, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, OFFSET>,
            GetSimulations: GetSimulations::<Identity, OFFSET>,
            GetFontFile: GetFontFile::<Identity, OFFSET>,
            GetLocalFileSize: GetLocalFileSize::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetFileTime: GetFileTime::<Identity, OFFSET>,
            GetLocality: GetLocality::<Identity, OFFSET>,
            EnqueueFontDownloadRequest: EnqueueFontDownloadRequest::<Identity, OFFSET>,
            EnqueueCharacterDownloadRequest: EnqueueCharacterDownloadRequest::<Identity, OFFSET>,
            EnqueueGlyphDownloadRequest: EnqueueGlyphDownloadRequest::<Identity, OFFSET>,
            EnqueueFileFragmentDownloadRequest: EnqueueFileFragmentDownloadRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFaceReference1_Impl: Sized + IDWriteFontFaceReference_Impl {
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace5>;
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontFaceReference1 {}
impl IDWriteFontFaceReference1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFaceReference1_Vtbl
    where
        Identity: IDWriteFontFaceReference1_Impl,
    {
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFaceReference1_Impl::CreateFontFace(this) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontFaceReference1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference1_Impl::GetFontAxisValueCount(this)
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFaceReference1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFaceReference1_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        Self {
            base__: IDWriteFontFaceReference_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference1 as windows_core::Interface>::IID || iid == &<IDWriteFontFaceReference as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFallback_Impl: Sized {
    fn MapCharacters(&self, analysissource: Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: Option<&IDWriteFontCollection>, basefamilyname: &windows_core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut Option<IDWriteFont>, scale: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontFallback {}
impl IDWriteFontFallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFallback_Vtbl
    where
        Identity: IDWriteFontFallback_Impl,
    {
        unsafe extern "system" fn MapCharacters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut core::ffi::c_void, basefamilyname: windows_core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut *mut core::ffi::c_void, scale: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFallback_Impl::MapCharacters(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&basefontcollection), core::mem::transmute(&basefamilyname), core::mem::transmute_copy(&baseweight), core::mem::transmute_copy(&basestyle), core::mem::transmute_copy(&basestretch), core::mem::transmute_copy(&mappedlength), core::mem::transmute_copy(&mappedfont), core::mem::transmute_copy(&scale)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MapCharacters: MapCharacters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFallback as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFallback1_Impl: Sized + IDWriteFontFallback_Impl {
    fn MapCharacters(&self, analysissource: Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: Option<&IDWriteFontCollection>, basefamilyname: &windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut Option<IDWriteFontFace5>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontFallback1 {}
impl IDWriteFontFallback1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFallback1_Vtbl
    where
        Identity: IDWriteFontFallback1_Impl,
    {
        unsafe extern "system" fn MapCharacters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut core::ffi::c_void, basefamilyname: windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFallback1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFallback1_Impl::MapCharacters(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&basefontcollection), core::mem::transmute(&basefamilyname), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&mappedlength), core::mem::transmute_copy(&scale), core::mem::transmute_copy(&mappedfontface)).into()
        }
        Self { base__: IDWriteFontFallback_Vtbl::new::<Identity, OFFSET>(), MapCharacters: MapCharacters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFallback1 as windows_core::Interface>::IID || iid == &<IDWriteFontFallback as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFallbackBuilder_Impl: Sized {
    fn AddMapping(&self, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: Option<&IDWriteFontCollection>, localename: &windows_core::PCWSTR, basefamilyname: &windows_core::PCWSTR, scale: f32) -> windows_core::Result<()>;
    fn AddMappings(&self, fontfallback: Option<&IDWriteFontFallback>) -> windows_core::Result<()>;
    fn CreateFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
}
impl windows_core::RuntimeName for IDWriteFontFallbackBuilder {}
impl IDWriteFontFallbackBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFallbackBuilder_Vtbl
    where
        Identity: IDWriteFontFallbackBuilder_Impl,
    {
        unsafe extern "system" fn AddMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: *mut core::ffi::c_void, localename: windows_core::PCWSTR, basefamilyname: windows_core::PCWSTR, scale: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFallbackBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFallbackBuilder_Impl::AddMapping(this, core::mem::transmute_copy(&ranges), core::mem::transmute_copy(&rangescount), core::mem::transmute_copy(&targetfamilynames), core::mem::transmute_copy(&targetfamilynamescount), windows_core::from_raw_borrowed(&fontcollection), core::mem::transmute(&localename), core::mem::transmute(&basefamilyname), core::mem::transmute_copy(&scale)).into()
        }
        unsafe extern "system" fn AddMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFallbackBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFallbackBuilder_Impl::AddMappings(this, windows_core::from_raw_borrowed(&fontfallback)).into()
        }
        unsafe extern "system" fn CreateFontFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFallbackBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFallbackBuilder_Impl::CreateFontFallback(this) {
                Ok(ok__) => {
                    fontfallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMapping: AddMapping::<Identity, OFFSET>,
            AddMappings: AddMappings::<Identity, OFFSET>,
            CreateFontFallback: CreateFontFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFallbackBuilder as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFamily_Impl: Sized + IDWriteFontList_Impl {
    fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFont>;
    fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontList>;
}
impl windows_core::RuntimeName for IDWriteFontFamily {}
impl IDWriteFontFamily_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFamily_Vtbl
    where
        Identity: IDWriteFontFamily_Impl,
    {
        unsafe extern "system" fn GetFamilyNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily_Impl::GetFamilyNames(this) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstMatchingFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily_Impl::GetFirstMatchingFont(this, core::mem::transmute_copy(&weight), core::mem::transmute_copy(&stretch), core::mem::transmute_copy(&style)) {
                Ok(ok__) => {
                    matchingfont.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&weight), core::mem::transmute_copy(&stretch), core::mem::transmute_copy(&style)) {
                Ok(ok__) => {
                    matchingfonts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontList_Vtbl::new::<Identity, OFFSET>(),
            GetFamilyNames: GetFamilyNames::<Identity, OFFSET>,
            GetFirstMatchingFont: GetFirstMatchingFont::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFamily as windows_core::Interface>::IID || iid == &<IDWriteFontList as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFamily1_Impl: Sized + IDWriteFontFamily_Impl {
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3>;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference>;
}
impl windows_core::RuntimeName for IDWriteFontFamily1 {}
impl IDWriteFontFamily1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFamily1_Vtbl
    where
        Identity: IDWriteFontFamily1_Impl,
    {
        unsafe extern "system" fn GetFontLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY
        where
            Identity: IDWriteFontFamily1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFamily1_Impl::GetFontLocality(this, core::mem::transmute_copy(&listindex))
        }
        unsafe extern "system" fn GetFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily1_Impl::GetFont(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    font.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily1_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFamily_Vtbl::new::<Identity, OFFSET>(),
            GetFontLocality: GetFontLocality::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFamily1 as windows_core::Interface>::IID || iid == &<IDWriteFontList as windows_core::Interface>::IID || iid == &<IDWriteFontFamily as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFamily2_Impl: Sized + IDWriteFontFamily1_Impl {
    fn GetMatchingFonts(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontList2>;
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1>;
}
impl windows_core::RuntimeName for IDWriteFontFamily2 {}
impl IDWriteFontFamily2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFamily2_Vtbl
    where
        Identity: IDWriteFontFamily2_Impl,
    {
        unsafe extern "system" fn GetMatchingFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily2_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                Ok(ok__) => {
                    matchingfonts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFamily2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFamily2_Impl::GetFontSet(this) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFamily1_Vtbl::new::<Identity, OFFSET>(),
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetFontSet: GetFontSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFamily2 as windows_core::Interface>::IID || iid == &<IDWriteFontList as windows_core::Interface>::IID || iid == &<IDWriteFontFamily as windows_core::Interface>::IID || iid == &<IDWriteFontFamily1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFile_Impl: Sized {
    fn GetReferenceKey(&self, fontfilereferencekey: *mut *mut core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> windows_core::Result<()>;
    fn GetLoader(&self) -> windows_core::Result<IDWriteFontFileLoader>;
    fn Analyze(&self, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontFile {}
impl IDWriteFontFile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFile_Vtbl
    where
        Identity: IDWriteFontFile_Impl,
    {
        unsafe extern "system" fn GetReferenceKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *mut *mut core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFile_Impl::GetReferenceKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)).into()
        }
        unsafe extern "system" fn GetLoader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfileloader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFile_Impl::GetLoader(this) {
                Ok(ok__) => {
                    fontfileloader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Analyze<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFile_Impl::Analyze(this, core::mem::transmute_copy(&issupportedfonttype), core::mem::transmute_copy(&fontfiletype), core::mem::transmute_copy(&fontfacetype), core::mem::transmute_copy(&numberoffaces)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReferenceKey: GetReferenceKey::<Identity, OFFSET>,
            GetLoader: GetLoader::<Identity, OFFSET>,
            Analyze: Analyze::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFile as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFileEnumerator_Impl: Sized {
    fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentFontFile(&self) -> windows_core::Result<IDWriteFontFile>;
}
impl windows_core::RuntimeName for IDWriteFontFileEnumerator {}
impl IDWriteFontFileEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFileEnumerator_Vtbl
    where
        Identity: IDWriteFontFileEnumerator_Impl,
    {
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrentfile: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFileEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFileEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hascurrentfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFontFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFileEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFileEnumerator_Impl::GetCurrentFontFile(this) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            GetCurrentFontFile: GetCurrentFontFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFileEnumerator as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFileLoader_Impl: Sized {
    fn CreateStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteFontFileStream>;
}
impl windows_core::RuntimeName for IDWriteFontFileLoader {}
impl IDWriteFontFileLoader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFileLoader_Vtbl
    where
        Identity: IDWriteFontFileLoader_Impl,
    {
        unsafe extern "system" fn CreateStreamFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFileLoader_Impl::CreateStreamFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                Ok(ok__) => {
                    fontfilestream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateStreamFromKey: CreateStreamFromKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontFileStream_Impl: Sized {
    fn ReadFileFragment(&self, fragmentstart: *mut *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseFileFragment(&self, fragmentcontext: *mut core::ffi::c_void);
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn GetLastWriteTime(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IDWriteFontFileStream {}
impl IDWriteFontFileStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontFileStream_Vtbl
    where
        Identity: IDWriteFontFileStream_Impl,
    {
        unsafe extern "system" fn ReadFileFragment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fragmentstart: *mut *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFileStream_Impl::ReadFileFragment(this, core::mem::transmute_copy(&fragmentstart), core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&fragmentsize), core::mem::transmute_copy(&fragmentcontext)).into()
        }
        unsafe extern "system" fn ReleaseFileFragment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fragmentcontext: *mut core::ffi::c_void)
        where
            Identity: IDWriteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontFileStream_Impl::ReleaseFileFragment(this, core::mem::transmute_copy(&fragmentcontext))
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFileStream_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    filesize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastWriteTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastwritetime: *mut u64) -> windows_core::HRESULT
        where
            Identity: IDWriteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontFileStream_Impl::GetLastWriteTime(this) {
                Ok(ok__) => {
                    lastwritetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadFileFragment: ReadFileFragment::<Identity, OFFSET>,
            ReleaseFileFragment: ReleaseFileFragment::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetLastWriteTime: GetLastWriteTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFileStream as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontList_Impl: Sized {
    fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection>;
    fn GetFontCount(&self) -> u32;
    fn GetFont(&self, index: u32) -> windows_core::Result<IDWriteFont>;
}
impl windows_core::RuntimeName for IDWriteFontList {}
impl IDWriteFontList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontList_Vtbl
    where
        Identity: IDWriteFontList_Impl,
    {
        unsafe extern "system" fn GetFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontList_Impl::GetFontCollection(this) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontList_Impl::GetFontCount(this)
        }
        unsafe extern "system" fn GetFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontList_Impl::GetFont(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    font.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontCount: GetFontCount::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontList as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontList1_Impl: Sized + IDWriteFontList_Impl {
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3>;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference>;
}
impl windows_core::RuntimeName for IDWriteFontList1 {}
impl IDWriteFontList1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontList1_Vtbl
    where
        Identity: IDWriteFontList1_Impl,
    {
        unsafe extern "system" fn GetFontLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY
        where
            Identity: IDWriteFontList1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontList1_Impl::GetFontLocality(this, core::mem::transmute_copy(&listindex))
        }
        unsafe extern "system" fn GetFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontList1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontList1_Impl::GetFont(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    font.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontList1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontList1_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontList_Vtbl::new::<Identity, OFFSET>(),
            GetFontLocality: GetFontLocality::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontList1 as windows_core::Interface>::IID || iid == &<IDWriteFontList as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontList2_Impl: Sized + IDWriteFontList1_Impl {
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1>;
}
impl windows_core::RuntimeName for IDWriteFontList2 {}
impl IDWriteFontList2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontList2_Vtbl
    where
        Identity: IDWriteFontList2_Impl,
    {
        unsafe extern "system" fn GetFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontList2_Impl::GetFontSet(this) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDWriteFontList1_Vtbl::new::<Identity, OFFSET>(), GetFontSet: GetFontSet::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontList2 as windows_core::Interface>::IID || iid == &<IDWriteFontList as windows_core::Interface>::IID || iid == &<IDWriteFontList1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontResource_Impl: Sized {
    fn GetFontFile(&self) -> windows_core::Result<IDWriteFontFile>;
    fn GetFontFaceIndex(&self) -> u32;
    fn GetFontAxisCount(&self) -> u32;
    fn GetDefaultFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn GetFontAxisRanges(&self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> windows_core::Result<()>;
    fn GetFontAxisAttributes(&self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES;
    fn GetAxisNames(&self, axisindex: u32) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetAxisValueNameCount(&self, axisindex: u32) -> u32;
    fn GetAxisValueNames(&self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut Option<IDWriteLocalizedStrings>) -> windows_core::Result<()>;
    fn HasVariations(&self) -> super::super::Foundation::BOOL;
    fn CreateFontFace(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontFace5>;
    fn CreateFontFaceReference(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontFaceReference1>;
}
impl windows_core::RuntimeName for IDWriteFontResource {}
impl IDWriteFontResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontResource_Vtbl
    where
        Identity: IDWriteFontResource_Impl,
    {
        unsafe extern "system" fn GetFontFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontResource_Impl::GetFontFile(this) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetFontFaceIndex(this)
        }
        unsafe extern "system" fn GetFontAxisCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetFontAxisCount(this)
        }
        unsafe extern "system" fn GetDefaultFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetDefaultFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetFontAxisRanges(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount)).into()
        }
        unsafe extern "system" fn GetFontAxisAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetFontAxisAttributes(this, core::mem::transmute_copy(&axisindex))
        }
        unsafe extern "system" fn GetAxisNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontResource_Impl::GetAxisNames(this, core::mem::transmute_copy(&axisindex)) {
                Ok(ok__) => {
                    names.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAxisValueNameCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32) -> u32
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetAxisValueNameCount(this, core::mem::transmute_copy(&axisindex))
        }
        unsafe extern "system" fn GetAxisValueNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::GetAxisValueNames(this, core::mem::transmute_copy(&axisindex), core::mem::transmute_copy(&axisvalueindex), core::mem::transmute_copy(&fontaxisrange), core::mem::transmute_copy(&names)).into()
        }
        unsafe extern "system" fn HasVariations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontResource_Impl::HasVariations(this)
        }
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontResource_Impl::CreateFontFace(this, core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontResource_Impl::CreateFontFaceReference(this, core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFile: GetFontFile::<Identity, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, OFFSET>,
            GetFontAxisCount: GetFontAxisCount::<Identity, OFFSET>,
            GetDefaultFontAxisValues: GetDefaultFontAxisValues::<Identity, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, OFFSET>,
            GetFontAxisAttributes: GetFontAxisAttributes::<Identity, OFFSET>,
            GetAxisNames: GetAxisNames::<Identity, OFFSET>,
            GetAxisValueNameCount: GetAxisValueNameCount::<Identity, OFFSET>,
            GetAxisValueNames: GetAxisValueNames::<Identity, OFFSET>,
            HasVariations: HasVariations::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontResource as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSet_Impl: Sized {
    fn GetFontCount(&self) -> u32;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference>;
    fn FindFontFaceReference(&self, fontfacereference: Option<&IDWriteFontFaceReference>, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FindFontFace(&self, fontface: Option<&IDWriteFontFace>, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> windows_core::Result<IDWriteStringList>;
    fn GetPropertyValues2(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: &windows_core::PCWSTR) -> windows_core::Result<IDWriteStringList>;
    fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut Option<IDWriteLocalizedStrings>) -> windows_core::Result<()>;
    fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> windows_core::Result<u32>;
    fn GetMatchingFonts(&self, familyname: &windows_core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontSet>;
    fn GetMatchingFonts2(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::Result<IDWriteFontSet>;
}
impl windows_core::RuntimeName for IDWriteFontSet {}
impl IDWriteFontSet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSet_Vtbl
    where
        Identity: IDWriteFontSet_Impl,
    {
        unsafe extern "system" fn GetFontCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet_Impl::GetFontCount(this)
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet_Impl::FindFontFaceReference(this, windows_core::from_raw_borrowed(&fontfacereference), core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn FindFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet_Impl::FindFontFace(this, windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetPropertyValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet_Impl::GetPropertyValues(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    values.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: windows_core::PCWSTR, values: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet_Impl::GetPropertyValues2(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&preferredlocalenames)) {
                Ok(ok__) => {
                    values.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValues3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet_Impl::GetPropertyValues3(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&exists), core::mem::transmute_copy(&values)).into()
        }
        unsafe extern "system" fn GetPropertyOccurrenceCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet_Impl::GetPropertyOccurrenceCount(this, core::mem::transmute_copy(&property)) {
                Ok(ok__) => {
                    propertyoccurrencecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet_Impl::GetMatchingFonts(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&fontstyle)) {
                Ok(ok__) => {
                    filteredset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingFonts2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet_Impl::GetMatchingFonts2(this, core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount)) {
                Ok(ok__) => {
                    filteredset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontCount: GetFontCount::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            FindFontFaceReference: FindFontFaceReference::<Identity, OFFSET>,
            FindFontFace: FindFontFace::<Identity, OFFSET>,
            GetPropertyValues: GetPropertyValues::<Identity, OFFSET>,
            GetPropertyValues2: GetPropertyValues2::<Identity, OFFSET>,
            GetPropertyValues3: GetPropertyValues3::<Identity, OFFSET>,
            GetPropertyOccurrenceCount: GetPropertyOccurrenceCount::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetMatchingFonts2: GetMatchingFonts2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSet1_Impl: Sized + IDWriteFontSet_Impl {
    fn GetMatchingFonts(&self, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFirstFontResources(&self) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts(&self, indices: *const u32, indexcount: u32) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts2(&self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts3(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFontIndices(&self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::Result<()>;
    fn GetFilteredFontIndices2(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::Result<()>;
    fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::Result<()>;
    fn GetFontAxisRanges2(&self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::Result<()>;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(&self, listindex: u32) -> windows_core::Result<IDWriteFontResource>;
    fn CreateFontFace(&self, listindex: u32) -> windows_core::Result<IDWriteFontFace5>;
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
}
impl windows_core::RuntimeName for IDWriteFontSet1 {}
impl IDWriteFontSet1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSet1_Vtbl
    where
        Identity: IDWriteFontSet1_Impl,
    {
        unsafe extern "system" fn GetMatchingFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&fontproperty), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                Ok(ok__) => {
                    matchingfonts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstFontResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::GetFirstFontResources(this) {
                Ok(ok__) => {
                    filteredfontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::GetFilteredFonts(this, core::mem::transmute_copy(&indices), core::mem::transmute_copy(&indexcount)) {
                Ok(ok__) => {
                    filteredfontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::GetFilteredFonts2(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount), core::mem::transmute_copy(&selectanyrange)) {
                Ok(ok__) => {
                    filteredfontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFonts3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::GetFilteredFonts3(this, core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount), core::mem::transmute_copy(&selectanyproperty)) {
                Ok(ok__) => {
                    filteredfontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredFontIndices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet1_Impl::GetFilteredFontIndices(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount), core::mem::transmute_copy(&selectanyrange), core::mem::transmute_copy(&indices), core::mem::transmute_copy(&maxindexcount), core::mem::transmute_copy(&actualindexcount)).into()
        }
        unsafe extern "system" fn GetFilteredFontIndices2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet1_Impl::GetFilteredFontIndices2(this, core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount), core::mem::transmute_copy(&selectanyproperty), core::mem::transmute_copy(&indices), core::mem::transmute_copy(&maxindexcount), core::mem::transmute_copy(&actualindexcount)).into()
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet1_Impl::GetFontAxisRanges(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&maxfontaxisrangecount), core::mem::transmute_copy(&actualfontaxisrangecount)).into()
        }
        unsafe extern "system" fn GetFontAxisRanges2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet1_Impl::GetFontAxisRanges2(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&maxfontaxisrangecount), core::mem::transmute_copy(&actualfontaxisrangecount)).into()
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    fontfacereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::CreateFontResource(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    fontresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet1_Impl::CreateFontFace(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY
        where
            Identity: IDWriteFontSet1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet1_Impl::GetFontLocality(this, core::mem::transmute_copy(&listindex))
        }
        Self {
            base__: IDWriteFontSet_Vtbl::new::<Identity, OFFSET>(),
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetFirstFontResources: GetFirstFontResources::<Identity, OFFSET>,
            GetFilteredFonts: GetFilteredFonts::<Identity, OFFSET>,
            GetFilteredFonts2: GetFilteredFonts2::<Identity, OFFSET>,
            GetFilteredFonts3: GetFilteredFonts3::<Identity, OFFSET>,
            GetFilteredFontIndices: GetFilteredFontIndices::<Identity, OFFSET>,
            GetFilteredFontIndices2: GetFilteredFontIndices2::<Identity, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, OFFSET>,
            GetFontAxisRanges2: GetFontAxisRanges2::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            GetFontLocality: GetFontLocality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet1 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSet2_Impl: Sized + IDWriteFontSet1_Impl {
    fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for IDWriteFontSet2 {}
impl IDWriteFontSet2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSet2_Vtbl
    where
        Identity: IDWriteFontSet2_Impl,
    {
        unsafe extern "system" fn GetExpirationEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE
        where
            Identity: IDWriteFontSet2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet2_Impl::GetExpirationEvent(this)
        }
        Self { base__: IDWriteFontSet1_Vtbl::new::<Identity, OFFSET>(), GetExpirationEvent: GetExpirationEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet2 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID || iid == &<IDWriteFontSet1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSet3_Impl: Sized + IDWriteFontSet2_Impl {
    fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE;
    fn GetFontSourceNameLength(&self, listindex: u32) -> u32;
    fn GetFontSourceName(&self, listindex: u32, stringbuffer: windows_core::PWSTR, stringbuffersize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontSet3 {}
impl IDWriteFontSet3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSet3_Vtbl
    where
        Identity: IDWriteFontSet3_Impl,
    {
        unsafe extern "system" fn GetFontSourceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE
        where
            Identity: IDWriteFontSet3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet3_Impl::GetFontSourceType(this, core::mem::transmute_copy(&fontindex))
        }
        unsafe extern "system" fn GetFontSourceNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> u32
        where
            Identity: IDWriteFontSet3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet3_Impl::GetFontSourceNameLength(this, core::mem::transmute_copy(&listindex))
        }
        unsafe extern "system" fn GetFontSourceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, stringbuffer: windows_core::PWSTR, stringbuffersize: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet3_Impl::GetFontSourceName(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&stringbuffer), core::mem::transmute_copy(&stringbuffersize)).into()
        }
        Self {
            base__: IDWriteFontSet2_Vtbl::new::<Identity, OFFSET>(),
            GetFontSourceType: GetFontSourceType::<Identity, OFFSET>,
            GetFontSourceNameLength: GetFontSourceNameLength::<Identity, OFFSET>,
            GetFontSourceName: GetFontSourceName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet3 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID || iid == &<IDWriteFontSet1 as windows_core::Interface>::IID || iid == &<IDWriteFontSet2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSet4_Impl: Sized + IDWriteFontSet3_Impl {
    fn ConvertWeightStretchStyleToFontAxisValues(&self, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32;
    fn GetMatchingFonts(&self, familyname: &windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontSet4>;
}
impl windows_core::RuntimeName for IDWriteFontSet4 {}
impl IDWriteFontSet4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSet4_Vtbl
    where
        Identity: IDWriteFontSet4_Impl,
    {
        unsafe extern "system" fn ConvertWeightStretchStyleToFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32
        where
            Identity: IDWriteFontSet4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSet4_Impl::ConvertWeightStretchStyleToFontAxisValues(this, core::mem::transmute_copy(&inputaxisvalues), core::mem::transmute_copy(&inputaxiscount), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&fontstyle), core::mem::transmute_copy(&fontsize), core::mem::transmute_copy(&outputaxisvalues))
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSet4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSet4_Impl::GetMatchingFonts(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&allowedsimulations)) {
                Ok(ok__) => {
                    matchingfonts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontSet3_Vtbl::new::<Identity, OFFSET>(),
            ConvertWeightStretchStyleToFontAxisValues: ConvertWeightStretchStyleToFontAxisValues::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet4 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID || iid == &<IDWriteFontSet1 as windows_core::Interface>::IID || iid == &<IDWriteFontSet2 as windows_core::Interface>::IID || iid == &<IDWriteFontSet3 as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSetBuilder_Impl: Sized {
    fn AddFontFaceReference(&self, fontfacereference: Option<&IDWriteFontFaceReference>, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::Result<()>;
    fn AddFontFaceReference2(&self, fontfacereference: Option<&IDWriteFontFaceReference>) -> windows_core::Result<()>;
    fn AddFontSet(&self, fontset: Option<&IDWriteFontSet>) -> windows_core::Result<()>;
    fn CreateFontSet(&self) -> windows_core::Result<IDWriteFontSet>;
}
impl windows_core::RuntimeName for IDWriteFontSetBuilder {}
impl IDWriteFontSetBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSetBuilder_Vtbl
    where
        Identity: IDWriteFontSetBuilder_Impl,
    {
        unsafe extern "system" fn AddFontFaceReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSetBuilder_Impl::AddFontFaceReference(this, windows_core::from_raw_borrowed(&fontfacereference), core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddFontFaceReference2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSetBuilder_Impl::AddFontFaceReference2(this, windows_core::from_raw_borrowed(&fontfacereference)).into()
        }
        unsafe extern "system" fn AddFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSetBuilder_Impl::AddFontSet(this, windows_core::from_raw_borrowed(&fontset)).into()
        }
        unsafe extern "system" fn CreateFontSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteFontSetBuilder_Impl::CreateFontSet(this) {
                Ok(ok__) => {
                    fontset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFontFaceReference: AddFontFaceReference::<Identity, OFFSET>,
            AddFontFaceReference2: AddFontFaceReference2::<Identity, OFFSET>,
            AddFontSet: AddFontSet::<Identity, OFFSET>,
            CreateFontSet: CreateFontSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSetBuilder1_Impl: Sized + IDWriteFontSetBuilder_Impl {
    fn AddFontFile(&self, fontfile: Option<&IDWriteFontFile>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontSetBuilder1 {}
impl IDWriteFontSetBuilder1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSetBuilder1_Vtbl
    where
        Identity: IDWriteFontSetBuilder1_Impl,
    {
        unsafe extern "system" fn AddFontFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSetBuilder1_Impl::AddFontFile(this, windows_core::from_raw_borrowed(&fontfile)).into()
        }
        Self { base__: IDWriteFontSetBuilder_Vtbl::new::<Identity, OFFSET>(), AddFontFile: AddFontFile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder1 as windows_core::Interface>::IID || iid == &<IDWriteFontSetBuilder as windows_core::Interface>::IID
    }
}
pub trait IDWriteFontSetBuilder2_Impl: Sized + IDWriteFontSetBuilder1_Impl {
    fn AddFont(&self, fontfile: Option<&IDWriteFontFile>, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::Result<()>;
    fn AddFontFile(&self, filepath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteFontSetBuilder2 {}
impl IDWriteFontSetBuilder2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteFontSetBuilder2_Vtbl
    where
        Identity: IDWriteFontSetBuilder2_Impl,
    {
        unsafe extern "system" fn AddFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSetBuilder2_Impl::AddFont(this, windows_core::from_raw_borrowed(&fontfile), core::mem::transmute_copy(&fontfaceindex), core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount), core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddFontFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDWriteFontSetBuilder2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteFontSetBuilder2_Impl::AddFontFile(this, core::mem::transmute(&filepath)).into()
        }
        Self {
            base__: IDWriteFontSetBuilder1_Vtbl::new::<Identity, OFFSET>(),
            AddFont: AddFont::<Identity, OFFSET>,
            AddFontFile: AddFontFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder2 as windows_core::Interface>::IID || iid == &<IDWriteFontSetBuilder as windows_core::Interface>::IID || iid == &<IDWriteFontSetBuilder1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDWriteGdiInterop_Impl: Sized {
    fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW) -> windows_core::Result<IDWriteFont>;
    fn ConvertFontToLOGFONT(&self, font: Option<&IDWriteFont>, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ConvertFontFaceToLOGFONT(&self, font: Option<&IDWriteFontFace>, logfont: *mut super::Gdi::LOGFONTW) -> windows_core::Result<()>;
    fn CreateFontFaceFromHdc(&self, hdc: super::Gdi::HDC) -> windows_core::Result<IDWriteFontFace>;
    fn CreateBitmapRenderTarget(&self, hdc: super::Gdi::HDC, width: u32, height: u32) -> windows_core::Result<IDWriteBitmapRenderTarget>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDWriteGdiInterop {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDWriteGdiInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteGdiInterop_Vtbl
    where
        Identity: IDWriteGdiInterop_Impl,
    {
        unsafe extern "system" fn CreateFontFromLOGFONT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteGdiInterop_Impl::CreateFontFromLOGFONT(this, core::mem::transmute_copy(&logfont)) {
                Ok(ok__) => {
                    font.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFontToLOGFONT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteGdiInterop_Impl::ConvertFontToLOGFONT(this, windows_core::from_raw_borrowed(&font), core::mem::transmute_copy(&logfont), core::mem::transmute_copy(&issystemfont)).into()
        }
        unsafe extern "system" fn ConvertFontFaceToLOGFONT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteGdiInterop_Impl::ConvertFontFaceToLOGFONT(this, windows_core::from_raw_borrowed(&font), core::mem::transmute_copy(&logfont)).into()
        }
        unsafe extern "system" fn CreateFontFaceFromHdc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Gdi::HDC, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteGdiInterop_Impl::CreateFontFaceFromHdc(this, core::mem::transmute_copy(&hdc)) {
                Ok(ok__) => {
                    fontface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBitmapRenderTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteGdiInterop_Impl::CreateBitmapRenderTarget(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)) {
                Ok(ok__) => {
                    rendertarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFromLOGFONT: CreateFontFromLOGFONT::<Identity, OFFSET>,
            ConvertFontToLOGFONT: ConvertFontToLOGFONT::<Identity, OFFSET>,
            ConvertFontFaceToLOGFONT: ConvertFontFaceToLOGFONT::<Identity, OFFSET>,
            CreateFontFaceFromHdc: CreateFontFaceFromHdc::<Identity, OFFSET>,
            CreateBitmapRenderTarget: CreateBitmapRenderTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteGdiInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInterop1_Impl: Sized + IDWriteGdiInterop_Impl {
    fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW, fontcollection: Option<&IDWriteFontCollection>) -> windows_core::Result<IDWriteFont>;
    fn GetFontSignature(&self, fontface: Option<&IDWriteFontFace>, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::Result<()>;
    fn GetFontSignature2(&self, font: Option<&IDWriteFont>, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::Result<()>;
    fn GetMatchingFontsByLOGFONT(&self, logfont: *const super::Gdi::LOGFONTA, fontset: Option<&IDWriteFontSet>) -> windows_core::Result<IDWriteFontSet>;
}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDWriteGdiInterop1 {}
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl IDWriteGdiInterop1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteGdiInterop1_Vtbl
    where
        Identity: IDWriteGdiInterop1_Impl,
    {
        unsafe extern "system" fn CreateFontFromLOGFONT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, fontcollection: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteGdiInterop1_Impl::CreateFontFromLOGFONT(this, core::mem::transmute_copy(&logfont), windows_core::from_raw_borrowed(&fontcollection)) {
                Ok(ok__) => {
                    font.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteGdiInterop1_Impl::GetFontSignature(this, windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&fontsignature)).into()
        }
        unsafe extern "system" fn GetFontSignature2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteGdiInterop1_Impl::GetFontSignature2(this, windows_core::from_raw_borrowed(&font), core::mem::transmute_copy(&fontsignature)).into()
        }
        unsafe extern "system" fn GetMatchingFontsByLOGFONT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfont: *const super::Gdi::LOGFONTA, fontset: *mut core::ffi::c_void, filteredset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteGdiInterop1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteGdiInterop1_Impl::GetMatchingFontsByLOGFONT(this, core::mem::transmute_copy(&logfont), windows_core::from_raw_borrowed(&fontset)) {
                Ok(ok__) => {
                    filteredset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteGdiInterop_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFromLOGFONT: CreateFontFromLOGFONT::<Identity, OFFSET>,
            GetFontSignature: GetFontSignature::<Identity, OFFSET>,
            GetFontSignature2: GetFontSignature2::<Identity, OFFSET>,
            GetMatchingFontsByLOGFONT: GetMatchingFontsByLOGFONT::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteGdiInterop1 as windows_core::Interface>::IID || iid == &<IDWriteGdiInterop as windows_core::Interface>::IID
    }
}
pub trait IDWriteGlyphRunAnalysis_Impl: Sized {
    fn GetAlphaTextureBounds(&self, texturetype: DWRITE_TEXTURE_TYPE) -> windows_core::Result<super::super::Foundation::RECT>;
    fn CreateAlphaTexture(&self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> windows_core::Result<()>;
    fn GetAlphaBlendParams(&self, renderingparams: Option<&IDWriteRenderingParams>, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteGlyphRunAnalysis {}
impl IDWriteGlyphRunAnalysis_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteGlyphRunAnalysis_Vtbl
    where
        Identity: IDWriteGlyphRunAnalysis_Impl,
    {
        unsafe extern "system" fn GetAlphaTextureBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IDWriteGlyphRunAnalysis_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteGlyphRunAnalysis_Impl::GetAlphaTextureBounds(this, core::mem::transmute_copy(&texturetype)) {
                Ok(ok__) => {
                    texturebounds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAlphaTexture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteGlyphRunAnalysis_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteGlyphRunAnalysis_Impl::CreateAlphaTexture(this, core::mem::transmute_copy(&texturetype), core::mem::transmute_copy(&texturebounds), core::mem::transmute_copy(&alphavalues), core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetAlphaBlendParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingparams: *mut core::ffi::c_void, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDWriteGlyphRunAnalysis_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteGlyphRunAnalysis_Impl::GetAlphaBlendParams(this, windows_core::from_raw_borrowed(&renderingparams), core::mem::transmute_copy(&blendgamma), core::mem::transmute_copy(&blendenhancedcontrast), core::mem::transmute_copy(&blendcleartypelevel)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAlphaTextureBounds: GetAlphaTextureBounds::<Identity, OFFSET>,
            CreateAlphaTexture: CreateAlphaTexture::<Identity, OFFSET>,
            GetAlphaBlendParams: GetAlphaBlendParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteGlyphRunAnalysis as windows_core::Interface>::IID
    }
}
pub trait IDWriteInMemoryFontFileLoader_Impl: Sized + IDWriteFontFileLoader_Impl {
    fn CreateInMemoryFontFileReference(&self, factory: Option<&IDWriteFactory>, fontdata: *const core::ffi::c_void, fontdatasize: u32, ownerobject: Option<&windows_core::IUnknown>) -> windows_core::Result<IDWriteFontFile>;
    fn GetFileCount(&self) -> u32;
}
impl windows_core::RuntimeName for IDWriteInMemoryFontFileLoader {}
impl IDWriteInMemoryFontFileLoader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteInMemoryFontFileLoader_Vtbl
    where
        Identity: IDWriteInMemoryFontFileLoader_Impl,
    {
        unsafe extern "system" fn CreateInMemoryFontFileReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, fontdata: *const core::ffi::c_void, fontdatasize: u32, ownerobject: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteInMemoryFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteInMemoryFontFileLoader_Impl::CreateInMemoryFontFileReference(this, windows_core::from_raw_borrowed(&factory), core::mem::transmute_copy(&fontdata), core::mem::transmute_copy(&fontdatasize), windows_core::from_raw_borrowed(&ownerobject)) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteInMemoryFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteInMemoryFontFileLoader_Impl::GetFileCount(this)
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, OFFSET>(),
            CreateInMemoryFontFileReference: CreateInMemoryFontFileReference::<Identity, OFFSET>,
            GetFileCount: GetFileCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteInMemoryFontFileLoader as windows_core::Interface>::IID || iid == &<IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
pub trait IDWriteInlineObject_Impl: Sized {
    fn Draw(&self, clientdrawingcontext: *const core::ffi::c_void, renderer: Option<&IDWriteTextRenderer>, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetMetrics(&self) -> windows_core::Result<DWRITE_INLINE_OBJECT_METRICS>;
    fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetBreakConditions(&self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteInlineObject {}
impl IDWriteInlineObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteInlineObject_Vtbl
    where
        Identity: IDWriteInlineObject_Impl,
    {
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, renderer: *mut core::ffi::c_void, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteInlineObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteInlineObject_Impl::Draw(this, core::mem::transmute_copy(&clientdrawingcontext), windows_core::from_raw_borrowed(&renderer), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteInlineObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteInlineObject_Impl::GetMetrics(this) {
                Ok(ok__) => {
                    metrics.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteInlineObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteInlineObject_Impl::GetOverhangMetrics(this) {
                Ok(ok__) => {
                    overhangs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakConditions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> windows_core::HRESULT
        where
            Identity: IDWriteInlineObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteInlineObject_Impl::GetBreakConditions(this, core::mem::transmute_copy(&breakconditionbefore), core::mem::transmute_copy(&breakconditionafter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, OFFSET>,
            GetBreakConditions: GetBreakConditions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteInlineObject as windows_core::Interface>::IID
    }
}
pub trait IDWriteLocalFontFileLoader_Impl: Sized + IDWriteFontFileLoader_Impl {
    fn GetFilePathLengthFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<u32>;
    fn GetFilePathFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepath: windows_core::PWSTR, filepathsize: u32) -> windows_core::Result<()>;
    fn GetLastWriteTimeFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl windows_core::RuntimeName for IDWriteLocalFontFileLoader {}
impl IDWriteLocalFontFileLoader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteLocalFontFileLoader_Vtbl
    where
        Identity: IDWriteLocalFontFileLoader_Impl,
    {
        unsafe extern "system" fn GetFilePathLengthFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteLocalFontFileLoader_Impl::GetFilePathLengthFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                Ok(ok__) => {
                    filepathlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilePathFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepath: windows_core::PWSTR, filepathsize: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteLocalFontFileLoader_Impl::GetFilePathFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize), core::mem::transmute_copy(&filepath), core::mem::transmute_copy(&filepathsize)).into()
        }
        unsafe extern "system" fn GetLastWriteTimeFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteLocalFontFileLoader_Impl::GetLastWriteTimeFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                Ok(ok__) => {
                    lastwritetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, OFFSET>(),
            GetFilePathLengthFromKey: GetFilePathLengthFromKey::<Identity, OFFSET>,
            GetFilePathFromKey: GetFilePathFromKey::<Identity, OFFSET>,
            GetLastWriteTimeFromKey: GetLastWriteTimeFromKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteLocalFontFileLoader as windows_core::Interface>::IID || iid == &<IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
pub trait IDWriteLocalizedStrings_Impl: Sized {
    fn GetCount(&self) -> u32;
    fn FindLocaleName(&self, localename: &windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLocaleNameLength(&self, index: u32) -> windows_core::Result<u32>;
    fn GetLocaleName(&self, index: u32, localename: windows_core::PWSTR, size: u32) -> windows_core::Result<()>;
    fn GetStringLength(&self, index: u32) -> windows_core::Result<u32>;
    fn GetString(&self, index: u32, stringbuffer: windows_core::PWSTR, size: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteLocalizedStrings {}
impl IDWriteLocalizedStrings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteLocalizedStrings_Vtbl
    where
        Identity: IDWriteLocalizedStrings_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteLocalizedStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteLocalizedStrings_Impl::GetCount(this)
        }
        unsafe extern "system" fn FindLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localename: windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalizedStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteLocalizedStrings_Impl::FindLocaleName(this, core::mem::transmute(&localename), core::mem::transmute_copy(&index), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, length: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalizedStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteLocalizedStrings_Impl::GetLocaleNameLength(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, localename: windows_core::PWSTR, size: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalizedStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteLocalizedStrings_Impl::GetLocaleName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&localename), core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn GetStringLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, length: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalizedStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteLocalizedStrings_Impl::GetStringLength(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stringbuffer: windows_core::PWSTR, size: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteLocalizedStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteLocalizedStrings_Impl::GetString(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&stringbuffer), core::mem::transmute_copy(&size)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            FindLocaleName: FindLocaleName::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteLocalizedStrings as windows_core::Interface>::IID
    }
}
pub trait IDWriteNumberSubstitution_Impl: Sized {}
impl windows_core::RuntimeName for IDWriteNumberSubstitution {}
impl IDWriteNumberSubstitution_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteNumberSubstitution_Vtbl
    where
        Identity: IDWriteNumberSubstitution_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteNumberSubstitution as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDWritePaintReader_Impl: Sized {
    fn SetCurrentGlyph(&self, glyphindex: u32, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32, clipbox: *mut super::Direct2D::Common::D2D_RECT_F, glyphattributes: *mut DWRITE_PAINT_ATTRIBUTES) -> windows_core::Result<()>;
    fn SetTextColor(&self, textcolor: *const DWRITE_COLOR_F) -> windows_core::Result<()>;
    fn SetColorPaletteIndex(&self, colorpaletteindex: u32) -> windows_core::Result<()>;
    fn SetCustomColorPalette(&self, paletteentries: *const DWRITE_COLOR_F, paletteentrycount: u32) -> windows_core::Result<()>;
    fn MoveToFirstChild(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::Result<()>;
    fn MoveToNextSibling(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::Result<()>;
    fn MoveToParent(&self) -> windows_core::Result<()>;
    fn GetGradientStops(&self, firstgradientstopindex: u32, gradientstopcount: u32, gradientstops: *mut super::Direct2D::Common::D2D1_GRADIENT_STOP) -> windows_core::Result<()>;
    fn GetGradientStopColors(&self, firstgradientstopindex: u32, gradientstopcount: u32, gradientstopcolors: *mut DWRITE_PAINT_COLOR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for IDWritePaintReader {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl IDWritePaintReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWritePaintReader_Vtbl
    where
        Identity: IDWritePaintReader_Impl,
    {
        unsafe extern "system" fn SetCurrentGlyph<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindex: u32, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32, clipbox: *mut super::Direct2D::Common::D2D_RECT_F, glyphattributes: *mut DWRITE_PAINT_ATTRIBUTES) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::SetCurrentGlyph(this, core::mem::transmute_copy(&glyphindex), core::mem::transmute_copy(&paintelement), core::mem::transmute_copy(&structsize), core::mem::transmute_copy(&clipbox), core::mem::transmute_copy(&glyphattributes)).into()
        }
        unsafe extern "system" fn SetTextColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textcolor: *const DWRITE_COLOR_F) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::SetTextColor(this, core::mem::transmute_copy(&textcolor)).into()
        }
        unsafe extern "system" fn SetColorPaletteIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorpaletteindex: u32) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::SetColorPaletteIndex(this, core::mem::transmute_copy(&colorpaletteindex)).into()
        }
        unsafe extern "system" fn SetCustomColorPalette<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paletteentries: *const DWRITE_COLOR_F, paletteentrycount: u32) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::SetCustomColorPalette(this, core::mem::transmute_copy(&paletteentries), core::mem::transmute_copy(&paletteentrycount)).into()
        }
        unsafe extern "system" fn MoveToFirstChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::MoveToFirstChild(this, core::mem::transmute_copy(&paintelement), core::mem::transmute_copy(&structsize)).into()
        }
        unsafe extern "system" fn MoveToNextSibling<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::MoveToNextSibling(this, core::mem::transmute_copy(&paintelement), core::mem::transmute_copy(&structsize)).into()
        }
        unsafe extern "system" fn MoveToParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::MoveToParent(this).into()
        }
        unsafe extern "system" fn GetGradientStops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstgradientstopindex: u32, gradientstopcount: u32, gradientstops: *mut super::Direct2D::Common::D2D1_GRADIENT_STOP) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::GetGradientStops(this, core::mem::transmute_copy(&firstgradientstopindex), core::mem::transmute_copy(&gradientstopcount), core::mem::transmute_copy(&gradientstops)).into()
        }
        unsafe extern "system" fn GetGradientStopColors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstgradientstopindex: u32, gradientstopcount: u32, gradientstopcolors: *mut DWRITE_PAINT_COLOR) -> windows_core::HRESULT
        where
            Identity: IDWritePaintReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePaintReader_Impl::GetGradientStopColors(this, core::mem::transmute_copy(&firstgradientstopindex), core::mem::transmute_copy(&gradientstopcount), core::mem::transmute_copy(&gradientstopcolors)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCurrentGlyph: SetCurrentGlyph::<Identity, OFFSET>,
            SetTextColor: SetTextColor::<Identity, OFFSET>,
            SetColorPaletteIndex: SetColorPaletteIndex::<Identity, OFFSET>,
            SetCustomColorPalette: SetCustomColorPalette::<Identity, OFFSET>,
            MoveToFirstChild: MoveToFirstChild::<Identity, OFFSET>,
            MoveToNextSibling: MoveToNextSibling::<Identity, OFFSET>,
            MoveToParent: MoveToParent::<Identity, OFFSET>,
            GetGradientStops: GetGradientStops::<Identity, OFFSET>,
            GetGradientStopColors: GetGradientStopColors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWritePaintReader as windows_core::Interface>::IID
    }
}
pub trait IDWritePixelSnapping_Impl: Sized {
    fn IsPixelSnappingDisabled(&self, clientdrawingcontext: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentTransform(&self, clientdrawingcontext: *const core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetPixelsPerDip(&self, clientdrawingcontext: *const core::ffi::c_void) -> windows_core::Result<f32>;
}
impl windows_core::RuntimeName for IDWritePixelSnapping {}
impl IDWritePixelSnapping_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWritePixelSnapping_Vtbl
    where
        Identity: IDWritePixelSnapping_Impl,
    {
        unsafe extern "system" fn IsPixelSnappingDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWritePixelSnapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWritePixelSnapping_Impl::IsPixelSnappingDisabled(this, core::mem::transmute_copy(&clientdrawingcontext)) {
                Ok(ok__) => {
                    isdisabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT
        where
            Identity: IDWritePixelSnapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWritePixelSnapping_Impl::GetCurrentTransform(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, pixelsperdip: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDWritePixelSnapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWritePixelSnapping_Impl::GetPixelsPerDip(this, core::mem::transmute_copy(&clientdrawingcontext)) {
                Ok(ok__) => {
                    pixelsperdip.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPixelSnappingDisabled: IsPixelSnappingDisabled::<Identity, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWritePixelSnapping as windows_core::Interface>::IID
    }
}
pub trait IDWriteRemoteFontFileLoader_Impl: Sized + IDWriteFontFileLoader_Impl {
    fn CreateRemoteStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteRemoteFontFileStream>;
    fn GetLocalityFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<DWRITE_LOCALITY>;
    fn CreateFontFileReferenceFromUrl(&self, factory: Option<&IDWriteFactory>, baseurl: &windows_core::PCWSTR, fontfileurl: &windows_core::PCWSTR) -> windows_core::Result<IDWriteFontFile>;
}
impl windows_core::RuntimeName for IDWriteRemoteFontFileLoader {}
impl IDWriteRemoteFontFileLoader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteRemoteFontFileLoader_Vtbl
    where
        Identity: IDWriteRemoteFontFileLoader_Impl,
    {
        unsafe extern "system" fn CreateRemoteStreamFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteRemoteFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteRemoteFontFileLoader_Impl::CreateRemoteStreamFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                Ok(ok__) => {
                    fontfilestream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalityFromKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> windows_core::HRESULT
        where
            Identity: IDWriteRemoteFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteRemoteFontFileLoader_Impl::GetLocalityFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                Ok(ok__) => {
                    locality.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFontFileReferenceFromUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, baseurl: windows_core::PCWSTR, fontfileurl: windows_core::PCWSTR, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteRemoteFontFileLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteRemoteFontFileLoader_Impl::CreateFontFileReferenceFromUrl(this, windows_core::from_raw_borrowed(&factory), core::mem::transmute(&baseurl), core::mem::transmute(&fontfileurl)) {
                Ok(ok__) => {
                    fontfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, OFFSET>(),
            CreateRemoteStreamFromKey: CreateRemoteStreamFromKey::<Identity, OFFSET>,
            GetLocalityFromKey: GetLocalityFromKey::<Identity, OFFSET>,
            CreateFontFileReferenceFromUrl: CreateFontFileReferenceFromUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileLoader as windows_core::Interface>::IID || iid == &<IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
pub trait IDWriteRemoteFontFileStream_Impl: Sized + IDWriteFontFileStream_Impl {
    fn GetLocalFileSize(&self) -> windows_core::Result<u64>;
    fn GetFileFragmentLocality(&self, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> windows_core::Result<()>;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
    fn BeginDownload(&self, downloadoperationid: *const windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32) -> windows_core::Result<IDWriteAsyncResult>;
}
impl windows_core::RuntimeName for IDWriteRemoteFontFileStream {}
impl IDWriteRemoteFontFileStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteRemoteFontFileStream_Vtbl
    where
        Identity: IDWriteRemoteFontFileStream_Impl,
    {
        unsafe extern "system" fn GetLocalFileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localfilesize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IDWriteRemoteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteRemoteFontFileStream_Impl::GetLocalFileSize(this) {
                Ok(ok__) => {
                    localfilesize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileFragmentLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IDWriteRemoteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRemoteFontFileStream_Impl::GetFileFragmentLocality(this, core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&fragmentsize), core::mem::transmute_copy(&islocal), core::mem::transmute_copy(&partialsize)).into()
        }
        unsafe extern "system" fn GetLocality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_LOCALITY
        where
            Identity: IDWriteRemoteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRemoteFontFileStream_Impl::GetLocality(this)
        }
        unsafe extern "system" fn BeginDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadoperationid: *const windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteRemoteFontFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteRemoteFontFileStream_Impl::BeginDownload(this, core::mem::transmute_copy(&downloadoperationid), core::mem::transmute_copy(&filefragments), core::mem::transmute_copy(&fragmentcount)) {
                Ok(ok__) => {
                    asyncresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteFontFileStream_Vtbl::new::<Identity, OFFSET>(),
            GetLocalFileSize: GetLocalFileSize::<Identity, OFFSET>,
            GetFileFragmentLocality: GetFileFragmentLocality::<Identity, OFFSET>,
            GetLocality: GetLocality::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileStream as windows_core::Interface>::IID || iid == &<IDWriteFontFileStream as windows_core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams_Impl: Sized {
    fn GetGamma(&self) -> f32;
    fn GetEnhancedContrast(&self) -> f32;
    fn GetClearTypeLevel(&self) -> f32;
    fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY;
    fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE;
}
impl windows_core::RuntimeName for IDWriteRenderingParams {}
impl IDWriteRenderingParams_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteRenderingParams_Vtbl
    where
        Identity: IDWriteRenderingParams_Impl,
    {
        unsafe extern "system" fn GetGamma<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteRenderingParams_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams_Impl::GetGamma(this)
        }
        unsafe extern "system" fn GetEnhancedContrast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteRenderingParams_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams_Impl::GetEnhancedContrast(this)
        }
        unsafe extern "system" fn GetClearTypeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteRenderingParams_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams_Impl::GetClearTypeLevel(this)
        }
        unsafe extern "system" fn GetPixelGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY
        where
            Identity: IDWriteRenderingParams_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams_Impl::GetPixelGeometry(this)
        }
        unsafe extern "system" fn GetRenderingMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_RENDERING_MODE
        where
            Identity: IDWriteRenderingParams_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams_Impl::GetRenderingMode(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGamma: GetGamma::<Identity, OFFSET>,
            GetEnhancedContrast: GetEnhancedContrast::<Identity, OFFSET>,
            GetClearTypeLevel: GetClearTypeLevel::<Identity, OFFSET>,
            GetPixelGeometry: GetPixelGeometry::<Identity, OFFSET>,
            GetRenderingMode: GetRenderingMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams as windows_core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams1_Impl: Sized + IDWriteRenderingParams_Impl {
    fn GetGrayscaleEnhancedContrast(&self) -> f32;
}
impl windows_core::RuntimeName for IDWriteRenderingParams1 {}
impl IDWriteRenderingParams1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteRenderingParams1_Vtbl
    where
        Identity: IDWriteRenderingParams1_Impl,
    {
        unsafe extern "system" fn GetGrayscaleEnhancedContrast<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteRenderingParams1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams1_Impl::GetGrayscaleEnhancedContrast(this)
        }
        Self { base__: IDWriteRenderingParams_Vtbl::new::<Identity, OFFSET>(), GetGrayscaleEnhancedContrast: GetGrayscaleEnhancedContrast::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams1 as windows_core::Interface>::IID || iid == &<IDWriteRenderingParams as windows_core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams2_Impl: Sized + IDWriteRenderingParams1_Impl {
    fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE;
}
impl windows_core::RuntimeName for IDWriteRenderingParams2 {}
impl IDWriteRenderingParams2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteRenderingParams2_Vtbl
    where
        Identity: IDWriteRenderingParams2_Impl,
    {
        unsafe extern "system" fn GetGridFitMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_GRID_FIT_MODE
        where
            Identity: IDWriteRenderingParams2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams2_Impl::GetGridFitMode(this)
        }
        Self { base__: IDWriteRenderingParams1_Vtbl::new::<Identity, OFFSET>(), GetGridFitMode: GetGridFitMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams2 as windows_core::Interface>::IID || iid == &<IDWriteRenderingParams as windows_core::Interface>::IID || iid == &<IDWriteRenderingParams1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteRenderingParams3_Impl: Sized + IDWriteRenderingParams2_Impl {
    fn GetRenderingMode1(&self) -> DWRITE_RENDERING_MODE1;
}
impl windows_core::RuntimeName for IDWriteRenderingParams3 {}
impl IDWriteRenderingParams3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteRenderingParams3_Vtbl
    where
        Identity: IDWriteRenderingParams3_Impl,
    {
        unsafe extern "system" fn GetRenderingMode1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_RENDERING_MODE1
        where
            Identity: IDWriteRenderingParams3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteRenderingParams3_Impl::GetRenderingMode1(this)
        }
        Self { base__: IDWriteRenderingParams2_Vtbl::new::<Identity, OFFSET>(), GetRenderingMode1: GetRenderingMode1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams3 as windows_core::Interface>::IID || iid == &<IDWriteRenderingParams as windows_core::Interface>::IID || iid == &<IDWriteRenderingParams1 as windows_core::Interface>::IID || iid == &<IDWriteRenderingParams2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteStringList_Impl: Sized {
    fn GetCount(&self) -> u32;
    fn GetLocaleNameLength(&self, listindex: u32) -> windows_core::Result<u32>;
    fn GetLocaleName(&self, listindex: u32, localename: windows_core::PWSTR, size: u32) -> windows_core::Result<()>;
    fn GetStringLength(&self, listindex: u32) -> windows_core::Result<u32>;
    fn GetString(&self, listindex: u32, stringbuffer: windows_core::PWSTR, stringbuffersize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteStringList {}
impl IDWriteStringList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteStringList_Vtbl
    where
        Identity: IDWriteStringList_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteStringList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteStringList_Impl::GetCount(this)
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, length: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteStringList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteStringList_Impl::GetLocaleNameLength(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, localename: windows_core::PWSTR, size: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteStringList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteStringList_Impl::GetLocaleName(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&localename), core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn GetStringLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, length: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteStringList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteStringList_Impl::GetStringLength(this, core::mem::transmute_copy(&listindex)) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, stringbuffer: windows_core::PWSTR, stringbuffersize: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteStringList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteStringList_Impl::GetString(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&stringbuffer), core::mem::transmute_copy(&stringbuffersize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteStringList as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSink_Impl: Sized {
    fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::Result<()>;
    fn SetLineBreakpoints(&self, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> windows_core::Result<()>;
    fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> windows_core::Result<()>;
    fn SetNumberSubstitution(&self, textposition: u32, textlength: u32, numbersubstitution: Option<&IDWriteNumberSubstitution>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalysisSink {}
impl IDWriteTextAnalysisSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalysisSink_Vtbl
    where
        Identity: IDWriteTextAnalysisSink_Impl,
    {
        unsafe extern "system" fn SetScriptAnalysis<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSink_Impl::SetScriptAnalysis(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&scriptanalysis)).into()
        }
        unsafe extern "system" fn SetLineBreakpoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSink_Impl::SetLineBreakpoints(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&linebreakpoints)).into()
        }
        unsafe extern "system" fn SetBidiLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSink_Impl::SetBidiLevel(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&explicitlevel), core::mem::transmute_copy(&resolvedlevel)).into()
        }
        unsafe extern "system" fn SetNumberSubstitution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSink_Impl::SetNumberSubstitution(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&numbersubstitution)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScriptAnalysis: SetScriptAnalysis::<Identity, OFFSET>,
            SetLineBreakpoints: SetLineBreakpoints::<Identity, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, OFFSET>,
            SetNumberSubstitution: SetNumberSubstitution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSink1_Impl: Sized + IDWriteTextAnalysisSink_Impl {
    fn SetGlyphOrientation(&self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalysisSink1 {}
impl IDWriteTextAnalysisSink1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalysisSink1_Vtbl
    where
        Identity: IDWriteTextAnalysisSink1_Impl,
    {
        unsafe extern "system" fn SetGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSink1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSink1_Impl::SetGlyphOrientation(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&glyphorientationangle), core::mem::transmute_copy(&adjustedbidilevel), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft)).into()
        }
        Self { base__: IDWriteTextAnalysisSink_Vtbl::new::<Identity, OFFSET>(), SetGlyphOrientation: SetGlyphOrientation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink1 as windows_core::Interface>::IID || iid == &<IDWriteTextAnalysisSink as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSource_Impl: Sized {
    fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::Result<()>;
    fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::Result<()>;
    fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> windows_core::Result<()>;
    fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut Option<IDWriteNumberSubstitution>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalysisSource {}
impl IDWriteTextAnalysisSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalysisSource_Vtbl
    where
        Identity: IDWriteTextAnalysisSource_Impl,
    {
        unsafe extern "system" fn GetTextAtPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSource_Impl::GetTextAtPosition(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textstring), core::mem::transmute_copy(&textlength)).into()
        }
        unsafe extern "system" fn GetTextBeforePosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSource_Impl::GetTextBeforePosition(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textstring), core::mem::transmute_copy(&textlength)).into()
        }
        unsafe extern "system" fn GetParagraphReadingDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_READING_DIRECTION
        where
            Identity: IDWriteTextAnalysisSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSource_Impl::GetParagraphReadingDirection(this)
        }
        unsafe extern "system" fn GetLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSource_Impl::GetLocaleName(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&localename)).into()
        }
        unsafe extern "system" fn GetNumberSubstitution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSource_Impl::GetNumberSubstitution(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&numbersubstitution)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTextAtPosition: GetTextAtPosition::<Identity, OFFSET>,
            GetTextBeforePosition: GetTextBeforePosition::<Identity, OFFSET>,
            GetParagraphReadingDirection: GetParagraphReadingDirection::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            GetNumberSubstitution: GetNumberSubstitution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalysisSource1_Impl: Sized + IDWriteTextAnalysisSource_Impl {
    fn GetVerticalGlyphOrientation(&self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalysisSource1 {}
impl IDWriteTextAnalysisSource1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalysisSource1_Vtbl
    where
        Identity: IDWriteTextAnalysisSource1_Impl,
    {
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalysisSource1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalysisSource1_Impl::GetVerticalGlyphOrientation(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&glyphorientation), core::mem::transmute_copy(&bidilevel)).into()
        }
        Self { base__: IDWriteTextAnalysisSource_Vtbl::new::<Identity, OFFSET>(), GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource1 as windows_core::Interface>::IID || iid == &<IDWriteTextAnalysisSource as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalyzer_Impl: Sized {
    fn AnalyzeScript(&self, analysissource: Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: Option<&IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn AnalyzeBidi(&self, analysissource: Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: Option<&IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn AnalyzeNumberSubstitution(&self, analysissource: Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: Option<&IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn AnalyzeLineBreakpoints(&self, analysissource: Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: Option<&IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn GetGlyphs(&self, textstring: &windows_core::PCWSTR, textlength: u32, fontface: Option<&IDWriteFontFace>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, numbersubstitution: Option<&IDWriteNumberSubstitution>, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> windows_core::Result<()>;
    fn GetGlyphPlacements(&self, textstring: &windows_core::PCWSTR, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: Option<&IDWriteFontFace>, fontemsize: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
    fn GetGdiCompatibleGlyphPlacements(&self, textstring: &windows_core::PCWSTR, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: Option<&IDWriteFontFace>, fontemsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalyzer {}
impl IDWriteTextAnalyzer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalyzer_Vtbl
    where
        Identity: IDWriteTextAnalyzer_Impl,
    {
        unsafe extern "system" fn AnalyzeScript<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::AnalyzeScript(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn AnalyzeBidi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::AnalyzeBidi(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn AnalyzeNumberSubstitution<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::AnalyzeNumberSubstitution(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn AnalyzeLineBreakpoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::AnalyzeLineBreakpoints(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn GetGlyphs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textstring: windows_core::PCWSTR, textlength: u32, fontface: *mut core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, numbersubstitution: *mut core::ffi::c_void, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::GetGlyphs(
                this,
                core::mem::transmute(&textstring),
                core::mem::transmute_copy(&textlength),
                windows_core::from_raw_borrowed(&fontface),
                core::mem::transmute_copy(&issideways),
                core::mem::transmute_copy(&isrighttoleft),
                core::mem::transmute_copy(&scriptanalysis),
                core::mem::transmute(&localename),
                windows_core::from_raw_borrowed(&numbersubstitution),
                core::mem::transmute_copy(&features),
                core::mem::transmute_copy(&featurerangelengths),
                core::mem::transmute_copy(&featureranges),
                core::mem::transmute_copy(&maxglyphcount),
                core::mem::transmute_copy(&clustermap),
                core::mem::transmute_copy(&textprops),
                core::mem::transmute_copy(&glyphindices),
                core::mem::transmute_copy(&glyphprops),
                core::mem::transmute_copy(&actualglyphcount),
            )
            .into()
        }
        unsafe extern "system" fn GetGlyphPlacements<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textstring: windows_core::PCWSTR, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: *mut core::ffi::c_void, fontemsize: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::GetGlyphPlacements(
                this,
                core::mem::transmute(&textstring),
                core::mem::transmute_copy(&clustermap),
                core::mem::transmute_copy(&textprops),
                core::mem::transmute_copy(&textlength),
                core::mem::transmute_copy(&glyphindices),
                core::mem::transmute_copy(&glyphprops),
                core::mem::transmute_copy(&glyphcount),
                windows_core::from_raw_borrowed(&fontface),
                core::mem::transmute_copy(&fontemsize),
                core::mem::transmute_copy(&issideways),
                core::mem::transmute_copy(&isrighttoleft),
                core::mem::transmute_copy(&scriptanalysis),
                core::mem::transmute(&localename),
                core::mem::transmute_copy(&features),
                core::mem::transmute_copy(&featurerangelengths),
                core::mem::transmute_copy(&featureranges),
                core::mem::transmute_copy(&glyphadvances),
                core::mem::transmute_copy(&glyphoffsets),
            )
            .into()
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphPlacements<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            textstring: windows_core::PCWSTR,
            clustermap: *const u16,
            textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: *mut core::ffi::c_void,
            fontemsize: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: super::super::Foundation::BOOL,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: windows_core::PCWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer_Impl::GetGdiCompatibleGlyphPlacements(
                this,
                core::mem::transmute(&textstring),
                core::mem::transmute_copy(&clustermap),
                core::mem::transmute_copy(&textprops),
                core::mem::transmute_copy(&textlength),
                core::mem::transmute_copy(&glyphindices),
                core::mem::transmute_copy(&glyphprops),
                core::mem::transmute_copy(&glyphcount),
                windows_core::from_raw_borrowed(&fontface),
                core::mem::transmute_copy(&fontemsize),
                core::mem::transmute_copy(&pixelsperdip),
                core::mem::transmute_copy(&transform),
                core::mem::transmute_copy(&usegdinatural),
                core::mem::transmute_copy(&issideways),
                core::mem::transmute_copy(&isrighttoleft),
                core::mem::transmute_copy(&scriptanalysis),
                core::mem::transmute(&localename),
                core::mem::transmute_copy(&features),
                core::mem::transmute_copy(&featurerangelengths),
                core::mem::transmute_copy(&featureranges),
                core::mem::transmute_copy(&glyphadvances),
                core::mem::transmute_copy(&glyphoffsets),
            )
            .into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AnalyzeScript: AnalyzeScript::<Identity, OFFSET>,
            AnalyzeBidi: AnalyzeBidi::<Identity, OFFSET>,
            AnalyzeNumberSubstitution: AnalyzeNumberSubstitution::<Identity, OFFSET>,
            AnalyzeLineBreakpoints: AnalyzeLineBreakpoints::<Identity, OFFSET>,
            GetGlyphs: GetGlyphs::<Identity, OFFSET>,
            GetGlyphPlacements: GetGlyphPlacements::<Identity, OFFSET>,
            GetGdiCompatibleGlyphPlacements: GetGdiCompatibleGlyphPlacements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalyzer1_Impl: Sized + IDWriteTextAnalyzer_Impl {
    fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
    fn GetBaseline(&self, fontface: Option<&IDWriteFontFace>, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AnalyzeVerticalGlyphOrientation(&self, analysissource: Option<&IDWriteTextAnalysisSource1>, textposition: u32, textlength: u32, analysissink: Option<&IDWriteTextAnalysisSink1>) -> windows_core::Result<()>;
    fn GetGlyphOrientationTransform(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetScriptProperties(&self, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::Result<()>;
    fn GetTextComplexity(&self, textstring: &windows_core::PCWSTR, textlength: u32, fontface: Option<&IDWriteFontFace>, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> windows_core::Result<()>;
    fn GetJustificationOpportunities(&self, fontface: Option<&IDWriteFontFace>, fontemsize: f32, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: &windows_core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::Result<()>;
    fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
    fn GetJustifiedGlyphs(&self, fontface: Option<&IDWriteFontFace>, fontemsize: f32, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalyzer1 {}
impl IDWriteTextAnalyzer1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalyzer1_Vtbl
    where
        Identity: IDWriteTextAnalyzer1_Impl,
    {
        unsafe extern "system" fn ApplyCharacterSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::ApplyCharacterSpacing(
                this,
                core::mem::transmute_copy(&leadingspacing),
                core::mem::transmute_copy(&trailingspacing),
                core::mem::transmute_copy(&minimumadvancewidth),
                core::mem::transmute_copy(&textlength),
                core::mem::transmute_copy(&glyphcount),
                core::mem::transmute_copy(&clustermap),
                core::mem::transmute_copy(&glyphadvances),
                core::mem::transmute_copy(&glyphoffsets),
                core::mem::transmute_copy(&glyphproperties),
                core::mem::transmute_copy(&modifiedglyphadvances),
                core::mem::transmute_copy(&modifiedglyphoffsets),
            )
            .into()
        }
        unsafe extern "system" fn GetBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::GetBaseline(this, windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&baseline), core::mem::transmute_copy(&isvertical), core::mem::transmute_copy(&issimulationallowed), core::mem::transmute(&scriptanalysis), core::mem::transmute(&localename), core::mem::transmute_copy(&baselinecoordinate), core::mem::transmute_copy(&exists)).into()
        }
        unsafe extern "system" fn AnalyzeVerticalGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::AnalyzeVerticalGlyphOrientation(this, windows_core::from_raw_borrowed(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&analysissink)).into()
        }
        unsafe extern "system" fn GetGlyphOrientationTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::GetGlyphOrientationTransform(this, core::mem::transmute_copy(&glyphorientationangle), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetScriptProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::GetScriptProperties(this, core::mem::transmute(&scriptanalysis), core::mem::transmute_copy(&scriptproperties)).into()
        }
        unsafe extern "system" fn GetTextComplexity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textstring: windows_core::PCWSTR, textlength: u32, fontface: *mut core::ffi::c_void, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::GetTextComplexity(this, core::mem::transmute(&textstring), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&istextsimple), core::mem::transmute_copy(&textlengthread), core::mem::transmute_copy(&glyphindices)).into()
        }
        unsafe extern "system" fn GetJustificationOpportunities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: windows_core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::GetJustificationOpportunities(this, windows_core::from_raw_borrowed(&fontface), core::mem::transmute_copy(&fontemsize), core::mem::transmute(&scriptanalysis), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&glyphcount), core::mem::transmute(&textstring), core::mem::transmute_copy(&clustermap), core::mem::transmute_copy(&glyphproperties), core::mem::transmute_copy(&justificationopportunities)).into()
        }
        unsafe extern "system" fn JustifyGlyphAdvances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::JustifyGlyphAdvances(this, core::mem::transmute_copy(&linewidth), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&justificationopportunities), core::mem::transmute_copy(&glyphadvances), core::mem::transmute_copy(&glyphoffsets), core::mem::transmute_copy(&justifiedglyphadvances), core::mem::transmute_copy(&justifiedglyphoffsets)).into()
        }
        unsafe extern "system" fn GetJustifiedGlyphs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer1_Impl::GetJustifiedGlyphs(
                this,
                windows_core::from_raw_borrowed(&fontface),
                core::mem::transmute_copy(&fontemsize),
                core::mem::transmute(&scriptanalysis),
                core::mem::transmute_copy(&textlength),
                core::mem::transmute_copy(&glyphcount),
                core::mem::transmute_copy(&maxglyphcount),
                core::mem::transmute_copy(&clustermap),
                core::mem::transmute_copy(&glyphindices),
                core::mem::transmute_copy(&glyphadvances),
                core::mem::transmute_copy(&justifiedglyphadvances),
                core::mem::transmute_copy(&justifiedglyphoffsets),
                core::mem::transmute_copy(&glyphproperties),
                core::mem::transmute_copy(&actualglyphcount),
                core::mem::transmute_copy(&modifiedclustermap),
                core::mem::transmute_copy(&modifiedglyphindices),
                core::mem::transmute_copy(&modifiedglyphadvances),
                core::mem::transmute_copy(&modifiedglyphoffsets),
            )
            .into()
        }
        Self {
            base__: IDWriteTextAnalyzer_Vtbl::new::<Identity, OFFSET>(),
            ApplyCharacterSpacing: ApplyCharacterSpacing::<Identity, OFFSET>,
            GetBaseline: GetBaseline::<Identity, OFFSET>,
            AnalyzeVerticalGlyphOrientation: AnalyzeVerticalGlyphOrientation::<Identity, OFFSET>,
            GetGlyphOrientationTransform: GetGlyphOrientationTransform::<Identity, OFFSET>,
            GetScriptProperties: GetScriptProperties::<Identity, OFFSET>,
            GetTextComplexity: GetTextComplexity::<Identity, OFFSET>,
            GetJustificationOpportunities: GetJustificationOpportunities::<Identity, OFFSET>,
            JustifyGlyphAdvances: JustifyGlyphAdvances::<Identity, OFFSET>,
            GetJustifiedGlyphs: GetJustifiedGlyphs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer1 as windows_core::Interface>::IID || iid == &<IDWriteTextAnalyzer as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextAnalyzer2_Impl: Sized + IDWriteTextAnalyzer1_Impl {
    fn GetGlyphOrientationTransform(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetTypographicFeatures(&self, fontface: Option<&IDWriteFontFace>, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> windows_core::Result<()>;
    fn CheckTypographicFeature(&self, fontface: Option<&IDWriteFontFace>, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextAnalyzer2 {}
impl IDWriteTextAnalyzer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextAnalyzer2_Vtbl
    where
        Identity: IDWriteTextAnalyzer2_Impl,
    {
        unsafe extern "system" fn GetGlyphOrientationTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer2_Impl::GetGlyphOrientationTransform(this, core::mem::transmute_copy(&glyphorientationangle), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn GetTypographicFeatures<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer2_Impl::GetTypographicFeatures(this, windows_core::from_raw_borrowed(&fontface), core::mem::transmute(&scriptanalysis), core::mem::transmute(&localename), core::mem::transmute_copy(&maxtagcount), core::mem::transmute_copy(&actualtagcount), core::mem::transmute_copy(&tags)).into()
        }
        unsafe extern "system" fn CheckTypographicFeature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> windows_core::HRESULT
        where
            Identity: IDWriteTextAnalyzer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextAnalyzer2_Impl::CheckTypographicFeature(this, windows_core::from_raw_borrowed(&fontface), core::mem::transmute(&scriptanalysis), core::mem::transmute(&localename), core::mem::transmute_copy(&featuretag), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&featureapplies)).into()
        }
        Self {
            base__: IDWriteTextAnalyzer1_Vtbl::new::<Identity, OFFSET>(),
            GetGlyphOrientationTransform: GetGlyphOrientationTransform::<Identity, OFFSET>,
            GetTypographicFeatures: GetTypographicFeatures::<Identity, OFFSET>,
            CheckTypographicFeature: CheckTypographicFeature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer2 as windows_core::Interface>::IID || iid == &<IDWriteTextAnalyzer as windows_core::Interface>::IID || iid == &<IDWriteTextAnalyzer1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextFormat_Impl: Sized {
    fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::Result<()>;
    fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::Result<()>;
    fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::Result<()>;
    fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> windows_core::Result<()>;
    fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::Result<()>;
    fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::Result<()>;
    fn SetTrimming(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: Option<&IDWriteInlineObject>) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> windows_core::Result<()>;
    fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT;
    fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT;
    fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING;
    fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION;
    fn GetIncrementalTabStop(&self) -> f32;
    fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut Option<IDWriteInlineObject>) -> windows_core::Result<()>;
    fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> windows_core::Result<()>;
    fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection>;
    fn GetFontFamilyNameLength(&self) -> u32;
    fn GetFontFamilyName(&self, fontfamilyname: windows_core::PWSTR, namesize: u32) -> windows_core::Result<()>;
    fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetFontStyle(&self) -> DWRITE_FONT_STYLE;
    fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetFontSize(&self) -> f32;
    fn GetLocaleNameLength(&self) -> u32;
    fn GetLocaleName(&self, localename: windows_core::PWSTR, namesize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextFormat {}
impl IDWriteTextFormat_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextFormat_Vtbl
    where
        Identity: IDWriteTextFormat_Impl,
    {
        unsafe extern "system" fn SetTextAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetTextAlignment(this, core::mem::transmute_copy(&textalignment)).into()
        }
        unsafe extern "system" fn SetParagraphAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetParagraphAlignment(this, core::mem::transmute_copy(&paragraphalignment)).into()
        }
        unsafe extern "system" fn SetWordWrapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetWordWrapping(this, core::mem::transmute_copy(&wordwrapping)).into()
        }
        unsafe extern "system" fn SetReadingDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetReadingDirection(this, core::mem::transmute_copy(&readingdirection)).into()
        }
        unsafe extern "system" fn SetFlowDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetFlowDirection(this, core::mem::transmute_copy(&flowdirection)).into()
        }
        unsafe extern "system" fn SetIncrementalTabStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, incrementaltabstop: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetIncrementalTabStop(this, core::mem::transmute_copy(&incrementaltabstop)).into()
        }
        unsafe extern "system" fn SetTrimming<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetTrimming(this, core::mem::transmute_copy(&trimmingoptions), windows_core::from_raw_borrowed(&trimmingsign)).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::SetLineSpacing(this, core::mem::transmute_copy(&linespacingmethod), core::mem::transmute_copy(&linespacing), core::mem::transmute_copy(&baseline)).into()
        }
        unsafe extern "system" fn GetTextAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetTextAlignment(this)
        }
        unsafe extern "system" fn GetParagraphAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetParagraphAlignment(this)
        }
        unsafe extern "system" fn GetWordWrapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_WORD_WRAPPING
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetWordWrapping(this)
        }
        unsafe extern "system" fn GetReadingDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_READING_DIRECTION
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetReadingDirection(this)
        }
        unsafe extern "system" fn GetFlowDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FLOW_DIRECTION
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFlowDirection(this)
        }
        unsafe extern "system" fn GetIncrementalTabStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetIncrementalTabStop(this)
        }
        unsafe extern "system" fn GetTrimming<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetTrimming(this, core::mem::transmute_copy(&trimmingoptions), core::mem::transmute_copy(&trimmingsign)).into()
        }
        unsafe extern "system" fn GetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetLineSpacing(this, core::mem::transmute_copy(&linespacingmethod), core::mem::transmute_copy(&linespacing), core::mem::transmute_copy(&baseline)).into()
        }
        unsafe extern "system" fn GetFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteTextFormat_Impl::GetFontCollection(this) {
                Ok(ok__) => {
                    fontcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFontFamilyNameLength(this)
        }
        unsafe extern "system" fn GetFontFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PWSTR, namesize: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFontFamilyName(this, core::mem::transmute_copy(&fontfamilyname), core::mem::transmute_copy(&namesize)).into()
        }
        unsafe extern "system" fn GetFontWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFontWeight(this)
        }
        unsafe extern "system" fn GetFontStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STYLE
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFontStyle(this)
        }
        unsafe extern "system" fn GetFontStretch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STRETCH
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFontStretch(this)
        }
        unsafe extern "system" fn GetFontSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetFontSize(this)
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetLocaleNameLength(this)
        }
        unsafe extern "system" fn GetLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localename: windows_core::PWSTR, namesize: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat_Impl::GetLocaleName(this, core::mem::transmute_copy(&localename), core::mem::transmute_copy(&namesize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTextAlignment: SetTextAlignment::<Identity, OFFSET>,
            SetParagraphAlignment: SetParagraphAlignment::<Identity, OFFSET>,
            SetWordWrapping: SetWordWrapping::<Identity, OFFSET>,
            SetReadingDirection: SetReadingDirection::<Identity, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, OFFSET>,
            SetIncrementalTabStop: SetIncrementalTabStop::<Identity, OFFSET>,
            SetTrimming: SetTrimming::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetTextAlignment: GetTextAlignment::<Identity, OFFSET>,
            GetParagraphAlignment: GetParagraphAlignment::<Identity, OFFSET>,
            GetWordWrapping: GetWordWrapping::<Identity, OFFSET>,
            GetReadingDirection: GetReadingDirection::<Identity, OFFSET>,
            GetFlowDirection: GetFlowDirection::<Identity, OFFSET>,
            GetIncrementalTabStop: GetIncrementalTabStop::<Identity, OFFSET>,
            GetTrimming: GetTrimming::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextFormat1_Impl: Sized + IDWriteTextFormat_Impl {
    fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::Result<()>;
    fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(&self, islastlinewrappingenabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL;
    fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::Result<()>;
    fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(&self, fontfallback: Option<&IDWriteFontFallback>) -> windows_core::Result<()>;
    fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
}
impl windows_core::RuntimeName for IDWriteTextFormat1 {}
impl IDWriteTextFormat1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextFormat1_Vtbl
    where
        Identity: IDWriteTextFormat1_Impl,
    {
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::SetVerticalGlyphOrientation(this, core::mem::transmute_copy(&glyphorientation)).into()
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::GetVerticalGlyphOrientation(this)
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::SetLastLineWrapping(this, core::mem::transmute_copy(&islastlinewrappingenabled)).into()
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::GetLastLineWrapping(this)
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::SetOpticalAlignment(this, core::mem::transmute_copy(&opticalalignment)).into()
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::GetOpticalAlignment(this)
        }
        unsafe extern "system" fn SetFontFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat1_Impl::SetFontFallback(this, windows_core::from_raw_borrowed(&fontfallback)).into()
        }
        unsafe extern "system" fn GetFontFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteTextFormat1_Impl::GetFontFallback(this) {
                Ok(ok__) => {
                    fontfallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteTextFormat_Vtbl::new::<Identity, OFFSET>(),
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat1 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextFormat2_Impl: Sized + IDWriteTextFormat1_Impl {
    fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::Result<()>;
    fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextFormat2 {}
impl IDWriteTextFormat2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextFormat2_Vtbl
    where
        Identity: IDWriteTextFormat2_Impl,
    {
        unsafe extern "system" fn SetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat2_Impl::SetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
        }
        unsafe extern "system" fn GetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat2_Impl::GetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
        }
        Self {
            base__: IDWriteTextFormat1_Vtbl::new::<Identity, OFFSET>(),
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat2 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID || iid == &<IDWriteTextFormat1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextFormat3_Impl: Sized + IDWriteTextFormat2_Impl {
    fn SetFontAxisValues(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextFormat3 {}
impl IDWriteTextFormat3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextFormat3_Vtbl
    where
        Identity: IDWriteTextFormat3_Impl,
    {
        unsafe extern "system" fn SetFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat3_Impl::SetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteTextFormat3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat3_Impl::GetFontAxisValueCount(this)
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat3_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES
        where
            Identity: IDWriteTextFormat3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat3_Impl::GetAutomaticFontAxes(this)
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT
        where
            Identity: IDWriteTextFormat3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextFormat3_Impl::SetAutomaticFontAxes(this, core::mem::transmute_copy(&automaticfontaxes)).into()
        }
        Self {
            base__: IDWriteTextFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetFontAxisValues: SetFontAxisValues::<Identity, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat3 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID || iid == &<IDWriteTextFormat1 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextLayout_Impl: Sized + IDWriteTextFormat_Impl {
    fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()>;
    fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()>;
    fn SetFontCollection(&self, fontcollection: Option<&IDWriteFontCollection>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontFamilyName(&self, fontfamilyname: &windows_core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontSize(&self, fontsize: f32, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetUnderline(&self, hasunderline: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetStrikethrough(&self, hasstrikethrough: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetDrawingEffect(&self, drawingeffect: Option<&windows_core::IUnknown>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetInlineObject(&self, inlineobject: Option<&IDWriteInlineObject>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetTypography(&self, typography: Option<&IDWriteTypography>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetLocaleName(&self, localename: &windows_core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetMaxWidth(&self) -> f32;
    fn GetMaxHeight(&self) -> f32;
    fn GetFontCollection(&self, currentposition: u32, fontcollection: *mut Option<IDWriteFontCollection>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontFamilyNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontFamilyName(&self, currentposition: u32, fontfamilyname: windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontWeight(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontStyle(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontStretch(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontSize(&self, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut Option<windows_core::IUnknown>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut Option<IDWriteInlineObject>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetTypography(&self, currentposition: u32, typography: *mut Option<IDWriteTypography>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetLocaleNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetLocaleName(&self, currentposition: u32, localename: windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn Draw(&self, clientdrawingcontext: *const core::ffi::c_void, renderer: Option<&IDWriteTextRenderer>, originx: f32, originy: f32) -> windows_core::Result<()>;
    fn GetLineMetrics(&self, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::Result<()>;
    fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::Result<()>;
    fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetClusterMetrics(&self, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> windows_core::Result<()>;
    fn DetermineMinWidth(&self) -> windows_core::Result<f32>;
    fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::Result<()>;
    fn HitTestTextPosition(&self, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::Result<()>;
    fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextLayout {}
impl IDWriteTextLayout_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextLayout_Vtbl
    where
        Identity: IDWriteTextLayout_Impl,
    {
        unsafe extern "system" fn SetMaxWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxwidth: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetMaxWidth(this, core::mem::transmute_copy(&maxwidth)).into()
        }
        unsafe extern "system" fn SetMaxHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxheight: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetMaxHeight(this, core::mem::transmute_copy(&maxheight)).into()
        }
        unsafe extern "system" fn SetFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetFontCollection(this, windows_core::from_raw_borrowed(&fontcollection), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetFontFamilyName(this, core::mem::transmute(&fontfamilyname), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetFontWeight(this, core::mem::transmute_copy(&fontweight), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetFontStyle(this, core::mem::transmute_copy(&fontstyle), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontStretch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetFontStretch(this, core::mem::transmute_copy(&fontstretch), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetFontSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetFontSize(this, core::mem::transmute_copy(&fontsize), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetUnderline(this, core::mem::transmute_copy(&hasunderline), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetStrikethrough<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetStrikethrough(this, core::mem::transmute_copy(&hasstrikethrough), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetDrawingEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingeffect: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetDrawingEffect(this, windows_core::from_raw_borrowed(&drawingeffect), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetInlineObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inlineobject: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetInlineObject(this, windows_core::from_raw_borrowed(&inlineobject), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetTypography<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, typography: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetTypography(this, windows_core::from_raw_borrowed(&typography), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn SetLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localename: windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::SetLocaleName(this, core::mem::transmute(&localename), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetMaxWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetMaxWidth(this)
        }
        unsafe extern "system" fn GetMaxHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetMaxHeight(this)
        }
        unsafe extern "system" fn GetFontCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontcollection: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontCollection(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontFamilyNameLength(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&namelength), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontfamilyname: windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontFamilyName(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontfamilyname), core::mem::transmute_copy(&namesize), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontWeight(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontStyle(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontstyle), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontStretch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontStretch(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetFontSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetFontSize(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontsize), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetUnderline(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&hasunderline), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetStrikethrough<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetStrikethrough(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&hasstrikethrough), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetDrawingEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetDrawingEffect(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&drawingeffect), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetInlineObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, inlineobject: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetInlineObject(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&inlineobject), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetTypography<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, typography: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetTypography(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&typography), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetLocaleNameLength(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&namelength), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetLocaleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, localename: windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetLocaleName(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&localename), core::mem::transmute_copy(&namesize), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, renderer: *mut core::ffi::c_void, originx: f32, originy: f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::Draw(this, core::mem::transmute_copy(&clientdrawingcontext), windows_core::from_raw_borrowed(&renderer), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy)).into()
        }
        unsafe extern "system" fn GetLineMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetLineMetrics(this, core::mem::transmute_copy(&linemetrics), core::mem::transmute_copy(&maxlinecount), core::mem::transmute_copy(&actuallinecount)).into()
        }
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetMetrics(this, core::mem::transmute_copy(&textmetrics)).into()
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteTextLayout_Impl::GetOverhangMetrics(this) {
                Ok(ok__) => {
                    overhangs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClusterMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::GetClusterMetrics(this, core::mem::transmute_copy(&clustermetrics), core::mem::transmute_copy(&maxclustercount), core::mem::transmute_copy(&actualclustercount)).into()
        }
        unsafe extern "system" fn DetermineMinWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minwidth: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteTextLayout_Impl::DetermineMinWidth(this) {
                Ok(ok__) => {
                    minwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HitTestPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::HitTestPoint(this, core::mem::transmute_copy(&pointx), core::mem::transmute_copy(&pointy), core::mem::transmute_copy(&istrailinghit), core::mem::transmute_copy(&isinside), core::mem::transmute_copy(&hittestmetrics)).into()
        }
        unsafe extern "system" fn HitTestTextPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::HitTestTextPosition(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&istrailinghit), core::mem::transmute_copy(&pointx), core::mem::transmute_copy(&pointy), core::mem::transmute_copy(&hittestmetrics)).into()
        }
        unsafe extern "system" fn HitTestTextRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout_Impl::HitTestTextRange(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&hittestmetrics), core::mem::transmute_copy(&maxhittestmetricscount), core::mem::transmute_copy(&actualhittestmetricscount)).into()
        }
        Self {
            base__: IDWriteTextFormat_Vtbl::new::<Identity, OFFSET>(),
            SetMaxWidth: SetMaxWidth::<Identity, OFFSET>,
            SetMaxHeight: SetMaxHeight::<Identity, OFFSET>,
            SetFontCollection: SetFontCollection::<Identity, OFFSET>,
            SetFontFamilyName: SetFontFamilyName::<Identity, OFFSET>,
            SetFontWeight: SetFontWeight::<Identity, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, OFFSET>,
            SetFontSize: SetFontSize::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, OFFSET>,
            SetDrawingEffect: SetDrawingEffect::<Identity, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, OFFSET>,
            SetTypography: SetTypography::<Identity, OFFSET>,
            SetLocaleName: SetLocaleName::<Identity, OFFSET>,
            GetMaxWidth: GetMaxWidth::<Identity, OFFSET>,
            GetMaxHeight: GetMaxHeight::<Identity, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetUnderline: GetUnderline::<Identity, OFFSET>,
            GetStrikethrough: GetStrikethrough::<Identity, OFFSET>,
            GetDrawingEffect: GetDrawingEffect::<Identity, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, OFFSET>,
            GetTypography: GetTypography::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, OFFSET>,
            GetClusterMetrics: GetClusterMetrics::<Identity, OFFSET>,
            DetermineMinWidth: DetermineMinWidth::<Identity, OFFSET>,
            HitTestPoint: HitTestPoint::<Identity, OFFSET>,
            HitTestTextPosition: HitTestTextPosition::<Identity, OFFSET>,
            HitTestTextRange: HitTestTextRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextLayout1_Impl: Sized + IDWriteTextLayout_Impl {
    fn SetPairKerning(&self, ispairkerningenabled: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextLayout1 {}
impl IDWriteTextLayout1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextLayout1_Vtbl
    where
        Identity: IDWriteTextLayout1_Impl,
    {
        unsafe extern "system" fn SetPairKerning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout1_Impl::SetPairKerning(this, core::mem::transmute_copy(&ispairkerningenabled), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetPairKerning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout1_Impl::GetPairKerning(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&ispairkerningenabled), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn SetCharacterSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout1_Impl::SetCharacterSpacing(this, core::mem::transmute_copy(&leadingspacing), core::mem::transmute_copy(&trailingspacing), core::mem::transmute_copy(&minimumadvancewidth), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetCharacterSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout1_Impl::GetCharacterSpacing(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&leadingspacing), core::mem::transmute_copy(&trailingspacing), core::mem::transmute_copy(&minimumadvancewidth), core::mem::transmute_copy(&textrange)).into()
        }
        Self {
            base__: IDWriteTextLayout_Vtbl::new::<Identity, OFFSET>(),
            SetPairKerning: SetPairKerning::<Identity, OFFSET>,
            GetPairKerning: GetPairKerning::<Identity, OFFSET>,
            SetCharacterSpacing: SetCharacterSpacing::<Identity, OFFSET>,
            GetCharacterSpacing: GetCharacterSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout1 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID || iid == &<IDWriteTextLayout as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextLayout2_Impl: Sized + IDWriteTextLayout1_Impl {
    fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> windows_core::Result<()>;
    fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::Result<()>;
    fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(&self, islastlinewrappingenabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL;
    fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::Result<()>;
    fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(&self, fontfallback: Option<&IDWriteFontFallback>) -> windows_core::Result<()>;
    fn GetFontFallback(&self) -> windows_core::Result<IDWriteFontFallback>;
}
impl windows_core::RuntimeName for IDWriteTextLayout2 {}
impl IDWriteTextLayout2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextLayout2_Vtbl
    where
        Identity: IDWriteTextLayout2_Impl,
    {
        unsafe extern "system" fn GetMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::GetMetrics(this, core::mem::transmute_copy(&textmetrics)).into()
        }
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::SetVerticalGlyphOrientation(this, core::mem::transmute_copy(&glyphorientation)).into()
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::GetVerticalGlyphOrientation(this)
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::SetLastLineWrapping(this, core::mem::transmute_copy(&islastlinewrappingenabled)).into()
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::GetLastLineWrapping(this)
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::SetOpticalAlignment(this, core::mem::transmute_copy(&opticalalignment)).into()
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::GetOpticalAlignment(this)
        }
        unsafe extern "system" fn SetFontFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout2_Impl::SetFontFallback(this, windows_core::from_raw_borrowed(&fontfallback)).into()
        }
        unsafe extern "system" fn GetFontFallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteTextLayout2_Impl::GetFontFallback(this) {
                Ok(ok__) => {
                    fontfallback.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDWriteTextLayout1_Vtbl::new::<Identity, OFFSET>(),
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout2 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID || iid == &<IDWriteTextLayout as windows_core::Interface>::IID || iid == &<IDWriteTextLayout1 as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextLayout3_Impl: Sized + IDWriteTextLayout2_Impl {
    fn InvalidateLayout(&self) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::Result<()>;
    fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::Result<()>;
    fn GetLineMetrics(&self, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextLayout3 {}
impl IDWriteTextLayout3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextLayout3_Vtbl
    where
        Identity: IDWriteTextLayout3_Impl,
    {
        unsafe extern "system" fn InvalidateLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout3_Impl::InvalidateLayout(this).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout3_Impl::SetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
        }
        unsafe extern "system" fn GetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout3_Impl::GetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
        }
        unsafe extern "system" fn GetLineMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout3_Impl::GetLineMetrics(this, core::mem::transmute_copy(&linemetrics), core::mem::transmute_copy(&maxlinecount), core::mem::transmute_copy(&actuallinecount)).into()
        }
        Self {
            base__: IDWriteTextLayout2_Vtbl::new::<Identity, OFFSET>(),
            InvalidateLayout: InvalidateLayout::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout3 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID || iid == &<IDWriteTextLayout as windows_core::Interface>::IID || iid == &<IDWriteTextLayout1 as windows_core::Interface>::IID || iid == &<IDWriteTextLayout2 as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextLayout4_Impl: Sized + IDWriteTextLayout3_Impl {
    fn SetFontAxisValues(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontAxisValueCount(&self, currentposition: u32) -> u32;
    fn GetFontAxisValues(&self, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextLayout4 {}
impl IDWriteTextLayout4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextLayout4_Vtbl
    where
        Identity: IDWriteTextLayout4_Impl,
    {
        unsafe extern "system" fn SetFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout4_Impl::SetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute(&textrange)).into()
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32) -> u32
        where
            Identity: IDWriteTextLayout4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout4_Impl::GetFontAxisValueCount(this, core::mem::transmute_copy(&currentposition))
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout4_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&textrange)).into()
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES
        where
            Identity: IDWriteTextLayout4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout4_Impl::GetAutomaticFontAxes(this)
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT
        where
            Identity: IDWriteTextLayout4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextLayout4_Impl::SetAutomaticFontAxes(this, core::mem::transmute_copy(&automaticfontaxes)).into()
        }
        Self {
            base__: IDWriteTextLayout3_Vtbl::new::<Identity, OFFSET>(),
            SetFontAxisValues: SetFontAxisValues::<Identity, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout4 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID || iid == &<IDWriteTextLayout as windows_core::Interface>::IID || iid == &<IDWriteTextLayout1 as windows_core::Interface>::IID || iid == &<IDWriteTextLayout2 as windows_core::Interface>::IID || iid == &<IDWriteTextLayout3 as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextRenderer_Impl: Sized + IDWritePixelSnapping_Impl {
    fn DrawGlyphRun(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawUnderline(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawStrikethrough(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawInlineObject(&self, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, inlineobject: Option<&IDWriteInlineObject>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextRenderer {}
impl IDWriteTextRenderer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextRenderer_Vtbl
    where
        Identity: IDWriteTextRenderer_Impl,
    {
        unsafe extern "system" fn DrawGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer_Impl::DrawGlyphRun(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer_Impl::DrawUnderline(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&underline), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawStrikethrough<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer_Impl::DrawStrikethrough(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&strikethrough), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawInlineObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, inlineobject: *mut core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer_Impl::DrawInlineObject(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), windows_core::from_raw_borrowed(&inlineobject), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        Self {
            base__: IDWritePixelSnapping_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextRenderer as windows_core::Interface>::IID || iid == &<IDWritePixelSnapping as windows_core::Interface>::IID
    }
}
pub trait IDWriteTextRenderer1_Impl: Sized + IDWriteTextRenderer_Impl {
    fn DrawGlyphRun(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawUnderline(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawStrikethrough(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawInlineObject(&self, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: Option<&IDWriteInlineObject>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDWriteTextRenderer1 {}
impl IDWriteTextRenderer1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTextRenderer1_Vtbl
    where
        Identity: IDWriteTextRenderer1_Impl,
    {
        unsafe extern "system" fn DrawGlyphRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer1_Impl::DrawGlyphRun(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer1_Impl::DrawUnderline(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&underline), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawStrikethrough<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer1_Impl::DrawStrikethrough(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&orientationangle), core::mem::transmute_copy(&strikethrough), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        unsafe extern "system" fn DrawInlineObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: *mut core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDWriteTextRenderer1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTextRenderer1_Impl::DrawInlineObject(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&orientationangle), windows_core::from_raw_borrowed(&inlineobject), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), windows_core::from_raw_borrowed(&clientdrawingeffect)).into()
        }
        Self {
            base__: IDWriteTextRenderer_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextRenderer1 as windows_core::Interface>::IID || iid == &<IDWritePixelSnapping as windows_core::Interface>::IID || iid == &<IDWriteTextRenderer as windows_core::Interface>::IID
    }
}
pub trait IDWriteTypography_Impl: Sized {
    fn AddFontFeature(&self, fontfeature: &DWRITE_FONT_FEATURE) -> windows_core::Result<()>;
    fn GetFontFeatureCount(&self) -> u32;
    fn GetFontFeature(&self, fontfeatureindex: u32) -> windows_core::Result<DWRITE_FONT_FEATURE>;
}
impl windows_core::RuntimeName for IDWriteTypography {}
impl IDWriteTypography_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDWriteTypography_Vtbl
    where
        Identity: IDWriteTypography_Impl,
    {
        unsafe extern "system" fn AddFontFeature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> windows_core::HRESULT
        where
            Identity: IDWriteTypography_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTypography_Impl::AddFontFeature(this, core::mem::transmute(&fontfeature)).into()
        }
        unsafe extern "system" fn GetFontFeatureCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IDWriteTypography_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDWriteTypography_Impl::GetFontFeatureCount(this)
        }
        unsafe extern "system" fn GetFontFeature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> windows_core::HRESULT
        where
            Identity: IDWriteTypography_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDWriteTypography_Impl::GetFontFeature(this, core::mem::transmute_copy(&fontfeatureindex)) {
                Ok(ok__) => {
                    fontfeature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFontFeature: AddFontFeature::<Identity, OFFSET>,
            GetFontFeatureCount: GetFontFeatureCount::<Identity, OFFSET>,
            GetFontFeature: GetFontFeature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTypography as windows_core::Interface>::IID
    }
}

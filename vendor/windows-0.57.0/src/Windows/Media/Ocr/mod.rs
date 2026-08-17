windows_core::imp::define_interface!(IOcrEngine, IOcrEngine_Vtbl, 0x5a14bc41_5b76_3140_b680_8825562683ac);
impl windows_core::RuntimeType for IOcrEngine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOcrEngine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub RecognizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Globalization")]
    pub RecognizerLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    RecognizerLanguage: usize,
}
windows_core::imp::define_interface!(IOcrEngineStatics, IOcrEngineStatics_Vtbl, 0x5bffa85a_3384_3540_9940_699120d428a8);
impl windows_core::RuntimeType for IOcrEngineStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOcrEngineStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxImageDimension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub AvailableRecognizerLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    AvailableRecognizerLanguages: usize,
    #[cfg(feature = "Globalization")]
    pub IsLanguageSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    IsLanguageSupported: usize,
    #[cfg(feature = "Globalization")]
    pub TryCreateFromLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    TryCreateFromLanguage: usize,
    pub TryCreateFromUserProfileLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOcrLine, IOcrLine_Vtbl, 0x0043a16f_e31f_3a24_899c_d444bd088124);
impl windows_core::RuntimeType for IOcrLine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOcrLine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Words: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Words: usize,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOcrResult, IOcrResult_Vtbl, 0x9bd235b2_175b_3d6a_92e2_388c206e2f63);
impl windows_core::RuntimeType for IOcrResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOcrResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
    pub TextAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOcrWord, IOcrWord_Vtbl, 0x3c2a477a_5cd9_3525_ba2a_23d1e0a68a1d);
impl windows_core::RuntimeType for IOcrWord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOcrWord_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OcrEngine(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OcrEngine, windows_core::IUnknown, windows_core::IInspectable);
impl OcrEngine {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn RecognizeAsync<P0>(&self, bitmap: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<OcrResult>>
    where
        P0: windows_core::Param<super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn RecognizerLanguage(&self) -> windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizerLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxImageDimension() -> windows_core::Result<u32> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxImageDimension)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn AvailableRecognizerLanguages() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailableRecognizerLanguages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn IsLanguageSupported<P0>(language: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Globalization::Language>,
    {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLanguageSupported)(windows_core::Interface::as_raw(this), language.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn TryCreateFromLanguage<P0>(language: P0) -> windows_core::Result<OcrEngine>
    where
        P0: windows_core::Param<super::super::Globalization::Language>,
    {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFromLanguage)(windows_core::Interface::as_raw(this), language.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryCreateFromUserProfileLanguages() -> windows_core::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFromUserProfileLanguages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IOcrEngineStatics<R, F: FnOnce(&IOcrEngineStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OcrEngine, IOcrEngineStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OcrEngine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOcrEngine>();
}
unsafe impl windows_core::Interface for OcrEngine {
    type Vtable = IOcrEngine_Vtbl;
    const IID: windows_core::GUID = <IOcrEngine as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrEngine";
}
unsafe impl Send for OcrEngine {}
unsafe impl Sync for OcrEngine {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OcrLine(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OcrLine, windows_core::IUnknown, windows_core::IInspectable);
impl OcrLine {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Words(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<OcrWord>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Words)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OcrLine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOcrLine>();
}
unsafe impl windows_core::Interface for OcrLine {
    type Vtable = IOcrLine_Vtbl;
    const IID: windows_core::GUID = <IOcrLine as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrLine";
}
unsafe impl Send for OcrLine {}
unsafe impl Sync for OcrLine {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OcrResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OcrResult, windows_core::IUnknown, windows_core::IInspectable);
impl OcrResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<OcrLine>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lines)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TextAngle(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextAngle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OcrResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOcrResult>();
}
unsafe impl windows_core::Interface for OcrResult {
    type Vtable = IOcrResult_Vtbl;
    const IID: windows_core::GUID = <IOcrResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.OcrResult";
}
unsafe impl Send for OcrResult {}
unsafe impl Sync for OcrResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct OcrWord(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OcrWord, windows_core::IUnknown, windows_core::IInspectable);
impl OcrWord {
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OcrWord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOcrWord>();
}
unsafe impl windows_core::Interface for OcrWord {
    type Vtable = IOcrWord_Vtbl;
    const IID: windows_core::GUID = <IOcrWord as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.OcrWord";
}
unsafe impl Send for OcrWord {}
unsafe impl Sync for OcrWord {}

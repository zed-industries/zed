windows_core::imp::define_interface!(ILanguageFont, ILanguageFont_Vtbl, 0xb12e5c3a_b76d_459b_beeb_901151cd77d1);
impl windows_core::RuntimeType for ILanguageFont {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILanguageFont_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Text::FontWeight) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Text::FontStretch) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Text::FontStyle) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    pub ScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanguageFontGroup, ILanguageFontGroup_Vtbl, 0xf33a7fc3_3a5c_4aea_b9ff_b39fb242f7f6);
impl windows_core::RuntimeType for ILanguageFontGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILanguageFontGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UITextFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UIHeadingFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UITitleFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UICaptionFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UINotificationHeadingFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TraditionalDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModernDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentHeadingFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FixedWidthTextFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentAlternate1Font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentAlternate2Font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanguageFontGroupFactory, ILanguageFontGroupFactory_Vtbl, 0xfcaeac67_4e77_49c7_b856_dde934fc735b);
impl windows_core::RuntimeType for ILanguageFontGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILanguageFontGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLanguageFontGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LanguageFont(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LanguageFont, windows_core::IUnknown, windows_core::IInspectable);
impl LanguageFont {
    pub fn FontFamily(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Text")]
    pub fn FontWeight(&self) -> windows_core::Result<super::super::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    pub fn FontStretch(&self) -> windows_core::Result<super::super::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    pub fn FontStyle(&self) -> windows_core::Result<super::super::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScaleFactor(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LanguageFont {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILanguageFont>();
}
unsafe impl windows_core::Interface for LanguageFont {
    type Vtable = ILanguageFont_Vtbl;
    const IID: windows_core::GUID = <ILanguageFont as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFont";
}
unsafe impl Send for LanguageFont {}
unsafe impl Sync for LanguageFont {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LanguageFontGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LanguageFontGroup, windows_core::IUnknown, windows_core::IInspectable);
impl LanguageFontGroup {
    pub fn UITextFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UITextFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UIHeadingFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIHeadingFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UITitleFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UITitleFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UICaptionFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UICaptionFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UINotificationHeadingFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UINotificationHeadingFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TraditionalDocumentFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TraditionalDocumentFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ModernDocumentFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModernDocumentFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentHeadingFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentHeadingFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FixedWidthTextFont(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FixedWidthTextFont)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentAlternate1Font(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentAlternate1Font)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentAlternate2Font(&self) -> windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentAlternate2Font)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateLanguageFontGroup(languagetag: &windows_core::HSTRING) -> windows_core::Result<LanguageFontGroup> {
        Self::ILanguageFontGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLanguageFontGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(languagetag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILanguageFontGroupFactory<R, F: FnOnce(&ILanguageFontGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LanguageFontGroup, ILanguageFontGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LanguageFontGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILanguageFontGroup>();
}
unsafe impl windows_core::Interface for LanguageFontGroup {
    type Vtable = ILanguageFontGroup_Vtbl;
    const IID: windows_core::GUID = <ILanguageFontGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFontGroup";
}
unsafe impl Send for LanguageFontGroup {}
unsafe impl Sync for LanguageFontGroup {}

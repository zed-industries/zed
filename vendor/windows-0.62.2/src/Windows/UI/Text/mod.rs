#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: Self = Self(0i32);
    pub const Null: Self = Self(1i32);
}
impl windows_core::TypeKind for CaretType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CaretType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.CaretType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentLinkInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContentLinkInfo, windows_core::IUnknown, windows_core::IInspectable);
impl ContentLinkInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentLinkInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDisplayText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SecondaryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecondaryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSecondaryText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSecondaryText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LinkContentKind(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkContentKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLinkContentKind(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLinkContentKind)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for ContentLinkInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContentLinkInfo>();
}
unsafe impl windows_core::Interface for ContentLinkInfo {
    type Vtable = <IContentLinkInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentLinkInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContentLinkInfo {
    const NAME: &'static str = "Windows.UI.Text.ContentLinkInfo";
}
unsafe impl Send for ContentLinkInfo {}
unsafe impl Sync for ContentLinkInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: Self = Self(0u32);
    pub const Word: Self = Self(2u32);
    pub const Case: Self = Self(4u32);
}
impl windows_core::TypeKind for FindOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FindOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FindOptions;u4)");
}
impl FindOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FindOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FindOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FindOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontStretch(pub i32);
impl FontStretch {
    pub const Undefined: Self = Self(0i32);
    pub const UltraCondensed: Self = Self(1i32);
    pub const ExtraCondensed: Self = Self(2i32);
    pub const Condensed: Self = Self(3i32);
    pub const SemiCondensed: Self = Self(4i32);
    pub const Normal: Self = Self(5i32);
    pub const SemiExpanded: Self = Self(6i32);
    pub const Expanded: Self = Self(7i32);
    pub const ExtraExpanded: Self = Self(8i32);
    pub const UltraExpanded: Self = Self(9i32);
}
impl windows_core::TypeKind for FontStretch {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FontStretch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStretch;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontStyle(pub i32);
impl FontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl windows_core::TypeKind for FontStyle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FontStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStyle;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontWeight {
    pub Weight: u16,
}
impl windows_core::TypeKind for FontWeight {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FontWeight {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontWeights(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FontWeights, windows_core::IUnknown, windows_core::IInspectable);
impl FontWeights {
    pub fn Black() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Black)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Bold() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExtraBlack() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtraBlack)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExtraBold() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtraBold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ExtraLight() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtraLight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Light() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Light)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Medium() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Medium)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Normal() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Normal)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SemiBold() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SemiBold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SemiLight() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SemiLight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Thin() -> windows_core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IFontWeightsStatics<R, F: FnOnce(&IFontWeightsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontWeights, IFontWeightsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontWeights {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFontWeights>();
}
unsafe impl windows_core::Interface for FontWeights {
    type Vtable = <IFontWeights as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontWeights as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FontWeights {
    const NAME: &'static str = "Windows.UI.Text.FontWeights";
}
unsafe impl Send for FontWeights {}
unsafe impl Sync for FontWeights {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Toggle: Self = Self(2i32);
    pub const Undefined: Self = Self(3i32);
}
impl windows_core::TypeKind for FormatEffect {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FormatEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FormatEffect;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl windows_core::TypeKind for HorizontalCharacterAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HorizontalCharacterAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.HorizontalCharacterAlignment;i4)");
}
windows_core::imp::define_interface!(IContentLinkInfo, IContentLinkInfo_Vtbl, 0x1ed52525_1c5f_48cb_b335_78b50a2ee642);
impl windows_core::RuntimeType for IContentLinkInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SecondaryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecondaryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LinkContentKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLinkContentKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFontWeights, IFontWeights_Vtbl, 0x7880a444_01ab_4997_8517_df822a0c45f1);
impl windows_core::RuntimeType for IFontWeights {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeights_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IFontWeightsStatics, IFontWeightsStatics_Vtbl, 0xb3b579d5_1ba9_48eb_9dad_c095e8c23ba3);
impl windows_core::RuntimeType for IFontWeightsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeightsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Black: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub ExtraBlack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub ExtraBold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub ExtraLight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Light: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Medium: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Normal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub SemiBold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub SemiLight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
    pub Thin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRichEditTextRange, IRichEditTextRange_Vtbl, 0x374e3515_ba8a_4a6e_8c59_0dde3d0cf5cd);
impl windows_core::RuntimeType for IRichEditTextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditTextRange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentLinkInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentLinkInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextCharacterFormat, ITextCharacterFormat_Vtbl, 0x5adef3db_05fb_442d_8065_642afea02ced);
impl windows_core::RuntimeType for ITextCharacterFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITextCharacterFormat, windows_core::IUnknown, windows_core::IInspectable);
impl ITextCharacterFormat {
    pub fn AllCaps(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllCaps)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllCaps(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllCaps)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bold(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBold(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBold)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: FontStretch) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: FontStyle) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Hidden(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hidden)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHidden(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHidden)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Italic(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Italic)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetItalic(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetItalic)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kerning(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kerning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKerning(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKerning)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LanguageTag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageTag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguageTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguageTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn LinkType(&self) -> windows_core::Result<LinkType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Outline(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Outline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutline(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutline)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtectedText(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedText)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtectedText(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedText)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Size(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SmallCaps(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmallCaps)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSmallCaps(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSmallCaps)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Spacing(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Spacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpacing(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Strikethrough(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Strikethrough)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrikethrough(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStrikethrough)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Subscript(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subscript)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSubscript(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubscript)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Superscript(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Superscript)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSuperscript(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuperscript)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TextScript(&self) -> windows_core::Result<TextScript> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextScript)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTextScript(&self, value: TextScript) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextScript)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Underline(&self) -> windows_core::Result<UnderlineType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Underline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnderline(&self, value: UnderlineType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnderline)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Weight(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Weight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWeight(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetClone<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClone)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetClone(&self) -> windows_core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetClone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, format: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), format.param().abi(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ITextCharacterFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextCharacterFormat";
}
pub trait ITextCharacterFormat_Impl: windows_core::IUnknownImpl {
    fn AllCaps(&self) -> windows_core::Result<FormatEffect>;
    fn SetAllCaps(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn BackgroundColor(&self) -> windows_core::Result<super::Color>;
    fn SetBackgroundColor(&self, value: &super::Color) -> windows_core::Result<()>;
    fn Bold(&self) -> windows_core::Result<FormatEffect>;
    fn SetBold(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn FontStretch(&self) -> windows_core::Result<FontStretch>;
    fn SetFontStretch(&self, value: FontStretch) -> windows_core::Result<()>;
    fn FontStyle(&self) -> windows_core::Result<FontStyle>;
    fn SetFontStyle(&self, value: FontStyle) -> windows_core::Result<()>;
    fn ForegroundColor(&self) -> windows_core::Result<super::Color>;
    fn SetForegroundColor(&self, value: &super::Color) -> windows_core::Result<()>;
    fn Hidden(&self) -> windows_core::Result<FormatEffect>;
    fn SetHidden(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Italic(&self) -> windows_core::Result<FormatEffect>;
    fn SetItalic(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Kerning(&self) -> windows_core::Result<f32>;
    fn SetKerning(&self, value: f32) -> windows_core::Result<()>;
    fn LanguageTag(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLanguageTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn LinkType(&self) -> windows_core::Result<LinkType>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Outline(&self) -> windows_core::Result<FormatEffect>;
    fn SetOutline(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Position(&self) -> windows_core::Result<f32>;
    fn SetPosition(&self, value: f32) -> windows_core::Result<()>;
    fn ProtectedText(&self) -> windows_core::Result<FormatEffect>;
    fn SetProtectedText(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<f32>;
    fn SetSize(&self, value: f32) -> windows_core::Result<()>;
    fn SmallCaps(&self) -> windows_core::Result<FormatEffect>;
    fn SetSmallCaps(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Spacing(&self) -> windows_core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> windows_core::Result<()>;
    fn Strikethrough(&self) -> windows_core::Result<FormatEffect>;
    fn SetStrikethrough(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Subscript(&self) -> windows_core::Result<FormatEffect>;
    fn SetSubscript(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Superscript(&self) -> windows_core::Result<FormatEffect>;
    fn SetSuperscript(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn TextScript(&self) -> windows_core::Result<TextScript>;
    fn SetTextScript(&self, value: TextScript) -> windows_core::Result<()>;
    fn Underline(&self) -> windows_core::Result<UnderlineType>;
    fn SetUnderline(&self, value: UnderlineType) -> windows_core::Result<()>;
    fn Weight(&self) -> windows_core::Result<i32>;
    fn SetWeight(&self, value: i32) -> windows_core::Result<()>;
    fn SetClone(&self, value: windows_core::Ref<ITextCharacterFormat>) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextCharacterFormat>;
    fn IsEqual(&self, format: windows_core::Ref<ITextCharacterFormat>) -> windows_core::Result<bool>;
}
impl ITextCharacterFormat_Vtbl {
    pub const fn new<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::AllCaps(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetAllCaps(this, value).into()
            }
        }
        unsafe extern "system" fn BackgroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::BackgroundColor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetBackgroundColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Bold<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Bold(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBold<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetBold(this, value).into()
            }
        }
        unsafe extern "system" fn FontStretch<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FontStretch) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::FontStretch(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontStretch<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FontStretch) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetFontStretch(this, value).into()
            }
        }
        unsafe extern "system" fn FontStyle<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FontStyle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::FontStyle(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontStyle<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FontStyle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetFontStyle(this, value).into()
            }
        }
        unsafe extern "system" fn ForegroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::ForegroundColor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetForegroundColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Hidden<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Hidden(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetHidden(this, value).into()
            }
        }
        unsafe extern "system" fn Italic<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Italic(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetItalic(this, value).into()
            }
        }
        unsafe extern "system" fn Kerning<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Kerning(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetKerning(this, value).into()
            }
        }
        unsafe extern "system" fn LanguageTag<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::LanguageTag(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguageTag<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetLanguageTag(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn LinkType<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LinkType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::LinkType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Outline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Outline(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetOutline(this, value).into()
            }
        }
        unsafe extern "system" fn Position<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetPosition(this, value).into()
            }
        }
        unsafe extern "system" fn ProtectedText<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::ProtectedText(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtectedText<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetProtectedText(this, value).into()
            }
        }
        unsafe extern "system" fn Size<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetSize(this, value).into()
            }
        }
        unsafe extern "system" fn SmallCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::SmallCaps(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetSmallCaps(this, value).into()
            }
        }
        unsafe extern "system" fn Spacing<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Spacing(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetSpacing(this, value).into()
            }
        }
        unsafe extern "system" fn Strikethrough<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Strikethrough(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetStrikethrough(this, value).into()
            }
        }
        unsafe extern "system" fn Subscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Subscript(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetSubscript(this, value).into()
            }
        }
        unsafe extern "system" fn Superscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Superscript(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetSuperscript(this, value).into()
            }
        }
        unsafe extern "system" fn TextScript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut TextScript) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::TextScript(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTextScript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: TextScript) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetTextScript(this, value).into()
            }
        }
        unsafe extern "system" fn Underline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnderlineType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Underline(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnderlineType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetUnderline(this, value).into()
            }
        }
        unsafe extern "system" fn Weight<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::Weight(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetWeight(this, value).into()
            }
        }
        unsafe extern "system" fn SetClone<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextCharacterFormat_Impl::SetClone(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetClone<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::GetClone(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextCharacterFormat_Impl::IsEqual(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextCharacterFormat, OFFSET>(),
            AllCaps: AllCaps::<Identity, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, OFFSET>,
            BackgroundColor: BackgroundColor::<Identity, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, OFFSET>,
            Bold: Bold::<Identity, OFFSET>,
            SetBold: SetBold::<Identity, OFFSET>,
            FontStretch: FontStretch::<Identity, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, OFFSET>,
            FontStyle: FontStyle::<Identity, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, OFFSET>,
            Hidden: Hidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            Italic: Italic::<Identity, OFFSET>,
            SetItalic: SetItalic::<Identity, OFFSET>,
            Kerning: Kerning::<Identity, OFFSET>,
            SetKerning: SetKerning::<Identity, OFFSET>,
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            SetLanguageTag: SetLanguageTag::<Identity, OFFSET>,
            LinkType: LinkType::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Outline: Outline::<Identity, OFFSET>,
            SetOutline: SetOutline::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            ProtectedText: ProtectedText::<Identity, OFFSET>,
            SetProtectedText: SetProtectedText::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SmallCaps: SmallCaps::<Identity, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, OFFSET>,
            Spacing: Spacing::<Identity, OFFSET>,
            SetSpacing: SetSpacing::<Identity, OFFSET>,
            Strikethrough: Strikethrough::<Identity, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, OFFSET>,
            Subscript: Subscript::<Identity, OFFSET>,
            SetSubscript: SetSubscript::<Identity, OFFSET>,
            Superscript: Superscript::<Identity, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, OFFSET>,
            TextScript: TextScript::<Identity, OFFSET>,
            SetTextScript: SetTextScript::<Identity, OFFSET>,
            Underline: Underline::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            Weight: Weight::<Identity, OFFSET>,
            SetWeight: SetWeight::<Identity, OFFSET>,
            SetClone: SetClone::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextCharacterFormat as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextCharacterFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetAllCaps: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Color) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::Color) -> windows_core::HRESULT,
    pub Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub FontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontStretch) -> windows_core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, FontStretch) -> windows_core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontStyle) -> windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, FontStyle) -> windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Color) -> windows_core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::Color) -> windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Italic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Kerning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub LanguageTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLanguageTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LinkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LinkType) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Outline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetOutline: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ProtectedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetProtectedText: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SmallCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetSmallCaps: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Spacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Subscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Superscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub TextScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextScript) -> windows_core::HRESULT,
    pub SetTextScript: unsafe extern "system" fn(*mut core::ffi::c_void, TextScript) -> windows_core::HRESULT,
    pub Underline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnderlineType) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, UnderlineType) -> windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetClone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextConstantsStatics, ITextConstantsStatics_Vtbl, 0x779e7c33_189d_4bfa_97c8_10db135d976e);
impl windows_core::RuntimeType for ITextConstantsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConstantsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Color) -> windows_core::HRESULT,
    pub MinUnitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxUnitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UndefinedColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Color) -> windows_core::HRESULT,
    pub UndefinedFloatValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub UndefinedInt32Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UndefinedFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontStretch) -> windows_core::HRESULT,
    pub UndefinedFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontStyle) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextDocument, ITextDocument_Vtbl, 0xbeee4ddb_90b2_408c_a2f6_0a0ac31e33e4);
impl windows_core::RuntimeType for ITextDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITextDocument, windows_core::IUnknown, windows_core::IInspectable);
impl ITextDocument {
    pub fn CaretType(&self) -> windows_core::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaretType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCaretType(&self, value: CaretType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCaretType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultTabStop(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Selection(&self) -> windows_core::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UndoLimit(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndoLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUndoLimit(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUndoLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanCopy(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCopy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanPaste(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPaste)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanRedo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRedo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanUndo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanUndo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ApplyDisplayUpdates(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyDisplayUpdates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BatchDisplayUpdates(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatchDisplayUpdates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BeginUndoGroup(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginUndoGroup)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EndUndoGroup(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndUndoGroup)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefaultCharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultCharacterFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultParagraphFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRange)(windows_core::Interface::as_raw(this), startposition, endposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRangeFromPoint(&self, point: super::super::Foundation::Point, options: PointOptions) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRangeFromPoint)(windows_core::Interface::as_raw(this), point, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<P1>(&self, options: TextSetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LoadFromStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn Redo(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Redo)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveToStream<P1>(&self, options: TextGetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SaveToStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn SetDefaultCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultCharacterFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetDefaultParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultParagraphFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetText(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), options, core::mem::transmute_copy(value)).ok() }
    }
    pub fn Undo(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Undo)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITextDocument {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument";
}
#[cfg(feature = "Storage_Streams")]
pub trait ITextDocument_Impl: windows_core::IUnknownImpl {
    fn CaretType(&self) -> windows_core::Result<CaretType>;
    fn SetCaretType(&self, value: CaretType) -> windows_core::Result<()>;
    fn DefaultTabStop(&self) -> windows_core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()>;
    fn Selection(&self) -> windows_core::Result<ITextSelection>;
    fn UndoLimit(&self) -> windows_core::Result<u32>;
    fn SetUndoLimit(&self, value: u32) -> windows_core::Result<()>;
    fn CanCopy(&self) -> windows_core::Result<bool>;
    fn CanPaste(&self) -> windows_core::Result<bool>;
    fn CanRedo(&self) -> windows_core::Result<bool>;
    fn CanUndo(&self) -> windows_core::Result<bool>;
    fn ApplyDisplayUpdates(&self) -> windows_core::Result<i32>;
    fn BatchDisplayUpdates(&self) -> windows_core::Result<i32>;
    fn BeginUndoGroup(&self) -> windows_core::Result<()>;
    fn EndUndoGroup(&self) -> windows_core::Result<()>;
    fn GetDefaultCharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn GetRange(&self, startPosition: i32, endPosition: i32) -> windows_core::Result<ITextRange>;
    fn GetRangeFromPoint(&self, point: &super::super::Foundation::Point, options: PointOptions) -> windows_core::Result<ITextRange>;
    fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()>;
    fn LoadFromStream(&self, options: TextSetOptions, value: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn Redo(&self) -> windows_core::Result<()>;
    fn SaveToStream(&self, options: TextGetOptions, value: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn SetDefaultCharacterFormat(&self, value: windows_core::Ref<ITextCharacterFormat>) -> windows_core::Result<()>;
    fn SetDefaultParagraphFormat(&self, value: windows_core::Ref<ITextParagraphFormat>) -> windows_core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Undo(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage_Streams")]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ITextDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CaretType<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CaretType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::CaretType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CaretType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetCaretType(this, value).into()
            }
        }
        unsafe extern "system" fn DefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::DefaultTabStop(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetDefaultTabStop(this, value).into()
            }
        }
        unsafe extern "system" fn Selection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Selection(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UndoLimit<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::UndoLimit(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUndoLimit<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetUndoLimit(this, value).into()
            }
        }
        unsafe extern "system" fn CanCopy<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::CanCopy(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::CanPaste(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanRedo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::CanRedo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanUndo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::CanUndo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplyDisplayUpdates<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::ApplyDisplayUpdates(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BatchDisplayUpdates<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::BatchDisplayUpdates(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginUndoGroup<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::BeginUndoGroup(this).into()
            }
        }
        unsafe extern "system" fn EndUndoGroup<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::EndUndoGroup(this).into()
            }
        }
        unsafe extern "system" fn GetDefaultCharacterFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetDefaultCharacterFormat(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultParagraphFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetDefaultParagraphFormat(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRange<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetRange(this, startposition, endposition) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRangeFromPoint<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetRangeFromPoint(this, core::mem::transmute(&point), options) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::GetText(this, options, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn LoadFromStream<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::LoadFromStream(this, options, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Redo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::Redo(this).into()
            }
        }
        unsafe extern "system" fn SaveToStream<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SaveToStream(this, options, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetDefaultCharacterFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetDefaultCharacterFormat(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetDefaultParagraphFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetDefaultParagraphFormat(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetText(this, options, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Undo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::Undo(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextDocument, OFFSET>(),
            CaretType: CaretType::<Identity, OFFSET>,
            SetCaretType: SetCaretType::<Identity, OFFSET>,
            DefaultTabStop: DefaultTabStop::<Identity, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, OFFSET>,
            Selection: Selection::<Identity, OFFSET>,
            UndoLimit: UndoLimit::<Identity, OFFSET>,
            SetUndoLimit: SetUndoLimit::<Identity, OFFSET>,
            CanCopy: CanCopy::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            CanRedo: CanRedo::<Identity, OFFSET>,
            CanUndo: CanUndo::<Identity, OFFSET>,
            ApplyDisplayUpdates: ApplyDisplayUpdates::<Identity, OFFSET>,
            BatchDisplayUpdates: BatchDisplayUpdates::<Identity, OFFSET>,
            BeginUndoGroup: BeginUndoGroup::<Identity, OFFSET>,
            EndUndoGroup: EndUndoGroup::<Identity, OFFSET>,
            GetDefaultCharacterFormat: GetDefaultCharacterFormat::<Identity, OFFSET>,
            GetDefaultParagraphFormat: GetDefaultParagraphFormat::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
            GetRangeFromPoint: GetRangeFromPoint::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            LoadFromStream: LoadFromStream::<Identity, OFFSET>,
            Redo: Redo::<Identity, OFFSET>,
            SaveToStream: SaveToStream::<Identity, OFFSET>,
            SetDefaultCharacterFormat: SetDefaultCharacterFormat::<Identity, OFFSET>,
            SetDefaultParagraphFormat: SetDefaultParagraphFormat::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            Undo: Undo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaretType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CaretType) -> windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, CaretType) -> windows_core::HRESULT,
    pub DefaultTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDefaultTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub Selection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UndoLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetUndoLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CanCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanPaste: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanRedo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanUndo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ApplyDisplayUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BatchDisplayUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BeginUndoGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndUndoGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultCharacterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultParagraphFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point, PointOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, TextGetOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, TextSetOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    pub Redo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SaveToStream: unsafe extern "system" fn(*mut core::ffi::c_void, TextGetOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SaveToStream: usize,
    pub SetDefaultCharacterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultParagraphFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, TextSetOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextDocument2, ITextDocument2_Vtbl, 0xf2311112_8c89_49c9_9118_f057cbb814ee);
impl windows_core::RuntimeType for ITextDocument2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlignmentIncludesTrailingWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlignmentIncludesTrailingWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IgnoreTrailingCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIgnoreTrailingCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextDocument3, ITextDocument3_Vtbl, 0x75ab03a1_a6f8_441d_aa18_0a851d6e5e3c);
impl windows_core::RuntimeType for ITextDocument3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClearUndoRedoHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextDocument4, ITextDocument4_Vtbl, 0x619c20f2_cb3b_4521_981f_2865b1b93f04);
impl windows_core::RuntimeType for ITextDocument4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetMath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMathMode: unsafe extern "system" fn(*mut core::ffi::c_void, RichEditMathMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextParagraphFormat, ITextParagraphFormat_Vtbl, 0x2cf8cfa6_4676_498a_93f5_bbdbfc0bd883);
impl windows_core::RuntimeType for ITextParagraphFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITextParagraphFormat, windows_core::IUnknown, windows_core::IInspectable);
impl ITextParagraphFormat {
    pub fn Alignment(&self) -> windows_core::Result<ParagraphAlignment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Alignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlignment(&self, value: ParagraphAlignment) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FirstLineIndent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstLineIndent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeepTogether(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepTogether)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeepTogether(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeepTogether)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepWithNext(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepWithNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeepWithNext(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeepWithNext)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftIndent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeftIndent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LineSpacing(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LineSpacingRule(&self) -> windows_core::Result<LineSpacingRule> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineSpacingRule)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ListAlignment(&self) -> windows_core::Result<MarkerAlignment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListAlignment(&self, value: MarkerAlignment) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ListLevelIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListLevelIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListLevelIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ListStart(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListStart(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListStart)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ListStyle(&self) -> windows_core::Result<MarkerStyle> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListStyle(&self, value: MarkerStyle) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ListTab(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListTab)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListTab(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListTab)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ListType(&self) -> windows_core::Result<MarkerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetListType(&self, value: MarkerType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NoLineNumber(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoLineNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNoLineNumber(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNoLineNumber)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PageBreakBefore(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageBreakBefore)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPageBreakBefore(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPageBreakBefore)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightIndent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightIndent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightIndent(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightIndent)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightToLeft(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightToLeft)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightToLeft(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightToLeft)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<ParagraphStyle> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStyle(&self, value: ParagraphStyle) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpaceAfter(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpaceAfter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpaceAfter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpaceBefore(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpaceBefore)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpaceBefore)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WidowControl(&self) -> windows_core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WidowControl)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidowControl(&self, value: FormatEffect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWidowControl)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTab)(windows_core::Interface::as_raw(this), position, align, leader).ok() }
    }
    pub fn ClearAllTabs(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearAllTabs)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeleteTab(&self, position: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DeleteTab)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn GetClone(&self) -> windows_core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetClone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetTab)(windows_core::Interface::as_raw(this), index, position, align, leader).ok() }
    }
    pub fn IsEqual<P0>(&self, format: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), format.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetClone<P0>(&self, format: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClone)(windows_core::Interface::as_raw(this), format.param().abi()).ok() }
    }
    pub fn SetIndents(&self, start: f32, left: f32, right: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIndents)(windows_core::Interface::as_raw(this), start, left, right).ok() }
    }
    pub fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLineSpacing)(windows_core::Interface::as_raw(this), rule, spacing).ok() }
    }
}
impl windows_core::RuntimeName for ITextParagraphFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextParagraphFormat";
}
pub trait ITextParagraphFormat_Impl: windows_core::IUnknownImpl {
    fn Alignment(&self) -> windows_core::Result<ParagraphAlignment>;
    fn SetAlignment(&self, value: ParagraphAlignment) -> windows_core::Result<()>;
    fn FirstLineIndent(&self) -> windows_core::Result<f32>;
    fn KeepTogether(&self) -> windows_core::Result<FormatEffect>;
    fn SetKeepTogether(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn KeepWithNext(&self) -> windows_core::Result<FormatEffect>;
    fn SetKeepWithNext(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn LeftIndent(&self) -> windows_core::Result<f32>;
    fn LineSpacing(&self) -> windows_core::Result<f32>;
    fn LineSpacingRule(&self) -> windows_core::Result<LineSpacingRule>;
    fn ListAlignment(&self) -> windows_core::Result<MarkerAlignment>;
    fn SetListAlignment(&self, value: MarkerAlignment) -> windows_core::Result<()>;
    fn ListLevelIndex(&self) -> windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()>;
    fn ListStart(&self) -> windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> windows_core::Result<()>;
    fn ListStyle(&self) -> windows_core::Result<MarkerStyle>;
    fn SetListStyle(&self, value: MarkerStyle) -> windows_core::Result<()>;
    fn ListTab(&self) -> windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> windows_core::Result<()>;
    fn ListType(&self) -> windows_core::Result<MarkerType>;
    fn SetListType(&self, value: MarkerType) -> windows_core::Result<()>;
    fn NoLineNumber(&self) -> windows_core::Result<FormatEffect>;
    fn SetNoLineNumber(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn PageBreakBefore(&self) -> windows_core::Result<FormatEffect>;
    fn SetPageBreakBefore(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn RightIndent(&self) -> windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> windows_core::Result<()>;
    fn RightToLeft(&self) -> windows_core::Result<FormatEffect>;
    fn SetRightToLeft(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Style(&self) -> windows_core::Result<ParagraphStyle>;
    fn SetStyle(&self, value: ParagraphStyle) -> windows_core::Result<()>;
    fn SpaceAfter(&self) -> windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()>;
    fn SpaceBefore(&self) -> windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()>;
    fn WidowControl(&self) -> windows_core::Result<FormatEffect>;
    fn SetWidowControl(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn TabCount(&self) -> windows_core::Result<i32>;
    fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> windows_core::Result<()>;
    fn ClearAllTabs(&self) -> windows_core::Result<()>;
    fn DeleteTab(&self, position: f32) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> windows_core::Result<()>;
    fn IsEqual(&self, format: windows_core::Ref<ITextParagraphFormat>) -> windows_core::Result<bool>;
    fn SetClone(&self, format: windows_core::Ref<ITextParagraphFormat>) -> windows_core::Result<()>;
    fn SetIndents(&self, start: f32, left: f32, right: f32) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> windows_core::Result<()>;
}
impl ITextParagraphFormat_Vtbl {
    pub const fn new<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ParagraphAlignment) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::Alignment(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ParagraphAlignment) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetAlignment(this, value).into()
            }
        }
        unsafe extern "system" fn FirstLineIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::FirstLineIndent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeepTogether<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::KeepTogether(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetKeepTogether(this, value).into()
            }
        }
        unsafe extern "system" fn KeepWithNext<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::KeepWithNext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetKeepWithNext(this, value).into()
            }
        }
        unsafe extern "system" fn LeftIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::LeftIndent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LineSpacing<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::LineSpacing(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LineSpacingRule<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LineSpacingRule) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::LineSpacingRule(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ListAlignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MarkerAlignment) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListAlignment(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MarkerAlignment) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListAlignment(this, value).into()
            }
        }
        unsafe extern "system" fn ListLevelIndex<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListLevelIndex(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListLevelIndex(this, value).into()
            }
        }
        unsafe extern "system" fn ListStart<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListStart(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListStart(this, value).into()
            }
        }
        unsafe extern "system" fn ListStyle<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MarkerStyle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListStyle(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListStyle<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MarkerStyle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListStyle(this, value).into()
            }
        }
        unsafe extern "system" fn ListTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListTab(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListTab(this, value).into()
            }
        }
        unsafe extern "system" fn ListType<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MarkerType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::ListType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListType<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MarkerType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetListType(this, value).into()
            }
        }
        unsafe extern "system" fn NoLineNumber<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::NoLineNumber(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetNoLineNumber(this, value).into()
            }
        }
        unsafe extern "system" fn PageBreakBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::PageBreakBefore(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetPageBreakBefore(this, value).into()
            }
        }
        unsafe extern "system" fn RightIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::RightIndent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetRightIndent(this, value).into()
            }
        }
        unsafe extern "system" fn RightToLeft<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::RightToLeft(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRightToLeft<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetRightToLeft(this, value).into()
            }
        }
        unsafe extern "system" fn Style<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ParagraphStyle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::Style(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ParagraphStyle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetStyle(this, value).into()
            }
        }
        unsafe extern "system" fn SpaceAfter<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::SpaceAfter(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetSpaceAfter(this, value).into()
            }
        }
        unsafe extern "system" fn SpaceBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::SpaceBefore(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetSpaceBefore(this, value).into()
            }
        }
        unsafe extern "system" fn WidowControl<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::WidowControl(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetWidowControl(this, value).into()
            }
        }
        unsafe extern "system" fn TabCount<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::TabCount(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::AddTab(this, position, align, leader).into()
            }
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::ClearAllTabs(this).into()
            }
        }
        unsafe extern "system" fn DeleteTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::DeleteTab(this, position).into()
            }
        }
        unsafe extern "system" fn GetClone<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::GetClone(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::GetTab(this, index, core::mem::transmute_copy(&position), core::mem::transmute_copy(&align), core::mem::transmute_copy(&leader)).into()
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextParagraphFormat_Impl::IsEqual(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClone<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetClone(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn SetIndents<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: f32, left: f32, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetIndents(this, start, left, right).into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextParagraphFormat_Impl::SetLineSpacing(this, rule, spacing).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextParagraphFormat, OFFSET>(),
            Alignment: Alignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            FirstLineIndent: FirstLineIndent::<Identity, OFFSET>,
            KeepTogether: KeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            KeepWithNext: KeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            LeftIndent: LeftIndent::<Identity, OFFSET>,
            LineSpacing: LineSpacing::<Identity, OFFSET>,
            LineSpacingRule: LineSpacingRule::<Identity, OFFSET>,
            ListAlignment: ListAlignment::<Identity, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, OFFSET>,
            ListLevelIndex: ListLevelIndex::<Identity, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, OFFSET>,
            ListStart: ListStart::<Identity, OFFSET>,
            SetListStart: SetListStart::<Identity, OFFSET>,
            ListStyle: ListStyle::<Identity, OFFSET>,
            SetListStyle: SetListStyle::<Identity, OFFSET>,
            ListTab: ListTab::<Identity, OFFSET>,
            SetListTab: SetListTab::<Identity, OFFSET>,
            ListType: ListType::<Identity, OFFSET>,
            SetListType: SetListType::<Identity, OFFSET>,
            NoLineNumber: NoLineNumber::<Identity, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, OFFSET>,
            PageBreakBefore: PageBreakBefore::<Identity, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, OFFSET>,
            RightIndent: RightIndent::<Identity, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, OFFSET>,
            RightToLeft: RightToLeft::<Identity, OFFSET>,
            SetRightToLeft: SetRightToLeft::<Identity, OFFSET>,
            Style: Style::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            SpaceAfter: SpaceAfter::<Identity, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, OFFSET>,
            SpaceBefore: SpaceBefore::<Identity, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, OFFSET>,
            WidowControl: WidowControl::<Identity, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, OFFSET>,
            TabCount: TabCount::<Identity, OFFSET>,
            AddTab: AddTab::<Identity, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, OFFSET>,
            DeleteTab: DeleteTab::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            GetTab: GetTab::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            SetClone: SetClone::<Identity, OFFSET>,
            SetIndents: SetIndents::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextParagraphFormat as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextParagraphFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Alignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ParagraphAlignment) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, ParagraphAlignment) -> windows_core::HRESULT,
    pub FirstLineIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub KeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub KeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub LeftIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub LineSpacingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LineSpacingRule) -> windows_core::HRESULT,
    pub ListAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MarkerAlignment) -> windows_core::HRESULT,
    pub SetListAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, MarkerAlignment) -> windows_core::HRESULT,
    pub ListLevelIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListLevelIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ListStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ListStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MarkerStyle) -> windows_core::HRESULT,
    pub SetListStyle: unsafe extern "system" fn(*mut core::ffi::c_void, MarkerStyle) -> windows_core::HRESULT,
    pub ListTab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ListType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MarkerType) -> windows_core::HRESULT,
    pub SetListType: unsafe extern "system" fn(*mut core::ffi::c_void, MarkerType) -> windows_core::HRESULT,
    pub NoLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetNoLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub PageBreakBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetPageBreakBefore: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub RightIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRightIndent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub RightToLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetRightToLeft: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub Style: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ParagraphStyle) -> windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, ParagraphStyle) -> windows_core::HRESULT,
    pub SpaceAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceAfter: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SpaceBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceBefore: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub WidowControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FormatEffect) -> windows_core::HRESULT,
    pub SetWidowControl: unsafe extern "system" fn(*mut core::ffi::c_void, FormatEffect) -> windows_core::HRESULT,
    pub TabCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32, TabAlignment, TabLeader) -> windows_core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetClone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTab: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut f32, *mut TabAlignment, *mut TabLeader) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetClone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIndents: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, LineSpacingRule, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextRange, ITextRange_Vtbl, 0x5b9e4e57_c072_42a0_8945_af503ee54768);
impl windows_core::RuntimeType for ITextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITextRange, windows_core::IUnknown, windows_core::IInspectable);
impl ITextRange {
    pub fn Character(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Character)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FormattedText(&self) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFormattedText<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormattedText)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EndPosition(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gravity(&self) -> windows_core::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gravity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGravity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Link(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Link)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLink(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLink)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParagraphFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetParagraphFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartPosition(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StoryLength(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoryLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CanPaste(&self, format: i32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPaste)(windows_core::Interface::as_raw(this), format, &mut result__).map(|| result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ChangeCase)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collapse(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Collapse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Copy(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Copy)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Cut(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cut)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delete)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndOf)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Expand)(windows_core::Interface::as_raw(this), unit, &mut result__).map(|| result__)
        }
    }
    pub fn FindText(&self, value: &windows_core::HSTRING, scanlength: i32, options: FindOptions) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), scanlength, options, &mut result__).map(|| result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetCharacterUtf32)(windows_core::Interface::as_raw(this), value, offset).ok() }
    }
    pub fn GetClone(&self) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetClone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndex)(windows_core::Interface::as_raw(this), unit, &mut result__).map(|| result__)
        }
    }
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), horizontalalign, verticalalign, options, point).ok() }
    }
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), options, rect, hit).ok() }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<P1>(&self, options: TextGetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetTextViaStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn InRange<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InRange)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<P5>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &windows_core::HSTRING, value: P5) -> windows_core::Result<()>
    where
        P5: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertImage)(windows_core::Interface::as_raw(this), width, height, ascent, verticalalign, core::mem::transmute_copy(alternatetext), value.param().abi()).ok() }
    }
    pub fn InStory<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InStory)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Move)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveEnd)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveStart)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn Paste(&self, format: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Paste)(windows_core::Interface::as_raw(this), format).ok() }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ScrollIntoView)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MatchSelection(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).MatchSelection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIndex)(windows_core::Interface::as_raw(this), unit, index, extend).ok() }
    }
    pub fn SetPoint(&self, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPoint)(windows_core::Interface::as_raw(this), point, options, extend).ok() }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRange)(windows_core::Interface::as_raw(this), startposition, endposition).ok() }
    }
    pub fn SetText2(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText2)(windows_core::Interface::as_raw(this), options, core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<P1>(&self, options: TextSetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextViaStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartOf)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.UI.Text.ITextRange";
}
#[cfg(feature = "Storage_Streams")]
pub trait ITextRange_Impl: windows_core::IUnknownImpl {
    fn Character(&self) -> windows_core::Result<u16>;
    fn SetCharacter(&self, value: u16) -> windows_core::Result<()>;
    fn CharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(&self, value: windows_core::Ref<ITextCharacterFormat>) -> windows_core::Result<()>;
    fn FormattedText(&self) -> windows_core::Result<ITextRange>;
    fn SetFormattedText(&self, value: windows_core::Ref<ITextRange>) -> windows_core::Result<()>;
    fn EndPosition(&self) -> windows_core::Result<i32>;
    fn SetEndPosition(&self, value: i32) -> windows_core::Result<()>;
    fn Gravity(&self) -> windows_core::Result<RangeGravity>;
    fn SetGravity(&self, value: RangeGravity) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn Link(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLink(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(&self, value: windows_core::Ref<ITextParagraphFormat>) -> windows_core::Result<()>;
    fn StartPosition(&self) -> windows_core::Result<i32>;
    fn SetStartPosition(&self, value: i32) -> windows_core::Result<()>;
    fn StoryLength(&self) -> windows_core::Result<i32>;
    fn Text(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn CanPaste(&self, format: i32) -> windows_core::Result<bool>;
    fn ChangeCase(&self, value: LetterCase) -> windows_core::Result<()>;
    fn Collapse(&self, value: bool) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<()>;
    fn Cut(&self) -> windows_core::Result<()>;
    fn Delete(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
    fn Expand(&self, unit: TextRangeUnit) -> windows_core::Result<i32>;
    fn FindText(&self, value: &windows_core::HSTRING, scanLength: i32, options: FindOptions) -> windows_core::Result<i32>;
    fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextRange>;
    fn GetIndex(&self, unit: TextRangeUnit) -> windows_core::Result<i32>;
    fn GetPoint(&self, horizontalAlign: HorizontalCharacterAlignment, verticalAlign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> windows_core::Result<()>;
    fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> windows_core::Result<()>;
    fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()>;
    fn GetTextViaStream(&self, options: TextGetOptions, value: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn InRange(&self, range: windows_core::Ref<ITextRange>) -> windows_core::Result<bool>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, verticalAlign: VerticalCharacterAlignment, alternateText: &windows_core::HSTRING, value: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn InStory(&self, range: windows_core::Ref<ITextRange>) -> windows_core::Result<bool>;
    fn IsEqual(&self, range: windows_core::Ref<ITextRange>) -> windows_core::Result<bool>;
    fn Move(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn Paste(&self, format: i32) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, value: PointOptions) -> windows_core::Result<()>;
    fn MatchSelection(&self) -> windows_core::Result<()>;
    fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::Result<()>;
    fn SetPoint(&self, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::Result<()>;
    fn SetRange(&self, startPosition: i32, endPosition: i32) -> windows_core::Result<()>;
    fn SetText2(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn SetTextViaStream(&self, options: TextSetOptions, value: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
}
#[cfg(feature = "Storage_Streams")]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ITextRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Character<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Character(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCharacter<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetCharacter(this, value).into()
            }
        }
        unsafe extern "system" fn CharacterFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::CharacterFormat(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCharacterFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetCharacterFormat(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn FormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FormattedText(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetFormattedText(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn EndPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::EndPosition(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetEndPosition(this, value).into()
            }
        }
        unsafe extern "system" fn Gravity<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut RangeGravity) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Gravity(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: RangeGravity) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetGravity(this, value).into()
            }
        }
        unsafe extern "system" fn Length<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Length(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Link<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Link(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLink<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetLink(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ParagraphFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::ParagraphFormat(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParagraphFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetParagraphFormat(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::StartPosition(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetStartPosition(this, value).into()
            }
        }
        unsafe extern "system" fn StoryLength<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::StoryLength(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Text<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Text(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetText(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: i32, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::CanPaste(this, format) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: LetterCase) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::ChangeCase(this, value).into()
            }
        }
        unsafe extern "system" fn Collapse<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Collapse(this, value).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Copy(this).into()
            }
        }
        unsafe extern "system" fn Cut<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Cut(this).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Delete(this, unit, count) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::EndOf(this, unit, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Expand<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Expand(this, unit) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, scanlength: i32, options: FindOptions, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindText(this, core::mem::transmute(&value), scanlength, options) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCharacterUtf32<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32, offset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetCharacterUtf32(this, core::mem::transmute_copy(&value), offset).into()
            }
        }
        unsafe extern "system" fn GetClone<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetClone(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetIndex(this, unit) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetPoint(this, horizontalalign, verticalalign, options, core::mem::transmute_copy(&point)).into()
            }
        }
        unsafe extern "system" fn GetRect<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetRect(this, options, core::mem::transmute_copy(&rect), core::mem::transmute_copy(&hit)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetText(this, options, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTextViaStream<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetTextViaStream(this, options, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::InRange(this, core::mem::transmute_copy(&range)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::InsertImage(this, width, height, ascent, verticalalign, core::mem::transmute(&alternatetext), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InStory<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::InStory(this, core::mem::transmute_copy(&range)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::IsEqual(this, core::mem::transmute_copy(&range)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Move<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Move(this, unit, count) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEnd(this, unit, count) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStart(this, unit, count) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Paste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Paste(this, format).into()
            }
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PointOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::ScrollIntoView(this, value).into()
            }
        }
        unsafe extern "system" fn MatchSelection<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::MatchSelection(this).into()
            }
        }
        unsafe extern "system" fn SetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetIndex(this, unit, index, extend).into()
            }
        }
        unsafe extern "system" fn SetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetPoint(this, core::mem::transmute(&point), options, extend).into()
            }
        }
        unsafe extern "system" fn SetRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startposition: i32, endposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetRange(this, startposition, endposition).into()
            }
        }
        unsafe extern "system" fn SetText2<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetText2(this, options, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn SetTextViaStream<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetTextViaStream(this, options, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::StartOf(this, unit, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextRange, OFFSET>(),
            Character: Character::<Identity, OFFSET>,
            SetCharacter: SetCharacter::<Identity, OFFSET>,
            CharacterFormat: CharacterFormat::<Identity, OFFSET>,
            SetCharacterFormat: SetCharacterFormat::<Identity, OFFSET>,
            FormattedText: FormattedText::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            EndPosition: EndPosition::<Identity, OFFSET>,
            SetEndPosition: SetEndPosition::<Identity, OFFSET>,
            Gravity: Gravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            SetLink: SetLink::<Identity, OFFSET>,
            ParagraphFormat: ParagraphFormat::<Identity, OFFSET>,
            SetParagraphFormat: SetParagraphFormat::<Identity, OFFSET>,
            StartPosition: StartPosition::<Identity, OFFSET>,
            SetStartPosition: SetStartPosition::<Identity, OFFSET>,
            StoryLength: StoryLength::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            ChangeCase: ChangeCase::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Cut: Cut::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            EndOf: EndOf::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
            FindText: FindText::<Identity, OFFSET>,
            GetCharacterUtf32: GetCharacterUtf32::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetPoint: GetPoint::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetTextViaStream: GetTextViaStream::<Identity, OFFSET>,
            InRange: InRange::<Identity, OFFSET>,
            InsertImage: InsertImage::<Identity, OFFSET>,
            InStory: InStory::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            MoveEnd: MoveEnd::<Identity, OFFSET>,
            MoveStart: MoveStart::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, OFFSET>,
            MatchSelection: MatchSelection::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            SetPoint: SetPoint::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
            SetText2: SetText2::<Identity, OFFSET>,
            SetTextViaStream: SetTextViaStream::<Identity, OFFSET>,
            StartOf: StartOf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Character: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub CharacterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCharacterFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEndPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Gravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RangeGravity) -> windows_core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, RangeGravity) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParagraphFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetParagraphFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanPaste: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut bool) -> windows_core::HRESULT,
    pub ChangeCase: unsafe extern "system" fn(*mut core::ffi::c_void, LetterCase) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub EndOf: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, bool, *mut i32) -> windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, *mut i32) -> windows_core::HRESULT,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, FindOptions, *mut i32) -> windows_core::HRESULT,
    pub GetCharacterUtf32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, i32) -> windows_core::HRESULT,
    pub GetClone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, *mut i32) -> windows_core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, HorizontalCharacterAlignment, VerticalCharacterAlignment, PointOptions, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, PointOptions, *mut super::super::Foundation::Rect, *mut i32) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, TextGetOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTextViaStream: unsafe extern "system" fn(*mut core::ffi::c_void, TextGetOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTextViaStream: usize,
    pub InRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InsertImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, VerticalCharacterAlignment, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InsertImage: usize,
    pub InStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, PointOptions) -> windows_core::HRESULT,
    pub MatchSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, bool) -> windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point, PointOptions, bool) -> windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetText2: unsafe extern "system" fn(*mut core::ffi::c_void, TextSetOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetTextViaStream: unsafe extern "system" fn(*mut core::ffi::c_void, TextSetOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetTextViaStream: usize,
    pub StartOf: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, bool, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextSelection, ITextSelection_Vtbl, 0xa6d36724_f28f_430a_b2cf_c343671ec0e9);
impl windows_core::RuntimeType for ITextSelection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITextSelection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ITextSelection, ITextRange);
impl ITextSelection {
    pub fn Options(&self) -> windows_core::Result<SelectionOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOptions(&self, value: SelectionOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<SelectionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndKey)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HomeKey)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveDown)(windows_core::Interface::as_raw(this), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveLeft)(windows_core::Interface::as_raw(this), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveRight)(windows_core::Interface::as_raw(this), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveUp)(windows_core::Interface::as_raw(this), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub fn TypeText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TypeText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Character(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Character)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FormattedText(&self) -> windows_core::Result<ITextRange> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFormattedText<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFormattedText)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EndPosition(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEndPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gravity(&self) -> windows_core::Result<RangeGravity> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gravity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetGravity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Link(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Link)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLink(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLink)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParagraphFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParagraphFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartPosition(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StoryLength(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoryLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CanPaste(&self, format: i32) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPaste)(windows_core::Interface::as_raw(this), format, &mut result__).map(|| result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ChangeCase)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collapse(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Collapse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Copy(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Copy)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Cut(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cut)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delete)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndOf)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Expand)(windows_core::Interface::as_raw(this), unit, &mut result__).map(|| result__)
        }
    }
    pub fn FindText(&self, value: &windows_core::HSTRING, scanlength: i32, options: FindOptions) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), scanlength, options, &mut result__).map(|| result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetCharacterUtf32)(windows_core::Interface::as_raw(this), value, offset).ok() }
    }
    pub fn GetClone(&self) -> windows_core::Result<ITextRange> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetClone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndex)(windows_core::Interface::as_raw(this), unit, &mut result__).map(|| result__)
        }
    }
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), horizontalalign, verticalalign, options, point).ok() }
    }
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), options, rect, hit).ok() }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<P1>(&self, options: TextGetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetTextViaStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn InRange<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InRange)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<P5>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &windows_core::HSTRING, value: P5) -> windows_core::Result<()>
    where
        P5: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertImage)(windows_core::Interface::as_raw(this), width, height, ascent, verticalalign, core::mem::transmute_copy(alternatetext), value.param().abi()).ok() }
    }
    pub fn InStory<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InStory)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Move)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveEnd)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveStart)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn Paste(&self, format: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Paste)(windows_core::Interface::as_raw(this), format).ok() }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ScrollIntoView)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MatchSelection(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).MatchSelection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIndex)(windows_core::Interface::as_raw(this), unit, index, extend).ok() }
    }
    pub fn SetPoint(&self, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPoint)(windows_core::Interface::as_raw(this), point, options, extend).ok() }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRange)(windows_core::Interface::as_raw(this), startposition, endposition).ok() }
    }
    pub fn SetText2(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetText2)(windows_core::Interface::as_raw(this), options, core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<P1>(&self, options: TextSetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTextViaStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartOf)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Windows.UI.Text.ITextSelection";
}
#[cfg(feature = "Storage_Streams")]
pub trait ITextSelection_Impl: ITextRange_Impl {
    fn Options(&self) -> windows_core::Result<SelectionOptions>;
    fn SetOptions(&self, value: SelectionOptions) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<SelectionType>;
    fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
    fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
    fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn TypeText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage_Streams")]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ITextSelection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Options<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SelectionOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::Options(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOptions<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SelectionOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextSelection_Impl::SetOptions(this, value).into()
            }
        }
        unsafe extern "system" fn Type<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SelectionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::Type(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::EndKey(this, unit, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::HomeKey(this, unit, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveDown(this, unit, count, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveLeft(this, unit, count, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveRight(this, unit, count, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveUp(this, unit, count, extend) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TypeText<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextSelection_Impl::TypeText(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextSelection, OFFSET>(),
            Options: Options::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            EndKey: EndKey::<Identity, OFFSET>,
            HomeKey: HomeKey::<Identity, OFFSET>,
            MoveDown: MoveDown::<Identity, OFFSET>,
            MoveLeft: MoveLeft::<Identity, OFFSET>,
            MoveRight: MoveRight::<Identity, OFFSET>,
            MoveUp: MoveUp::<Identity, OFFSET>,
            TypeText: TypeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SelectionOptions) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, SelectionOptions) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SelectionType) -> windows_core::HRESULT,
    pub EndKey: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, bool, *mut i32) -> windows_core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, bool, *mut i32) -> windows_core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, bool, *mut i32) -> windows_core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, bool, *mut i32) -> windows_core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, bool, *mut i32) -> windows_core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(*mut core::ffi::c_void, TextRangeUnit, i32, bool, *mut i32) -> windows_core::HRESULT,
    pub TypeText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: Self = Self(0i32);
    pub const Upper: Self = Self(1i32);
}
impl windows_core::TypeKind for LetterCase {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LetterCase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LetterCase;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const OneAndHalf: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const AtLeast: Self = Self(4i32);
    pub const Exactly: Self = Self(5i32);
    pub const Multiple: Self = Self(6i32);
    pub const Percent: Self = Self(7i32);
}
impl windows_core::TypeKind for LineSpacingRule {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LineSpacingRule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LineSpacingRule;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: Self = Self(0i32);
    pub const NotALink: Self = Self(1i32);
    pub const ClientLink: Self = Self(2i32);
    pub const FriendlyLinkName: Self = Self(3i32);
    pub const FriendlyLinkAddress: Self = Self(4i32);
    pub const AutoLink: Self = Self(5i32);
    pub const AutoLinkEmail: Self = Self(6i32);
    pub const AutoLinkPhone: Self = Self(7i32);
    pub const AutoLinkPath: Self = Self(8i32);
}
impl windows_core::TypeKind for LinkType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for LinkType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LinkType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl windows_core::TypeKind for MarkerAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MarkerAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: Self = Self(0i32);
    pub const Parenthesis: Self = Self(1i32);
    pub const Parentheses: Self = Self(2i32);
    pub const Period: Self = Self(3i32);
    pub const Plain: Self = Self(4i32);
    pub const Minus: Self = Self(5i32);
    pub const NoNumber: Self = Self(6i32);
}
impl windows_core::TypeKind for MarkerStyle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MarkerStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerStyle;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Bullet: Self = Self(2i32);
    pub const Arabic: Self = Self(3i32);
    pub const LowercaseEnglishLetter: Self = Self(4i32);
    pub const UppercaseEnglishLetter: Self = Self(5i32);
    pub const LowercaseRoman: Self = Self(6i32);
    pub const UppercaseRoman: Self = Self(7i32);
    pub const UnicodeSequence: Self = Self(8i32);
    pub const CircledNumber: Self = Self(9i32);
    pub const BlackCircleWingding: Self = Self(10i32);
    pub const WhiteCircleWingding: Self = Self(11i32);
    pub const ArabicWide: Self = Self(12i32);
    pub const SimplifiedChinese: Self = Self(13i32);
    pub const TraditionalChinese: Self = Self(14i32);
    pub const JapanSimplifiedChinese: Self = Self(15i32);
    pub const JapanKorea: Self = Self(16i32);
    pub const ArabicDictionary: Self = Self(17i32);
    pub const ArabicAbjad: Self = Self(18i32);
    pub const Hebrew: Self = Self(19i32);
    pub const ThaiAlphabetic: Self = Self(20i32);
    pub const ThaiNumeric: Self = Self(21i32);
    pub const DevanagariVowel: Self = Self(22i32);
    pub const DevanagariConsonant: Self = Self(23i32);
    pub const DevanagariNumeric: Self = Self(24i32);
}
impl windows_core::TypeKind for MarkerType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MarkerType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Justify: Self = Self(4i32);
}
impl windows_core::TypeKind for ParagraphAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ParagraphAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.ParagraphAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const Heading1: Self = Self(3i32);
    pub const Heading2: Self = Self(4i32);
    pub const Heading3: Self = Self(5i32);
    pub const Heading4: Self = Self(6i32);
    pub const Heading5: Self = Self(7i32);
    pub const Heading6: Self = Self(8i32);
    pub const Heading7: Self = Self(9i32);
    pub const Heading8: Self = Self(10i32);
    pub const Heading9: Self = Self(11i32);
}
impl windows_core::TypeKind for ParagraphStyle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ParagraphStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.ParagraphStyle;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeInset: Self = Self(1u32);
    pub const Start: Self = Self(32u32);
    pub const ClientCoordinates: Self = Self(256u32);
    pub const AllowOffClient: Self = Self(512u32);
    pub const Transform: Self = Self(1024u32);
    pub const NoHorizontalScroll: Self = Self(65536u32);
    pub const NoVerticalScroll: Self = Self(262144u32);
}
impl windows_core::TypeKind for PointOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PointOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.PointOptions;u4)");
}
impl PointOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PointOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PointOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PointOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PointOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PointOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Inward: Self = Self(3i32);
    pub const Outward: Self = Self(4i32);
}
impl windows_core::TypeKind for RangeGravity {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RangeGravity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.RangeGravity;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: Self = Self(0i32);
    pub const MathOnly: Self = Self(1i32);
}
impl windows_core::TypeKind for RichEditMathMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RichEditMathMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.RichEditMathMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditTextDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RichEditTextDocument, windows_core::IUnknown, windows_core::IInspectable, ITextDocument);
impl RichEditTextDocument {
    pub fn CaretType(&self) -> windows_core::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaretType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCaretType(&self, value: CaretType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCaretType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DefaultTabStop(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Selection(&self) -> windows_core::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UndoLimit(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndoLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUndoLimit(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUndoLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanCopy(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCopy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanPaste(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPaste)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanRedo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRedo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanUndo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanUndo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ApplyDisplayUpdates(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyDisplayUpdates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BatchDisplayUpdates(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatchDisplayUpdates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BeginUndoGroup(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginUndoGroup)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EndUndoGroup(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndUndoGroup)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefaultCharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultCharacterFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultParagraphFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRange)(windows_core::Interface::as_raw(this), startposition, endposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRangeFromPoint(&self, point: super::super::Foundation::Point, options: PointOptions) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRangeFromPoint)(windows_core::Interface::as_raw(this), point, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<P1>(&self, options: TextSetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LoadFromStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn Redo(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Redo)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveToStream<P1>(&self, options: TextGetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SaveToStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn SetDefaultCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultCharacterFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetDefaultParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultParagraphFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetText(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), options, core::mem::transmute_copy(value)).ok() }
    }
    pub fn Undo(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Undo)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AlignmentIncludesTrailingWhitespace(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ITextDocument2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlignmentIncludesTrailingWhitespace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlignmentIncludesTrailingWhitespace(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextDocument2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlignmentIncludesTrailingWhitespace)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IgnoreTrailingCharacterSpacing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ITextDocument2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnoreTrailingCharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextDocument2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIgnoreTrailingCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClearUndoRedoHistory(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextDocument3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ClearUndoRedoHistory)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetMath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetMath(&self, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetMath)(windows_core::Interface::as_raw(this), value as *mut _ as _).ok() }
    }
    pub fn SetMathMode(&self, mode: RichEditMathMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMathMode)(windows_core::Interface::as_raw(this), mode).ok() }
    }
}
impl windows_core::RuntimeType for RichEditTextDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextDocument>();
}
unsafe impl windows_core::Interface for RichEditTextDocument {
    type Vtable = <ITextDocument as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextDocument as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RichEditTextDocument {
    const NAME: &'static str = "Windows.UI.Text.RichEditTextDocument";
}
unsafe impl Send for RichEditTextDocument {}
unsafe impl Sync for RichEditTextDocument {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichEditTextRange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RichEditTextRange, windows_core::IUnknown, windows_core::IInspectable, ITextRange);
impl RichEditTextRange {
    pub fn ContentLinkInfo(&self) -> windows_core::Result<ContentLinkInfo> {
        let this = &windows_core::Interface::cast::<IRichEditTextRange>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLinkInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentLinkInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ContentLinkInfo>,
    {
        let this = &windows_core::Interface::cast::<IRichEditTextRange>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContentLinkInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Character(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Character)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCharacterFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextCharacterFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FormattedText(&self) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFormattedText<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormattedText)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EndPosition(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gravity(&self) -> windows_core::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gravity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGravity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Link(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Link)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLink(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLink)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParagraphFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetParagraphFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITextParagraphFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetParagraphFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartPosition(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StoryLength(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoryLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CanPaste(&self, format: i32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPaste)(windows_core::Interface::as_raw(this), format, &mut result__).map(|| result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ChangeCase)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Collapse(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Collapse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Copy(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Copy)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Cut(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cut)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delete)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndOf)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Expand)(windows_core::Interface::as_raw(this), unit, &mut result__).map(|| result__)
        }
    }
    pub fn FindText(&self, value: &windows_core::HSTRING, scanlength: i32, options: FindOptions) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), scanlength, options, &mut result__).map(|| result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetCharacterUtf32)(windows_core::Interface::as_raw(this), value, offset).ok() }
    }
    pub fn GetClone(&self) -> windows_core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetClone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndex)(windows_core::Interface::as_raw(this), unit, &mut result__).map(|| result__)
        }
    }
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), horizontalalign, verticalalign, options, point).ok() }
    }
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), options, rect, hit).ok() }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<P1>(&self, options: TextGetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetTextViaStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn InRange<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InRange)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<P5>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &windows_core::HSTRING, value: P5) -> windows_core::Result<()>
    where
        P5: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertImage)(windows_core::Interface::as_raw(this), width, height, ascent, verticalalign, core::mem::transmute_copy(alternatetext), value.param().abi()).ok() }
    }
    pub fn InStory<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InStory)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, range: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ITextRange>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Move)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveEnd)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveStart)(windows_core::Interface::as_raw(this), unit, count, &mut result__).map(|| result__)
        }
    }
    pub fn Paste(&self, format: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Paste)(windows_core::Interface::as_raw(this), format).ok() }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ScrollIntoView)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MatchSelection(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).MatchSelection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIndex)(windows_core::Interface::as_raw(this), unit, index, extend).ok() }
    }
    pub fn SetPoint(&self, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPoint)(windows_core::Interface::as_raw(this), point, options, extend).ok() }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRange)(windows_core::Interface::as_raw(this), startposition, endposition).ok() }
    }
    pub fn SetText2(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText2)(windows_core::Interface::as_raw(this), options, core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<P1>(&self, options: TextSetOptions, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextViaStream)(windows_core::Interface::as_raw(this), options, value.param().abi()).ok() }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartOf)(windows_core::Interface::as_raw(this), unit, extend, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RichEditTextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextRange>();
}
unsafe impl windows_core::Interface for RichEditTextRange {
    type Vtable = <ITextRange as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextRange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RichEditTextRange {
    const NAME: &'static str = "Windows.UI.Text.RichEditTextRange";
}
unsafe impl Send for RichEditTextRange {}
unsafe impl Sync for RichEditTextRange {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: Self = Self(1u32);
    pub const AtEndOfLine: Self = Self(2u32);
    pub const Overtype: Self = Self(4u32);
    pub const Active: Self = Self(8u32);
    pub const Replace: Self = Self(16u32);
}
impl windows_core::TypeKind for SelectionOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SelectionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.SelectionOptions;u4)");
}
impl SelectionOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SelectionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SelectionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SelectionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SelectionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: Self = Self(0i32);
    pub const InsertionPoint: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const InlineShape: Self = Self(7i32);
    pub const Shape: Self = Self(8i32);
}
impl windows_core::TypeKind for SelectionType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SelectionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.SelectionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Decimal: Self = Self(3i32);
    pub const Bar: Self = Self(4i32);
}
impl windows_core::TypeKind for TabAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TabAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TabAlignment;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: Self = Self(0i32);
    pub const Dots: Self = Self(1i32);
    pub const Dashes: Self = Self(2i32);
    pub const Lines: Self = Self(3i32);
    pub const ThickLines: Self = Self(4i32);
    pub const Equals: Self = Self(5i32);
}
impl windows_core::TypeKind for TabLeader {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TabLeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TabLeader;i4)");
}
pub struct TextConstants;
impl TextConstants {
    pub fn AutoColor() -> windows_core::Result<super::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MinUnitCount() -> windows_core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinUnitCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MaxUnitCount() -> windows_core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxUnitCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UndefinedColor() -> windows_core::Result<super::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UndefinedFloatValue() -> windows_core::Result<f32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedFloatValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UndefinedInt32Value() -> windows_core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedInt32Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UndefinedFontStretch() -> windows_core::Result<FontStretch> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedFontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn UndefinedFontStyle() -> windows_core::Result<FontStyle> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndefinedFontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn ITextConstantsStatics<R, F: FnOnce(&ITextConstantsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextConstants, ITextConstantsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for TextConstants {
    const NAME: &'static str = "Windows.UI.Text.TextConstants";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextDecorations(pub u32);
impl TextDecorations {
    pub const None: Self = Self(0u32);
    pub const Underline: Self = Self(1u32);
    pub const Strikethrough: Self = Self(2u32);
}
impl windows_core::TypeKind for TextDecorations {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextDecorations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextDecorations;u4)");
}
impl TextDecorations {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextDecorations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextDecorations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextDecorations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextDecorations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextDecorations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: Self = Self(0u32);
    pub const AdjustCrlf: Self = Self(1u32);
    pub const UseCrlf: Self = Self(2u32);
    pub const UseObjectText: Self = Self(4u32);
    pub const AllowFinalEop: Self = Self(8u32);
    pub const NoHidden: Self = Self(32u32);
    pub const IncludeNumbering: Self = Self(64u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const UseLf: Self = Self(16777216u32);
}
impl windows_core::TypeKind for TextGetOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextGetOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextGetOptions;u4)");
}
impl TextGetOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: Self = Self(0i32);
    pub const Word: Self = Self(1i32);
    pub const Sentence: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const Story: Self = Self(5i32);
    pub const Screen: Self = Self(6i32);
    pub const Section: Self = Self(7i32);
    pub const Window: Self = Self(8i32);
    pub const CharacterFormat: Self = Self(9i32);
    pub const ParagraphFormat: Self = Self(10i32);
    pub const Object: Self = Self(11i32);
    pub const HardParagraph: Self = Self(12i32);
    pub const Cluster: Self = Self(13i32);
    pub const Bold: Self = Self(14i32);
    pub const Italic: Self = Self(15i32);
    pub const Underline: Self = Self(16i32);
    pub const Strikethrough: Self = Self(17i32);
    pub const ProtectedText: Self = Self(18i32);
    pub const Link: Self = Self(19i32);
    pub const SmallCaps: Self = Self(20i32);
    pub const AllCaps: Self = Self(21i32);
    pub const Hidden: Self = Self(22i32);
    pub const Outline: Self = Self(23i32);
    pub const Shadow: Self = Self(24i32);
    pub const Imprint: Self = Self(25i32);
    pub const Disabled: Self = Self(26i32);
    pub const Revised: Self = Self(27i32);
    pub const Subscript: Self = Self(28i32);
    pub const Superscript: Self = Self(29i32);
    pub const FontBound: Self = Self(30i32);
    pub const LinkProtected: Self = Self(31i32);
    pub const ContentLink: Self = Self(32i32);
}
impl windows_core::TypeKind for TextRangeUnit {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextRangeUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextRangeUnit;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: Self = Self(0i32);
    pub const Ansi: Self = Self(1i32);
    pub const EastEurope: Self = Self(2i32);
    pub const Cyrillic: Self = Self(3i32);
    pub const Greek: Self = Self(4i32);
    pub const Turkish: Self = Self(5i32);
    pub const Hebrew: Self = Self(6i32);
    pub const Arabic: Self = Self(7i32);
    pub const Baltic: Self = Self(8i32);
    pub const Vietnamese: Self = Self(9i32);
    pub const Default: Self = Self(10i32);
    pub const Symbol: Self = Self(11i32);
    pub const Thai: Self = Self(12i32);
    pub const ShiftJis: Self = Self(13i32);
    pub const GB2312: Self = Self(14i32);
    pub const Hangul: Self = Self(15i32);
    pub const Big5: Self = Self(16i32);
    pub const PC437: Self = Self(17i32);
    pub const Oem: Self = Self(18i32);
    pub const Mac: Self = Self(19i32);
    pub const Armenian: Self = Self(20i32);
    pub const Syriac: Self = Self(21i32);
    pub const Thaana: Self = Self(22i32);
    pub const Devanagari: Self = Self(23i32);
    pub const Bengali: Self = Self(24i32);
    pub const Gurmukhi: Self = Self(25i32);
    pub const Gujarati: Self = Self(26i32);
    pub const Oriya: Self = Self(27i32);
    pub const Tamil: Self = Self(28i32);
    pub const Telugu: Self = Self(29i32);
    pub const Kannada: Self = Self(30i32);
    pub const Malayalam: Self = Self(31i32);
    pub const Sinhala: Self = Self(32i32);
    pub const Lao: Self = Self(33i32);
    pub const Tibetan: Self = Self(34i32);
    pub const Myanmar: Self = Self(35i32);
    pub const Georgian: Self = Self(36i32);
    pub const Jamo: Self = Self(37i32);
    pub const Ethiopic: Self = Self(38i32);
    pub const Cherokee: Self = Self(39i32);
    pub const Aboriginal: Self = Self(40i32);
    pub const Ogham: Self = Self(41i32);
    pub const Runic: Self = Self(42i32);
    pub const Khmer: Self = Self(43i32);
    pub const Mongolian: Self = Self(44i32);
    pub const Braille: Self = Self(45i32);
    pub const Yi: Self = Self(46i32);
    pub const Limbu: Self = Self(47i32);
    pub const TaiLe: Self = Self(48i32);
    pub const NewTaiLue: Self = Self(49i32);
    pub const SylotiNagri: Self = Self(50i32);
    pub const Kharoshthi: Self = Self(51i32);
    pub const Kayahli: Self = Self(52i32);
    pub const UnicodeSymbol: Self = Self(53i32);
    pub const Emoji: Self = Self(54i32);
    pub const Glagolitic: Self = Self(55i32);
    pub const Lisu: Self = Self(56i32);
    pub const Vai: Self = Self(57i32);
    pub const NKo: Self = Self(58i32);
    pub const Osmanya: Self = Self(59i32);
    pub const PhagsPa: Self = Self(60i32);
    pub const Gothic: Self = Self(61i32);
    pub const Deseret: Self = Self(62i32);
    pub const Tifinagh: Self = Self(63i32);
}
impl windows_core::TypeKind for TextScript {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextScript {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextScript;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: Self = Self(0u32);
    pub const UnicodeBidi: Self = Self(1u32);
    pub const Unlink: Self = Self(8u32);
    pub const Unhide: Self = Self(16u32);
    pub const CheckTextLimit: Self = Self(32u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const ApplyRtfDocumentDefaults: Self = Self(16384u32);
}
impl windows_core::TypeKind for TextSetOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextSetOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextSetOptions;u4)");
}
impl TextSetOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Single: Self = Self(2i32);
    pub const Words: Self = Self(3i32);
    pub const Double: Self = Self(4i32);
    pub const Dotted: Self = Self(5i32);
    pub const Dash: Self = Self(6i32);
    pub const DashDot: Self = Self(7i32);
    pub const DashDotDot: Self = Self(8i32);
    pub const Wave: Self = Self(9i32);
    pub const Thick: Self = Self(10i32);
    pub const Thin: Self = Self(11i32);
    pub const DoubleWave: Self = Self(12i32);
    pub const HeavyWave: Self = Self(13i32);
    pub const LongDash: Self = Self(14i32);
    pub const ThickDash: Self = Self(15i32);
    pub const ThickDashDot: Self = Self(16i32);
    pub const ThickDashDotDot: Self = Self(17i32);
    pub const ThickDotted: Self = Self(18i32);
    pub const ThickLongDash: Self = Self(19i32);
}
impl windows_core::TypeKind for UnderlineType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for UnderlineType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.UnderlineType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: Self = Self(0i32);
    pub const Baseline: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl windows_core::TypeKind for VerticalCharacterAlignment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VerticalCharacterAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.VerticalCharacterAlignment;i4)");
}

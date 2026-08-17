windows_core::imp::define_interface!(IAlternateWordForm, IAlternateWordForm_Vtbl, 0x47396c1e_51b9_4207_9146_248e636a1d1d);
impl windows_core::RuntimeType for IAlternateWordForm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAlternateWordForm_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceTextSegment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextSegment) -> windows_core::HRESULT,
    pub AlternateText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NormalizationFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AlternateNormalizationFormat) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectableWordSegment, ISelectableWordSegment_Vtbl, 0x916a4cb7_8aa7_4c78_b374_5dedb752e60b);
impl windows_core::RuntimeType for ISelectableWordSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISelectableWordSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextSegment) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectableWordsSegmenter, ISelectableWordsSegmenter_Vtbl, 0xf6dc31e7_4b13_45c5_8897_7d71269e085d);
impl windows_core::RuntimeType for ISelectableWordsSegmenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISelectableWordsSegmenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetTokenAt: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTokens: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTokens: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Tokenize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tokenize: usize,
}
windows_core::imp::define_interface!(ISelectableWordsSegmenterFactory, ISelectableWordsSegmenterFactory_Vtbl, 0x8c7a7648_6057_4339_bc70_f210010a4150);
impl windows_core::RuntimeType for ISelectableWordsSegmenterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISelectableWordsSegmenterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISemanticTextQuery, ISemanticTextQuery_Vtbl, 0x6a1cab51_1fb2_4909_80b8_35731a2b3e7f);
impl windows_core::RuntimeType for ISemanticTextQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISemanticTextQuery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Find: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Find: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindInProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindInProperty: usize,
}
windows_core::imp::define_interface!(ISemanticTextQueryFactory, ISemanticTextQueryFactory_Vtbl, 0x238c0503_f995_4587_8777_a2b7d80acfef);
impl windows_core::RuntimeType for ISemanticTextQueryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISemanticTextQueryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextConversionGenerator, ITextConversionGenerator_Vtbl, 0x03606a5e_2aa9_4ab6_af8b_a562b63a8992);
impl windows_core::RuntimeType for ITextConversionGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextConversionGenerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithMaxCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithMaxCountAsync: usize,
}
windows_core::imp::define_interface!(ITextConversionGeneratorFactory, ITextConversionGeneratorFactory_Vtbl, 0xfcaa3781_3083_49ab_be15_56dfbbb74d6f);
impl windows_core::RuntimeType for ITextConversionGeneratorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextConversionGeneratorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextPhoneme, ITextPhoneme_Vtbl, 0x9362a40a_9b7a_4569_94cf_d84f2f38cf9b);
impl windows_core::RuntimeType for ITextPhoneme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextPhoneme_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReadingText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextPredictionGenerator, ITextPredictionGenerator_Vtbl, 0x5eacab07_abf1_4cb6_9d9e_326f2b468756);
impl windows_core::RuntimeType for ITextPredictionGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextPredictionGenerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithMaxCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithMaxCountAsync: usize,
}
windows_core::imp::define_interface!(ITextPredictionGenerator2, ITextPredictionGenerator2_Vtbl, 0xb84723b8_2c77_486a_900a_a3453eedc15d);
impl windows_core::RuntimeType for ITextPredictionGenerator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextPredictionGenerator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithParametersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, TextPredictionOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithParametersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNextWordCandidatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNextWordCandidatesAsync: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub InputScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Text::Core::CoreTextInputScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    InputScope: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub SetInputScope: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Text::Core::CoreTextInputScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    SetInputScope: usize,
}
windows_core::imp::define_interface!(ITextPredictionGeneratorFactory, ITextPredictionGeneratorFactory_Vtbl, 0x7257b416_8ba2_4751_9d30_9d85435653a2);
impl windows_core::RuntimeType for ITextPredictionGeneratorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextPredictionGeneratorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextReverseConversionGenerator, ITextReverseConversionGenerator_Vtbl, 0x51e7f514_9c51_4d86_ae1b_b498fbad8313);
impl windows_core::RuntimeType for ITextReverseConversionGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextReverseConversionGenerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ConvertBackAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextReverseConversionGenerator2, ITextReverseConversionGenerator2_Vtbl, 0x1aafd2ec_85d6_46fd_828a_3a4830fa6e18);
impl windows_core::RuntimeType for ITextReverseConversionGenerator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextReverseConversionGenerator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPhonemesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPhonemesAsync: usize,
}
windows_core::imp::define_interface!(ITextReverseConversionGeneratorFactory, ITextReverseConversionGeneratorFactory_Vtbl, 0x63bed326_1fda_41f6_89d5_23ddea3c729a);
impl windows_core::RuntimeType for ITextReverseConversionGeneratorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextReverseConversionGeneratorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnicodeCharactersStatics, IUnicodeCharactersStatics_Vtbl, 0x97909e87_9291_4f91_b6c8_b6e359d7a7fb);
impl windows_core::RuntimeType for IUnicodeCharactersStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnicodeCharactersStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCodepointFromSurrogatePair: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetSurrogatePairFromCodepoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut u16) -> windows_core::HRESULT,
    pub IsHighSurrogate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsLowSurrogate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsSupplementary: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsNoncharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsAlphabetic: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsCased: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsUppercase: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsLowercase: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsIdStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsIdContinue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsGraphemeBase: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub IsGraphemeExtend: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub GetNumericType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut UnicodeNumericType) -> windows_core::HRESULT,
    pub GetGeneralCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut UnicodeGeneralCategory) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWordSegment, IWordSegment_Vtbl, 0xd2d4ba6d_987c_4cc0_b6bd_d49a11b38f9a);
impl windows_core::RuntimeType for IWordSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWordSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextSegment) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateForms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateForms: usize,
}
windows_core::imp::define_interface!(IWordsSegmenter, IWordsSegmenter_Vtbl, 0x86b4d4d1_b2fe_4e34_a81d_66640300454f);
impl windows_core::RuntimeType for IWordsSegmenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWordsSegmenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetTokenAt: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTokens: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTokens: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Tokenize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tokenize: usize,
}
windows_core::imp::define_interface!(IWordsSegmenterFactory, IWordsSegmenterFactory_Vtbl, 0xe6977274_fc35_455c_8bfb_6d7f4653ca97);
impl windows_core::RuntimeType for IWordsSegmenterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWordsSegmenterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AlternateWordForm(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AlternateWordForm, windows_core::IUnknown, windows_core::IInspectable);
impl AlternateWordForm {
    pub fn SourceTextSegment(&self) -> windows_core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceTextSegment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AlternateText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlternateText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NormalizationFormat(&self) -> windows_core::Result<AlternateNormalizationFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizationFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AlternateWordForm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAlternateWordForm>();
}
unsafe impl windows_core::Interface for AlternateWordForm {
    type Vtable = IAlternateWordForm_Vtbl;
    const IID: windows_core::GUID = <IAlternateWordForm as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.AlternateWordForm";
}
unsafe impl Send for AlternateWordForm {}
unsafe impl Sync for AlternateWordForm {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SelectableWordSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SelectableWordSegment, windows_core::IUnknown, windows_core::IInspectable);
impl SelectableWordSegment {
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceTextSegment(&self) -> windows_core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceTextSegment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SelectableWordSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISelectableWordSegment>();
}
unsafe impl windows_core::Interface for SelectableWordSegment {
    type Vtable = ISelectableWordSegment_Vtbl;
    const IID: windows_core::GUID = <ISelectableWordSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordSegment";
}
unsafe impl Send for SelectableWordSegment {}
unsafe impl Sync for SelectableWordSegment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SelectableWordsSegmenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SelectableWordsSegmenter, windows_core::IUnknown, windows_core::IInspectable);
impl SelectableWordsSegmenter {
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTokenAt(&self, text: &windows_core::HSTRING, startindex: u32) -> windows_core::Result<SelectableWordSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTokenAt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), startindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTokens)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize<P0>(&self, text: &windows_core::HSTRING, startindex: u32, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SelectableWordSegmentsTokenizingHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Tokenize)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), startindex, handler.param().abi()).ok() }
    }
    pub fn CreateWithLanguage(language: &windows_core::HSTRING) -> windows_core::Result<SelectableWordsSegmenter> {
        Self::ISelectableWordsSegmenterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(language), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISelectableWordsSegmenterFactory<R, F: FnOnce(&ISelectableWordsSegmenterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SelectableWordsSegmenter, ISelectableWordsSegmenterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SelectableWordsSegmenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISelectableWordsSegmenter>();
}
unsafe impl windows_core::Interface for SelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_Vtbl;
    const IID: windows_core::GUID = <ISelectableWordsSegmenter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordsSegmenter";
}
unsafe impl Send for SelectableWordsSegmenter {}
unsafe impl Sync for SelectableWordsSegmenter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SemanticTextQuery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SemanticTextQuery, windows_core::IUnknown, windows_core::IInspectable);
impl SemanticTextQuery {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Find(&self, content: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Find)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(content), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindInProperty(&self, propertycontent: &windows_core::HSTRING, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindInProperty)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertycontent), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(aqsfilter: &windows_core::HSTRING) -> windows_core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithLanguage(aqsfilter: &windows_core::HSTRING, filterlanguage: &windows_core::HSTRING) -> windows_core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), core::mem::transmute_copy(filterlanguage), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISemanticTextQueryFactory<R, F: FnOnce(&ISemanticTextQueryFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SemanticTextQuery, ISemanticTextQueryFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SemanticTextQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISemanticTextQuery>();
}
unsafe impl windows_core::Interface for SemanticTextQuery {
    type Vtable = ISemanticTextQuery_Vtbl;
    const IID: windows_core::GUID = <ISemanticTextQuery as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.SemanticTextQuery";
}
unsafe impl Send for SemanticTextQuery {}
unsafe impl Sync for SemanticTextQuery {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TextConversionGenerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextConversionGenerator, windows_core::IUnknown, windows_core::IInspectable);
impl TextConversionGenerator {
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageAvailableButNotInstalled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesAsync(&self, input: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCandidatesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesWithMaxCountAsync(&self, input: &windows_core::HSTRING, maxcandidates: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCandidatesWithMaxCountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), maxcandidates, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(languagetag: &windows_core::HSTRING) -> windows_core::Result<TextConversionGenerator> {
        Self::ITextConversionGeneratorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(languagetag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITextConversionGeneratorFactory<R, F: FnOnce(&ITextConversionGeneratorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextConversionGenerator, ITextConversionGeneratorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextConversionGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextConversionGenerator>();
}
unsafe impl windows_core::Interface for TextConversionGenerator {
    type Vtable = ITextConversionGenerator_Vtbl;
    const IID: windows_core::GUID = <ITextConversionGenerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextConversionGenerator";
}
unsafe impl Send for TextConversionGenerator {}
unsafe impl Sync for TextConversionGenerator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TextPhoneme(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextPhoneme, windows_core::IUnknown, windows_core::IInspectable);
impl TextPhoneme {
    pub fn DisplayText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadingText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TextPhoneme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextPhoneme>();
}
unsafe impl windows_core::Interface for TextPhoneme {
    type Vtable = ITextPhoneme_Vtbl;
    const IID: windows_core::GUID = <ITextPhoneme as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.TextPhoneme";
}
unsafe impl Send for TextPhoneme {}
unsafe impl Sync for TextPhoneme {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TextPredictionGenerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextPredictionGenerator, windows_core::IUnknown, windows_core::IInspectable);
impl TextPredictionGenerator {
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageAvailableButNotInstalled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesAsync(&self, input: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCandidatesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesWithMaxCountAsync(&self, input: &windows_core::HSTRING, maxcandidates: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCandidatesWithMaxCountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), maxcandidates, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesWithParametersAsync<P0>(&self, input: &windows_core::HSTRING, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCandidatesWithParametersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), maxcandidates, predictionoptions, previousstrings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNextWordCandidatesAsync<P0>(&self, maxcandidates: u32, previousstrings: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNextWordCandidatesAsync)(windows_core::Interface::as_raw(this), maxcandidates, previousstrings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn InputScope(&self) -> windows_core::Result<super::super::UI::Text::Core::CoreTextInputScope> {
        let this = &windows_core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInputScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(languagetag: &windows_core::HSTRING) -> windows_core::Result<TextPredictionGenerator> {
        Self::ITextPredictionGeneratorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(languagetag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITextPredictionGeneratorFactory<R, F: FnOnce(&ITextPredictionGeneratorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextPredictionGenerator, ITextPredictionGeneratorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextPredictionGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextPredictionGenerator>();
}
unsafe impl windows_core::Interface for TextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_Vtbl;
    const IID: windows_core::GUID = <ITextPredictionGenerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextPredictionGenerator";
}
unsafe impl Send for TextPredictionGenerator {}
unsafe impl Sync for TextPredictionGenerator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TextReverseConversionGenerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextReverseConversionGenerator, windows_core::IUnknown, windows_core::IInspectable);
impl TextReverseConversionGenerator {
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageAvailableButNotInstalled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConvertBackAsync(&self, input: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertBackAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPhonemesAsync(&self, input: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>> {
        let this = &windows_core::Interface::cast::<ITextReverseConversionGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPhonemesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(languagetag: &windows_core::HSTRING) -> windows_core::Result<TextReverseConversionGenerator> {
        Self::ITextReverseConversionGeneratorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(languagetag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITextReverseConversionGeneratorFactory<R, F: FnOnce(&ITextReverseConversionGeneratorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextReverseConversionGenerator, ITextReverseConversionGeneratorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextReverseConversionGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextReverseConversionGenerator>();
}
unsafe impl windows_core::Interface for TextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_Vtbl;
    const IID: windows_core::GUID = <ITextReverseConversionGenerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextReverseConversionGenerator";
}
unsafe impl Send for TextReverseConversionGenerator {}
unsafe impl Sync for TextReverseConversionGenerator {}
pub struct UnicodeCharacters;
impl UnicodeCharacters {
    pub fn GetCodepointFromSurrogatePair(highsurrogate: u32, lowsurrogate: u32) -> windows_core::Result<u32> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCodepointFromSurrogatePair)(windows_core::Interface::as_raw(this), highsurrogate, lowsurrogate, &mut result__).map(|| result__)
        })
    }
    pub fn GetSurrogatePairFromCodepoint(codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> windows_core::Result<()> {
        Self::IUnicodeCharactersStatics(|this| unsafe { (windows_core::Interface::vtable(this).GetSurrogatePairFromCodepoint)(windows_core::Interface::as_raw(this), codepoint, highsurrogate, lowsurrogate).ok() })
    }
    pub fn IsHighSurrogate(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHighSurrogate)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsLowSurrogate(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLowSurrogate)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsSupplementary(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupplementary)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsNoncharacter(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNoncharacter)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsWhitespace(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWhitespace)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsAlphabetic(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAlphabetic)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsCased(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCased)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsUppercase(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUppercase)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsLowercase(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLowercase)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsIdStart(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIdStart)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsIdContinue(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIdContinue)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsGraphemeBase(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGraphemeBase)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn IsGraphemeExtend(codepoint: u32) -> windows_core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGraphemeExtend)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn GetNumericType(codepoint: u32) -> windows_core::Result<UnicodeNumericType> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumericType)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    pub fn GetGeneralCategory(codepoint: u32) -> windows_core::Result<UnicodeGeneralCategory> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGeneralCategory)(windows_core::Interface::as_raw(this), codepoint, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IUnicodeCharactersStatics<R, F: FnOnce(&IUnicodeCharactersStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UnicodeCharacters, IUnicodeCharactersStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for UnicodeCharacters {
    const NAME: &'static str = "Windows.Data.Text.UnicodeCharacters";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WordSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WordSegment, windows_core::IUnknown, windows_core::IInspectable);
impl WordSegment {
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceTextSegment(&self) -> windows_core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceTextSegment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateForms(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlternateForms)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WordSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWordSegment>();
}
unsafe impl windows_core::Interface for WordSegment {
    type Vtable = IWordSegment_Vtbl;
    const IID: windows_core::GUID = <IWordSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WordSegment {
    const NAME: &'static str = "Windows.Data.Text.WordSegment";
}
unsafe impl Send for WordSegment {}
unsafe impl Sync for WordSegment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WordsSegmenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WordsSegmenter, windows_core::IUnknown, windows_core::IInspectable);
impl WordsSegmenter {
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTokenAt(&self, text: &windows_core::HSTRING, startindex: u32) -> windows_core::Result<WordSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTokenAt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), startindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTokens)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize<P0>(&self, text: &windows_core::HSTRING, startindex: u32, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WordSegmentsTokenizingHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Tokenize)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), startindex, handler.param().abi()).ok() }
    }
    pub fn CreateWithLanguage(language: &windows_core::HSTRING) -> windows_core::Result<WordsSegmenter> {
        Self::IWordsSegmenterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(language), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWordsSegmenterFactory<R, F: FnOnce(&IWordsSegmenterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WordsSegmenter, IWordsSegmenterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WordsSegmenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWordsSegmenter>();
}
unsafe impl windows_core::Interface for WordsSegmenter {
    type Vtable = IWordsSegmenter_Vtbl;
    const IID: windows_core::GUID = <IWordsSegmenter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.WordsSegmenter";
}
unsafe impl Send for WordsSegmenter {}
unsafe impl Sync for WordsSegmenter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Currency: Self = Self(3i32);
    pub const Date: Self = Self(4i32);
    pub const Time: Self = Self(5i32);
}
impl windows_core::TypeKind for AlternateNormalizationFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AlternateNormalizationFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AlternateNormalizationFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AlternateNormalizationFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.AlternateNormalizationFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: Self = Self(0u32);
    pub const Predictions: Self = Self(1u32);
    pub const Corrections: Self = Self(2u32);
}
impl windows_core::TypeKind for TextPredictionOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TextPredictionOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextPredictionOptions").field(&self.0).finish()
    }
}
impl TextPredictionOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextPredictionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextPredictionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextPredictionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextPredictionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextPredictionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for TextPredictionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.TextPredictionOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UnicodeGeneralCategory(pub i32);
impl UnicodeGeneralCategory {
    pub const UppercaseLetter: Self = Self(0i32);
    pub const LowercaseLetter: Self = Self(1i32);
    pub const TitlecaseLetter: Self = Self(2i32);
    pub const ModifierLetter: Self = Self(3i32);
    pub const OtherLetter: Self = Self(4i32);
    pub const NonspacingMark: Self = Self(5i32);
    pub const SpacingCombiningMark: Self = Self(6i32);
    pub const EnclosingMark: Self = Self(7i32);
    pub const DecimalDigitNumber: Self = Self(8i32);
    pub const LetterNumber: Self = Self(9i32);
    pub const OtherNumber: Self = Self(10i32);
    pub const SpaceSeparator: Self = Self(11i32);
    pub const LineSeparator: Self = Self(12i32);
    pub const ParagraphSeparator: Self = Self(13i32);
    pub const Control: Self = Self(14i32);
    pub const Format: Self = Self(15i32);
    pub const Surrogate: Self = Self(16i32);
    pub const PrivateUse: Self = Self(17i32);
    pub const ConnectorPunctuation: Self = Self(18i32);
    pub const DashPunctuation: Self = Self(19i32);
    pub const OpenPunctuation: Self = Self(20i32);
    pub const ClosePunctuation: Self = Self(21i32);
    pub const InitialQuotePunctuation: Self = Self(22i32);
    pub const FinalQuotePunctuation: Self = Self(23i32);
    pub const OtherPunctuation: Self = Self(24i32);
    pub const MathSymbol: Self = Self(25i32);
    pub const CurrencySymbol: Self = Self(26i32);
    pub const ModifierSymbol: Self = Self(27i32);
    pub const OtherSymbol: Self = Self(28i32);
    pub const NotAssigned: Self = Self(29i32);
}
impl windows_core::TypeKind for UnicodeGeneralCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UnicodeGeneralCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnicodeGeneralCategory").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UnicodeGeneralCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeGeneralCategory;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: Self = Self(0i32);
    pub const Decimal: Self = Self(1i32);
    pub const Digit: Self = Self(2i32);
    pub const Numeric: Self = Self(3i32);
}
impl windows_core::TypeKind for UnicodeNumericType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UnicodeNumericType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnicodeNumericType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UnicodeNumericType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeNumericType;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextSegment {
    pub StartPosition: u32,
    pub Length: u32,
}
impl windows_core::TypeKind for TextSegment {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Data.Text.TextSegment;u4;u4)");
}
impl Default for TextSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::define_interface!(SelectableWordSegmentsTokenizingHandler, SelectableWordSegmentsTokenizingHandler_Vtbl, 0x3a3dfc9c_aede_4dc7_9e6c_41c044bd3592);
#[cfg(feature = "Foundation_Collections")]
impl SelectableWordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = SelectableWordSegmentsTokenizingHandlerBox::<F> { vtable: &SelectableWordSegmentsTokenizingHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<P0, P1>(&self, precedingwords: P0, words: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), precedingwords.param().abi(), words.param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct SelectableWordSegmentsTokenizingHandlerBox<F: FnMut(Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const SelectableWordSegmentsTokenizingHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> windows_core::Result<()> + Send + 'static> SelectableWordSegmentsTokenizingHandlerBox<F> {
    const VTABLE: SelectableWordSegmentsTokenizingHandler_Vtbl = SelectableWordSegmentsTokenizingHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <SelectableWordSegmentsTokenizingHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, precedingwords: *mut core::ffi::c_void, words: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&precedingwords), windows_core::from_raw_borrowed(&words)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for SelectableWordSegmentsTokenizingHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
pub struct SelectableWordSegmentsTokenizingHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::define_interface!(WordSegmentsTokenizingHandler, WordSegmentsTokenizingHandler_Vtbl, 0xa5dd6357_bf2a_4c4f_a31f_29e71c6f8b35);
#[cfg(feature = "Foundation_Collections")]
impl WordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(Option<&super::super::Foundation::Collections::IIterable<WordSegment>>, Option<&super::super::Foundation::Collections::IIterable<WordSegment>>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = WordSegmentsTokenizingHandlerBox::<F> { vtable: &WordSegmentsTokenizingHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<P0, P1>(&self, precedingwords: P0, words: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<WordSegment>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<WordSegment>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), precedingwords.param().abi(), words.param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct WordSegmentsTokenizingHandlerBox<F: FnMut(Option<&super::super::Foundation::Collections::IIterable<WordSegment>>, Option<&super::super::Foundation::Collections::IIterable<WordSegment>>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const WordSegmentsTokenizingHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(Option<&super::super::Foundation::Collections::IIterable<WordSegment>>, Option<&super::super::Foundation::Collections::IIterable<WordSegment>>) -> windows_core::Result<()> + Send + 'static> WordSegmentsTokenizingHandlerBox<F> {
    const VTABLE: WordSegmentsTokenizingHandler_Vtbl = WordSegmentsTokenizingHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <WordSegmentsTokenizingHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, precedingwords: *mut core::ffi::c_void, words: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&precedingwords), windows_core::from_raw_borrowed(&words)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for WordSegmentsTokenizingHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
pub struct WordSegmentsTokenizingHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}

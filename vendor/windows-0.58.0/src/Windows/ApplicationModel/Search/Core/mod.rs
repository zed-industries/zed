windows_core::imp::define_interface!(IRequestingFocusOnKeyboardInputEventArgs, IRequestingFocusOnKeyboardInputEventArgs_Vtbl, 0xa1195f27_b1a7_41a2_879d_6a68687e5985);
impl windows_core::RuntimeType for IRequestingFocusOnKeyboardInputEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRequestingFocusOnKeyboardInputEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISearchSuggestion, ISearchSuggestion_Vtbl, 0x5b5554b0_1527_437b_95c5_8d18d2b8af55);
impl windows_core::RuntimeType for ISearchSuggestion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISearchSuggestion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SearchSuggestionKind) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DetailText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    pub ImageAlternateText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchSuggestionManager, ISearchSuggestionManager_Vtbl, 0x3f0c50a1_cb9d_497b_b500_3c04ac959ad2);
impl windows_core::RuntimeType for ISearchSuggestionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISearchSuggestionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SearchHistoryEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SearchHistoryContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSearchHistoryContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetQueryWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetQueryWithSearchQueryLinguisticDetails: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Suggestions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Suggestions: usize,
    pub AddToHistory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddToHistoryWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ClearHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestionsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestingFocusOnKeyboardInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRequestingFocusOnKeyboardInput: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchSuggestionsRequestedEventArgs, ISearchSuggestionsRequestedEventArgs_Vtbl, 0x6fd519e5_9e7e_4ab4_8be3_c76b1bd4344a);
impl windows_core::RuntimeType for ISearchSuggestionsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISearchSuggestionsRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LinguisticDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RequestingFocusOnKeyboardInputEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RequestingFocusOnKeyboardInputEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RequestingFocusOnKeyboardInputEventArgs {}
impl windows_core::RuntimeType for RequestingFocusOnKeyboardInputEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRequestingFocusOnKeyboardInputEventArgs>();
}
unsafe impl windows_core::Interface for RequestingFocusOnKeyboardInputEventArgs {
    type Vtable = IRequestingFocusOnKeyboardInputEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRequestingFocusOnKeyboardInputEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RequestingFocusOnKeyboardInputEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.RequestingFocusOnKeyboardInputEventArgs";
}
unsafe impl Send for RequestingFocusOnKeyboardInputEventArgs {}
unsafe impl Sync for RequestingFocusOnKeyboardInputEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SearchSuggestion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchSuggestion, windows_core::IUnknown, windows_core::IInspectable);
impl SearchSuggestion {
    pub fn Kind(&self) -> windows_core::Result<SearchSuggestionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetailText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetailText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> windows_core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Image)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImageAlternateText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageAlternateText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchSuggestion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchSuggestion>();
}
unsafe impl windows_core::Interface for SearchSuggestion {
    type Vtable = ISearchSuggestion_Vtbl;
    const IID: windows_core::GUID = <ISearchSuggestion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchSuggestion {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.SearchSuggestion";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SearchSuggestionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchSuggestionManager, windows_core::IUnknown, windows_core::IInspectable);
impl SearchSuggestionManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SearchSuggestionManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchHistoryEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchHistoryEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSearchHistoryEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SearchHistoryContext(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchHistoryContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSearchHistoryContext(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSearchHistoryContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetLocalContentSuggestionSettings<P0>(&self, settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::LocalContentSuggestionSettings>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalContentSuggestionSettings)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() }
    }
    pub fn SetQuery(&self, querytext: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuery)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(querytext)).ok() }
    }
    pub fn SetQueryWithLanguage(&self, querytext: &windows_core::HSTRING, language: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQueryWithLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(querytext), core::mem::transmute_copy(language)).ok() }
    }
    pub fn SetQueryWithSearchQueryLinguisticDetails<P0>(&self, querytext: &windows_core::HSTRING, language: &windows_core::HSTRING, linguisticdetails: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::SearchQueryLinguisticDetails>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQueryWithSearchQueryLinguisticDetails)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(querytext), core::mem::transmute_copy(language), linguisticdetails.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Suggestions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IObservableVector<SearchSuggestion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suggestions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddToHistory(&self, querytext: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddToHistory)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(querytext)).ok() }
    }
    pub fn AddToHistoryWithLanguage(&self, querytext: &windows_core::HSTRING, language: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddToHistoryWithLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(querytext), core::mem::transmute_copy(language)).ok() }
    }
    pub fn ClearHistory(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearHistory)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SuggestionsRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, SearchSuggestionsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestionsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSuggestionsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSuggestionsRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestingFocusOnKeyboardInput<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SearchSuggestionManager, RequestingFocusOnKeyboardInputEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestingFocusOnKeyboardInput)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRequestingFocusOnKeyboardInput(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestingFocusOnKeyboardInput)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for SearchSuggestionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchSuggestionManager>();
}
unsafe impl windows_core::Interface for SearchSuggestionManager {
    type Vtable = ISearchSuggestionManager_Vtbl;
    const IID: windows_core::GUID = <ISearchSuggestionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchSuggestionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.SearchSuggestionManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SearchSuggestionsRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchSuggestionsRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SearchSuggestionsRequestedEventArgs {
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LinguisticDetails(&self) -> windows_core::Result<super::SearchQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Request(&self) -> windows_core::Result<super::SearchSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchSuggestionsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchSuggestionsRequestedEventArgs>();
}
unsafe impl windows_core::Interface for SearchSuggestionsRequestedEventArgs {
    type Vtable = ISearchSuggestionsRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISearchSuggestionsRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.Core.SearchSuggestionsRequestedEventArgs";
}
unsafe impl Send for SearchSuggestionsRequestedEventArgs {}
unsafe impl Sync for SearchSuggestionsRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SearchSuggestionKind(pub i32);
impl SearchSuggestionKind {
    pub const Query: Self = Self(0i32);
    pub const Result: Self = Self(1i32);
    pub const Separator: Self = Self(2i32);
}
impl windows_core::TypeKind for SearchSuggestionKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SearchSuggestionKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SearchSuggestionKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SearchSuggestionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Search.Core.SearchSuggestionKind;i4)");
}

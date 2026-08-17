#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
windows_core::imp::define_interface!(ILocalContentSuggestionSettings, ILocalContentSuggestionSettings_Vtbl, 0xeeaeb062_743d_456e_84a3_23f06f2d15d7);
impl windows_core::RuntimeType for ILocalContentSuggestionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub Locations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    Locations: usize,
    pub SetAqsFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AqsFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PropertiesToMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPane, ISearchPane_Vtbl, 0xfdacec38_3700_4d73_91a1_2f998674238a);
impl windows_core::RuntimeType for ISearchPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPane_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SearchHistoryEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSearchHistoryContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchHistoryContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlaceholderText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub VisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub QueryChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveQueryChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SuggestionsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub QuerySubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveQuerySubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ResultSuggestionChosen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveResultSuggestionChosen: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowOverloadDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowOverloadWithQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetShowOnKeyboardInput: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShowOnKeyboardInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetQueryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneQueryChangedEventArgs, ISearchPaneQueryChangedEventArgs_Vtbl, 0x3c064fe9_2351_4248_a529_7110f464a785);
impl windows_core::RuntimeType for ISearchPaneQueryChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISearchPaneQueryChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ISearchPaneQueryChangedEventArgs {
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LinguisticDetails(&self) -> windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ISearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.ISearchPaneQueryChangedEventArgs";
}
pub trait ISearchPaneQueryChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Language(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn LinguisticDetails(&self) -> windows_core::Result<SearchPaneQueryLinguisticDetails>;
}
impl ISearchPaneQueryChangedEventArgs_Vtbl {
    pub const fn new<Identity: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryText<Identity: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchPaneQueryChangedEventArgs_Impl::QueryText(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Language<Identity: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchPaneQueryChangedEventArgs_Impl::Language(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LinguisticDetails<Identity: ISearchPaneQueryChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchPaneQueryChangedEventArgs_Impl::LinguisticDetails(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISearchPaneQueryChangedEventArgs, OFFSET>(),
            QueryText: QueryText::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            LinguisticDetails: LinguisticDetails::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchPaneQueryChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LinguisticDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneQueryLinguisticDetails, ISearchPaneQueryLinguisticDetails_Vtbl, 0x82fb460e_0940_4b6d_b8d0_642b30989e15);
impl windows_core::RuntimeType for ISearchPaneQueryLinguisticDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryTextAlternatives: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryTextCompositionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneQuerySubmittedEventArgs, ISearchPaneQuerySubmittedEventArgs_Vtbl, 0x143ba4fc_e9c5_4736_91b2_e8eb9cb88356);
impl windows_core::RuntimeType for ISearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails, ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl, 0x460c92e5_4c32_4538_a4d4_b6b4400d140f);
impl windows_core::RuntimeType for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LinguisticDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneResultSuggestionChosenEventArgs, ISearchPaneResultSuggestionChosenEventArgs_Vtbl, 0xc8316cc0_aed2_41e0_bce0_c26ca74f85ec);
impl windows_core::RuntimeType for ISearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneStatics, ISearchPaneStatics_Vtbl, 0x9572adf1_8f1d_481f_a15b_c61655f16a0e);
impl windows_core::RuntimeType for ISearchPaneStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneStaticsWithHideThisApplication, ISearchPaneStaticsWithHideThisApplication_Vtbl, 0x00732830_50f1_4d03_99ac_c6644c8ed8b5);
impl windows_core::RuntimeType for ISearchPaneStaticsWithHideThisApplication {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HideThisApplication: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneSuggestionsRequest, ISearchPaneSuggestionsRequest_Vtbl, 0x81b10b1c_e561_4093_9b4d_2ad482794a53);
impl windows_core::RuntimeType for ISearchPaneSuggestionsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneSuggestionsRequestDeferral, ISearchPaneSuggestionsRequestDeferral_Vtbl, 0xa0d009f7_8748_4ee2_ad44_afa6be997c51);
impl windows_core::RuntimeType for ISearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneSuggestionsRequestedEventArgs, ISearchPaneSuggestionsRequestedEventArgs_Vtbl, 0xc89b8a2f_ac56_4460_8d2f_80023bec4fc5);
impl windows_core::RuntimeType for ISearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchPaneVisibilityChangedEventArgs, ISearchPaneVisibilityChangedEventArgs_Vtbl, 0x3c4d3046_ac4b_49f2_97d6_020e6182cb9c);
impl windows_core::RuntimeType for ISearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchQueryLinguisticDetails, ISearchQueryLinguisticDetails_Vtbl, 0x46a1205b_69c9_4745_b72f_a8a4fc8f24ae);
impl windows_core::RuntimeType for ISearchQueryLinguisticDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QueryTextAlternatives: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryTextCompositionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchQueryLinguisticDetailsFactory, ISearchQueryLinguisticDetailsFactory_Vtbl, 0xcac6c3b8_3c64_4dfd_ad9f_479e4d4065a4);
impl windows_core::RuntimeType for ISearchQueryLinguisticDetailsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchSuggestionCollection, ISearchSuggestionCollection_Vtbl, 0x323a8a4b_fbea_4446_abbc_3da7915fdd3a);
impl windows_core::RuntimeType for ISearchSuggestionCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AppendQuerySuggestion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendQuerySuggestions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AppendResultSuggestion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendResultSuggestion: usize,
    pub AppendSearchSeparator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchSuggestionsRequest, ISearchSuggestionsRequest_Vtbl, 0x4e4e26a7_44e5_4039_9099_6000ead1f0c6);
impl windows_core::RuntimeType for ISearchSuggestionsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchSuggestionsRequestDeferral, ISearchSuggestionsRequestDeferral_Vtbl, 0xb71598a9_c065_456d_a845_1eccec5dc28b);
impl windows_core::RuntimeType for ISearchSuggestionsRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalContentSuggestionSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LocalContentSuggestionSettings, windows_core::IUnknown, windows_core::IInspectable);
impl LocalContentSuggestionSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LocalContentSuggestionSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn Locations(&self) -> windows_core::Result<windows_collections::IVector<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Locations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAqsFilter(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAqsFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AqsFilter(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AqsFilter)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PropertiesToMatch(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertiesToMatch)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LocalContentSuggestionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILocalContentSuggestionSettings>();
}
unsafe impl windows_core::Interface for LocalContentSuggestionSettings {
    type Vtable = <ILocalContentSuggestionSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILocalContentSuggestionSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPane(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPane, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPane {
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSearchHistoryEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SearchHistoryEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchHistoryEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSearchHistoryContext(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSearchHistoryContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SearchHistoryContext(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchHistoryContext)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPlaceholderText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaceholderText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PlaceholderText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaceholderText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VisibilityChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisibilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVisibilityChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVisibilityChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn QueryChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveQueryChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveQueryChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SuggestionsRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestionsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSuggestionsRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSuggestionsRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn QuerySubmitted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuerySubmitted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveQuerySubmitted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveQuerySubmitted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ResultSuggestionChosen<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultSuggestionChosen)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResultSuggestionChosen(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveResultSuggestionChosen)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetLocalContentSuggestionSettings<P0>(&self, settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LocalContentSuggestionSettings>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalContentSuggestionSettings)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() }
    }
    pub fn ShowOverloadDefault(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowOverloadDefault)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ShowOverloadWithQuery(&self, query: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowOverloadWithQuery)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(query)).ok() }
    }
    pub fn SetShowOnKeyboardInput(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowOnKeyboardInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowOnKeyboardInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowOnKeyboardInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TrySetQueryText(&self, query: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetQueryText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(query), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<SearchPane> {
        Self::ISearchPaneStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HideThisApplication() -> windows_core::Result<()> {
        Self::ISearchPaneStaticsWithHideThisApplication(|this| unsafe { (windows_core::Interface::vtable(this).HideThisApplication)(windows_core::Interface::as_raw(this)).ok() })
    }
    fn ISearchPaneStatics<R, F: FnOnce(&ISearchPaneStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SearchPane, ISearchPaneStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISearchPaneStaticsWithHideThisApplication<R, F: FnOnce(&ISearchPaneStaticsWithHideThisApplication) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SearchPane, ISearchPaneStaticsWithHideThisApplication> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SearchPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPane>();
}
unsafe impl windows_core::Interface for SearchPane {
    type Vtable = <ISearchPane as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPane as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPane";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneQueryChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneQueryChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ISearchPaneQueryChangedEventArgs);
impl SearchPaneQueryChangedEventArgs {
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LinguisticDetails(&self) -> windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchPaneQueryChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneQueryChangedEventArgs>();
}
unsafe impl windows_core::Interface for SearchPaneQueryChangedEventArgs {
    type Vtable = <ISearchPaneQueryChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneQueryChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
}
unsafe impl Send for SearchPaneQueryChangedEventArgs {}
unsafe impl Sync for SearchPaneQueryChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneQueryLinguisticDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneQueryLinguisticDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPaneQueryLinguisticDetails {
    pub fn QueryTextAlternatives(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryTextAlternatives)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn QueryTextCompositionStart(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryTextCompositionStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryTextCompositionLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SearchPaneQueryLinguisticDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneQueryLinguisticDetails>();
}
unsafe impl windows_core::Interface for SearchPaneQueryLinguisticDetails {
    type Vtable = <ISearchPaneQueryLinguisticDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneQueryLinguisticDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
}
unsafe impl Send for SearchPaneQueryLinguisticDetails {}
unsafe impl Sync for SearchPaneQueryLinguisticDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneQuerySubmittedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneQuerySubmittedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPaneQuerySubmittedEventArgs {
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LinguisticDetails(&self) -> windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &windows_core::Interface::cast::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneQuerySubmittedEventArgs>();
}
unsafe impl windows_core::Interface for SearchPaneQuerySubmittedEventArgs {
    type Vtable = <ISearchPaneQuerySubmittedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneQuerySubmittedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
}
unsafe impl Send for SearchPaneQuerySubmittedEventArgs {}
unsafe impl Sync for SearchPaneQuerySubmittedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneResultSuggestionChosenEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneResultSuggestionChosenEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPaneResultSuggestionChosenEventArgs {
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneResultSuggestionChosenEventArgs>();
}
unsafe impl windows_core::Interface for SearchPaneResultSuggestionChosenEventArgs {
    type Vtable = <ISearchPaneResultSuggestionChosenEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneResultSuggestionChosenEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
}
unsafe impl Send for SearchPaneResultSuggestionChosenEventArgs {}
unsafe impl Sync for SearchPaneResultSuggestionChosenEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneSuggestionsRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneSuggestionsRequest, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPaneSuggestionsRequest {
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SearchSuggestionCollection(&self) -> windows_core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchSuggestionCollection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<SearchPaneSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchPaneSuggestionsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneSuggestionsRequest>();
}
unsafe impl windows_core::Interface for SearchPaneSuggestionsRequest {
    type Vtable = <ISearchPaneSuggestionsRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneSuggestionsRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
}
unsafe impl Send for SearchPaneSuggestionsRequest {}
unsafe impl Sync for SearchPaneSuggestionsRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneSuggestionsRequestDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneSuggestionsRequestDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPaneSuggestionsRequestDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneSuggestionsRequestDeferral>();
}
unsafe impl windows_core::Interface for SearchPaneSuggestionsRequestDeferral {
    type Vtable = <ISearchPaneSuggestionsRequestDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneSuggestionsRequestDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
}
unsafe impl Send for SearchPaneSuggestionsRequestDeferral {}
unsafe impl Sync for SearchPaneSuggestionsRequestDeferral {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneSuggestionsRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneSuggestionsRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SearchPaneSuggestionsRequestedEventArgs, ISearchPaneQueryChangedEventArgs);
impl SearchPaneSuggestionsRequestedEventArgs {
    pub fn QueryText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LinguisticDetails(&self) -> windows_core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &windows_core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LinguisticDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Request(&self) -> windows_core::Result<SearchPaneSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneSuggestionsRequestedEventArgs>();
}
unsafe impl windows_core::Interface for SearchPaneSuggestionsRequestedEventArgs {
    type Vtable = <ISearchPaneSuggestionsRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneSuggestionsRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
}
unsafe impl Send for SearchPaneSuggestionsRequestedEventArgs {}
unsafe impl Sync for SearchPaneSuggestionsRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchPaneVisibilityChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchPaneVisibilityChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SearchPaneVisibilityChangedEventArgs {
    pub fn Visible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchPaneVisibilityChangedEventArgs>();
}
unsafe impl windows_core::Interface for SearchPaneVisibilityChangedEventArgs {
    type Vtable = <ISearchPaneVisibilityChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchPaneVisibilityChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
}
unsafe impl Send for SearchPaneVisibilityChangedEventArgs {}
unsafe impl Sync for SearchPaneVisibilityChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchQueryLinguisticDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchQueryLinguisticDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SearchQueryLinguisticDetails {
    pub fn QueryTextAlternatives(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryTextAlternatives)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn QueryTextCompositionStart(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryTextCompositionStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryTextCompositionLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance<P0>(querytextalternatives: P0, querytextcompositionstart: u32, querytextcompositionlength: u32) -> windows_core::Result<SearchQueryLinguisticDetails>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ISearchQueryLinguisticDetailsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), querytextalternatives.param().abi(), querytextcompositionstart, querytextcompositionlength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISearchQueryLinguisticDetailsFactory<R, F: FnOnce(&ISearchQueryLinguisticDetailsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SearchQueryLinguisticDetails, ISearchQueryLinguisticDetailsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SearchQueryLinguisticDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchQueryLinguisticDetails>();
}
unsafe impl windows_core::Interface for SearchQueryLinguisticDetails {
    type Vtable = <ISearchQueryLinguisticDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchQueryLinguisticDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
}
unsafe impl Send for SearchQueryLinguisticDetails {}
unsafe impl Sync for SearchQueryLinguisticDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchSuggestionCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchSuggestionCollection, windows_core::IUnknown, windows_core::IInspectable);
impl SearchSuggestionCollection {
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppendQuerySuggestion(&self, text: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AppendQuerySuggestion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text)).ok() }
    }
    pub fn AppendQuerySuggestions<P0>(&self, suggestions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AppendQuerySuggestions)(windows_core::Interface::as_raw(this), suggestions.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendResultSuggestion<P3>(&self, text: &windows_core::HSTRING, detailtext: &windows_core::HSTRING, tag: &windows_core::HSTRING, image: P3, imagealternatetext: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P3: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AppendResultSuggestion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), core::mem::transmute_copy(detailtext), core::mem::transmute_copy(tag), image.param().abi(), core::mem::transmute_copy(imagealternatetext)).ok() }
    }
    pub fn AppendSearchSeparator(&self, label: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AppendSearchSeparator)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(label)).ok() }
    }
}
impl windows_core::RuntimeType for SearchSuggestionCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchSuggestionCollection>();
}
unsafe impl windows_core::Interface for SearchSuggestionCollection {
    type Vtable = <ISearchSuggestionCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchSuggestionCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionCollection";
}
unsafe impl Send for SearchSuggestionCollection {}
unsafe impl Sync for SearchSuggestionCollection {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchSuggestionsRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchSuggestionsRequest, windows_core::IUnknown, windows_core::IInspectable);
impl SearchSuggestionsRequest {
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SearchSuggestionCollection(&self) -> windows_core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchSuggestionCollection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<SearchSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SearchSuggestionsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchSuggestionsRequest>();
}
unsafe impl windows_core::Interface for SearchSuggestionsRequest {
    type Vtable = <ISearchSuggestionsRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchSuggestionsRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequest";
}
unsafe impl Send for SearchSuggestionsRequest {}
unsafe impl Sync for SearchSuggestionsRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchSuggestionsRequestDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SearchSuggestionsRequestDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl SearchSuggestionsRequestDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SearchSuggestionsRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISearchSuggestionsRequestDeferral>();
}
unsafe impl windows_core::Interface for SearchSuggestionsRequestDeferral {
    type Vtable = <ISearchSuggestionsRequestDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISearchSuggestionsRequestDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
}
unsafe impl Send for SearchSuggestionsRequestDeferral {}
unsafe impl Sync for SearchSuggestionsRequestDeferral {}

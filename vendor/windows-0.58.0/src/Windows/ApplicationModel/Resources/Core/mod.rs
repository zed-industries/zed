windows_core::imp::define_interface!(INamedResource, INamedResource_Vtbl, 0x1c98c219_0b13_4240_89a5_d495dc189a00);
impl windows_core::RuntimeType for INamedResource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INamedResource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Candidates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Candidates: usize,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResolveForContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveAllForContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveAllForContext: usize,
}
windows_core::imp::define_interface!(IResourceCandidate, IResourceCandidate_Vtbl, 0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
impl windows_core::RuntimeType for IResourceCandidate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceCandidate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Qualifiers: usize,
    pub IsMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMatchAsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ValueAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub GetValueAsFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetValueAsFileAsync: usize,
    pub GetQualifierValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceCandidate2, IResourceCandidate2_Vtbl, 0x69e5b468_f6fc_4013_aaa2_d53f1757d3b5);
impl windows_core::RuntimeType for IResourceCandidate2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceCandidate2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetValueAsStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetValueAsStreamAsync: usize,
}
windows_core::imp::define_interface!(IResourceCandidate3, IResourceCandidate3_Vtbl, 0x08ae97f8_517a_4674_958c_4a3c7cd2cc6b);
impl windows_core::RuntimeType for IResourceCandidate3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceCandidate3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ResourceCandidateKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceContext, IResourceContext_Vtbl, 0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
impl windows_core::RuntimeType for IResourceContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub QualifierValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QualifierValues: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetQualifierValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetQualifierValues: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OverrideToMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OverrideToMatch: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetLanguages: usize,
}
windows_core::imp::define_interface!(IResourceContextStatics, IResourceContextStatics_Vtbl, 0x98be9d6c_6338_4b31_99df_b2b442f17149);
impl windows_core::RuntimeType for IResourceContextStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceContextStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMatchingContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMatchingContext: usize,
}
windows_core::imp::define_interface!(IResourceContextStatics2, IResourceContextStatics2_Vtbl, 0x41f752ef_12af_41b9_ab36_b1eb4b512460);
impl windows_core::RuntimeType for IResourceContextStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceContextStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGlobalQualifierValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ResetGlobalQualifierValues: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetGlobalQualifierValuesForSpecifiedQualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetGlobalQualifierValuesForSpecifiedQualifiers: usize,
    pub GetForViewIndependentUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceContextStatics3, IResourceContextStatics3_Vtbl, 0x20cf492c_af0f_450b_9da6_106dd0c29a39);
impl windows_core::RuntimeType for IResourceContextStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceContextStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetGlobalQualifierValueWithPersistence: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, ResourceQualifierPersistence) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceContextStatics4, IResourceContextStatics4_Vtbl, 0x22eb9ccd_fb31_4bfa_b86b_df9d9d7bdc39);
impl windows_core::RuntimeType for IResourceContextStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceContextStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
windows_core::imp::define_interface!(IResourceManager, IResourceManager_Vtbl, 0xf744d97b_9988_44fb_abd6_5378844cfa8b);
impl windows_core::RuntimeType for IResourceManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MainResourceMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllResourceMaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllResourceMaps: usize,
    pub DefaultContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub LoadPriFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    LoadPriFiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub UnloadPriFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    UnloadPriFiles: usize,
}
windows_core::imp::define_interface!(IResourceManager2, IResourceManager2_Vtbl, 0x9d66fe6c_a4d7_4c23_9e85_675f304c252d);
impl windows_core::RuntimeType for IResourceManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllNamedResourcesForPackage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, ResourceLayoutInfo, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllNamedResourcesForPackage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSubtreesForPackage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, ResourceLayoutInfo, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSubtreesForPackage: usize,
}
windows_core::imp::define_interface!(IResourceManagerStatics, IResourceManagerStatics_Vtbl, 0x1cc0fdfc_69ee_4e43_9901_47f12687baf7);
impl windows_core::RuntimeType for IResourceManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsResourceReference: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceMap, IResourceMap_Vtbl, 0x72284824_db8c_42f8_b08c_53ff357dad82);
impl windows_core::RuntimeType for IResourceMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceMap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValueForContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceQualifier, IResourceQualifier_Vtbl, 0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
impl windows_core::RuntimeType for IResourceQualifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResourceQualifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QualifierName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Score: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NamedResource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NamedResource, windows_core::IUnknown, windows_core::IInspectable);
impl NamedResource {
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Candidates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Candidates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resolve(&self) -> windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resolve)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolveForContext<P0>(&self, resourcecontext: P0) -> windows_core::Result<ResourceCandidate>
    where
        P0: windows_core::Param<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolveForContext)(windows_core::Interface::as_raw(this), resourcecontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveAll(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolveAll)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveAllForContext<P0>(&self, resourcecontext: P0) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>
    where
        P0: windows_core::Param<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolveAllForContext)(windows_core::Interface::as_raw(this), resourcecontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NamedResource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INamedResource>();
}
unsafe impl windows_core::Interface for NamedResource {
    type Vtable = INamedResource_Vtbl;
    const IID: windows_core::GUID = <INamedResource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.NamedResource";
}
unsafe impl Send for NamedResource {}
unsafe impl Sync for NamedResource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceCandidate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceCandidate, windows_core::IUnknown, windows_core::IInspectable);
impl ResourceCandidate {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Qualifiers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Qualifiers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsMatch(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMatch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMatchAsDefault(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMatchAsDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDefault(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValueAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueAsString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetValueAsFileAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValueAsFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetQualifierValue(&self, qualifiername: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetQualifierValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(qualifiername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetValueAsStreamAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &windows_core::Interface::cast::<IResourceCandidate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValueAsStreamAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ResourceCandidateKind> {
        let this = &windows_core::Interface::cast::<IResourceCandidate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ResourceCandidate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceCandidate>();
}
unsafe impl windows_core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
    const IID: windows_core::GUID = <IResourceCandidate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidate";
}
unsafe impl Send for ResourceCandidate {}
unsafe impl Sync for ResourceCandidate {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceCandidateVectorView(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceCandidateVectorView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceCandidateVectorView, super::super::super::Foundation::Collections::IIterable::<ResourceCandidate>, super::super::super::Foundation::Collections::IVectorView::<ResourceCandidate>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceCandidateVectorView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<ResourceCandidate>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ResourceCandidate>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<ResourceCandidate>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceCandidateVectorView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceCandidateVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<ResourceCandidate>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceCandidateVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceCandidateVectorView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceContext, windows_core::IUnknown, windows_core::IInspectable);
impl ResourceContext {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceContext, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn QualifierValues(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IObservableMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualifierValues)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetQualifierValues<P0>(&self, qualifiernames: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResetQualifierValues)(windows_core::Interface::as_raw(this), qualifiernames.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OverrideToMatch<P0>(&self, result: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OverrideToMatch)(windows_core::Interface::as_raw(this), result.param().abi()).ok() }
    }
    pub fn Clone(&self) -> windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetLanguages<P0>(&self, languages: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguages)(windows_core::Interface::as_raw(this), languages.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMatchingContext<P0>(result: P0) -> windows_core::Result<ResourceContext>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>,
    {
        Self::IResourceContextStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMatchingContext)(windows_core::Interface::as_raw(this), result.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentView() -> windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetGlobalQualifierValue(key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetGlobalQualifierValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value)).ok() })
    }
    pub fn ResetGlobalQualifierValues() -> windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (windows_core::Interface::vtable(this).ResetGlobalQualifierValues)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetGlobalQualifierValuesForSpecifiedQualifiers<P0>(qualifiernames: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IResourceContextStatics2(|this| unsafe { (windows_core::Interface::vtable(this).ResetGlobalQualifierValuesForSpecifiedQualifiers)(windows_core::Interface::as_raw(this), qualifiernames.param().abi()).ok() })
    }
    pub fn GetForViewIndependentUse() -> windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForViewIndependentUse)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetGlobalQualifierValueWithPersistence(key: &windows_core::HSTRING, value: &windows_core::HSTRING, persistence: ResourceQualifierPersistence) -> windows_core::Result<()> {
        Self::IResourceContextStatics3(|this| unsafe { (windows_core::Interface::vtable(this).SetGlobalQualifierValueWithPersistence)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), persistence).ok() })
    }
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<ResourceContext>
    where
        P0: windows_core::Param<super::super::super::UI::UIContext>,
    {
        Self::IResourceContextStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics<R, F: FnOnce(&IResourceContextStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics2<R, F: FnOnce(&IResourceContextStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics3<R, F: FnOnce(&IResourceContextStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics4<R, F: FnOnce(&IResourceContextStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceContext>();
}
unsafe impl windows_core::Interface for ResourceContext {
    type Vtable = IResourceContext_Vtbl;
    const IID: windows_core::GUID = <IResourceContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContext";
}
unsafe impl Send for ResourceContext {}
unsafe impl Sync for ResourceContext {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceContextLanguagesVectorView(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceContextLanguagesVectorView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceContextLanguagesVectorView, super::super::super::Foundation::Collections::IIterable::<windows_core::HSTRING>, super::super::super::Foundation::Collections::IVectorView::<windows_core::HSTRING>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceContextLanguagesVectorView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &windows_core::HSTRING, index: &mut u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [windows_core::HSTRING]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceContextLanguagesVectorView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceContextLanguagesVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<windows_core::HSTRING>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceContextLanguagesVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceContextLanguagesVectorView {
    type Item = windows_core::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceContextLanguagesVectorView {
    type Item = windows_core::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceContextLanguagesVectorView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceManager, windows_core::IUnknown, windows_core::IInspectable);
impl ResourceManager {
    pub fn MainResourceMap(&self) -> windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MainResourceMap)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllResourceMaps(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllResourceMaps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultContext(&self) -> windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn LoadPriFiles<P0>(&self, files: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LoadPriFiles)(windows_core::Interface::as_raw(this), files.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn UnloadPriFiles<P0>(&self, files: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnloadPriFiles)(windows_core::Interface::as_raw(this), files.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllNamedResourcesForPackage(&self, packagename: &windows_core::HSTRING, resourcelayoutinfo: ResourceLayoutInfo) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>> {
        let this = &windows_core::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllNamedResourcesForPackage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagename), resourcelayoutinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSubtreesForPackage(&self, packagename: &windows_core::HSTRING, resourcelayoutinfo: ResourceLayoutInfo) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>> {
        let this = &windows_core::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllSubtreesForPackage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagename), resourcelayoutinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<ResourceManager> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsResourceReference(resourcereference: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResourceReference)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(resourcereference), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceManagerStatics<R, F: FnOnce(&IResourceManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceManager, IResourceManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceManager>();
}
unsafe impl windows_core::Interface for ResourceManager {
    type Vtable = IResourceManager_Vtbl;
    const IID: windows_core::GUID = <IResourceManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceManager";
}
unsafe impl Send for ResourceManager {}
unsafe impl Sync for ResourceManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceMap, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceMap, super::super::super::Foundation::Collections::IIterable::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, NamedResource>>, super::super::super::Foundation::Collections::IMapView::<windows_core::HSTRING, NamedResource>);
impl ResourceMap {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<NamedResource> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut Option<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, NamedResource>>, second: &mut Option<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, NamedResource>>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, NamedResource>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetValue(&self, resource: &windows_core::HSTRING) -> windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(resource), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetValueForContext<P0>(&self, resource: &windows_core::HSTRING, context: P0) -> windows_core::Result<ResourceCandidate>
    where
        P0: windows_core::Param<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetValueForContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(resource), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSubtree(&self, reference: &windows_core::HSTRING) -> windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSubtree)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(reference), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ResourceMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceMap>();
}
unsafe impl windows_core::Interface for ResourceMap {
    type Vtable = IResourceMap_Vtbl;
    const IID: windows_core::GUID = <IResourceMap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMap";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for ResourceMap {}
unsafe impl Sync for ResourceMap {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceMapIterator(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceMapIterator, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceMapIterator, super::super::super::Foundation::Collections::IIterator::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, NamedResource>>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [Option<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceMapIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceMapIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, NamedResource>> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceMapIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceMapMapView(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceMapMapView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceMapMapView, super::super::super::Foundation::Collections::IIterable::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, ResourceMap>>, super::super::super::Foundation::Collections::IMapView::<windows_core::HSTRING, ResourceMap>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut Option<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ResourceMap>>, second: &mut Option<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ResourceMap>>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceMapMapView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ResourceMap>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceMapMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapView_Vtbl<windows_core::HSTRING, ResourceMap>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ResourceMap> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceMapMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceMapMapViewIterator(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceMapMapViewIterator, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceMapMapViewIterator, super::super::super::Foundation::Collections::IIterator::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, ResourceMap>>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapViewIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [Option<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceMapMapViewIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceMapMapViewIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, ResourceMap>> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceMapMapViewIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceMapMapViewIterator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceQualifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceQualifier, windows_core::IUnknown, windows_core::IInspectable);
impl ResourceQualifier {
    pub fn QualifierName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualifierName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn QualifierValue(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualifierValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsDefault(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMatch(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMatch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Score(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Score)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ResourceQualifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceQualifier>();
}
unsafe impl windows_core::Interface for ResourceQualifier {
    type Vtable = IResourceQualifier_Vtbl;
    const IID: windows_core::GUID = <IResourceQualifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifier";
}
unsafe impl Send for ResourceQualifier {}
unsafe impl Sync for ResourceQualifier {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceQualifierMapView(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceQualifierMapView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceQualifierMapView, super::super::super::Foundation::Collections::IIterable::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, windows_core::HSTRING>>, super::super::super::Foundation::Collections::IMapView::<windows_core::HSTRING, windows_core::HSTRING>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierMapView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut Option<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>, second: &mut Option<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceQualifierMapView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceQualifierMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapView_Vtbl<windows_core::HSTRING, windows_core::HSTRING>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceQualifierMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceQualifierObservableMap(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceQualifierObservableMap, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceQualifierObservableMap, super::super::super::Foundation::Collections::IIterable::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, windows_core::HSTRING>>, super::super::super::Foundation::Collections::IMap::<windows_core::HSTRING, windows_core::HSTRING>, super::super::super::Foundation::Collections::IObservableMap::<windows_core::HSTRING, windows_core::HSTRING>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierObservableMap {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::MapChangedEventHandler<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceQualifierObservableMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceQualifierObservableMap {
    type Vtable = super::super::super::Foundation::Collections::IObservableMap_Vtbl<windows_core::HSTRING, windows_core::HSTRING>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IObservableMap<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceQualifierObservableMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ResourceQualifierVectorView(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(ResourceQualifierVectorView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(ResourceQualifierVectorView, super::super::super::Foundation::Collections::IIterable::<ResourceQualifier>, super::super::super::Foundation::Collections::IVectorView::<ResourceQualifier>);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierVectorView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<ResourceQualifier>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<ResourceQualifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ResourceQualifier>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<ResourceQualifier>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for ResourceQualifierVectorView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for ResourceQualifierVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<ResourceQualifier>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ResourceQualifierVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for ResourceQualifierVectorView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl windows_core::TypeKind for ResourceCandidateKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceCandidateKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
impl windows_core::TypeKind for ResourceQualifierPersistence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ResourceQualifierPersistence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ResourceQualifierPersistence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ResourceQualifierPersistence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl windows_core::TypeKind for ResourceLayoutInfo {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ResourceLayoutInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo;u4;u4;u4;u4;i4)");
}
impl Default for ResourceLayoutInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

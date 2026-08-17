windows_core::imp::define_interface!(IContentRestrictionsBrowsePolicy, IContentRestrictionsBrowsePolicy_Vtbl, 0x8c0133a4_442e_461a_8757_fad2f5bd37e4);
impl windows_core::RuntimeType for IContentRestrictionsBrowsePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentRestrictionsBrowsePolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MaxBrowsableAgeRating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreferredAgeRating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRatedContentDescription, IRatedContentDescription_Vtbl, 0x694866df_66b2_4dc3_96b1_f090eedee255);
impl windows_core::RuntimeType for IRatedContentDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRatedContentDescription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RatedContentCategory) -> windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, RatedContentCategory) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Ratings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ratings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRatings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRatings: usize,
}
windows_core::imp::define_interface!(IRatedContentDescriptionFactory, IRatedContentDescriptionFactory_Vtbl, 0x2e38df62_9b90_4fa6_89c1_4b8d2ffb3573);
impl windows_core::RuntimeType for IRatedContentDescriptionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRatedContentDescriptionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, RatedContentCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRatedContentRestrictions, IRatedContentRestrictions_Vtbl, 0x3f7f23cb_ba07_4401_a49d_8b9222205723);
impl windows_core::RuntimeType for IRatedContentRestrictions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRatedContentRestrictions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetBrowsePolicyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictionLevelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestContentAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestrictionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRestrictionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRatedContentRestrictionsFactory, IRatedContentRestrictionsFactory_Vtbl, 0xfb4b2996_c3bd_4910_9619_97cfd0694d56);
impl windows_core::RuntimeType for IRatedContentRestrictionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRatedContentRestrictionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithMaxAgeRating: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ContentRestrictionsBrowsePolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContentRestrictionsBrowsePolicy, windows_core::IUnknown, windows_core::IInspectable);
impl ContentRestrictionsBrowsePolicy {
    pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxBrowsableAgeRating(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBrowsableAgeRating)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreferredAgeRating(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredAgeRating)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContentRestrictionsBrowsePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContentRestrictionsBrowsePolicy>();
}
unsafe impl windows_core::Interface for ContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
    const IID: windows_core::GUID = <IContentRestrictionsBrowsePolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContentRestrictionsBrowsePolicy {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy";
}
unsafe impl Send for ContentRestrictionsBrowsePolicy {}
unsafe impl Sync for ContentRestrictionsBrowsePolicy {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RatedContentDescription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RatedContentDescription, windows_core::IUnknown, windows_core::IInspectable);
impl RatedContentDescription {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Image)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Category(&self) -> windows_core::Result<RatedContentCategory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCategory(&self, value: RatedContentCategory) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCategory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ratings(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ratings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRatings<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRatings)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create(id: &windows_core::HSTRING, title: &windows_core::HSTRING, category: RatedContentCategory) -> windows_core::Result<RatedContentDescription> {
        Self::IRatedContentDescriptionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), core::mem::transmute_copy(title), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRatedContentDescriptionFactory<R, F: FnOnce(&IRatedContentDescriptionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RatedContentDescription, IRatedContentDescriptionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RatedContentDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRatedContentDescription>();
}
unsafe impl windows_core::Interface for RatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
    const IID: windows_core::GUID = <IRatedContentDescription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RatedContentDescription {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentDescription";
}
unsafe impl Send for RatedContentDescription {}
unsafe impl Sync for RatedContentDescription {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RatedContentRestrictions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RatedContentRestrictions, windows_core::IUnknown, windows_core::IInspectable);
impl RatedContentRestrictions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RatedContentRestrictions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetBrowsePolicyAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBrowsePolicyAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRestrictionLevelAsync<P0>(&self, ratedcontentdescription: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>
    where
        P0: windows_core::Param<RatedContentDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRestrictionLevelAsync)(windows_core::Interface::as_raw(this), ratedcontentdescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestContentAccessAsync<P0>(&self, ratedcontentdescription: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<RatedContentDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestContentAccessAsync)(windows_core::Interface::as_raw(this), ratedcontentdescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RestrictionsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RestrictionsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRestrictionsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRestrictionsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateWithMaxAgeRating(maxagerating: u32) -> windows_core::Result<RatedContentRestrictions> {
        Self::IRatedContentRestrictionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMaxAgeRating)(windows_core::Interface::as_raw(this), maxagerating, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRatedContentRestrictionsFactory<R, F: FnOnce(&IRatedContentRestrictionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RatedContentRestrictions, IRatedContentRestrictionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RatedContentRestrictions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRatedContentRestrictions>();
}
unsafe impl windows_core::Interface for RatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
    const IID: windows_core::GUID = <IRatedContentRestrictions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RatedContentRestrictions {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentRestrictions";
}
unsafe impl Send for RatedContentRestrictions {}
unsafe impl Sync for RatedContentRestrictions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ContentAccessRestrictionLevel(pub i32);
impl ContentAccessRestrictionLevel {
    pub const Allow: Self = Self(0i32);
    pub const Warn: Self = Self(1i32);
    pub const Block: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
}
impl windows_core::TypeKind for ContentAccessRestrictionLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ContentAccessRestrictionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ContentAccessRestrictionLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ContentAccessRestrictionLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RatedContentCategory(pub i32);
impl RatedContentCategory {
    pub const General: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Game: Self = Self(2i32);
    pub const Movie: Self = Self(3i32);
    pub const Television: Self = Self(4i32);
    pub const Music: Self = Self(5i32);
}
impl windows_core::TypeKind for RatedContentCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RatedContentCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RatedContentCategory").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RatedContentCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.RatedContentCategory;i4)");
}

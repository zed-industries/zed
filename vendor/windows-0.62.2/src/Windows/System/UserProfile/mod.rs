#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AccountPictureKind(pub i32);
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl windows_core::TypeKind for AccountPictureKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AccountPictureKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.AccountPictureKind;i4)");
}
pub struct AdvertisingManager;
impl AdvertisingManager {
    pub fn AdvertisingId() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAdvertisingManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisingId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<AdvertisingManagerForUser>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IAdvertisingManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAdvertisingManagerStatics<R, F: FnOnce(&IAdvertisingManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAdvertisingManagerStatics2<R, F: FnOnce(&IAdvertisingManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AdvertisingManager {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManager";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvertisingManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdvertisingManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl AdvertisingManagerForUser {
    pub fn AdvertisingId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisingId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdvertisingManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdvertisingManagerForUser>();
}
unsafe impl windows_core::Interface for AdvertisingManagerForUser {
    type Vtable = <IAdvertisingManagerForUser as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAdvertisingManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManagerForUser";
}
unsafe impl Send for AdvertisingManagerForUser {}
unsafe impl Sync for AdvertisingManagerForUser {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssignedAccessSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AssignedAccessSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AssignedAccessSettings {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSingleAppKioskMode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSingleAppKioskMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<AssignedAccessSettings>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAssignedAccessSettingsStatics<R, F: FnOnce(&IAssignedAccessSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AssignedAccessSettings, IAssignedAccessSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AssignedAccessSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAssignedAccessSettings>();
}
unsafe impl windows_core::Interface for AssignedAccessSettings {
    type Vtable = <IAssignedAccessSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAssignedAccessSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.AssignedAccessSettings";
}
unsafe impl Send for AssignedAccessSettings {}
unsafe impl Sync for AssignedAccessSettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticsSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DiagnosticsSettings, windows_core::IUnknown, windows_core::IInspectable);
impl DiagnosticsSettings {
    pub fn CanUseDiagnosticsToTailorExperiences(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanUseDiagnosticsToTailorExperiences)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<DiagnosticsSettings>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDiagnosticsSettingsStatics<R, F: FnOnce(&IDiagnosticsSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DiagnosticsSettings, IDiagnosticsSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DiagnosticsSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDiagnosticsSettings>();
}
unsafe impl windows_core::Interface for DiagnosticsSettings {
    type Vtable = <IDiagnosticsSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDiagnosticsSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.DiagnosticsSettings";
}
unsafe impl Send for DiagnosticsSettings {}
unsafe impl Sync for DiagnosticsSettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstSignInSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FirstSignInSettings, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( FirstSignInSettings , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > > , windows_collections:: IMapView < windows_core::HSTRING , windows_core::IInspectable > );
impl FirstSignInSettings {
    pub fn GetDefault() -> windows_core::Result<FirstSignInSettings> {
        Self::IFirstSignInSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn Split(&self, first: &mut Option<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>, second: &mut Option<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    fn IFirstSignInSettingsStatics<R, F: FnOnce(&IFirstSignInSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FirstSignInSettings, IFirstSignInSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FirstSignInSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFirstSignInSettings>();
}
unsafe impl windows_core::Interface for FirstSignInSettings {
    type Vtable = <IFirstSignInSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFirstSignInSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.FirstSignInSettings";
}
unsafe impl Send for FirstSignInSettings {}
unsafe impl Sync for FirstSignInSettings {}
impl IntoIterator for FirstSignInSettings {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &FirstSignInSettings {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
pub struct GlobalizationPreferences;
impl GlobalizationPreferences {
    pub fn Calendars() -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Calendars)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Clocks() -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clocks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Currencies() -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Currencies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Languages() -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HomeGeographicRegion() -> windows_core::Result<windows_core::HSTRING> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HomeGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn WeekStartsOn() -> windows_core::Result<super::super::Globalization::DayOfWeek> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WeekStartsOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TrySetHomeGeographicRegion(region: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetHomeGeographicRegion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(region), &mut result__).map(|| result__)
        })
    }
    pub fn TrySetLanguages<P0>(languagetags: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetLanguages)(windows_core::Interface::as_raw(this), languagetags.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<GlobalizationPreferencesForUser>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IGlobalizationPreferencesStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGlobalizationPreferencesStatics<R, F: FnOnce(&IGlobalizationPreferencesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGlobalizationPreferencesStatics2<R, F: FnOnce(&IGlobalizationPreferencesStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGlobalizationPreferencesStatics3<R, F: FnOnce(&IGlobalizationPreferencesStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GlobalizationPreferences {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferences";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalizationPreferencesForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalizationPreferencesForUser, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalizationPreferencesForUser {
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Calendars(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Calendars)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Clocks(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clocks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Currencies(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Currencies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HomeGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HomeGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn WeekStartsOn(&self) -> windows_core::Result<super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WeekStartsOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GlobalizationPreferencesForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalizationPreferencesForUser>();
}
unsafe impl windows_core::Interface for GlobalizationPreferencesForUser {
    type Vtable = <IGlobalizationPreferencesForUser as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalizationPreferencesForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferencesForUser";
}
unsafe impl Send for GlobalizationPreferencesForUser {}
unsafe impl Sync for GlobalizationPreferencesForUser {}
windows_core::imp::define_interface!(IAdvertisingManagerForUser, IAdvertisingManagerForUser_Vtbl, 0x928bf3d0_cf7c_4ab0_a7dc_6dc5bcd44252);
impl windows_core::RuntimeType for IAdvertisingManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AdvertisingId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvertisingManagerStatics, IAdvertisingManagerStatics_Vtbl, 0xadd3468c_a273_48cb_b346_3544522d5581);
impl windows_core::RuntimeType for IAdvertisingManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AdvertisingId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvertisingManagerStatics2, IAdvertisingManagerStatics2_Vtbl, 0xdd0947af_1a6d_46b0_95bc_f3f9d6beb9fb);
impl windows_core::RuntimeType for IAdvertisingManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAssignedAccessSettings, IAssignedAccessSettings_Vtbl, 0x1bc57f1c_e971_5757_b8e0_512f8b8c46d2);
impl windows_core::RuntimeType for IAssignedAccessSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSingleAppKioskMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAssignedAccessSettingsStatics, IAssignedAccessSettingsStatics_Vtbl, 0x34a81d0d_8a29_5ef3_a7be_618e6ac3bd01);
impl windows_core::RuntimeType for IAssignedAccessSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiagnosticsSettings, IDiagnosticsSettings_Vtbl, 0xe5e9eccd_2711_44e0_973c_491d78048d24);
impl windows_core::RuntimeType for IDiagnosticsSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanUseDiagnosticsToTailorExperiences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiagnosticsSettingsStatics, IDiagnosticsSettingsStatics_Vtbl, 0x72d2e80f_5390_4793_990b_3ccc7d6ac9c8);
impl windows_core::RuntimeType for IDiagnosticsSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFirstSignInSettings, IFirstSignInSettings_Vtbl, 0x3e945153_3a5e_452e_a601_f5baad2a4870);
impl windows_core::RuntimeType for IFirstSignInSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IFirstSignInSettingsStatics, IFirstSignInSettingsStatics_Vtbl, 0x1ce18f0f_1c41_4ea0_b7a2_6f0c1c7e8438);
impl windows_core::RuntimeType for IFirstSignInSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalizationPreferencesForUser, IGlobalizationPreferencesForUser_Vtbl, 0x150f0795_4f6e_40ba_a010_e27d81bda7f5);
impl windows_core::RuntimeType for IGlobalizationPreferencesForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Calendars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Currencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HomeGeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Globalization::DayOfWeek) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
windows_core::imp::define_interface!(IGlobalizationPreferencesStatics, IGlobalizationPreferencesStatics_Vtbl, 0x01bf4326_ed37_4e96_b0e9_c1340d1ea158);
impl windows_core::RuntimeType for IGlobalizationPreferencesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Calendars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Currencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HomeGeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Globalization::DayOfWeek) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
windows_core::imp::define_interface!(IGlobalizationPreferencesStatics2, IGlobalizationPreferencesStatics2_Vtbl, 0xfcce85f1_4300_4cd0_9cac_1a8e7b7e18f4);
impl windows_core::RuntimeType for IGlobalizationPreferencesStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TrySetHomeGeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalizationPreferencesStatics3, IGlobalizationPreferencesStatics3_Vtbl, 0x1e059733_35f5_40d8_b9e8_aef3ef856fce);
impl windows_core::RuntimeType for IGlobalizationPreferencesStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenImageFeedStatics, ILockScreenImageFeedStatics_Vtbl, 0x2c0d73f6_03a9_41a6_9b01_495251ff51d5);
impl windows_core::RuntimeType for ILockScreenImageFeedStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenImageFeedStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestSetImageFeedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryRemoveImageFeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenStatics, ILockScreenStatics_Vtbl, 0x3ee9d3ad_b607_40ae_b426_7631d9821269);
impl windows_core::RuntimeType for ILockScreenStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OriginalImageFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetImageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetImageStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImageFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImageStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImageStreamAsync: usize,
}
windows_core::imp::define_interface!(IUserInformationStatics, IUserInformationStatics_Vtbl, 0x77f3a910_48fa_489c_934e_2ae85ba8f772);
impl windows_core::RuntimeType for IUserInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountPictureChangeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NameAccessAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAccountPicture: unsafe extern "system" fn(*mut core::ffi::c_void, AccountPictureKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAccountPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAccountPictureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAccountPictureAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAccountPicturesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAccountPicturesAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAccountPictureFromStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAccountPictureFromStreamAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAccountPicturesFromStreamsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAccountPicturesFromStreamsAsync: usize,
    pub AccountPictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAccountPictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetDisplayNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrincipalNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSessionInitiationProtocolUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDomainNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserProfilePersonalizationSettings, IUserProfilePersonalizationSettings_Vtbl, 0x8ceddab4_7998_46d5_8dd3_184f1c5f9ab9);
impl windows_core::RuntimeType for IUserProfilePersonalizationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub TrySetLockScreenImageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TrySetLockScreenImageAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TrySetWallpaperImageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TrySetWallpaperImageAsync: usize,
}
windows_core::imp::define_interface!(IUserProfilePersonalizationSettingsStatics, IUserProfilePersonalizationSettingsStatics_Vtbl, 0x91acb841_5037_454b_9883_bb772d08dd16);
impl windows_core::RuntimeType for IUserProfilePersonalizationSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct LockScreen;
impl LockScreen {
    pub fn RequestSetImageFeedAsync<P0>(syndicationfeeduri: P0) -> windows_core::Result<windows_future::IAsyncOperation<SetImageFeedResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSetImageFeedAsync)(windows_core::Interface::as_raw(this), syndicationfeeduri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryRemoveImageFeed() -> windows_core::Result<bool> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRemoveImageFeed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OriginalImageFile() -> windows_core::Result<super::super::Foundation::Uri> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalImageFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetImageStream() -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetImageStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImageFileAsync<P0>(value: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetImageFileAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImageStreamAsync<P0>(value: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetImageStreamAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ILockScreenImageFeedStatics<R, F: FnOnce(&ILockScreenImageFeedStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LockScreen, ILockScreenImageFeedStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ILockScreenStatics<R, F: FnOnce(&ILockScreenStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LockScreen, ILockScreenStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for LockScreen {
    const NAME: &'static str = "Windows.System.UserProfile.LockScreen";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SetAccountPictureResult(pub i32);
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
impl windows_core::TypeKind for SetAccountPictureResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SetAccountPictureResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetAccountPictureResult;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl windows_core::TypeKind for SetImageFeedResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SetImageFeedResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetImageFeedResult;i4)");
}
pub struct UserInformation;
impl UserInformation {
    pub fn AccountPictureChangeEnabled() -> windows_core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPictureChangeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn NameAccessAllowed() -> windows_core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NameAccessAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAccountPicture(kind: AccountPictureKind) -> windows_core::Result<super::super::Storage::IStorageFile> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccountPicture)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAccountPictureAsync<P0>(image: P0) -> windows_core::Result<windows_future::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAccountPictureAsync)(windows_core::Interface::as_raw(this), image.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAccountPicturesAsync<P0, P1, P2>(smallimage: P0, largeimage: P1, video: P2) -> windows_core::Result<windows_future::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
        P2: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAccountPicturesAsync)(windows_core::Interface::as_raw(this), smallimage.param().abi(), largeimage.param().abi(), video.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAccountPictureFromStreamAsync<P0>(image: P0) -> windows_core::Result<windows_future::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAccountPictureFromStreamAsync)(windows_core::Interface::as_raw(this), image.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAccountPicturesFromStreamsAsync<P0, P1, P2>(smallimage: P0, largeimage: P1, video: P2) -> windows_core::Result<windows_future::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P2: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAccountPicturesFromStreamsAsync)(windows_core::Interface::as_raw(this), smallimage.param().abi(), largeimage.param().abi(), video.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AccountPictureChanged<P0>(changehandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPictureChanged)(windows_core::Interface::as_raw(this), changehandler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveAccountPictureChanged(token: i64) -> windows_core::Result<()> {
        Self::IUserInformationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveAccountPictureChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn GetDisplayNameAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDisplayNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetFirstNameAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFirstNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetLastNameAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLastNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetPrincipalNameAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPrincipalNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSessionInitiationProtocolUriAsync() -> windows_core::Result<windows_future::IAsyncOperation<super::super::Foundation::Uri>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSessionInitiationProtocolUriAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDomainNameAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDomainNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUserInformationStatics<R, F: FnOnce(&IUserInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserInformation, IUserInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for UserInformation {
    const NAME: &'static str = "Windows.System.UserProfile.UserInformation";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserProfilePersonalizationSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserProfilePersonalizationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl UserProfilePersonalizationSettings {
    #[cfg(feature = "Storage_Streams")]
    pub fn TrySetLockScreenImageAsync<P0>(&self, imagefile: P0) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetLockScreenImageAsync)(windows_core::Interface::as_raw(this), imagefile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TrySetWallpaperImageAsync<P0>(&self, imagefile: P0) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetWallpaperImageAsync)(windows_core::Interface::as_raw(this), imagefile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<UserProfilePersonalizationSettings> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IUserProfilePersonalizationSettingsStatics<R, F: FnOnce(&IUserProfilePersonalizationSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserProfilePersonalizationSettings, IUserProfilePersonalizationSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserProfilePersonalizationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserProfilePersonalizationSettings>();
}
unsafe impl windows_core::Interface for UserProfilePersonalizationSettings {
    type Vtable = <IUserProfilePersonalizationSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUserProfilePersonalizationSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.UserProfilePersonalizationSettings";
}
unsafe impl Send for UserProfilePersonalizationSettings {}
unsafe impl Sync for UserProfilePersonalizationSettings {}

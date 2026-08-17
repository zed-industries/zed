windows_core::imp::define_interface!(IGeolocationProvider, IGeolocationProvider_Vtbl, 0xe4cf071d_3f64_509f_8dc2_0b74a059829d);
impl windows_core::RuntimeType for IGeolocationProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeolocationProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOverridden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOverridePosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::BasicGeoposition, super::PositionSource, f64, *mut LocationOverrideStatus) -> windows_core::HRESULT,
    pub ClearOverridePosition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOverriddenChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsOverriddenChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GeolocationProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeolocationProvider, windows_core::IUnknown, windows_core::IInspectable);
impl GeolocationProvider {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeolocationProvider, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsOverridden(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverridden)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOverridePosition(&self, newposition: super::BasicGeoposition, positionsource: super::PositionSource, accuracyinmeters: f64) -> windows_core::Result<LocationOverrideStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetOverridePosition)(windows_core::Interface::as_raw(this), newposition, positionsource, accuracyinmeters, &mut result__).map(|| result__)
        }
    }
    pub fn ClearOverridePosition(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearOverridePosition)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsOverriddenChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverriddenChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsOverriddenChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsOverriddenChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for GeolocationProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeolocationProvider>();
}
unsafe impl windows_core::Interface for GeolocationProvider {
    type Vtable = IGeolocationProvider_Vtbl;
    const IID: windows_core::GUID = <IGeolocationProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeolocationProvider {
    const NAME: &'static str = "Windows.Devices.Geolocation.Provider.GeolocationProvider";
}
unsafe impl Send for GeolocationProvider {}
unsafe impl Sync for GeolocationProvider {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LocationOverrideStatus(pub i32);
impl LocationOverrideStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const AlreadyStarted: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl windows_core::TypeKind for LocationOverrideStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LocationOverrideStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LocationOverrideStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LocationOverrideStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.Provider.LocationOverrideStatus;i4)");
}

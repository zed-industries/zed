windows_core::imp::define_interface!(IDeviceLockdownProfileInformation, IDeviceLockdownProfileInformation_Vtbl, 0x7980e14e_45b1_4a96_92fc_62756b739678);
impl windows_core::RuntimeType for IDeviceLockdownProfileInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceLockdownProfileInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceLockdownProfileStatics, IDeviceLockdownProfileStatics_Vtbl, 0x622f6965_f9a8_41a1_a691_88cd80c7a069);
impl windows_core::RuntimeType for IDeviceLockdownProfileStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceLockdownProfileStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockdownProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockdownProfiles: usize,
    pub GetCurrentLockdownProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ApplyLockdownProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLockdownProfileInformation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct DeviceLockdownProfile;
impl DeviceLockdownProfile {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockdownProfiles() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::GUID>> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedLockdownProfiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentLockdownProfile() -> windows_core::Result<windows_core::GUID> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentLockdownProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ApplyLockdownProfileAsync(profileid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyLockdownProfileAsync)(windows_core::Interface::as_raw(this), profileid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetLockdownProfileInformation(profileid: windows_core::GUID) -> windows_core::Result<DeviceLockdownProfileInformation> {
        Self::IDeviceLockdownProfileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLockdownProfileInformation)(windows_core::Interface::as_raw(this), profileid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeviceLockdownProfileStatics<R, F: FnOnce(&IDeviceLockdownProfileStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceLockdownProfile, IDeviceLockdownProfileStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for DeviceLockdownProfile {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfile";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceLockdownProfileInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceLockdownProfileInformation, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceLockdownProfileInformation {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceLockdownProfileInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceLockdownProfileInformation>();
}
unsafe impl windows_core::Interface for DeviceLockdownProfileInformation {
    type Vtable = IDeviceLockdownProfileInformation_Vtbl;
    const IID: windows_core::GUID = <IDeviceLockdownProfileInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceLockdownProfileInformation {
    const NAME: &'static str = "Windows.Embedded.DeviceLockdown.DeviceLockdownProfileInformation";
}
unsafe impl Send for DeviceLockdownProfileInformation {}
unsafe impl Sync for DeviceLockdownProfileInformation {}

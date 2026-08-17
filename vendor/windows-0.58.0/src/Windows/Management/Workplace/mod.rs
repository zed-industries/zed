windows_core::imp::define_interface!(IMdmAllowPolicyStatics, IMdmAllowPolicyStatics_Vtbl, 0xc39709e7_741c_41f2_a4b6_314c31502586);
impl windows_core::RuntimeType for IMdmAllowPolicyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMdmAllowPolicyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBrowserAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCameraAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMicrosoftAccountAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStoreAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMdmPolicyStatics2, IMdmPolicyStatics2_Vtbl, 0xc99c7526_03d4_49f9_a993_43efccd265c4);
impl windows_core::RuntimeType for IMdmPolicyStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMdmPolicyStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetMessagingSyncPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MessagingSyncPolicy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkplaceSettingsStatics, IWorkplaceSettingsStatics_Vtbl, 0xe4676ffd_2d92_4c08_bad4_f6590b54a6d3);
impl windows_core::RuntimeType for IWorkplaceSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWorkplaceSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsMicrosoftAccountOptional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct MdmPolicy;
impl MdmPolicy {
    pub fn IsBrowserAllowed() -> windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBrowserAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsCameraAllowed() -> windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCameraAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsMicrosoftAccountAllowed() -> windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMicrosoftAccountAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsStoreAllowed() -> windows_core::Result<bool> {
        Self::IMdmAllowPolicyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStoreAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetMessagingSyncPolicy() -> windows_core::Result<MessagingSyncPolicy> {
        Self::IMdmPolicyStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessagingSyncPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IMdmAllowPolicyStatics<R, F: FnOnce(&IMdmAllowPolicyStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MdmPolicy, IMdmAllowPolicyStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMdmPolicyStatics2<R, F: FnOnce(&IMdmPolicyStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MdmPolicy, IMdmPolicyStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for MdmPolicy {
    const NAME: &'static str = "Windows.Management.Workplace.MdmPolicy";
}
pub struct WorkplaceSettings;
impl WorkplaceSettings {
    pub fn IsMicrosoftAccountOptional() -> windows_core::Result<bool> {
        Self::IWorkplaceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMicrosoftAccountOptional)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IWorkplaceSettingsStatics<R, F: FnOnce(&IWorkplaceSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WorkplaceSettings, IWorkplaceSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WorkplaceSettings {
    const NAME: &'static str = "Windows.Management.Workplace.WorkplaceSettings";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MessagingSyncPolicy(pub i32);
impl MessagingSyncPolicy {
    pub const Disallowed: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl windows_core::TypeKind for MessagingSyncPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MessagingSyncPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MessagingSyncPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MessagingSyncPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Workplace.MessagingSyncPolicy;i4)");
}

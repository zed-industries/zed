windows_core::imp::define_interface!(IPowerManagerStatics, IPowerManagerStatics_Vtbl, 0x25de8fd0_1c5b_11e1_bddb_0800200c9a66);
impl windows_core::RuntimeType for IPowerManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerSavingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PowerSavingMode) -> windows_core::HRESULT,
    pub PowerSavingModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePowerSavingModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerManagerStatics2, IPowerManagerStatics2_Vtbl, 0x596236cf_1918_4551_a466_c51aae373bf8);
impl windows_core::RuntimeType for IPowerManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerSavingModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct PowerManager;
impl PowerManager {
    pub fn PowerSavingMode() -> windows_core::Result<PowerSavingMode> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSavingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PowerSavingModeChanged<P0>(changehandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSavingModeChanged)(windows_core::Interface::as_raw(this), changehandler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemovePowerSavingModeChanged(token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemovePowerSavingModeChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn PowerSavingModeEnabled() -> windows_core::Result<bool> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSavingModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics<R, F: FnOnce(&IPowerManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics2<R, F: FnOnce(&IPowerManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.Phone.System.Power.PowerManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl windows_core::TypeKind for PowerSavingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PowerSavingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PowerSavingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PowerSavingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.Power.PowerSavingMode;i4)");
}

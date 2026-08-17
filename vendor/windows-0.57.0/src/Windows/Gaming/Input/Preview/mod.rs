windows_core::imp::define_interface!(IGameControllerProviderInfoStatics, IGameControllerProviderInfoStatics_Vtbl, 0x0be1e6c5_d9bd_44ee_8362_488b2e464bfb);
impl windows_core::RuntimeType for IGameControllerProviderInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameControllerProviderInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetParentProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetParentProviderId: usize,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetProviderId: usize,
}
pub struct GameControllerProviderInfo;
impl GameControllerProviderInfo {
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetParentProviderId<P0>(provider: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::Custom::IGameControllerProvider>,
    {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParentProviderId)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetProviderId<P0>(provider: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::Custom::IGameControllerProvider>,
    {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProviderId)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}

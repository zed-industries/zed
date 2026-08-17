windows_core::imp::define_interface!(ISoundLevelBrokerStatics, ISoundLevelBrokerStatics_Vtbl, 0x6a633961_dbed_464c_a09a_33412f5caa3f);
impl windows_core::RuntimeType for ISoundLevelBrokerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISoundLevelBrokerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::SoundLevel) -> windows_core::HRESULT,
    pub SoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
pub struct SoundLevelBroker;
impl SoundLevelBroker {
    pub fn SoundLevel() -> windows_core::Result<super::super::SoundLevel> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SoundLevelChanged<P0>(handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveSoundLevelChanged(token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ISoundLevelBrokerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn ISoundLevelBrokerStatics<R, F: FnOnce(&ISoundLevelBrokerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SoundLevelBroker, ISoundLevelBrokerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}

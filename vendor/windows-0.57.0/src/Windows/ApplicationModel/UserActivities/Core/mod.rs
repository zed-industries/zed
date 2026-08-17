windows_core::imp::define_interface!(ICoreUserActivityManagerStatics, ICoreUserActivityManagerStatics_Vtbl, 0xca3adb02_a4be_4d4d_bfa8_6795f4264efb);
impl windows_core::RuntimeType for ICoreUserActivityManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreUserActivityManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::DateTime, super::super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct CoreUserActivityManager;
impl CoreUserActivityManager {
    pub fn CreateUserActivitySessionInBackground<P0>(activity: P0) -> windows_core::Result<super::UserActivitySession>
    where
        P0: windows_core::Param<super::UserActivity>,
    {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUserActivitySessionInBackground)(windows_core::Interface::as_raw(this), activity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DeleteUserActivitySessionsInTimeRangeAsync<P0>(channel: P0, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::UserActivityChannel>,
    {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteUserActivitySessionsInTimeRangeAsync)(windows_core::Interface::as_raw(this), channel.param().abi(), starttime, endtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreUserActivityManagerStatics<R, F: FnOnce(&ICoreUserActivityManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreUserActivityManager, ICoreUserActivityManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}

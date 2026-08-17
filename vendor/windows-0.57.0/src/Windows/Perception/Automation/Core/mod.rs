windows_core::imp::define_interface!(ICorePerceptionAutomationStatics, ICorePerceptionAutomationStatics_Vtbl, 0x0bb04541_4ce2_4923_9a76_8187ecc59112);
impl windows_core::RuntimeType for ICorePerceptionAutomationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICorePerceptionAutomationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetActivationFactoryProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct CorePerceptionAutomation;
impl CorePerceptionAutomation {
    pub fn SetActivationFactoryProvider<P0>(provider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IGetActivationFactory>,
    {
        Self::ICorePerceptionAutomationStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetActivationFactoryProvider)(windows_core::Interface::as_raw(this), provider.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICorePerceptionAutomationStatics<R, F: FnOnce(&ICorePerceptionAutomationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CorePerceptionAutomation, ICorePerceptionAutomationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CorePerceptionAutomation {
    const NAME: &'static str = "Windows.Perception.Automation.Core.CorePerceptionAutomation";
}

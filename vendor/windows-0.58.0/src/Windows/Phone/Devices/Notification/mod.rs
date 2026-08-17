windows_core::imp::define_interface!(IVibrationDevice, IVibrationDevice_Vtbl, 0x1b4a6595_cfcd_4e08_92fb_c1906d04498c);
impl windows_core::RuntimeType for IVibrationDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVibrationDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Vibrate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVibrationDeviceStatics, IVibrationDeviceStatics_Vtbl, 0x332fd2f1_1c69_4c91_949e_4bb67a85bdc7);
impl windows_core::RuntimeType for IVibrationDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVibrationDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VibrationDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VibrationDevice, windows_core::IUnknown, windows_core::IInspectable);
impl VibrationDevice {
    pub fn Vibrate(&self, duration: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Vibrate)(windows_core::Interface::as_raw(this), duration).ok() }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<VibrationDevice> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VibrationDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVibrationDevice>();
}
unsafe impl windows_core::Interface for VibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
    const IID: windows_core::GUID = <IVibrationDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Phone.Devices.Notification.VibrationDevice";
}
unsafe impl Send for VibrationDevice {}
unsafe impl Sync for VibrationDevice {}

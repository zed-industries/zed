pub trait IDeviceEnumerationSettings_Impl: Sized {}
impl windows_core::RuntimeName for IDeviceEnumerationSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceEnumerationSettings";
}
impl IDeviceEnumerationSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeviceEnumerationSettings_Vtbl
    where
        Identity: IDeviceEnumerationSettings_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceEnumerationSettings, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceEnumerationSettings as windows_core::Interface>::IID
    }
}
pub trait IDevicePairingSettings_Impl: Sized {}
impl windows_core::RuntimeName for IDevicePairingSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingSettings";
}
impl IDevicePairingSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDevicePairingSettings_Vtbl
    where
        Identity: IDevicePairingSettings_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDevicePairingSettings, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDevicePairingSettings as windows_core::Interface>::IID
    }
}

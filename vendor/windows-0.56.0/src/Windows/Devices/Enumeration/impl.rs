pub trait IDevicePairingSettings_Impl: Sized {}
impl windows_core::RuntimeName for IDevicePairingSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingSettings";
}
impl IDevicePairingSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDevicePairingSettings_Impl, const OFFSET: isize>() -> IDevicePairingSettings_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDevicePairingSettings, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDevicePairingSettings as windows_core::Interface>::IID
    }
}

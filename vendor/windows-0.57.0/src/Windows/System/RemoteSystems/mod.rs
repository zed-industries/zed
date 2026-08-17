windows_core::imp::define_interface!(IKnownRemoteSystemCapabilitiesStatics, IKnownRemoteSystemCapabilitiesStatics_Vtbl, 0x8108e380_7f8a_44e4_92cd_03b6469b94a3);
impl windows_core::RuntimeType for IKnownRemoteSystemCapabilitiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownRemoteSystemCapabilitiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LaunchUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoteSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SpatialEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystem, IRemoteSystem_Vtbl, 0xed5838cd_1e10_4a8c_b4a6_4e5fd6f97721);
impl windows_core::RuntimeType for IRemoteSystem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemStatus) -> windows_core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystem2, IRemoteSystem2_Vtbl, 0x09dfe4ec_fb8b_4a08_a758_6876435d769e);
impl windows_core::RuntimeType for IRemoteSystem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetCapabilitySupportedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystem3, IRemoteSystem3_Vtbl, 0x72b4b495_b7c6_40be_831b_73562f12ffa8);
impl windows_core::RuntimeType for IRemoteSystem3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystem3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ManufacturerDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ModelDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystem4, IRemoteSystem4_Vtbl, 0xf164ffe5_b987_4ca5_9926_fa0438be6273);
impl windows_core::RuntimeType for IRemoteSystem4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystem4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Platform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemPlatform) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystem5, IRemoteSystem5_Vtbl, 0xeb2ad723_e5e2_4ae2_a7a7_a1097a098e90);
impl windows_core::RuntimeType for IRemoteSystem5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystem5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Apps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Apps: usize,
}
windows_core::imp::define_interface!(IRemoteSystem6, IRemoteSystem6_Vtbl, 0xd4cda942_c027_533e_9384_3a19b4f7eef3);
impl windows_core::RuntimeType for IRemoteSystem6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystem6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemAddedEventArgs, IRemoteSystemAddedEventArgs_Vtbl, 0x8f39560f_e534_4697_8836_7abea151516e);
impl windows_core::RuntimeType for IRemoteSystemAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemApp, IRemoteSystemApp_Vtbl, 0x80e5bcbd_d54d_41b1_9b16_6810a871ed4f);
impl windows_core::RuntimeType for IRemoteSystemApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemApp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attributes: usize,
}
windows_core::imp::define_interface!(IRemoteSystemApp2, IRemoteSystemApp2_Vtbl, 0x6369bf15_0a96_577a_8ff6_c35904dfa8f3);
impl windows_core::RuntimeType for IRemoteSystemApp2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemApp2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemAppRegistration, IRemoteSystemAppRegistration_Vtbl, 0xb47947b5_7035_4a5a_b8df_962d8f8431f4);
impl windows_core::RuntimeType for IRemoteSystemAppRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemAppRegistration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attributes: usize,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemAppRegistrationStatics, IRemoteSystemAppRegistrationStatics_Vtbl, 0x01b99840_cfd2_453f_ae25_c2539f086afd);
impl windows_core::RuntimeType for IRemoteSystemAppRegistrationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemAppRegistrationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemAuthorizationKindFilter, IRemoteSystemAuthorizationKindFilter_Vtbl, 0x6b0dde8e_04d0_40f4_a27f_c2acbbd6b734);
impl windows_core::RuntimeType for IRemoteSystemAuthorizationKindFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemAuthorizationKindFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystemAuthorizationKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemAuthorizationKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemAuthorizationKindFilterFactory, IRemoteSystemAuthorizationKindFilterFactory_Vtbl, 0xad65df4d_b66a_45a4_8177_8caed75d9e5a);
impl windows_core::RuntimeType for IRemoteSystemAuthorizationKindFilterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemAuthorizationKindFilterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, RemoteSystemAuthorizationKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionInfo, IRemoteSystemConnectionInfo_Vtbl, 0x23278bc3_0d09_52cb_9c6a_eed2940bee43);
impl windows_core::RuntimeType for IRemoteSystemConnectionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsProximal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionInfoStatics, IRemoteSystemConnectionInfoStatics_Vtbl, 0xac831e2d_66c5_56d7_a4ce_705d94925ad6);
impl windows_core::RuntimeType for IRemoteSystemConnectionInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub TryCreateFromAppServiceConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    TryCreateFromAppServiceConnection: usize,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionRequest, IRemoteSystemConnectionRequest_Vtbl, 0x84ed4104_8d5e_4d72_8238_7621576c7a67);
impl windows_core::RuntimeType for IRemoteSystemConnectionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionRequest2, IRemoteSystemConnectionRequest2_Vtbl, 0x12df6d6f_bffc_483a_8abe_d34a6c19f92b);
impl windows_core::RuntimeType for IRemoteSystemConnectionRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystemApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionRequest3, IRemoteSystemConnectionRequest3_Vtbl, 0xde86c3e7_c9cc_5a50_b8d9_ba7b34bb8d0e);
impl windows_core::RuntimeType for IRemoteSystemConnectionRequest3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionRequestFactory, IRemoteSystemConnectionRequestFactory_Vtbl, 0xaa0a0a20_baeb_4575_b530_810bb9786334);
impl windows_core::RuntimeType for IRemoteSystemConnectionRequestFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionRequestStatics, IRemoteSystemConnectionRequestStatics_Vtbl, 0x86ca143d_8214_425c_8932_db49032d1306);
impl windows_core::RuntimeType for IRemoteSystemConnectionRequestStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemConnectionRequestStatics2, IRemoteSystemConnectionRequestStatics2_Vtbl, 0x460f1027_64ec_598e_a800_4f2ee58def19);
impl windows_core::RuntimeType for IRemoteSystemConnectionRequestStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromConnectionToken: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromConnectionTokenForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemDiscoveryTypeFilter, IRemoteSystemDiscoveryTypeFilter_Vtbl, 0x42d9041f_ee5a_43da_ac6a_6fee25460741);
impl windows_core::RuntimeType for IRemoteSystemDiscoveryTypeFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemDiscoveryTypeFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystemDiscoveryType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemDiscoveryType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemDiscoveryTypeFilterFactory, IRemoteSystemDiscoveryTypeFilterFactory_Vtbl, 0x9f9eb993_c260_4161_92f2_9c021f23fe5d);
impl windows_core::RuntimeType for IRemoteSystemDiscoveryTypeFilterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, RemoteSystemDiscoveryType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemEnumerationCompletedEventArgs, IRemoteSystemEnumerationCompletedEventArgs_Vtbl, 0xc6e83d5f_4030_4354_a060_14f1b22c545d);
impl windows_core::RuntimeType for IRemoteSystemEnumerationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemEnumerationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRemoteSystemFilter, IRemoteSystemFilter_Vtbl, 0x4a3ba9e4_99eb_45eb_ba16_0367728ff374);
impl core::ops::Deref for IRemoteSystemFilter {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRemoteSystemFilter, windows_core::IUnknown, windows_core::IInspectable);
impl IRemoteSystemFilter {}
impl windows_core::RuntimeType for IRemoteSystemFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRemoteSystemKindFilter, IRemoteSystemKindFilter_Vtbl, 0x38e1c9ec_22c3_4ef6_901a_bbb1c7aad4ed);
impl windows_core::RuntimeType for IRemoteSystemKindFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemKindFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteSystemKinds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteSystemKinds: usize,
}
windows_core::imp::define_interface!(IRemoteSystemKindFilterFactory, IRemoteSystemKindFilterFactory_Vtbl, 0xa1fb18ee_99ea_40bc_9a39_c670aa804a28);
impl windows_core::RuntimeType for IRemoteSystemKindFilterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemKindFilterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
windows_core::imp::define_interface!(IRemoteSystemKindStatics, IRemoteSystemKindStatics_Vtbl, 0xf6317633_ab14_41d0_9553_796aadb882db);
impl windows_core::RuntimeType for IRemoteSystemKindStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemKindStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Phone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Hub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Holographic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Desktop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Xbox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemKindStatics2, IRemoteSystemKindStatics2_Vtbl, 0xb9e3a3d0_0466_4749_91e8_65f9d19a96a5);
impl windows_core::RuntimeType for IRemoteSystemKindStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemKindStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Iot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tablet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Laptop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemRemovedEventArgs, IRemoteSystemRemovedEventArgs_Vtbl, 0x8b3d16bb_7306_49ea_b7df_67d5714cb013);
impl windows_core::RuntimeType for IRemoteSystemRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSession, IRemoteSystemSession_Vtbl, 0x69476a01_9ada_490f_9549_d31cb14c9e95);
impl windows_core::RuntimeType for IRemoteSystemSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateParticipantWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendInvitationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionAddedEventArgs, IRemoteSystemSessionAddedEventArgs_Vtbl, 0xd585d754_bc97_4c39_99b4_beca76e04c3f);
impl windows_core::RuntimeType for IRemoteSystemSessionAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionController, IRemoteSystemSessionController_Vtbl, 0xe48b2dd2_6820_4867_b425_d89c0a3ef7ba);
impl windows_core::RuntimeType for IRemoteSystemSessionController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub JoinRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveJoinRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveParticipantAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSessionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionControllerFactory, IRemoteSystemSessionControllerFactory_Vtbl, 0xbfcc2f6b_ac3d_4199_82cd_6670a773ef2e);
impl windows_core::RuntimeType for IRemoteSystemSessionControllerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionControllerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateController: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateControllerWithSessionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionCreationResult, IRemoteSystemSessionCreationResult_Vtbl, 0xa79812c2_37de_448c_8b83_a30aa3c4ead6);
impl windows_core::RuntimeType for IRemoteSystemSessionCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionCreationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemSessionCreationStatus) -> windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionDisconnectedEventArgs, IRemoteSystemSessionDisconnectedEventArgs_Vtbl, 0xde0bc69b_77c5_461c_8209_7c6c5d3111ab);
impl windows_core::RuntimeType for IRemoteSystemSessionDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionDisconnectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemSessionDisconnectedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionInfo, IRemoteSystemSessionInfo_Vtbl, 0xff4df648_8b0a_4e9a_9905_69e4b841c588);
impl windows_core::RuntimeType for IRemoteSystemSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub JoinAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionInvitation, IRemoteSystemSessionInvitation_Vtbl, 0x3e32cc91_51d7_4766_a121_25516c3b8294);
impl windows_core::RuntimeType for IRemoteSystemSessionInvitation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Sender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionInvitationListener, IRemoteSystemSessionInvitationListener_Vtbl, 0x08f4003f_bc71_49e1_874a_31ddff9a27b9);
impl windows_core::RuntimeType for IRemoteSystemSessionInvitationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitationListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvitationReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInvitationReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionInvitationReceivedEventArgs, IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl, 0x5e964a2d_a10d_4edb_8dea_54d20ac19543);
impl windows_core::RuntimeType for IRemoteSystemSessionInvitationReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Invitation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionJoinRequest, IRemoteSystemSessionJoinRequest_Vtbl, 0x20600068_7994_4331_86d1_d89d882585ee);
impl windows_core::RuntimeType for IRemoteSystemSessionJoinRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionJoinRequestedEventArgs, IRemoteSystemSessionJoinRequestedEventArgs_Vtbl, 0xdbca4fc3_82b9_4816_9c24_e40e61774bd8);
impl windows_core::RuntimeType for IRemoteSystemSessionJoinRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub JoinRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionJoinResult, IRemoteSystemSessionJoinResult_Vtbl, 0xce7b1f04_a03e_41a4_900b_1e79328c1267);
impl windows_core::RuntimeType for IRemoteSystemSessionJoinResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemSessionJoinStatus) -> windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionMessageChannel, IRemoteSystemSessionMessageChannel_Vtbl, 0x9524d12a_73d9_4c10_b751_c26784437127);
impl windows_core::RuntimeType for IRemoteSystemSessionMessageChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionMessageChannel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastValueSetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastValueSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendValueSetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendValueSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendValueSetToParticipantsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendValueSetToParticipantsAsync: usize,
    pub ValueSetReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveValueSetReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionMessageChannelFactory, IRemoteSystemSessionMessageChannelFactory_Vtbl, 0x295e1c4a_bd16_4298_b7ce_415482b0e11d);
impl windows_core::RuntimeType for IRemoteSystemSessionMessageChannelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionMessageChannelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithReliability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, RemoteSystemSessionMessageChannelReliability, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionOptions, IRemoteSystemSessionOptions_Vtbl, 0x740ed755_8418_4f01_9353_e21c9ecc6cfc);
impl windows_core::RuntimeType for IRemoteSystemSessionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsInviteOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsInviteOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionParticipant, IRemoteSystemSessionParticipant_Vtbl, 0x7e90058c_acf9_4729_8a17_44e7baed5dcc);
impl windows_core::RuntimeType for IRemoteSystemSessionParticipant {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipant_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetHostNames: usize,
}
windows_core::imp::define_interface!(IRemoteSystemSessionParticipantAddedEventArgs, IRemoteSystemSessionParticipantAddedEventArgs_Vtbl, 0xd35a57d8_c9a1_4bb7_b6b0_79bb91adf93d);
impl windows_core::RuntimeType for IRemoteSystemSessionParticipantAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionParticipantRemovedEventArgs, IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl, 0x866ef088_de68_4abf_88a1_f90d16274192);
impl windows_core::RuntimeType for IRemoteSystemSessionParticipantRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionParticipantWatcher, IRemoteSystemSessionParticipantWatcher_Vtbl, 0xdcdd02cc_aa87_4d79_b6cc_4459b3e92075);
impl windows_core::RuntimeType for IRemoteSystemSessionParticipantWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemSessionParticipantWatcherStatus) -> windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionRemovedEventArgs, IRemoteSystemSessionRemovedEventArgs_Vtbl, 0xaf82914e_39a1_4dea_9d63_43798d5bbbd0);
impl windows_core::RuntimeType for IRemoteSystemSessionRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionStatics, IRemoteSystemSessionStatics_Vtbl, 0x8524899f_fd20_44e3_9565_e75a3b14c66e);
impl windows_core::RuntimeType for IRemoteSystemSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionUpdatedEventArgs, IRemoteSystemSessionUpdatedEventArgs_Vtbl, 0x16875069_231e_4c91_8ec8_b3a39d9e55a3);
impl windows_core::RuntimeType for IRemoteSystemSessionUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemSessionValueSetReceivedEventArgs, IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl, 0x06f31785_2da5_4e58_a78f_9e8d0784ee25);
impl windows_core::RuntimeType for IRemoteSystemSessionValueSetReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Sender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
}
windows_core::imp::define_interface!(IRemoteSystemSessionWatcher, IRemoteSystemSessionWatcher_Vtbl, 0x8003e340_0c41_4a62_b6d7_bdbe2b19be2d);
impl windows_core::RuntimeType for IRemoteSystemSessionWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemSessionWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemSessionWatcherStatus) -> windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemStatics, IRemoteSystemStatics_Vtbl, 0xa485b392_ff2b_4b47_be62_743f2f140f30);
impl windows_core::RuntimeType for IRemoteSystemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking")]
    pub FindByHostNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    FindByHostNameAsync: usize,
    pub CreateWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithFilters: usize,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemStatics2, IRemoteSystemStatics2_Vtbl, 0x0c98edca_6f99_4c52_a272_ea4f36471744);
impl windows_core::RuntimeType for IRemoteSystemStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAuthorizationKindEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, RemoteSystemAuthorizationKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemStatics3, IRemoteSystemStatics3_Vtbl, 0x9995f16f_0b3c_5ac5_b325_cc73f437dfcd);
impl windows_core::RuntimeType for IRemoteSystemStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWatcherForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithFiltersForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithFiltersForUser: usize,
}
windows_core::imp::define_interface!(IRemoteSystemStatusTypeFilter, IRemoteSystemStatusTypeFilter_Vtbl, 0x0c39514e_cbb6_4777_8534_2e0c521affa2);
impl windows_core::RuntimeType for IRemoteSystemStatusTypeFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemStatusTypeFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystemStatusType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemStatusType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemStatusTypeFilterFactory, IRemoteSystemStatusTypeFilterFactory_Vtbl, 0x33cf78fa_d724_4125_ac7a_8d281e44c949);
impl windows_core::RuntimeType for IRemoteSystemStatusTypeFilterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemStatusTypeFilterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, RemoteSystemStatusType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemUpdatedEventArgs, IRemoteSystemUpdatedEventArgs_Vtbl, 0x7502ff0e_dbcb_4155_b4ca_b30a04f27627);
impl windows_core::RuntimeType for IRemoteSystemUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemWatcher, IRemoteSystemWatcher_Vtbl, 0x5d600c7e_2c07_48c5_889c_455d2b099771);
impl windows_core::RuntimeType for IRemoteSystemWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteSystemAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoteSystemAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoteSystemUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoteSystemUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoteSystemRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoteSystemRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemWatcher2, IRemoteSystemWatcher2_Vtbl, 0x73436700_19ca_48f9_a4cd_780f7ad58c71);
impl windows_core::RuntimeType for IRemoteSystemWatcher2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemWatcher2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemWatcher3, IRemoteSystemWatcher3_Vtbl, 0xf79c0fcf_a913_55d3_8413_418fcf15ba54);
impl windows_core::RuntimeType for IRemoteSystemWatcher3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemWatcher3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemWatcherErrorOccurredEventArgs, IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl, 0x74c5c6af_5114_4426_9216_20d81f8519ae);
impl windows_core::RuntimeType for IRemoteSystemWatcherErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteSystemWatcherError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemWebAccountFilter, IRemoteSystemWebAccountFilter_Vtbl, 0x3fb75873_87c8_5d8f_977e_f69f96d67238);
impl windows_core::RuntimeType for IRemoteSystemWebAccountFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemWebAccountFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
windows_core::imp::define_interface!(IRemoteSystemWebAccountFilterFactory, IRemoteSystemWebAccountFilterFactory_Vtbl, 0x348a2709_5f4d_5127_b4a7_bf99d5252b1b);
impl windows_core::RuntimeType for IRemoteSystemWebAccountFilterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteSystemWebAccountFilterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
}
pub struct KnownRemoteSystemCapabilities;
impl KnownRemoteSystemCapabilities {
    pub fn AppService() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppService)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LaunchUri() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RemoteSession() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SpatialEntity() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialEntity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownRemoteSystemCapabilitiesStatics<R, F: FnOnce(&IKnownRemoteSystemCapabilitiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownRemoteSystemCapabilities, IKnownRemoteSystemCapabilitiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownRemoteSystemCapabilities {
    const NAME: &'static str = "Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystem, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystem {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<RemoteSystemStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAvailableByProximity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailableByProximity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAvailableBySpatialProximity(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailableBySpatialProximity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCapabilitySupportedAsync(&self, capabilityname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilitySupportedAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(capabilityname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ManufacturerDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManufacturerDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ModelDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Platform(&self) -> windows_core::Result<RemoteSystemPlatform> {
        let this = &windows_core::Interface::cast::<IRemoteSystem4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Platform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Apps(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>> {
        let this = &windows_core::Interface::cast::<IRemoteSystem5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Apps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = &windows_core::Interface::cast::<IRemoteSystem6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking")]
    pub fn FindByHostNameAsync<P0>(hostname: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>>
    where
        P0: windows_core::Param<super::super::Networking::HostName>,
    {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindByHostNameAsync)(windows_core::Interface::as_raw(this), hostname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcher() -> windows_core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithFilters<P0>(filters: P0) -> windows_core::Result<RemoteSystemWatcher>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>,
    {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherWithFilters)(windows_core::Interface::as_raw(this), filters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsAuthorizationKindEnabled(kind: RemoteSystemAuthorizationKind) -> windows_core::Result<bool> {
        Self::IRemoteSystemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAuthorizationKindEnabled)(windows_core::Interface::as_raw(this), kind, &mut result__).map(|| result__)
        })
    }
    pub fn CreateWatcherForUser<P0>(user: P0) -> windows_core::Result<RemoteSystemWatcher>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithFiltersForUser<P0, P1>(user: P0, filters: P1) -> windows_core::Result<RemoteSystemWatcher>
    where
        P0: windows_core::Param<super::User>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>,
    {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherWithFiltersForUser)(windows_core::Interface::as_raw(this), user.param().abi(), filters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatics<R, F: FnOnce(&IRemoteSystemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystem, IRemoteSystemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatics2<R, F: FnOnce(&IRemoteSystemStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystem, IRemoteSystemStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatics3<R, F: FnOnce(&IRemoteSystemStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystem, IRemoteSystemStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystem>();
}
unsafe impl windows_core::Interface for RemoteSystem {
    type Vtable = IRemoteSystem_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystem";
}
unsafe impl Send for RemoteSystem {}
unsafe impl Sync for RemoteSystem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemAddedEventArgs {
    pub fn RemoteSystem(&self) -> windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemAddedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
}
unsafe impl Send for RemoteSystemAddedEventArgs {}
unsafe impl Sync for RemoteSystemAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemApp(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemApp, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemApp {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsAvailableByProximity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailableByProximity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAvailableBySpatialProximity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailableBySpatialProximity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attributes(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = &windows_core::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionToken(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemApp>();
}
unsafe impl windows_core::Interface for RemoteSystemApp {
    type Vtable = IRemoteSystemApp_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemApp as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemApp";
}
unsafe impl Send for RemoteSystemApp {}
unsafe impl Sync for RemoteSystemApp {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemAppRegistration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemAppRegistration, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemAppRegistration {
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attributes(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<RemoteSystemAppRegistration>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemAppRegistrationStatics<R, F: FnOnce(&IRemoteSystemAppRegistrationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemAppRegistration, IRemoteSystemAppRegistrationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemAppRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemAppRegistration>();
}
unsafe impl windows_core::Interface for RemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemAppRegistration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAppRegistration";
}
unsafe impl Send for RemoteSystemAppRegistration {}
unsafe impl Sync for RemoteSystemAppRegistration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemAuthorizationKindFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemAuthorizationKindFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteSystemAuthorizationKindFilter, IRemoteSystemFilter);
impl RemoteSystemAuthorizationKindFilter {
    pub fn RemoteSystemAuthorizationKind(&self) -> windows_core::Result<RemoteSystemAuthorizationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemAuthorizationKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> windows_core::Result<RemoteSystemAuthorizationKindFilter> {
        Self::IRemoteSystemAuthorizationKindFilterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), remotesystemauthorizationkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemAuthorizationKindFilterFactory<R, F: FnOnce(&IRemoteSystemAuthorizationKindFilterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemAuthorizationKindFilter, IRemoteSystemAuthorizationKindFilterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemAuthorizationKindFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemAuthorizationKindFilter>();
}
unsafe impl windows_core::Interface for RemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemAuthorizationKindFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
}
unsafe impl Send for RemoteSystemAuthorizationKindFilter {}
unsafe impl Sync for RemoteSystemAuthorizationKindFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemConnectionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemConnectionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemConnectionInfo {
    pub fn IsProximal(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsProximal)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn TryCreateFromAppServiceConnection<P0>(connection: P0) -> windows_core::Result<RemoteSystemConnectionInfo>
    where
        P0: windows_core::Param<super::super::ApplicationModel::AppService::AppServiceConnection>,
    {
        Self::IRemoteSystemConnectionInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFromAppServiceConnection)(windows_core::Interface::as_raw(this), connection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionInfoStatics<R, F: FnOnce(&IRemoteSystemConnectionInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemConnectionInfo, IRemoteSystemConnectionInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemConnectionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemConnectionInfo>();
}
unsafe impl windows_core::Interface for RemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemConnectionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
}
unsafe impl Send for RemoteSystemConnectionInfo {}
unsafe impl Sync for RemoteSystemConnectionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemConnectionRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemConnectionRequest, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemConnectionRequest {
    pub fn RemoteSystem(&self) -> windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteSystemApp(&self) -> windows_core::Result<RemoteSystemApp> {
        let this = &windows_core::Interface::cast::<IRemoteSystemConnectionRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemApp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionToken(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRemoteSystemConnectionRequest3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(remotesystem: P0) -> windows_core::Result<RemoteSystemConnectionRequest>
    where
        P0: windows_core::Param<RemoteSystem>,
    {
        Self::IRemoteSystemConnectionRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), remotesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateForApp<P0>(remotesystemapp: P0) -> windows_core::Result<RemoteSystemConnectionRequest>
    where
        P0: windows_core::Param<RemoteSystemApp>,
    {
        Self::IRemoteSystemConnectionRequestStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForApp)(windows_core::Interface::as_raw(this), remotesystemapp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromConnectionToken(connectiontoken: &windows_core::HSTRING) -> windows_core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromConnectionToken)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(connectiontoken), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromConnectionTokenForUser<P0>(user: P0, connectiontoken: &windows_core::HSTRING) -> windows_core::Result<RemoteSystemConnectionRequest>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromConnectionTokenForUser)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(connectiontoken), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionRequestFactory<R, F: FnOnce(&IRemoteSystemConnectionRequestFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionRequestStatics<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionRequestStatics2<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemConnectionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemConnectionRequest>();
}
unsafe impl windows_core::Interface for RemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemConnectionRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
}
unsafe impl Send for RemoteSystemConnectionRequest {}
unsafe impl Sync for RemoteSystemConnectionRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemDiscoveryTypeFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemDiscoveryTypeFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteSystemDiscoveryTypeFilter, IRemoteSystemFilter);
impl RemoteSystemDiscoveryTypeFilter {
    pub fn RemoteSystemDiscoveryType(&self) -> windows_core::Result<RemoteSystemDiscoveryType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemDiscoveryType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(discoverytype: RemoteSystemDiscoveryType) -> windows_core::Result<RemoteSystemDiscoveryTypeFilter> {
        Self::IRemoteSystemDiscoveryTypeFilterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), discoverytype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemDiscoveryTypeFilterFactory<R, F: FnOnce(&IRemoteSystemDiscoveryTypeFilterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemDiscoveryTypeFilter, IRemoteSystemDiscoveryTypeFilterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemDiscoveryTypeFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemDiscoveryTypeFilter>();
}
unsafe impl windows_core::Interface for RemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemDiscoveryTypeFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
}
unsafe impl Send for RemoteSystemDiscoveryTypeFilter {}
unsafe impl Sync for RemoteSystemDiscoveryTypeFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemEnumerationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemEnumerationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemEnumerationCompletedEventArgs {}
impl windows_core::RuntimeType for RemoteSystemEnumerationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemEnumerationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemEnumerationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemEnumerationCompletedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
}
unsafe impl Send for RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl Sync for RemoteSystemEnumerationCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemKindFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemKindFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteSystemKindFilter, IRemoteSystemFilter);
impl RemoteSystemKindFilter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoteSystemKinds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemKinds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(remotesystemkinds: P0) -> windows_core::Result<RemoteSystemKindFilter>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IRemoteSystemKindFilterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), remotesystemkinds.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemKindFilterFactory<R, F: FnOnce(&IRemoteSystemKindFilterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemKindFilter, IRemoteSystemKindFilterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemKindFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemKindFilter>();
}
unsafe impl windows_core::Interface for RemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemKindFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKindFilter";
}
unsafe impl Send for RemoteSystemKindFilter {}
unsafe impl Sync for RemoteSystemKindFilter {}
pub struct RemoteSystemKinds;
impl RemoteSystemKinds {
    pub fn Phone() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Phone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Hub() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hub)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Holographic() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Holographic)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Desktop() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Desktop)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Xbox() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Xbox)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Iot() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Iot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Tablet() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tablet)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Laptop() -> windows_core::Result<windows_core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Laptop)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemKindStatics<R, F: FnOnce(&IRemoteSystemKindStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemKindStatics2<R, F: FnOnce(&IRemoteSystemKindStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for RemoteSystemKinds {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKinds";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemRemovedEventArgs {
    pub fn RemoteSystemId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemRemovedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
}
unsafe impl Send for RemoteSystemRemovedEventArgs {}
unsafe impl Sync for RemoteSystemRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteSystemSession, super::super::Foundation::IClosable);
impl RemoteSystemSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ControllerDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControllerDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Disconnected<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Disconnected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateParticipantWatcher(&self) -> windows_core::Result<RemoteSystemSessionParticipantWatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateParticipantWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendInvitationAsync<P0>(&self, invitee: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<RemoteSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendInvitationAsync)(windows_core::Interface::as_raw(this), invitee.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWatcher() -> windows_core::Result<RemoteSystemSessionWatcher> {
        Self::IRemoteSystemSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemSessionStatics<R, F: FnOnce(&IRemoteSystemSessionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemSession, IRemoteSystemSessionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSession>();
}
unsafe impl windows_core::Interface for RemoteSystemSession {
    type Vtable = IRemoteSystemSession_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSession";
}
unsafe impl Send for RemoteSystemSession {}
unsafe impl Sync for RemoteSystemSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionAddedEventArgs {
    pub fn SessionInfo(&self) -> windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionAddedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
}
unsafe impl Send for RemoteSystemSessionAddedEventArgs {}
unsafe impl Sync for RemoteSystemSessionAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionController, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionController {
    pub fn JoinRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JoinRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveJoinRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveJoinRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RemoveParticipantAsync<P0>(&self, pparticipant: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<RemoteSystemSessionParticipant>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveParticipantAsync)(windows_core::Interface::as_raw(this), pparticipant.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateSessionAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSessionAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateController(displayname: &windows_core::HSTRING) -> windows_core::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateControllerWithSessionOptions<P0>(displayname: &windows_core::HSTRING, options: P0) -> windows_core::Result<RemoteSystemSessionController>
    where
        P0: windows_core::Param<RemoteSystemSessionOptions>,
    {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateControllerWithSessionOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displayname), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemSessionControllerFactory<R, F: FnOnce(&IRemoteSystemSessionControllerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemSessionController, IRemoteSystemSessionControllerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionController>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionController";
}
unsafe impl Send for RemoteSystemSessionController {}
unsafe impl Sync for RemoteSystemSessionController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionCreationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionCreationResult, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionCreationResult {
    pub fn Status(&self) -> windows_core::Result<RemoteSystemSessionCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Session(&self) -> windows_core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionCreationResult>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionCreationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
}
unsafe impl Send for RemoteSystemSessionCreationResult {}
unsafe impl Sync for RemoteSystemSessionCreationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionDisconnectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionDisconnectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionDisconnectedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<RemoteSystemSessionDisconnectedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionDisconnectedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionDisconnectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
}
unsafe impl Send for RemoteSystemSessionDisconnectedEventArgs {}
unsafe impl Sync for RemoteSystemSessionDisconnectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ControllerDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControllerDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JoinAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JoinAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionInfo>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInfo";
}
unsafe impl Send for RemoteSystemSessionInfo {}
unsafe impl Sync for RemoteSystemSessionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionInvitation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionInvitation, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionInvitation {
    pub fn Sender(&self) -> windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sender)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionInfo(&self) -> windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionInvitation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionInvitation>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionInvitation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
}
unsafe impl Send for RemoteSystemSessionInvitation {}
unsafe impl Sync for RemoteSystemSessionInvitation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionInvitationListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionInvitationListener, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionInvitationListener {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemSessionInvitationListener, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn InvitationReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvitationReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInvitationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInvitationReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionInvitationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionInvitationListener>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionInvitationListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
}
unsafe impl Send for RemoteSystemSessionInvitationListener {}
unsafe impl Sync for RemoteSystemSessionInvitationListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionInvitationReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionInvitationReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionInvitationReceivedEventArgs {
    pub fn Invitation(&self) -> windows_core::Result<RemoteSystemSessionInvitation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invitation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionInvitationReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionInvitationReceivedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionInvitationReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
}
unsafe impl Send for RemoteSystemSessionInvitationReceivedEventArgs {}
unsafe impl Sync for RemoteSystemSessionInvitationReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionJoinRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionJoinRequest, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionJoinRequest {
    pub fn Participant(&self) -> windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Participant)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Accept(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Accept)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionJoinRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionJoinRequest>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionJoinRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
}
unsafe impl Send for RemoteSystemSessionJoinRequest {}
unsafe impl Sync for RemoteSystemSessionJoinRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionJoinRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionJoinRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionJoinRequestedEventArgs {
    pub fn JoinRequest(&self) -> windows_core::Result<RemoteSystemSessionJoinRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JoinRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionJoinRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionJoinRequestedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionJoinRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
}
unsafe impl Send for RemoteSystemSessionJoinRequestedEventArgs {}
unsafe impl Sync for RemoteSystemSessionJoinRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionJoinResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionJoinResult, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionJoinResult {
    pub fn Status(&self) -> windows_core::Result<RemoteSystemSessionJoinStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Session(&self) -> windows_core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionJoinResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionJoinResult>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionJoinResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
}
unsafe impl Send for RemoteSystemSessionJoinResult {}
unsafe impl Sync for RemoteSystemSessionJoinResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionMessageChannel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionMessageChannel, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionMessageChannel {
    pub fn Session(&self) -> windows_core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Session)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BroadcastValueSetAsync<P0>(&self, messagedata: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastValueSetAsync)(windows_core::Interface::as_raw(this), messagedata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendValueSetAsync<P0, P1>(&self, messagedata: P0, participant: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
        P1: windows_core::Param<RemoteSystemSessionParticipant>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendValueSetAsync)(windows_core::Interface::as_raw(this), messagedata.param().abi(), participant.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendValueSetToParticipantsAsync<P0, P1>(&self, messagedata: P0, participants: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendValueSetToParticipantsAsync)(windows_core::Interface::as_raw(this), messagedata.param().abi(), participants.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValueSetReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueSetReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveValueSetReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveValueSetReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Create<P0>(session: P0, channelname: &windows_core::HSTRING) -> windows_core::Result<RemoteSystemSessionMessageChannel>
    where
        P0: windows_core::Param<RemoteSystemSession>,
    {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), session.param().abi(), core::mem::transmute_copy(channelname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithReliability<P0>(session: P0, channelname: &windows_core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability) -> windows_core::Result<RemoteSystemSessionMessageChannel>
    where
        P0: windows_core::Param<RemoteSystemSession>,
    {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithReliability)(windows_core::Interface::as_raw(this), session.param().abi(), core::mem::transmute_copy(channelname), reliability, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemSessionMessageChannelFactory<R, F: FnOnce(&IRemoteSystemSessionMessageChannelFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemSessionMessageChannel, IRemoteSystemSessionMessageChannelFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionMessageChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionMessageChannel>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionMessageChannel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
}
unsafe impl Send for RemoteSystemSessionMessageChannel {}
unsafe impl Sync for RemoteSystemSessionMessageChannel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionOptions, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemSessionOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsInviteOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInviteOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsInviteOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsInviteOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionOptions>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionOptions";
}
unsafe impl Send for RemoteSystemSessionOptions {}
unsafe impl Sync for RemoteSystemSessionOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionParticipant(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionParticipant, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionParticipant {
    pub fn RemoteSystem(&self) -> windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetHostNames(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHostNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionParticipant {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionParticipant>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionParticipant as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
}
unsafe impl Send for RemoteSystemSessionParticipant {}
unsafe impl Sync for RemoteSystemSessionParticipant {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionParticipantAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionParticipantAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionParticipantAddedEventArgs {
    pub fn Participant(&self) -> windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Participant)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionParticipantAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionParticipantAddedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionParticipantAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
}
unsafe impl Send for RemoteSystemSessionParticipantAddedEventArgs {}
unsafe impl Sync for RemoteSystemSessionParticipantAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionParticipantRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionParticipantRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionParticipantRemovedEventArgs {
    pub fn Participant(&self) -> windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Participant)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionParticipantRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionParticipantRemovedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionParticipantRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
}
unsafe impl Send for RemoteSystemSessionParticipantRemovedEventArgs {}
unsafe impl Sync for RemoteSystemSessionParticipantRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionParticipantWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionParticipantWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionParticipantWatcher {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<RemoteSystemSessionParticipantWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Added)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionParticipantWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionParticipantWatcher>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionParticipantWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
}
unsafe impl Send for RemoteSystemSessionParticipantWatcher {}
unsafe impl Sync for RemoteSystemSessionParticipantWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionRemovedEventArgs {
    pub fn SessionInfo(&self) -> windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionRemovedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
}
unsafe impl Send for RemoteSystemSessionRemovedEventArgs {}
unsafe impl Sync for RemoteSystemSessionRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionUpdatedEventArgs {
    pub fn SessionInfo(&self) -> windows_core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
}
unsafe impl Send for RemoteSystemSessionUpdatedEventArgs {}
unsafe impl Sync for RemoteSystemSessionUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionValueSetReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionValueSetReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionValueSetReceivedEventArgs {
    pub fn Sender(&self) -> windows_core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sender)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionValueSetReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionValueSetReceivedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionValueSetReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
}
unsafe impl Send for RemoteSystemSessionValueSetReceivedEventArgs {}
unsafe impl Sync for RemoteSystemSessionValueSetReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemSessionWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemSessionWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemSessionWatcher {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<RemoteSystemSessionWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Added)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemSessionWatcher>();
}
unsafe impl windows_core::Interface for RemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemSessionWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
}
unsafe impl Send for RemoteSystemSessionWatcher {}
unsafe impl Sync for RemoteSystemSessionWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemStatusTypeFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemStatusTypeFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteSystemStatusTypeFilter, IRemoteSystemFilter);
impl RemoteSystemStatusTypeFilter {
    pub fn RemoteSystemStatusType(&self) -> windows_core::Result<RemoteSystemStatusType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemStatusType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(remotesystemstatustype: RemoteSystemStatusType) -> windows_core::Result<RemoteSystemStatusTypeFilter> {
        Self::IRemoteSystemStatusTypeFilterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), remotesystemstatustype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatusTypeFilterFactory<R, F: FnOnce(&IRemoteSystemStatusTypeFilterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemStatusTypeFilter, IRemoteSystemStatusTypeFilterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemStatusTypeFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemStatusTypeFilter>();
}
unsafe impl windows_core::Interface for RemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemStatusTypeFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
}
unsafe impl Send for RemoteSystemStatusTypeFilter {}
unsafe impl Sync for RemoteSystemStatusTypeFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemUpdatedEventArgs {
    pub fn RemoteSystem(&self) -> windows_core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
}
unsafe impl Send for RemoteSystemUpdatedEventArgs {}
unsafe impl Sync for RemoteSystemUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemWatcher {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoteSystemAdded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoteSystemAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoteSystemAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RemoteSystemUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoteSystemUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoteSystemUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RemoteSystemRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteSystemRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoteSystemRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoteSystemRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ErrorOccurred<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorOccurred)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveErrorOccurred)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn User(&self) -> windows_core::Result<super::User> {
        let this = &windows_core::Interface::cast::<IRemoteSystemWatcher3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemWatcher>();
}
unsafe impl windows_core::Interface for RemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcher";
}
unsafe impl Send for RemoteSystemWatcher {}
unsafe impl Sync for RemoteSystemWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemWatcherErrorOccurredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemWatcherErrorOccurredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteSystemWatcherErrorOccurredEventArgs {
    pub fn Error(&self) -> windows_core::Result<RemoteSystemWatcherError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RemoteSystemWatcherErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemWatcherErrorOccurredEventArgs>();
}
unsafe impl windows_core::Interface for RemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemWatcherErrorOccurredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
}
unsafe impl Send for RemoteSystemWatcherErrorOccurredEventArgs {}
unsafe impl Sync for RemoteSystemWatcherErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteSystemWebAccountFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteSystemWebAccountFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteSystemWebAccountFilter, IRemoteSystemFilter);
impl RemoteSystemWebAccountFilter {
    #[cfg(feature = "Security_Credentials")]
    pub fn Account(&self) -> windows_core::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Account)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn Create<P0>(account: P0) -> windows_core::Result<RemoteSystemWebAccountFilter>
    where
        P0: windows_core::Param<super::super::Security::Credentials::WebAccount>,
    {
        Self::IRemoteSystemWebAccountFilterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), account.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemWebAccountFilterFactory<R, F: FnOnce(&IRemoteSystemWebAccountFilterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteSystemWebAccountFilter, IRemoteSystemWebAccountFilterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteSystemWebAccountFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteSystemWebAccountFilter>();
}
unsafe impl windows_core::Interface for RemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_Vtbl;
    const IID: windows_core::GUID = <IRemoteSystemWebAccountFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
}
unsafe impl Send for RemoteSystemWebAccountFilter {}
unsafe impl Sync for RemoteSystemWebAccountFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemAccessStatus(pub i32);
impl RemoteSystemAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl windows_core::TypeKind for RemoteSystemAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemAuthorizationKind(pub i32);
impl RemoteSystemAuthorizationKind {
    pub const SameUser: Self = Self(0i32);
    pub const Anonymous: Self = Self(1i32);
}
impl windows_core::TypeKind for RemoteSystemAuthorizationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemAuthorizationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemAuthorizationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemAuthorizationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAuthorizationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemDiscoveryType(pub i32);
impl RemoteSystemDiscoveryType {
    pub const Any: Self = Self(0i32);
    pub const Proximal: Self = Self(1i32);
    pub const Cloud: Self = Self(2i32);
    pub const SpatiallyProximal: Self = Self(3i32);
}
impl windows_core::TypeKind for RemoteSystemDiscoveryType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemDiscoveryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemDiscoveryType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemDiscoveryType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemDiscoveryType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemPlatform(pub i32);
impl RemoteSystemPlatform {
    pub const Unknown: Self = Self(0i32);
    pub const Windows: Self = Self(1i32);
    pub const Android: Self = Self(2i32);
    pub const Ios: Self = Self(3i32);
    pub const Linux: Self = Self(4i32);
}
impl windows_core::TypeKind for RemoteSystemPlatform {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemPlatform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemPlatform").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemPlatform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemPlatform;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemSessionCreationStatus(pub i32);
impl RemoteSystemSessionCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
}
impl windows_core::TypeKind for RemoteSystemSessionCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemSessionCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemSessionDisconnectedReason(pub i32);
impl RemoteSystemSessionDisconnectedReason {
    pub const SessionUnavailable: Self = Self(0i32);
    pub const RemovedByController: Self = Self(1i32);
    pub const SessionClosed: Self = Self(2i32);
}
impl windows_core::TypeKind for RemoteSystemSessionDisconnectedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemSessionDisconnectedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionDisconnectedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionDisconnectedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemSessionJoinStatus(pub i32);
impl RemoteSystemSessionJoinStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
    pub const SessionUnavailable: Self = Self(3i32);
    pub const RejectedByController: Self = Self(4i32);
}
impl windows_core::TypeKind for RemoteSystemSessionJoinStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemSessionJoinStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionJoinStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemSessionMessageChannelReliability(pub i32);
impl RemoteSystemSessionMessageChannelReliability {
    pub const Reliable: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
}
impl windows_core::TypeKind for RemoteSystemSessionMessageChannelReliability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemSessionMessageChannelReliability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionMessageChannelReliability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionMessageChannelReliability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemSessionParticipantWatcherStatus(pub i32);
impl RemoteSystemSessionParticipantWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl windows_core::TypeKind for RemoteSystemSessionParticipantWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemSessionParticipantWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionParticipantWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemSessionWatcherStatus(pub i32);
impl RemoteSystemSessionWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl windows_core::TypeKind for RemoteSystemSessionWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemSessionWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemSessionWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemStatus(pub i32);
impl RemoteSystemStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const DiscoveringAvailability: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl windows_core::TypeKind for RemoteSystemStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemStatusType(pub i32);
impl RemoteSystemStatusType {
    pub const Any: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
}
impl windows_core::TypeKind for RemoteSystemStatusType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemStatusType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemStatusType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemStatusType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatusType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteSystemWatcherError(pub i32);
impl RemoteSystemWatcherError {
    pub const Unknown: Self = Self(0i32);
    pub const InternetNotAvailable: Self = Self(1i32);
    pub const AuthenticationError: Self = Self(2i32);
}
impl windows_core::TypeKind for RemoteSystemWatcherError {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteSystemWatcherError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcherError").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RemoteSystemWatcherError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemWatcherError;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

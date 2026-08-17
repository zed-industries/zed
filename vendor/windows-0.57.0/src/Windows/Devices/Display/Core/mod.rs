windows_core::imp::define_interface!(IDisplayAdapter, IDisplayAdapter_Vtbl, 0xa56f5287_f000_5f2e_b5ac_3783a2b69af5);
impl windows_core::RuntimeType for IDisplayAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayAdapter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DisplayAdapterId) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Id: usize,
    pub DeviceInterfacePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PciVendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PciDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PciSubSystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PciRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDisplayAdapterStatics, IDisplayAdapterStatics_Vtbl, 0x1dac3cda_481f_5469_8470_82c4ba680a28);
impl windows_core::RuntimeType for IDisplayAdapterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayAdapterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::DisplayAdapterId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    FromId: usize,
}
windows_core::imp::define_interface!(IDisplayDevice, IDisplayDevice_Vtbl, 0xa4c9b62c_335f_5731_8cb4_c1ccd4731070);
impl windows_core::RuntimeType for IDisplayDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateScanoutSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTaskPool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePeriodicFence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSimpleScanout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCapabilitySupported: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayDeviceCapability, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayDevice2, IDisplayDevice2_Vtbl, 0x3fefe50c_0940_54bd_a02f_f9c7a536ad60);
impl windows_core::RuntimeType for IDisplayDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))]
    pub CreateSimpleScanoutWithDirtyRectsAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, DisplayScanoutOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics")))]
    CreateSimpleScanoutWithDirtyRectsAndOptions: usize,
}
windows_core::imp::define_interface!(IDisplayFence, IDisplayFence_Vtbl, 0x04dcf9ef_3406_5700_8fec_77eba4c5a74b);
impl windows_core::RuntimeType for IDisplayFence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayFence_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDisplayManager, IDisplayManager_Vtbl, 0x4ed9245b_15ec_56e2_9072_7fe5084a31a7);
impl windows_core::RuntimeType for IDisplayManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentTargets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentAdapters: usize,
    pub TryAcquireTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut DisplayManagerResult) -> windows_core::HRESULT,
    pub ReleaseTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryReadCurrentStateForAllTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndReadCurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndReadCurrentState: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndCreateEmptyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndCreateEmptyState: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndCreateSubstate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndCreateSubstate: usize,
    pub CreateDisplayDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Disabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PathsFailedOrInvalidated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePathsFailedOrInvalidated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayManagerChangedEventArgs, IDisplayManagerChangedEventArgs_Vtbl, 0x6abfa285_6cca_5731_bcdc_42e5d2f5c50f);
impl windows_core::RuntimeType for IDisplayManagerChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManagerChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayManagerDisabledEventArgs, IDisplayManagerDisabledEventArgs_Vtbl, 0x8726dde4_6793_5973_a11f_5ffbc93fdb90);
impl windows_core::RuntimeType for IDisplayManagerDisabledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManagerDisabledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayManagerEnabledEventArgs, IDisplayManagerEnabledEventArgs_Vtbl, 0xf0cf3f6f_42fa_59a2_b297_26e1713de848);
impl windows_core::RuntimeType for IDisplayManagerEnabledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManagerEnabledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayManagerPathsFailedOrInvalidatedEventArgs, IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl, 0x03a65659_1dec_5c15_b2a2_8fe9129869fe);
impl windows_core::RuntimeType for IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayManagerResultWithState, IDisplayManagerResultWithState_Vtbl, 0x8e656aa6_6614_54be_bfef_4994547f7be1);
impl windows_core::RuntimeType for IDisplayManagerResultWithState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManagerResultWithState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayManagerResult) -> windows_core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayManagerStatics, IDisplayManagerStatics_Vtbl, 0x2b6b9446_b999_5535_9d69_53f092c780a1);
impl windows_core::RuntimeType for IDisplayManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayManagerOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayModeInfo, IDisplayModeInfo_Vtbl, 0x48d513a0_f79b_5a74_a05e_da821f470868);
impl windows_core::RuntimeType for IDisplayModeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayModeInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub SourceResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SourceResolution: usize,
    pub IsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub SourcePixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SourcePixelFormat: usize,
    #[cfg(feature = "Graphics")]
    pub TargetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    TargetResolution: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PresentationRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayPresentationRate) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PresentationRate: usize,
    pub IsInterlaced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetWireFormatSupportedBitsPerChannel: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayWireFormatPixelEncoding, *mut DisplayBitsPerChannel) -> windows_core::HRESULT,
    pub IsWireFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDisplayModeInfo2, IDisplayModeInfo2_Vtbl, 0xc86fa386_0ddb_5473_bfb0_4b7807b5f909);
impl windows_core::RuntimeType for IDisplayModeInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayModeInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PhysicalPresentationRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayPresentationRate) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PhysicalPresentationRate: usize,
}
windows_core::imp::define_interface!(IDisplayPath, IDisplayPath_Vtbl, 0xb3dfd64a_7460_5cde_811b_d5ae9f3d9f84);
impl windows_core::RuntimeType for IDisplayPath {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayPath_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub View: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayPathStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub SourceResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SourceResolution: usize,
    #[cfg(feature = "Graphics")]
    pub SetSourceResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetSourceResolution: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SourcePixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SourcePixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetSourcePixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetSourcePixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub TargetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    TargetResolution: usize,
    #[cfg(feature = "Graphics")]
    pub SetTargetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetTargetResolution: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PresentationRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PresentationRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPresentationRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPresentationRate: usize,
    pub IsInterlaced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsInterlaced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WireFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWireFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayRotation) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayRotation) -> windows_core::HRESULT,
    pub Scaling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayPathScaling) -> windows_core::HRESULT,
    pub SetScaling: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayPathScaling) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindModes: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayModeQueryOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindModes: usize,
    pub ApplyPropertiesFromMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDisplayPath2, IDisplayPath2_Vtbl, 0xf32459c5_e994_570b_9ec8_ef42c35a8547);
impl windows_core::RuntimeType for IDisplayPath2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayPath2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PhysicalPresentationRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PhysicalPresentationRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPhysicalPresentationRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPhysicalPresentationRate: usize,
}
windows_core::imp::define_interface!(IDisplayPrimaryDescription, IDisplayPrimaryDescription_Vtbl, 0x872591d2_d533_50ff_a85e_06696194b77c);
impl windows_core::RuntimeType for IDisplayPrimaryDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayPrimaryDescription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub ColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXColorSpace) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    ColorSpace: usize,
    pub IsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub MultisampleDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    MultisampleDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDisplayPrimaryDescriptionFactory, IDisplayPrimaryDescriptionFactory_Vtbl, 0x1a6aff7b_3637_5c46_b479_76d576216e65);
impl windows_core::RuntimeType for IDisplayPrimaryDescriptionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayPrimaryDescriptionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::super::Graphics::DirectX::DirectXPixelFormat, super::super::super::Graphics::DirectX::DirectXColorSpace, bool, super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IDisplayPrimaryDescriptionStatics, IDisplayPrimaryDescriptionStatics_Vtbl, 0xe60e4cfb_36c9_56dd_8fa1_6ff8c4e0ff07);
impl windows_core::RuntimeType for IDisplayPrimaryDescriptionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayPrimaryDescriptionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, super::super::super::Graphics::DirectX::DirectXPixelFormat, super::super::super::Graphics::DirectX::DirectXColorSpace, bool, super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    CreateWithProperties: usize,
}
windows_core::imp::define_interface!(IDisplayScanout, IDisplayScanout_Vtbl, 0xe3051828_1ba5_50e7_8a39_bb1fd2f4f8b9);
impl windows_core::RuntimeType for IDisplayScanout {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayScanout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDisplaySource, IDisplaySource_Vtbl, 0xecd15fc1_eadc_51bc_971d_3bc628db2dd4);
impl windows_core::RuntimeType for IDisplaySource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplaySource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DisplayAdapterId) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    pub SourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetMetadata: usize,
}
windows_core::imp::define_interface!(IDisplaySource2, IDisplaySource2_Vtbl, 0x71e18952_b321_5af4_bfe8_03fbea31e40d);
impl windows_core::RuntimeType for IDisplaySource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplaySource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplaySourceStatus) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayState, IDisplayState_Vtbl, 0x08129321_11b5_5cb2_99f8_e90b479a8a1d);
impl windows_core::RuntimeType for IDisplayState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Targets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Targets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub ConnectTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectTargetToView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanConnectTargetToView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetViewForTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPathForTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryFunctionalize: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayStateFunctionalizeOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryApply: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayStateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayStateOperationResult, IDisplayStateOperationResult_Vtbl, 0xfcadbfdf_dc27_5638_b7f2_ebdfa4f7ea93);
impl windows_core::RuntimeType for IDisplayStateOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayStateOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayStateOperationStatus) -> windows_core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplaySurface, IDisplaySurface_Vtbl, 0x594f6cc6_139a_56d6_a4b1_15fe2cb76adb);
impl windows_core::RuntimeType for IDisplaySurface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplaySurface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDisplayTarget, IDisplayTarget_Vtbl, 0xaec57c6f_47b4_546b_987c_e73fa791fe3a);
impl windows_core::RuntimeType for IDisplayTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Adapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceInterfacePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AdapterRelativeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVirtualModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVirtualTopologyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DisplayMonitorUsageKind) -> windows_core::HRESULT,
    pub MonitorPersistence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayTargetPersistence) -> windows_core::HRESULT,
    pub StableMonitorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TryGetMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub IsStale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayTask, IDisplayTask_Vtbl, 0x5e087448_135b_5bb0_bf63_637f84227c7a);
impl windows_core::RuntimeType for IDisplayTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayTask_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetScanout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayTask2, IDisplayTask2_Vtbl, 0x0957ea19_bd55_55de_9267_c97b61e71c37);
impl windows_core::RuntimeType for IDisplayTask2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayTask2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSignal: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayTaskSignalKind, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayTaskPool, IDisplayTaskPool_Vtbl, 0xc676253d_237d_5548_aafa_3e517fefef1c);
impl windows_core::RuntimeType for IDisplayTaskPool {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayTaskPool_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ExecuteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExecuteTask: usize,
}
windows_core::imp::define_interface!(IDisplayTaskPool2, IDisplayTaskPool2_Vtbl, 0x46b879b6_5d17_5955_a872_eb38003db586);
impl windows_core::RuntimeType for IDisplayTaskPool2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayTaskPool2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryExecuteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayTaskResult, IDisplayTaskResult_Vtbl, 0x6fbc7d67_f9b1_55e0_9d88_d3a5197a3f59);
impl windows_core::RuntimeType for IDisplayTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayTaskResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PresentStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayPresentStatus) -> windows_core::HRESULT,
    pub PresentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SourceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplaySourceStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayView, IDisplayView_Vtbl, 0xb0c98ca1_b759_5b59_b1ad_f0786aa9e53d);
impl windows_core::RuntimeType for IDisplayView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Paths: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Paths: usize,
    #[cfg(feature = "Graphics")]
    pub ContentResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    ContentResolution: usize,
    #[cfg(feature = "Graphics")]
    pub SetContentResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetContentResolution: usize,
    pub SetPrimaryPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDisplayWireFormat, IDisplayWireFormat_Vtbl, 0x1acc967d_872c_5a38_bbb9_1d4872b76255);
impl windows_core::RuntimeType for IDisplayWireFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayWireFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PixelEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayWireFormatPixelEncoding) -> windows_core::HRESULT,
    pub BitsPerChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayWireFormatColorSpace) -> windows_core::HRESULT,
    pub Eotf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayWireFormatEotf) -> windows_core::HRESULT,
    pub HdrMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayWireFormatHdrMetadata) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDisplayWireFormatFactory, IDisplayWireFormatFactory_Vtbl, 0xb2efc8d5_09d6_55e6_ad22_9014b3d25229);
impl windows_core::RuntimeType for IDisplayWireFormatFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayWireFormatFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayWireFormatPixelEncoding, i32, DisplayWireFormatColorSpace, DisplayWireFormatEotf, DisplayWireFormatHdrMetadata, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayWireFormatStatics, IDisplayWireFormatStatics_Vtbl, 0xc575a22d_c3e6_5f7a_bdfb_87c6ab8661d5);
impl windows_core::RuntimeType for IDisplayWireFormatStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayWireFormatStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DisplayWireFormatPixelEncoding, i32, DisplayWireFormatColorSpace, DisplayWireFormatEotf, DisplayWireFormatHdrMetadata, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayAdapter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayAdapter, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayAdapter {
    #[cfg(feature = "Graphics")]
    pub fn Id(&self) -> windows_core::Result<super::super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceInterfacePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInterfacePath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PciVendorId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PciVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PciDeviceId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PciDeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PciSubSystemId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PciSubSystemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PciRevision(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PciRevision)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn FromId(id: super::super::super::Graphics::DisplayAdapterId) -> windows_core::Result<DisplayAdapter> {
        Self::IDisplayAdapterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDisplayAdapterStatics<R, F: FnOnce(&IDisplayAdapterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayAdapter, IDisplayAdapterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayAdapter>();
}
unsafe impl windows_core::Interface for DisplayAdapter {
    type Vtable = IDisplayAdapter_Vtbl;
    const IID: windows_core::GUID = <IDisplayAdapter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayAdapter {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayAdapter";
}
unsafe impl Send for DisplayAdapter {}
unsafe impl Sync for DisplayAdapter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayDevice, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayDevice {
    pub fn CreateScanoutSource<P0>(&self, target: P0) -> windows_core::Result<DisplaySource>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateScanoutSource)(windows_core::Interface::as_raw(this), target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreatePrimary<P0, P1>(&self, target: P0, desc: P1) -> windows_core::Result<DisplaySurface>
    where
        P0: windows_core::Param<DisplayTarget>,
        P1: windows_core::Param<DisplayPrimaryDescription>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePrimary)(windows_core::Interface::as_raw(this), target.param().abi(), desc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTaskPool(&self) -> windows_core::Result<DisplayTaskPool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTaskPool)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreatePeriodicFence<P0>(&self, target: P0, offsetfromvblank: super::super::super::Foundation::TimeSpan) -> windows_core::Result<DisplayFence>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePeriodicFence)(windows_core::Interface::as_raw(this), target.param().abi(), offsetfromvblank, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WaitForVBlank<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplaySource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WaitForVBlank)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn CreateSimpleScanout<P0, P1>(&self, psource: P0, psurface: P1, subresourceindex: u32, syncinterval: u32) -> windows_core::Result<DisplayScanout>
    where
        P0: windows_core::Param<DisplaySource>,
        P1: windows_core::Param<DisplaySurface>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSimpleScanout)(windows_core::Interface::as_raw(this), psource.param().abi(), psurface.param().abi(), subresourceindex, syncinterval, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsCapabilitySupported(&self, capability: DisplayDeviceCapability) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCapabilitySupported)(windows_core::Interface::as_raw(this), capability, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))]
    pub fn CreateSimpleScanoutWithDirtyRectsAndOptions<P0, P1, P2>(&self, source: P0, surface: P1, subresourceindex: u32, syncinterval: u32, dirtyrects: P2, options: DisplayScanoutOptions) -> windows_core::Result<DisplayScanout>
    where
        P0: windows_core::Param<DisplaySource>,
        P1: windows_core::Param<DisplaySurface>,
        P2: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32>>,
    {
        let this = &windows_core::Interface::cast::<IDisplayDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSimpleScanoutWithDirtyRectsAndOptions)(windows_core::Interface::as_raw(this), source.param().abi(), surface.param().abi(), subresourceindex, syncinterval, dirtyrects.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayDevice>();
}
unsafe impl windows_core::Interface for DisplayDevice {
    type Vtable = IDisplayDevice_Vtbl;
    const IID: windows_core::GUID = <IDisplayDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayDevice {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayDevice";
}
unsafe impl Send for DisplayDevice {}
unsafe impl Sync for DisplayDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayFence(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayFence, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayFence {}
impl windows_core::RuntimeType for DisplayFence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayFence>();
}
unsafe impl windows_core::Interface for DisplayFence {
    type Vtable = IDisplayFence_Vtbl;
    const IID: windows_core::GUID = <IDisplayFence as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayFence {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayFence";
}
unsafe impl Send for DisplayFence {}
unsafe impl Sync for DisplayFence {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayManager, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DisplayManager, super::super::super::Foundation::IClosable);
impl DisplayManager {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentTargets(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentTargets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentAdapters(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentAdapters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryAcquireTarget<P0>(&self, target: P0) -> windows_core::Result<DisplayManagerResult>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireTarget)(windows_core::Interface::as_raw(this), target.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn ReleaseTarget<P0>(&self, target: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReleaseTarget)(windows_core::Interface::as_raw(this), target.param().abi()).ok() }
    }
    pub fn TryReadCurrentStateForAllTargets(&self) -> windows_core::Result<DisplayManagerResultWithState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryReadCurrentStateForAllTargets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndReadCurrentState<P0>(&self, targets: P0) -> windows_core::Result<DisplayManagerResultWithState>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireTargetsAndReadCurrentState)(windows_core::Interface::as_raw(this), targets.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndCreateEmptyState<P0>(&self, targets: P0) -> windows_core::Result<DisplayManagerResultWithState>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireTargetsAndCreateEmptyState)(windows_core::Interface::as_raw(this), targets.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryAcquireTargetsAndCreateSubstate<P0, P1>(&self, existingstate: P0, targets: P1) -> windows_core::Result<DisplayManagerResultWithState>
    where
        P0: windows_core::Param<DisplayState>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireTargetsAndCreateSubstate)(windows_core::Interface::as_raw(this), existingstate.param().abi(), targets.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDisplayDevice<P0>(&self, adapter: P0) -> windows_core::Result<DisplayDevice>
    where
        P0: windows_core::Param<DisplayAdapter>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDisplayDevice)(windows_core::Interface::as_raw(this), adapter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Enabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnabled(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnabled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Disabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Disabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisabled(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisabled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PathsFailedOrInvalidated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PathsFailedOrInvalidated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePathsFailedOrInvalidated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePathsFailedOrInvalidated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Create(options: DisplayManagerOptions) -> windows_core::Result<DisplayManager> {
        Self::IDisplayManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDisplayManagerStatics<R, F: FnOnce(&IDisplayManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayManager, IDisplayManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayManager>();
}
unsafe impl windows_core::Interface for DisplayManager {
    type Vtable = IDisplayManager_Vtbl;
    const IID: windows_core::GUID = <IDisplayManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayManager {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManager";
}
unsafe impl Send for DisplayManager {}
unsafe impl Sync for DisplayManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayManagerChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayManagerChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayManagerChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayManagerChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayManagerChangedEventArgs>();
}
unsafe impl windows_core::Interface for DisplayManagerChangedEventArgs {
    type Vtable = IDisplayManagerChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDisplayManagerChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayManagerChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerChangedEventArgs";
}
unsafe impl Send for DisplayManagerChangedEventArgs {}
unsafe impl Sync for DisplayManagerChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayManagerDisabledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayManagerDisabledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayManagerDisabledEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayManagerDisabledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayManagerDisabledEventArgs>();
}
unsafe impl windows_core::Interface for DisplayManagerDisabledEventArgs {
    type Vtable = IDisplayManagerDisabledEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDisplayManagerDisabledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayManagerDisabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerDisabledEventArgs";
}
unsafe impl Send for DisplayManagerDisabledEventArgs {}
unsafe impl Sync for DisplayManagerDisabledEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayManagerEnabledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayManagerEnabledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayManagerEnabledEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayManagerEnabledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayManagerEnabledEventArgs>();
}
unsafe impl windows_core::Interface for DisplayManagerEnabledEventArgs {
    type Vtable = IDisplayManagerEnabledEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDisplayManagerEnabledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayManagerEnabledEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerEnabledEventArgs";
}
unsafe impl Send for DisplayManagerEnabledEventArgs {}
unsafe impl Sync for DisplayManagerEnabledEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayManagerPathsFailedOrInvalidatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayManagerPathsFailedOrInvalidatedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayManagerPathsFailedOrInvalidatedEventArgs>();
}
unsafe impl windows_core::Interface for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    type Vtable = IDisplayManagerPathsFailedOrInvalidatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDisplayManagerPathsFailedOrInvalidatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayManagerPathsFailedOrInvalidatedEventArgs {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerPathsFailedOrInvalidatedEventArgs";
}
unsafe impl Send for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
unsafe impl Sync for DisplayManagerPathsFailedOrInvalidatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayManagerResultWithState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayManagerResultWithState, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayManagerResultWithState {
    pub fn ErrorCode(&self) -> windows_core::Result<DisplayManagerResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<DisplayState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayManagerResultWithState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayManagerResultWithState>();
}
unsafe impl windows_core::Interface for DisplayManagerResultWithState {
    type Vtable = IDisplayManagerResultWithState_Vtbl;
    const IID: windows_core::GUID = <IDisplayManagerResultWithState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayManagerResultWithState {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayManagerResultWithState";
}
unsafe impl Send for DisplayManagerResultWithState {}
unsafe impl Sync for DisplayManagerResultWithState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayModeInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayModeInfo, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayModeInfo {
    #[cfg(feature = "Graphics")]
    pub fn SourceResolution(&self) -> windows_core::Result<super::super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStereo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStereo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SourcePixelFormat(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourcePixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn TargetResolution(&self) -> windows_core::Result<super::super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PresentationRate(&self) -> windows_core::Result<DisplayPresentationRate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInterlaced(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterlaced)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetWireFormatSupportedBitsPerChannel(&self, encoding: DisplayWireFormatPixelEncoding) -> windows_core::Result<DisplayBitsPerChannel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetWireFormatSupportedBitsPerChannel)(windows_core::Interface::as_raw(this), encoding, &mut result__).map(|| result__)
        }
    }
    pub fn IsWireFormatSupported<P0>(&self, wireformat: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DisplayWireFormat>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireFormatSupported)(windows_core::Interface::as_raw(this), wireformat.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PhysicalPresentationRate(&self) -> windows_core::Result<DisplayPresentationRate> {
        let this = &windows_core::Interface::cast::<IDisplayModeInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalPresentationRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DisplayModeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayModeInfo>();
}
unsafe impl windows_core::Interface for DisplayModeInfo {
    type Vtable = IDisplayModeInfo_Vtbl;
    const IID: windows_core::GUID = <IDisplayModeInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayModeInfo {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayModeInfo";
}
unsafe impl Send for DisplayModeInfo {}
unsafe impl Sync for DisplayModeInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayPath(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayPath, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayPath {
    pub fn View(&self) -> windows_core::Result<DisplayView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).View)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Target(&self) -> windows_core::Result<DisplayTarget> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Target)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<DisplayPathStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn SourceResolution(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceResolution)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn SetSourceResolution<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourceResolution)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SourcePixelFormat(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourcePixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetSourcePixelFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourcePixelFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStereo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsStereo(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsStereo)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics")]
    pub fn TargetResolution(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetResolution)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn SetTargetResolution<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTargetResolution)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PresentationRate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPresentationRate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<DisplayPresentationRate>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPresentationRate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsInterlaced(&self) -> windows_core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterlaced)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsInterlaced<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsInterlaced)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn WireFormat(&self) -> windows_core::Result<DisplayWireFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WireFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWireFormat<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayWireFormat>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWireFormat)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<DisplayRotation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: DisplayRotation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scaling(&self) -> windows_core::Result<DisplayPathScaling> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scaling)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaling(&self, value: DisplayPathScaling) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaling)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindModes(&self, flags: DisplayModeQueryOptions) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindModes)(windows_core::Interface::as_raw(this), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplyPropertiesFromMode<P0>(&self, moderesult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayModeInfo>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ApplyPropertiesFromMode)(windows_core::Interface::as_raw(this), moderesult.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMap<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PhysicalPresentationRate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>> {
        let this = &windows_core::Interface::cast::<IDisplayPath2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalPresentationRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPhysicalPresentationRate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<DisplayPresentationRate>>,
    {
        let this = &windows_core::Interface::cast::<IDisplayPath2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPhysicalPresentationRate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for DisplayPath {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayPath>();
}
unsafe impl windows_core::Interface for DisplayPath {
    type Vtable = IDisplayPath_Vtbl;
    const IID: windows_core::GUID = <IDisplayPath as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayPath {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayPath";
}
unsafe impl Send for DisplayPath {}
unsafe impl Sync for DisplayPath {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayPrimaryDescription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayPrimaryDescription, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayPrimaryDescription {
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn ColorSpace(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorSpace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStereo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStereo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn MultisampleDescription(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MultisampleDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateInstance(width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> windows_core::Result<DisplayPrimaryDescription> {
        Self::IDisplayPrimaryDescriptionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), width, height, pixelformat, colorspace, isstereo, multisampledescription, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateWithProperties<P0>(extraproperties: P0, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> windows_core::Result<DisplayPrimaryDescription>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::GUID, windows_core::IInspectable>>>,
    {
        Self::IDisplayPrimaryDescriptionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithProperties)(windows_core::Interface::as_raw(this), extraproperties.param().abi(), width, height, pixelformat, colorspace, isstereo, multisampledescription, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDisplayPrimaryDescriptionFactory<R, F: FnOnce(&IDisplayPrimaryDescriptionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayPrimaryDescription, IDisplayPrimaryDescriptionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDisplayPrimaryDescriptionStatics<R, F: FnOnce(&IDisplayPrimaryDescriptionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayPrimaryDescription, IDisplayPrimaryDescriptionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayPrimaryDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayPrimaryDescription>();
}
unsafe impl windows_core::Interface for DisplayPrimaryDescription {
    type Vtable = IDisplayPrimaryDescription_Vtbl;
    const IID: windows_core::GUID = <IDisplayPrimaryDescription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayPrimaryDescription {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayPrimaryDescription";
}
unsafe impl Send for DisplayPrimaryDescription {}
unsafe impl Sync for DisplayPrimaryDescription {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayScanout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayScanout, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayScanout {}
impl windows_core::RuntimeType for DisplayScanout {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayScanout>();
}
unsafe impl windows_core::Interface for DisplayScanout {
    type Vtable = IDisplayScanout_Vtbl;
    const IID: windows_core::GUID = <IDisplayScanout as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayScanout {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayScanout";
}
unsafe impl Send for DisplayScanout {}
unsafe impl Sync for DisplayScanout {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplaySource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplaySource, windows_core::IUnknown, windows_core::IInspectable);
impl DisplaySource {
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> windows_core::Result<super::super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetMetadata(&self, key: windows_core::GUID) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMetadata)(windows_core::Interface::as_raw(this), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<DisplaySourceStatus> {
        let this = &windows_core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DisplaySource, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDisplaySource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for DisplaySource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplaySource>();
}
unsafe impl windows_core::Interface for DisplaySource {
    type Vtable = IDisplaySource_Vtbl;
    const IID: windows_core::GUID = <IDisplaySource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplaySource {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplaySource";
}
unsafe impl Send for DisplaySource {}
unsafe impl Sync for DisplaySource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayState, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayState {
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStale(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Targets(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Targets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayView>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Views)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMap<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectTarget<P0>(&self, target: P0) -> windows_core::Result<DisplayPath>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectTarget)(windows_core::Interface::as_raw(this), target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectTargetToView<P0, P1>(&self, target: P0, view: P1) -> windows_core::Result<DisplayPath>
    where
        P0: windows_core::Param<DisplayTarget>,
        P1: windows_core::Param<DisplayView>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectTargetToView)(windows_core::Interface::as_raw(this), target.param().abi(), view.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanConnectTargetToView<P0, P1>(&self, target: P0, view: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DisplayTarget>,
        P1: windows_core::Param<DisplayView>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanConnectTargetToView)(windows_core::Interface::as_raw(this), target.param().abi(), view.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetViewForTarget<P0>(&self, target: P0) -> windows_core::Result<DisplayView>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetViewForTarget)(windows_core::Interface::as_raw(this), target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPathForTarget<P0>(&self, target: P0) -> windows_core::Result<DisplayPath>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPathForTarget)(windows_core::Interface::as_raw(this), target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisconnectTarget<P0>(&self, target: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DisconnectTarget)(windows_core::Interface::as_raw(this), target.param().abi()).ok() }
    }
    pub fn TryFunctionalize(&self, options: DisplayStateFunctionalizeOptions) -> windows_core::Result<DisplayStateOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryFunctionalize)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryApply(&self, options: DisplayStateApplyOptions) -> windows_core::Result<DisplayStateOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryApply)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Clone(&self) -> windows_core::Result<DisplayState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayState>();
}
unsafe impl windows_core::Interface for DisplayState {
    type Vtable = IDisplayState_Vtbl;
    const IID: windows_core::GUID = <IDisplayState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayState {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayState";
}
unsafe impl Send for DisplayState {}
unsafe impl Sync for DisplayState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayStateOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayStateOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayStateOperationResult {
    pub fn Status(&self) -> windows_core::Result<DisplayStateOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DisplayStateOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayStateOperationResult>();
}
unsafe impl windows_core::Interface for DisplayStateOperationResult {
    type Vtable = IDisplayStateOperationResult_Vtbl;
    const IID: windows_core::GUID = <IDisplayStateOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayStateOperationResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayStateOperationResult";
}
unsafe impl Send for DisplayStateOperationResult {}
unsafe impl Sync for DisplayStateOperationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplaySurface(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplaySurface, windows_core::IUnknown, windows_core::IInspectable);
impl DisplaySurface {}
impl windows_core::RuntimeType for DisplaySurface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplaySurface>();
}
unsafe impl windows_core::Interface for DisplaySurface {
    type Vtable = IDisplaySurface_Vtbl;
    const IID: windows_core::GUID = <IDisplaySurface as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplaySurface {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplaySurface";
}
unsafe impl Send for DisplaySurface {}
unsafe impl Sync for DisplaySurface {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayTarget, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayTarget {
    pub fn Adapter(&self) -> windows_core::Result<DisplayAdapter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Adapter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceInterfacePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInterfacePath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdapterRelativeId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdapterRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVirtualModeEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVirtualModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVirtualTopologyEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVirtualTopologyEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageKind(&self) -> windows_core::Result<super::DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MonitorPersistence(&self) -> windows_core::Result<DisplayTargetPersistence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonitorPersistence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StableMonitorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StableMonitorId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetMonitor(&self) -> windows_core::Result<super::DisplayMonitor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetMonitor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsStale(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSame<P0>(&self, othertarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSame)(windows_core::Interface::as_raw(this), othertarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, othertarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<DisplayTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), othertarget.param().abi(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DisplayTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayTarget>();
}
unsafe impl windows_core::Interface for DisplayTarget {
    type Vtable = IDisplayTarget_Vtbl;
    const IID: windows_core::GUID = <IDisplayTarget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayTarget {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTarget";
}
unsafe impl Send for DisplayTarget {}
unsafe impl Sync for DisplayTarget {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayTask(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayTask, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayTask {
    pub fn SetScanout<P0>(&self, scanout: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayScanout>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScanout)(windows_core::Interface::as_raw(this), scanout.param().abi()).ok() }
    }
    pub fn SetWait<P0>(&self, readyfence: P0, readyfencevalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayFence>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWait)(windows_core::Interface::as_raw(this), readyfence.param().abi(), readyfencevalue).ok() }
    }
    pub fn SetSignal<P0>(&self, signalkind: DisplayTaskSignalKind, fence: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayFence>,
    {
        let this = &windows_core::Interface::cast::<IDisplayTask2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSignal)(windows_core::Interface::as_raw(this), signalkind, fence.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for DisplayTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayTask>();
}
unsafe impl windows_core::Interface for DisplayTask {
    type Vtable = IDisplayTask_Vtbl;
    const IID: windows_core::GUID = <IDisplayTask as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayTask {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTask";
}
unsafe impl Send for DisplayTask {}
unsafe impl Sync for DisplayTask {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayTaskPool(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayTaskPool, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayTaskPool {
    pub fn CreateTask(&self) -> windows_core::Result<DisplayTask> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTask)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExecuteTask<P0>(&self, task: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayTask>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ExecuteTask)(windows_core::Interface::as_raw(this), task.param().abi()).ok() }
    }
    pub fn TryExecuteTask<P0>(&self, task: P0) -> windows_core::Result<DisplayTaskResult>
    where
        P0: windows_core::Param<DisplayTask>,
    {
        let this = &windows_core::Interface::cast::<IDisplayTaskPool2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryExecuteTask)(windows_core::Interface::as_raw(this), task.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayTaskPool {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayTaskPool>();
}
unsafe impl windows_core::Interface for DisplayTaskPool {
    type Vtable = IDisplayTaskPool_Vtbl;
    const IID: windows_core::GUID = <IDisplayTaskPool as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayTaskPool {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTaskPool";
}
unsafe impl Send for DisplayTaskPool {}
unsafe impl Sync for DisplayTaskPool {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayTaskResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayTaskResult, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayTaskResult {
    pub fn PresentStatus(&self) -> windows_core::Result<DisplayPresentStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PresentId(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceStatus(&self) -> windows_core::Result<DisplaySourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DisplayTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayTaskResult>();
}
unsafe impl windows_core::Interface for DisplayTaskResult {
    type Vtable = IDisplayTaskResult_Vtbl;
    const IID: windows_core::GUID = <IDisplayTaskResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayTaskResult {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayTaskResult";
}
unsafe impl Send for DisplayTaskResult {}
unsafe impl Sync for DisplayTaskResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayView, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Paths(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayPath>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Paths)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn ContentResolution(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentResolution)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn SetContentResolution<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentResolution)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetPrimaryPath<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayPath>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrimaryPath)(windows_core::Interface::as_raw(this), path.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMap<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayView>();
}
unsafe impl windows_core::Interface for DisplayView {
    type Vtable = IDisplayView_Vtbl;
    const IID: windows_core::GUID = <IDisplayView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayView {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayView";
}
unsafe impl Send for DisplayView {}
unsafe impl Sync for DisplayView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DisplayWireFormat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayWireFormat, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayWireFormat {
    pub fn PixelEncoding(&self) -> windows_core::Result<DisplayWireFormatPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelEncoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitsPerChannel(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitsPerChannel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColorSpace(&self) -> windows_core::Result<DisplayWireFormatColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorSpace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Eotf(&self) -> windows_core::Result<DisplayWireFormatEotf> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Eotf)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HdrMetadata(&self) -> windows_core::Result<DisplayWireFormatHdrMetadata> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HdrMetadata)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> windows_core::Result<DisplayWireFormat> {
        Self::IDisplayWireFormatFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(extraproperties: P0, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> windows_core::Result<DisplayWireFormat>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::GUID, windows_core::IInspectable>>>,
    {
        Self::IDisplayWireFormatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithProperties)(windows_core::Interface::as_raw(this), extraproperties.param().abi(), pixelencoding, bitsperchannel, colorspace, eotf, hdrmetadata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDisplayWireFormatFactory<R, F: FnOnce(&IDisplayWireFormatFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayWireFormat, IDisplayWireFormatFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDisplayWireFormatStatics<R, F: FnOnce(&IDisplayWireFormatStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayWireFormat, IDisplayWireFormatStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayWireFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayWireFormat>();
}
unsafe impl windows_core::Interface for DisplayWireFormat {
    type Vtable = IDisplayWireFormat_Vtbl;
    const IID: windows_core::GUID = <IDisplayWireFormat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayWireFormat {
    const NAME: &'static str = "Windows.Devices.Display.Core.DisplayWireFormat";
}
unsafe impl Send for DisplayWireFormat {}
unsafe impl Sync for DisplayWireFormat {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: Self = Self(0u32);
    pub const Bpc6: Self = Self(1u32);
    pub const Bpc8: Self = Self(2u32);
    pub const Bpc10: Self = Self(4u32);
    pub const Bpc12: Self = Self(8u32);
    pub const Bpc14: Self = Self(16u32);
    pub const Bpc16: Self = Self(32u32);
}
impl windows_core::TypeKind for DisplayBitsPerChannel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayBitsPerChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayBitsPerChannel").field(&self.0).finish()
    }
}
impl DisplayBitsPerChannel {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayBitsPerChannel {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayBitsPerChannel {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayBitsPerChannel {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayBitsPerChannel {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayBitsPerChannel {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayBitsPerChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayBitsPerChannel;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: Self = Self(0i32);
}
impl windows_core::TypeKind for DisplayDeviceCapability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayDeviceCapability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayDeviceCapability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayDeviceCapability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayDeviceCapability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayManagerOptions(pub u32);
impl DisplayManagerOptions {
    pub const None: Self = Self(0u32);
    pub const EnforceSourceOwnership: Self = Self(1u32);
    pub const VirtualRefreshRateAware: Self = Self(2u32);
}
impl windows_core::TypeKind for DisplayManagerOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayManagerOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayManagerOptions").field(&self.0).finish()
    }
}
impl DisplayManagerOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayManagerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayManagerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayManagerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayManagerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayManagerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayManagerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayManagerOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const TargetAccessDenied: Self = Self(2i32);
    pub const TargetStale: Self = Self(3i32);
    pub const RemoteSessionNotSupported: Self = Self(4i32);
}
impl windows_core::TypeKind for DisplayManagerResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayManagerResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayManagerResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayManagerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayManagerResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: Self = Self(0u32);
    pub const OnlyPreferredResolution: Self = Self(1u32);
}
impl windows_core::TypeKind for DisplayModeQueryOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayModeQueryOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayModeQueryOptions").field(&self.0).finish()
    }
}
impl DisplayModeQueryOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayModeQueryOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayModeQueryOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayModeQueryOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayModeQueryOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayModeQueryOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayModeQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayModeQueryOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: Self = Self(0i32);
    pub const Centered: Self = Self(1i32);
    pub const Stretched: Self = Self(2i32);
    pub const AspectRatioStretched: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const DriverPreferred: Self = Self(5i32);
}
impl windows_core::TypeKind for DisplayPathScaling {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayPathScaling {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayPathScaling").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayPathScaling {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPathScaling;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const FailedAsync: Self = Self(4i32);
    pub const InvalidatedAsync: Self = Self(5i32);
}
impl windows_core::TypeKind for DisplayPathStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayPathStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayPathStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayPathStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPathStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: Self = Self(0i32);
    pub const SourceStatusPreventedPresent: Self = Self(1i32);
    pub const ScanoutInvalid: Self = Self(2i32);
    pub const SourceInvalid: Self = Self(3i32);
    pub const DeviceInvalid: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl windows_core::TypeKind for DisplayPresentStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayPresentStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayPresentStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayPresentStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayPresentStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl windows_core::TypeKind for DisplayRotation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayRotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayRotation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayRotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayRotation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: Self = Self(0u32);
    pub const AllowTearing: Self = Self(2u32);
}
impl windows_core::TypeKind for DisplayScanoutOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayScanoutOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayScanoutOptions").field(&self.0).finish()
    }
}
impl DisplayScanoutOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayScanoutOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayScanoutOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayScanoutOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayScanoutOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayScanoutOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayScanoutOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayScanoutOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: Self = Self(0i32);
    pub const PoweredOff: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
    pub const OwnedByAnotherDevice: Self = Self(3i32);
    pub const Unowned: Self = Self(4i32);
}
impl windows_core::TypeKind for DisplaySourceStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplaySourceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplaySourceStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplaySourceStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplaySourceStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ForceReapply: Self = Self(2u32);
    pub const ForceModeEnumeration: Self = Self(4u32);
}
impl windows_core::TypeKind for DisplayStateApplyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayStateApplyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayStateApplyOptions").field(&self.0).finish()
    }
}
impl DisplayStateApplyOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayStateApplyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayStateApplyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayStateApplyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayStateApplyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayStateApplyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayStateApplyOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateApplyOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ValidateTopologyOnly: Self = Self(2u32);
}
impl windows_core::TypeKind for DisplayStateFunctionalizeOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayStateFunctionalizeOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayStateFunctionalizeOptions").field(&self.0).finish()
    }
}
impl DisplayStateFunctionalizeOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayStateFunctionalizeOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayStateFunctionalizeOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayStateFunctionalizeOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayStateFunctionalizeOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateFunctionalizeOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const PartialFailure: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
    pub const TargetOwnershipLost: Self = Self(3i32);
    pub const SystemStateChanged: Self = Self(4i32);
    pub const TooManyPathsForAdapter: Self = Self(5i32);
    pub const ModesNotSupported: Self = Self(6i32);
    pub const RemoteSessionNotSupported: Self = Self(7i32);
}
impl windows_core::TypeKind for DisplayStateOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayStateOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayStateOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayStateOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayStateOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: Self = Self(0i32);
    pub const BootPersisted: Self = Self(1i32);
    pub const TemporaryPersisted: Self = Self(2i32);
    pub const PathPersisted: Self = Self(3i32);
}
impl windows_core::TypeKind for DisplayTargetPersistence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayTargetPersistence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayTargetPersistence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayTargetPersistence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayTargetPersistence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayTaskSignalKind(pub i32);
impl DisplayTaskSignalKind {
    pub const OnPresentFlipAway: Self = Self(0i32);
    pub const OnPresentFlipTo: Self = Self(1i32);
}
impl windows_core::TypeKind for DisplayTaskSignalKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayTaskSignalKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayTaskSignalKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayTaskSignalKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayTaskSignalKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: Self = Self(0i32);
    pub const BT2020: Self = Self(1i32);
    pub const ProfileDefinedWideColorGamut: Self = Self(2i32);
}
impl windows_core::TypeKind for DisplayWireFormatColorSpace {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayWireFormatColorSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayWireFormatColorSpace").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayWireFormatColorSpace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatColorSpace;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: Self = Self(0i32);
    pub const HdrSmpte2084: Self = Self(1i32);
}
impl windows_core::TypeKind for DisplayWireFormatEotf {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayWireFormatEotf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayWireFormatEotf").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayWireFormatEotf {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatEotf;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: Self = Self(0i32);
    pub const Hdr10: Self = Self(1i32);
    pub const Hdr10Plus: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl windows_core::TypeKind for DisplayWireFormatHdrMetadata {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayWireFormatHdrMetadata {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayWireFormatHdrMetadata").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayWireFormatHdrMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatHdrMetadata;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
    pub const Intensity: Self = Self(4i32);
}
impl windows_core::TypeKind for DisplayWireFormatPixelEncoding {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayWireFormatPixelEncoding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayWireFormatPixelEncoding").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayWireFormatPixelEncoding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.Core.DisplayWireFormatPixelEncoding;i4)");
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayPresentationRate {
    pub VerticalSyncRate: super::super::super::Foundation::Numerics::Rational,
    pub VerticalSyncsPerPresentation: i32,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for DisplayPresentationRate {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for DisplayPresentationRate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Display.Core.DisplayPresentationRate;struct(Windows.Foundation.Numerics.Rational;u4;u4);i4)");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for DisplayPresentationRate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
windows_core::imp::define_interface!(ISpatialAnchor, ISpatialAnchor_Vtbl, 0x0529e5ce_1d34_3702_bcec_eabff578a869);
impl windows_core::RuntimeType for ISpatialAnchor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawCoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawCoordinateSystemAdjusted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRawCoordinateSystemAdjusted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAnchor2, ISpatialAnchor2_Vtbl, 0xed17c908_a695_4cf6_92fd_97263ba71047);
impl windows_core::RuntimeType for ISpatialAnchor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemovedByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAnchorExportSufficiency, ISpatialAnchorExportSufficiency_Vtbl, 0x77c25b2b_3409_4088_b91b_fdfd05d1648f);
impl windows_core::RuntimeType for ISpatialAnchorExportSufficiency {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorExportSufficiency_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsMinimallySufficient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SufficiencyLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub RecommendedSufficiencyLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAnchorExporter, ISpatialAnchorExporter_Vtbl, 0x9a2a4338_24fb_4269_89c5_88304aeef20f);
impl windows_core::RuntimeType for ISpatialAnchorExporter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorExporter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAnchorExportSufficiencyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpatialAnchorExportPurpose, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub TryExportAnchorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpatialAnchorExportPurpose, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryExportAnchorAsync: usize,
}
windows_core::imp::define_interface!(ISpatialAnchorExporterStatics, ISpatialAnchorExporterStatics_Vtbl, 0xed2507b8_2475_439c_85ff_7fed341fdc88);
impl windows_core::RuntimeType for ISpatialAnchorExporterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorExporterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAnchorManagerStatics, ISpatialAnchorManagerStatics_Vtbl, 0x88e30eab_f3b7_420b_b086_8a80c07d910d);
impl windows_core::RuntimeType for ISpatialAnchorManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAnchorRawCoordinateSystemAdjustedEventArgs, ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl, 0xa1e81eb8_56c7_3117_a2e4_81e0fcf28e00);
impl windows_core::RuntimeType for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub OldRawCoordinateSystemToNewRawCoordinateSystemTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OldRawCoordinateSystemToNewRawCoordinateSystemTransform: usize,
}
windows_core::imp::define_interface!(ISpatialAnchorStatics, ISpatialAnchorStatics_Vtbl, 0xa9928642_0174_311c_ae79_0e5107669f16);
impl windows_core::RuntimeType for ISpatialAnchorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCreateRelativeTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionRelativeTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionRelativeTo: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionAndOrientationRelativeTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionAndOrientationRelativeTo: usize,
}
windows_core::imp::define_interface!(ISpatialAnchorStore, ISpatialAnchorStore_Vtbl, 0xb0bc3636_486a_3cb0_9e6f_1245165c4db6);
impl windows_core::RuntimeType for ISpatialAnchorStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAnchorStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSavedAnchors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSavedAnchors: usize,
    pub TrySave: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(ISpatialAnchorTransferManagerStatics, ISpatialAnchorTransferManagerStatics_Vtbl, 0x03bbf9b9_12d8_4bce_8835_c5df3ac0adab);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for ISpatialAnchorTransferManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISpatialAnchorTransferManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryImportAnchorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryImportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryExportAnchorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryExportAnchorsAsync: usize,
    #[cfg(feature = "deprecated")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestAccessAsync: usize,
}
windows_core::imp::define_interface!(ISpatialBoundingVolume, ISpatialBoundingVolume_Vtbl, 0xfb2065da_68c3_33df_b7af_4c787207999c);
impl windows_core::RuntimeType for ISpatialBoundingVolume {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialBoundingVolume_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISpatialBoundingVolumeStatics, ISpatialBoundingVolumeStatics_Vtbl, 0x05889117_b3e1_36d8_b017_566181a5b196);
impl windows_core::RuntimeType for ISpatialBoundingVolumeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialBoundingVolumeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpatialBoundingBox, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromOrientedBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpatialBoundingOrientedBox, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromOrientedBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromSphere: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpatialBoundingSphere, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromSphere: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromFrustum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpatialBoundingFrustum, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromFrustum: usize,
}
windows_core::imp::define_interface!(ISpatialCoordinateSystem, ISpatialCoordinateSystem_Vtbl, 0x69ebca4b_60a3_3586_a653_59a7bd676d07);
impl windows_core::RuntimeType for ISpatialCoordinateSystem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialCoordinateSystem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetTransformTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetTransformTo: usize,
}
windows_core::imp::define_interface!(ISpatialEntity, ISpatialEntity_Vtbl, 0x166de955_e1eb_454c_ba08_e6c0668ddc65);
impl windows_core::RuntimeType for ISpatialEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Anchor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(ISpatialEntityAddedEventArgs, ISpatialEntityAddedEventArgs_Vtbl, 0xa397f49b_156a_4707_ac2c_d31d570ed399);
impl windows_core::RuntimeType for ISpatialEntityAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Entity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialEntityFactory, ISpatialEntityFactory_Vtbl, 0xe1f1e325_349f_4225_a2f3_4b01c15fe056);
impl windows_core::RuntimeType for ISpatialEntityFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithSpatialAnchor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithSpatialAnchorAndProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithSpatialAnchorAndProperties: usize,
}
windows_core::imp::define_interface!(ISpatialEntityRemovedEventArgs, ISpatialEntityRemovedEventArgs_Vtbl, 0x91741800_536d_4e9f_abf6_415b5444d651);
impl windows_core::RuntimeType for ISpatialEntityRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Entity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialEntityStore, ISpatialEntityStore_Vtbl, 0x329788ba_e513_4f06_889d_1be30ecf43e6);
impl windows_core::RuntimeType for ISpatialEntityStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEntityWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialEntityStoreStatics, ISpatialEntityStoreStatics_Vtbl, 0x6b4b389e_7c50_4e92_8a62_4d1d4b7ccd3e);
impl windows_core::RuntimeType for ISpatialEntityStoreStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityStoreStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "System_RemoteSystems")]
    pub TryGetForRemoteSystemSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))]
    TryGetForRemoteSystemSession: usize,
}
windows_core::imp::define_interface!(ISpatialEntityUpdatedEventArgs, ISpatialEntityUpdatedEventArgs_Vtbl, 0xe5671766_627b_43cb_a49f_b3be6d47deed);
impl windows_core::RuntimeType for ISpatialEntityUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Entity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialEntityWatcher, ISpatialEntityWatcher_Vtbl, 0xb3b85fa0_6d5e_4bbc_805d_5fe5b9ba1959);
impl windows_core::RuntimeType for ISpatialEntityWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialEntityWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialEntityWatcherStatus) -> windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialLocation, ISpatialLocation_Vtbl, 0x1d81d29d_24a1_37d5_8fa1_39b4f9ad67e2);
impl windows_core::RuntimeType for ISpatialLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialLocation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearAcceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearAcceleration: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularVelocity: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularAcceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularAcceleration: usize,
}
windows_core::imp::define_interface!(ISpatialLocation2, ISpatialLocation2_Vtbl, 0x117f2416_38a7_4a18_b404_ab8fabe1d78b);
impl windows_core::RuntimeType for ISpatialLocation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialLocation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularVelocityAxisAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularVelocityAxisAngle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularAccelerationAxisAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularAccelerationAxisAngle: usize,
}
windows_core::imp::define_interface!(ISpatialLocator, ISpatialLocator_Vtbl, 0xf6478925_9e0c_3bb6_997e_b64ecca24cf4);
impl windows_core::RuntimeType for ISpatialLocator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialLocator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Locatability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialLocatability) -> windows_core::HRESULT,
    pub LocatabilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLocatabilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PositionalTrackingDeactivating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePositionalTrackingDeactivating: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TryLocateAtTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAttachedFrameOfReferenceAtCurrentHeading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: usize,
    pub CreateStationaryFrameOfReferenceAtCurrentLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: usize,
}
windows_core::imp::define_interface!(ISpatialLocatorAttachedFrameOfReference, ISpatialLocatorAttachedFrameOfReference_Vtbl, 0xe1774ef6_1f4f_499c_9625_ef5e6ed7a048);
impl windows_core::RuntimeType for ISpatialLocatorAttachedFrameOfReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialLocatorAttachedFrameOfReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativePosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeOrientation: usize,
    pub AdjustHeading: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetStationaryCoordinateSystemAtTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryGetRelativeHeadingAtTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialLocatorPositionalTrackingDeactivatingEventArgs, ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl, 0xb8a84063_e3f4_368b_9061_9ea9d1d6cc16);
impl windows_core::RuntimeType for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Canceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialLocatorStatics, ISpatialLocatorStatics_Vtbl, 0xb76e3340_a7c2_361b_bb82_56e93b89b1bb);
impl windows_core::RuntimeType for ISpatialLocatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialLocatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialStageFrameOfReference, ISpatialStageFrameOfReference_Vtbl, 0x7a8a3464_ad0d_4590_ab86_33062b674926);
impl windows_core::RuntimeType for ISpatialStageFrameOfReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialStageFrameOfReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MovementRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialMovementRange) -> windows_core::HRESULT,
    pub LookDirectionRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialLookDirectionRange) -> windows_core::HRESULT,
    pub GetCoordinateSystemAtCurrentLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMovementBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMovementBounds: usize,
}
windows_core::imp::define_interface!(ISpatialStageFrameOfReferenceStatics, ISpatialStageFrameOfReferenceStatics_Vtbl, 0xf78d5c4d_a0a4_499c_8d91_a8c965d40654);
impl windows_core::RuntimeType for ISpatialStageFrameOfReferenceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialStageFrameOfReferenceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCurrentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestNewStageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialStationaryFrameOfReference, ISpatialStationaryFrameOfReference_Vtbl, 0x09dbccb9_bcf8_3e7f_be7e_7edccbb178a8);
impl windows_core::RuntimeType for ISpatialStationaryFrameOfReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialStationaryFrameOfReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAnchor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAnchor, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAnchor {
    pub fn CoordinateSystem(&self) -> windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawCoordinateSystem(&self) -> windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawCoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawCoordinateSystemAdjusted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawCoordinateSystemAdjusted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRawCoordinateSystemAdjusted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRawCoordinateSystemAdjusted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn RemovedByUser(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialAnchor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovedByUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryCreateRelativeTo<P0>(coordinatesystem: P0) -> windows_core::Result<SpatialAnchor>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateRelativeTo)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionRelativeTo<P0>(coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<SpatialAnchor>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateWithPositionRelativeTo)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionAndOrientationRelativeTo<P0>(coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<SpatialAnchor>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateWithPositionAndOrientationRelativeTo)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), position, orientation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorStatics<R, F: FnOnce(&ISpatialAnchorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAnchor, ISpatialAnchorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialAnchor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAnchor>();
}
unsafe impl windows_core::Interface for SpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
    const IID: windows_core::GUID = <ISpatialAnchor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchor";
}
unsafe impl Send for SpatialAnchor {}
unsafe impl Sync for SpatialAnchor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAnchorExportSufficiency(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAnchorExportSufficiency, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAnchorExportSufficiency {
    pub fn IsMinimallySufficient(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMinimallySufficient)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SufficiencyLevel(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SufficiencyLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RecommendedSufficiencyLevel(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecommendedSufficiencyLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialAnchorExportSufficiency {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAnchorExportSufficiency>();
}
unsafe impl windows_core::Interface for SpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
    const IID: windows_core::GUID = <ISpatialAnchorExportSufficiency as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
}
unsafe impl Send for SpatialAnchorExportSufficiency {}
unsafe impl Sync for SpatialAnchorExportSufficiency {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAnchorExporter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAnchorExporter, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAnchorExporter {
    pub fn GetAnchorExportSufficiencyAsync<P0>(&self, anchor: P0, purpose: SpatialAnchorExportPurpose) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>
    where
        P0: windows_core::Param<SpatialAnchor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnchorExportSufficiencyAsync)(windows_core::Interface::as_raw(this), anchor.param().abi(), purpose, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryExportAnchorAsync<P0, P1>(&self, anchor: P0, purpose: SpatialAnchorExportPurpose, stream: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<SpatialAnchor>,
        P1: windows_core::Param<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryExportAnchorAsync)(windows_core::Interface::as_raw(this), anchor.param().abi(), purpose, stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<SpatialAnchorExporter> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorExporterStatics<R, F: FnOnce(&ISpatialAnchorExporterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAnchorExporter, ISpatialAnchorExporterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialAnchorExporter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAnchorExporter>();
}
unsafe impl windows_core::Interface for SpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
    const IID: windows_core::GUID = <ISpatialAnchorExporter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExporter";
}
unsafe impl Send for SpatialAnchorExporter {}
unsafe impl Sync for SpatialAnchorExporter {}
pub struct SpatialAnchorManager;
impl SpatialAnchorManager {
    pub fn RequestStoreAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>> {
        Self::ISpatialAnchorManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorManagerStatics<R, F: FnOnce(&ISpatialAnchorManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAnchorManager, ISpatialAnchorManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for SpatialAnchorManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAnchorRawCoordinateSystemAdjustedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> windows_core::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAnchorRawCoordinateSystemAdjustedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialAnchorRawCoordinateSystemAdjustedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
unsafe impl Send for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
unsafe impl Sync for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAnchorStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAnchorStore, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAnchorStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSavedAnchors(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, SpatialAnchor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllSavedAnchors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySave<P0>(&self, id: &windows_core::HSTRING, anchor: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<SpatialAnchor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySave)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), anchor.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, id: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SpatialAnchorStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAnchorStore>();
}
unsafe impl windows_core::Interface for SpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
    const IID: windows_core::GUID = <ISpatialAnchorStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorStore";
}
unsafe impl Send for SpatialAnchorStore {}
unsafe impl Sync for SpatialAnchorStore {}
#[cfg(feature = "deprecated")]
pub struct SpatialAnchorTransferManager;
#[cfg(feature = "deprecated")]
impl SpatialAnchorTransferManager {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryImportAnchorsAsync<P0>(stream: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, SpatialAnchor>>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryImportAnchorsAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryExportAnchorsAsync<P0, P1>(anchors: P0, stream: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, SpatialAnchor>>>,
        P1: windows_core::Param<super::super::Storage::Streams::IOutputStream>,
    {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryExportAnchorsAsync)(windows_core::Interface::as_raw(this), anchors.param().abi(), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISpatialAnchorTransferManagerStatics<R, F: FnOnce(&ISpatialAnchorTransferManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAnchorTransferManager, ISpatialAnchorTransferManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for SpatialAnchorTransferManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorTransferManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialBoundingVolume(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialBoundingVolume, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialBoundingVolume {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromBox<P0>(coordinatesystem: P0, r#box: SpatialBoundingBox) -> windows_core::Result<SpatialBoundingVolume>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBox)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), r#box, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromOrientedBox<P0>(coordinatesystem: P0, r#box: SpatialBoundingOrientedBox) -> windows_core::Result<SpatialBoundingVolume>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromOrientedBox)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), r#box, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromSphere<P0>(coordinatesystem: P0, sphere: SpatialBoundingSphere) -> windows_core::Result<SpatialBoundingVolume>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromSphere)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), sphere, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromFrustum<P0>(coordinatesystem: P0, frustum: SpatialBoundingFrustum) -> windows_core::Result<SpatialBoundingVolume>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromFrustum)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), frustum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialBoundingVolumeStatics<R, F: FnOnce(&ISpatialBoundingVolumeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialBoundingVolume, ISpatialBoundingVolumeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialBoundingVolume {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialBoundingVolume>();
}
unsafe impl windows_core::Interface for SpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
    const IID: windows_core::GUID = <ISpatialBoundingVolume as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialBoundingVolume";
}
unsafe impl Send for SpatialBoundingVolume {}
unsafe impl Sync for SpatialBoundingVolume {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialCoordinateSystem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialCoordinateSystem, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialCoordinateSystem {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetTransformTo<P0>(&self, target: P0) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetTransformTo)(windows_core::Interface::as_raw(this), target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialCoordinateSystem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialCoordinateSystem>();
}
unsafe impl windows_core::Interface for SpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
    const IID: windows_core::GUID = <ISpatialCoordinateSystem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialCoordinateSystem";
}
unsafe impl Send for SpatialCoordinateSystem {}
unsafe impl Sync for SpatialCoordinateSystem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialEntity, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialEntity {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Anchor(&self) -> windows_core::Result<SpatialAnchor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Anchor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWithSpatialAnchor<P0>(spatialanchor: P0) -> windows_core::Result<SpatialEntity>
    where
        P0: windows_core::Param<SpatialAnchor>,
    {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithSpatialAnchor)(windows_core::Interface::as_raw(this), spatialanchor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithSpatialAnchorAndProperties<P0, P1>(spatialanchor: P0, propertyset: P1) -> windows_core::Result<SpatialEntity>
    where
        P0: windows_core::Param<SpatialAnchor>,
        P1: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithSpatialAnchorAndProperties)(windows_core::Interface::as_raw(this), spatialanchor.param().abi(), propertyset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialEntityFactory<R, F: FnOnce(&ISpatialEntityFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialEntity, ISpatialEntityFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialEntity>();
}
unsafe impl windows_core::Interface for SpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
    const IID: windows_core::GUID = <ISpatialEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntity";
}
unsafe impl Send for SpatialEntity {}
unsafe impl Sync for SpatialEntity {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialEntityAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialEntityAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialEntityAddedEventArgs {
    pub fn Entity(&self) -> windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialEntityAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialEntityAddedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialEntityAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
}
unsafe impl Send for SpatialEntityAddedEventArgs {}
unsafe impl Sync for SpatialEntityAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialEntityRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialEntityRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialEntityRemovedEventArgs {
    pub fn Entity(&self) -> windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialEntityRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialEntityRemovedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialEntityRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
}
unsafe impl Send for SpatialEntityRemovedEventArgs {}
unsafe impl Sync for SpatialEntityRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialEntityStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialEntityStore, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialEntityStore {
    pub fn SaveAsync<P0>(&self, entity: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<SpatialEntity>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), entity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveAsync<P0>(&self, entity: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<SpatialEntity>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAsync)(windows_core::Interface::as_raw(this), entity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEntityWatcher(&self) -> windows_core::Result<SpatialEntityWatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEntityWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "System_RemoteSystems")]
    pub fn TryGetForRemoteSystemSession<P0>(session: P0) -> windows_core::Result<SpatialEntityStore>
    where
        P0: windows_core::Param<super::super::System::RemoteSystems::RemoteSystemSession>,
    {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetForRemoteSystemSession)(windows_core::Interface::as_raw(this), session.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialEntityStoreStatics<R, F: FnOnce(&ISpatialEntityStoreStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialEntityStore, ISpatialEntityStoreStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialEntityStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialEntityStore>();
}
unsafe impl windows_core::Interface for SpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
    const IID: windows_core::GUID = <ISpatialEntityStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityStore";
}
unsafe impl Send for SpatialEntityStore {}
unsafe impl Sync for SpatialEntityStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialEntityUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialEntityUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialEntityUpdatedEventArgs {
    pub fn Entity(&self) -> windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialEntityUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialEntityUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialEntityUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
}
unsafe impl Send for SpatialEntityUpdatedEventArgs {}
unsafe impl Sync for SpatialEntityUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialEntityWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialEntityWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialEntityWatcher {
    pub fn Status(&self) -> windows_core::Result<SpatialEntityWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>,
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
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>,
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
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>,
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
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, windows_core::IInspectable>>,
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
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SpatialEntityWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialEntityWatcher>();
}
unsafe impl windows_core::Interface for SpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
    const IID: windows_core::GUID = <ISpatialEntityWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityWatcher";
}
unsafe impl Send for SpatialEntityWatcher {}
unsafe impl Sync for SpatialEntityWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialLocation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialLocation, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialLocation {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearVelocity(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteLinearVelocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearAcceleration(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteLinearAcceleration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularVelocity(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteAngularVelocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularAcceleration(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteAngularAcceleration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocityAxisAngle(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteAngularVelocityAxisAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAccelerationAxisAngle(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteAngularAccelerationAxisAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialLocation>();
}
unsafe impl windows_core::Interface for SpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
    const IID: windows_core::GUID = <ISpatialLocation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocation";
}
unsafe impl Send for SpatialLocation {}
unsafe impl Sync for SpatialLocation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialLocator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialLocator, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialLocator {
    pub fn Locatability(&self) -> windows_core::Result<SpatialLocatability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Locatability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LocatabilityChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialLocator, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocatabilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLocatabilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLocatabilityChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PositionalTrackingDeactivating<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionalTrackingDeactivating)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePositionalTrackingDeactivating(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePositionalTrackingDeactivating)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn TryLocateAtTimestamp<P0, P1>(&self, timestamp: P0, coordinatesystem: P1) -> windows_core::Result<SpatialLocation>
    where
        P0: windows_core::Param<super::PerceptionTimestamp>,
        P1: windows_core::Param<SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryLocateAtTimestamp)(windows_core::Interface::as_raw(this), timestamp.param().abi(), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativeposition: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(windows_core::Interface::as_raw(this), relativeposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(windows_core::Interface::as_raw(this), relativeposition, relativeorientation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(windows_core::Interface::as_raw(this), relativeposition, relativeorientation, relativeheadinginradians, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativeposition: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(windows_core::Interface::as_raw(this), relativeposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(windows_core::Interface::as_raw(this), relativeposition, relativeorientation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(windows_core::Interface::as_raw(this), relativeposition, relativeorientation, relativeheadinginradians, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<SpatialLocator> {
        Self::ISpatialLocatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialLocatorStatics<R, F: FnOnce(&ISpatialLocatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialLocator, ISpatialLocatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialLocator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialLocator>();
}
unsafe impl windows_core::Interface for SpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
    const IID: windows_core::GUID = <ISpatialLocator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocator";
}
unsafe impl Send for SpatialLocator {}
unsafe impl Sync for SpatialLocator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialLocatorAttachedFrameOfReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialLocatorAttachedFrameOfReference, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialLocatorAttachedFrameOfReference {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativePosition(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativePosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativePosition(&self, value: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelativePosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOrientation(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelativeOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AdjustHeading(&self, headingoffsetinradians: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AdjustHeading)(windows_core::Interface::as_raw(this), headingoffsetinradians).ok() }
    }
    pub fn GetStationaryCoordinateSystemAtTimestamp<P0>(&self, timestamp: P0) -> windows_core::Result<SpatialCoordinateSystem>
    where
        P0: windows_core::Param<super::PerceptionTimestamp>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStationaryCoordinateSystemAtTimestamp)(windows_core::Interface::as_raw(this), timestamp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetRelativeHeadingAtTimestamp<P0>(&self, timestamp: P0) -> windows_core::Result<super::super::Foundation::IReference<f64>>
    where
        P0: windows_core::Param<super::PerceptionTimestamp>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetRelativeHeadingAtTimestamp)(windows_core::Interface::as_raw(this), timestamp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialLocatorAttachedFrameOfReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialLocatorAttachedFrameOfReference>();
}
unsafe impl windows_core::Interface for SpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
    const IID: windows_core::GUID = <ISpatialLocatorAttachedFrameOfReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
}
unsafe impl Send for SpatialLocatorAttachedFrameOfReference {}
unsafe impl Sync for SpatialLocatorAttachedFrameOfReference {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialLocatorPositionalTrackingDeactivatingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    pub fn Canceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Canceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanceled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCanceled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialLocatorPositionalTrackingDeactivatingEventArgs>();
}
unsafe impl windows_core::Interface for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialLocatorPositionalTrackingDeactivatingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
unsafe impl Send for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
unsafe impl Sync for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialStageFrameOfReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialStageFrameOfReference, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialStageFrameOfReference {
    pub fn CoordinateSystem(&self) -> windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MovementRange(&self) -> windows_core::Result<SpatialMovementRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MovementRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LookDirectionRange(&self) -> windows_core::Result<SpatialLookDirectionRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LookDirectionRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCoordinateSystemAtCurrentLocation<P0>(&self, locator: P0) -> windows_core::Result<SpatialCoordinateSystem>
    where
        P0: windows_core::Param<SpatialLocator>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCoordinateSystemAtCurrentLocation)(windows_core::Interface::as_raw(this), locator.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetMovementBounds<P0>(&self, coordinatesystem: P0) -> windows_core::Result<windows_core::Array<super::super::Foundation::Numerics::Vector3>>
    where
        P0: windows_core::Param<SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).TryGetMovementBounds)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), windows_core::Array::<super::super::Foundation::Numerics::Vector3>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Current() -> windows_core::Result<SpatialStageFrameOfReference> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CurrentChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveCurrentChanged(cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveCurrentChanged)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    pub fn RequestNewStageAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestNewStageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialStageFrameOfReferenceStatics<R, F: FnOnce(&ISpatialStageFrameOfReferenceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialStageFrameOfReference, ISpatialStageFrameOfReferenceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialStageFrameOfReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialStageFrameOfReference>();
}
unsafe impl windows_core::Interface for SpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
    const IID: windows_core::GUID = <ISpatialStageFrameOfReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStageFrameOfReference";
}
unsafe impl Send for SpatialStageFrameOfReference {}
unsafe impl Sync for SpatialStageFrameOfReference {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialStationaryFrameOfReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialStationaryFrameOfReference, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialStationaryFrameOfReference {
    pub fn CoordinateSystem(&self) -> windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialStationaryFrameOfReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialStationaryFrameOfReference>();
}
unsafe impl windows_core::Interface for SpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
    const IID: windows_core::GUID = <ISpatialStationaryFrameOfReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
}
unsafe impl Send for SpatialStationaryFrameOfReference {}
unsafe impl Sync for SpatialStationaryFrameOfReference {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: Self = Self(0i32);
    pub const Sharing: Self = Self(1i32);
}
impl windows_core::TypeKind for SpatialAnchorExportPurpose {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAnchorExportPurpose {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportPurpose").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialAnchorExportPurpose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialAnchorExportPurpose;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl windows_core::TypeKind for SpatialEntityWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialEntityWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialEntityWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialEntityWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const PositionalTrackingActivating: Self = Self(2i32);
    pub const PositionalTrackingActive: Self = Self(3i32);
    pub const PositionalTrackingInhibited: Self = Self(4i32);
}
impl windows_core::TypeKind for SpatialLocatability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialLocatability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialLocatability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialLocatability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLocatability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: Self = Self(0i32);
    pub const Omnidirectional: Self = Self(1i32);
}
impl windows_core::TypeKind for SpatialLookDirectionRange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialLookDirectionRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialLookDirectionRange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialLookDirectionRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLookDirectionRange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: Self = Self(0i32);
    pub const Bounded: Self = Self(1i32);
}
impl windows_core::TypeKind for SpatialMovementRange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialMovementRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialMovementRange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialMovementRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialMovementRange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl windows_core::TypeKind for SpatialPerceptionAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialPerceptionAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialPerceptionAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialPerceptionAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialPerceptionAccessStatus;i4)");
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialBoundingBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for SpatialBoundingBox {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for SpatialBoundingBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for SpatialBoundingBox {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialBoundingFrustum {
    pub Near: super::super::Foundation::Numerics::Plane,
    pub Far: super::super::Foundation::Numerics::Plane,
    pub Right: super::super::Foundation::Numerics::Plane,
    pub Left: super::super::Foundation::Numerics::Plane,
    pub Top: super::super::Foundation::Numerics::Plane,
    pub Bottom: super::super::Foundation::Numerics::Plane,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for SpatialBoundingFrustum {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for SpatialBoundingFrustum {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingFrustum;struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for SpatialBoundingFrustum {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialBoundingOrientedBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for SpatialBoundingOrientedBox {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for SpatialBoundingOrientedBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingOrientedBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for SpatialBoundingOrientedBox {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialBoundingSphere {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for SpatialBoundingSphere {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for SpatialBoundingSphere {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingSphere;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialRay {
    pub Origin: super::super::Foundation::Numerics::Vector3,
    pub Direction: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for SpatialRay {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for SpatialRay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialRay;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for SpatialRay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

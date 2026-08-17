#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[cfg(feature = "Devices_Geolocation_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(ICivicAddress, ICivicAddress_Vtbl, 0xa8567a1a_64f4_4d48_bcea_f6b008eca34c);
impl windows_core::RuntimeType for ICivicAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICivicAddress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Country: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeoboundingBox, IGeoboundingBox_Vtbl, 0x0896c80b_274f_43da_9a06_cbfcdaeb4ec2);
impl windows_core::RuntimeType for IGeoboundingBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeoboundingBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NorthwestCorner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BasicGeoposition) -> windows_core::HRESULT,
    pub SoutheastCorner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BasicGeoposition) -> windows_core::HRESULT,
    pub Center: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BasicGeoposition) -> windows_core::HRESULT,
    pub MinAltitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MaxAltitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeoboundingBoxFactory, IGeoboundingBoxFactory_Vtbl, 0x4dfba589_0411_4abc_b3b5_5bbccb57d98c);
impl windows_core::RuntimeType for IGeoboundingBoxFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeoboundingBoxFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, BasicGeoposition, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAltitudeReference: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, BasicGeoposition, AltitudeReferenceSystem, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, BasicGeoposition, AltitudeReferenceSystem, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeoboundingBoxStatics, IGeoboundingBoxStatics_Vtbl, 0x67b80708_e61a_4cd0_841b_93233792b5ca);
impl windows_core::RuntimeType for IGeoboundingBoxStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeoboundingBoxStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TryCompute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryCompute: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AltitudeReferenceSystem, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AltitudeReferenceSystem, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReferenceAndSpatialReference: usize,
}
windows_core::imp::define_interface!(IGeocircle, IGeocircle_Vtbl, 0x39e45843_a7f9_4e63_92a7_ba0c28d124b1);
impl windows_core::RuntimeType for IGeocircle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocircle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Center: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BasicGeoposition) -> windows_core::HRESULT,
    pub Radius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocircleFactory, IGeocircleFactory_Vtbl, 0xafd6531f_72b1_4f7d_87cc_4ed4c9849c05);
impl windows_core::RuntimeType for IGeocircleFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocircleFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, f64, AltitudeReferenceSystem, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, f64, AltitudeReferenceSystem, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinate, IGeocoordinate_Vtbl, 0xee21a3aa_976a_4c70_803d_083ea55bcbc4);
impl windows_core::RuntimeType for IGeocoordinate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Latitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Latitude: usize,
    #[cfg(feature = "deprecated")]
    pub Longitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Longitude: usize,
    #[cfg(feature = "deprecated")]
    pub Altitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Altitude: usize,
    pub Accuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AltitudeAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Heading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Speed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinateSatelliteData, IGeocoordinateSatelliteData_Vtbl, 0xc32a74d9_2608_474c_912c_06dd490f4af7);
impl windows_core::RuntimeType for IGeocoordinateSatelliteData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinateSatelliteData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PositionDilutionOfPrecision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HorizontalDilutionOfPrecision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VerticalDilutionOfPrecision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinateSatelliteData2, IGeocoordinateSatelliteData2_Vtbl, 0x761c8cfd_a19d_5a51_80f5_71676115483e);
impl windows_core::RuntimeType for IGeocoordinateSatelliteData2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinateSatelliteData2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GeometricDilutionOfPrecision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimeDilutionOfPrecision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinateWithPoint, IGeocoordinateWithPoint_Vtbl, 0xfeea0525_d22c_4d46_b527_0b96066fc7db);
impl windows_core::RuntimeType for IGeocoordinateWithPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinateWithPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinateWithPositionData, IGeocoordinateWithPositionData_Vtbl, 0x95e634be_dbd6_40ac_b8f2_a65c0340d9a6);
impl windows_core::RuntimeType for IGeocoordinateWithPositionData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinateWithPositionData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PositionSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PositionSource) -> windows_core::HRESULT,
    pub SatelliteData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinateWithPositionSourceTimestamp, IGeocoordinateWithPositionSourceTimestamp_Vtbl, 0x8543fc02_c9f1_4610_afe0_8bc3a6a87036);
impl windows_core::RuntimeType for IGeocoordinateWithPositionSourceTimestamp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinateWithPositionSourceTimestamp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PositionSourceTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeocoordinateWithRemoteSource, IGeocoordinateWithRemoteSource_Vtbl, 0x397cebd7_ee38_5f3b_8900_c4a7bc9cf953);
impl windows_core::RuntimeType for IGeocoordinateWithRemoteSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeocoordinateWithRemoteSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRemoteSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeolocator, IGeolocator_Vtbl, 0xa9c3bf62_4524_4989_8aa9_de019d2e551f);
impl windows_core::RuntimeType for IGeolocator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeolocator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PositionAccuracy) -> windows_core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, PositionAccuracy) -> windows_core::HRESULT,
    pub MovementThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMovementThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub LocationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PositionStatus) -> windows_core::HRESULT,
    pub GetGeopositionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGeopositionAsyncWithAgeAndTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeolocator2, IGeolocator2_Vtbl, 0xd1b42e6d_8891_43b4_ad36_27c6fe9a97b1);
impl windows_core::RuntimeType for IGeolocator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeolocator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowFallbackToConsentlessPositions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeolocatorStatics, IGeolocatorStatics_Vtbl, 0x9a8e7571_2df5_4591_9f87_eb5fd894e9b7);
impl windows_core::RuntimeType for IGeolocatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeolocatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryWithDurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryWithDurationAsync: usize,
}
windows_core::imp::define_interface!(IGeolocatorStatics2, IGeolocatorStatics2_Vtbl, 0x993011a2_fa1c_4631_a71d_0dbeb1250d9c);
impl windows_core::RuntimeType for IGeolocatorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeolocatorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDefaultGeopositionRecommended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDefaultGeoposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultGeoposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeolocatorWithScalarAccuracy, IGeolocatorWithScalarAccuracy_Vtbl, 0x96f5d3c1_b80f_460a_994d_a96c47a51aa4);
impl windows_core::RuntimeType for IGeolocatorWithScalarAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeolocatorWithScalarAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredAccuracyInMeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesiredAccuracyInMeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeopath, IGeopath_Vtbl, 0xe53fd7b9_2da4_4714_a652_de8593289898);
impl windows_core::RuntimeType for IGeopath {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeopath_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Positions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Positions: usize,
}
windows_core::imp::define_interface!(IGeopathFactory, IGeopathFactory_Vtbl, 0x27bea9c8_c7e7_4359_9b9b_fca3e05ef593);
impl windows_core::RuntimeType for IGeopathFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeopathFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AltitudeReferenceSystem, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AltitudeReferenceSystem, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReferenceAndSpatialReference: usize,
}
windows_core::imp::define_interface!(IGeopoint, IGeopoint_Vtbl, 0x6bfa00eb_e56e_49bb_9caf_cbaa78a8bcef);
impl windows_core::RuntimeType for IGeopoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeopoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BasicGeoposition) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeopointFactory, IGeopointFactory_Vtbl, 0xdb6b8d33_76bd_4e30_8af7_a844dc37b7a0);
impl windows_core::RuntimeType for IGeopointFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeopointFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, AltitudeReferenceSystem, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(*mut core::ffi::c_void, BasicGeoposition, AltitudeReferenceSystem, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeoposition, IGeoposition_Vtbl, 0xc18d0454_7d41_4ff7_a957_9dffb4ef7f5b);
impl windows_core::RuntimeType for IGeoposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeoposition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Coordinate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CivicAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeoposition2, IGeoposition2_Vtbl, 0x7f62f697_8671_4b0d_86f8_474a8496187c);
impl windows_core::RuntimeType for IGeoposition2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeoposition2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VenueData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeoshape, IGeoshape_Vtbl, 0xc99ca2af_c729_43c1_8fab_d6dec914df7e);
impl core::ops::Deref for IGeoshape {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGeoshape, windows_core::IUnknown, windows_core::IInspectable);
impl IGeoshape {
    pub fn GeoshapeType(&self) -> windows_core::Result<GeoshapeType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeoshapeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> windows_core::Result<AltitudeReferenceSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeReferenceSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IGeoshape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeoshape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GeoshapeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GeoshapeType) -> windows_core::HRESULT,
    pub SpatialReferenceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AltitudeReferenceSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AltitudeReferenceSystem) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeovisit, IGeovisit_Vtbl, 0xb1877a76_9ef6_41ab_a0dd_793ece76e2de);
impl windows_core::RuntimeType for IGeovisit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeovisit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VisitStateChange) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeovisitMonitor, IGeovisitMonitor_Vtbl, 0x80118aaf_5944_4591_83c1_396647f54f2c);
impl windows_core::RuntimeType for IGeovisitMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeovisitMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VisitMonitoringScope) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, VisitMonitoringScope) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VisitStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVisitStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeovisitMonitorStatics, IGeovisitMonitorStatics_Vtbl, 0xbcf976a7_bbf2_4cdd_95cf_554c82edfb87);
impl windows_core::RuntimeType for IGeovisitMonitorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeovisitMonitorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetLastReportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeovisitStateChangedEventArgs, IGeovisitStateChangedEventArgs_Vtbl, 0xceb4d1ff_8b53_4968_beed_4cecd029ce15);
impl windows_core::RuntimeType for IGeovisitStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeovisitStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Visit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeovisitTriggerDetails, IGeovisitTriggerDetails_Vtbl, 0xea770d9e_d1c9_454b_99b7_b2f8cdd2482f);
impl windows_core::RuntimeType for IGeovisitTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeovisitTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
windows_core::imp::define_interface!(IPositionChangedEventArgs, IPositionChangedEventArgs_Vtbl, 0x37859ce5_9d1e_46c5_bf3b_6ad8cac1a093);
impl windows_core::RuntimeType for IPositionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPositionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStatusChangedEventArgs, IStatusChangedEventArgs_Vtbl, 0x3453d2da_8c93_4111_a205_9aecfc9be5c0);
impl windows_core::RuntimeType for IStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PositionStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVenueData, IVenueData_Vtbl, 0x66f39187_60e3_4b2f_b527_4f53f1c3c677);
impl windows_core::RuntimeType for IVenueData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVenueData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CivicAddress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CivicAddress, windows_core::IUnknown, windows_core::IInspectable);
impl CivicAddress {
    pub fn Country(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Country)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn City(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).City)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PostalCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostalCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CivicAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICivicAddress>();
}
unsafe impl windows_core::Interface for CivicAddress {
    type Vtable = ICivicAddress_Vtbl;
    const IID: windows_core::GUID = <ICivicAddress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.CivicAddress";
}
unsafe impl Send for CivicAddress {}
unsafe impl Sync for CivicAddress {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GeoboundingBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeoboundingBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GeoboundingBox, IGeoshape);
impl GeoboundingBox {
    pub fn NorthwestCorner(&self) -> windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NorthwestCorner)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SoutheastCorner(&self) -> windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoutheastCorner)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Center(&self) -> windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Center)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinAltitude(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinAltitude)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxAltitude(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAltitude)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition) -> windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), northwestcorner, southeastcorner, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAltitudeReference(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReference)(windows_core::Interface::as_raw(this), northwestcorner, southeastcorner, altitudereferencesystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAltitudeReferenceAndSpatialReference(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(windows_core::Interface::as_raw(this), northwestcorner, southeastcorner, altitudereferencesystem, spatialreferenceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryCompute<P0>(positions: P0) -> windows_core::Result<GeoboundingBox>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCompute)(windows_core::Interface::as_raw(this), positions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReference<P0>(positions: P0, altituderefsystem: AltitudeReferenceSystem) -> windows_core::Result<GeoboundingBox>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryComputeWithAltitudeReference)(windows_core::Interface::as_raw(this), positions.param().abi(), altituderefsystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReferenceAndSpatialReference<P0>(positions: P0, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> windows_core::Result<GeoboundingBox>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryComputeWithAltitudeReferenceAndSpatialReference)(windows_core::Interface::as_raw(this), positions.param().abi(), altituderefsystem, spatialreferenceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GeoshapeType(&self) -> windows_core::Result<GeoshapeType> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeoshapeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> windows_core::Result<AltitudeReferenceSystem> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeReferenceSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeoboundingBoxFactory<R, F: FnOnce(&IGeoboundingBoxFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeoboundingBox, IGeoboundingBoxFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeoboundingBoxStatics<R, F: FnOnce(&IGeoboundingBoxStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeoboundingBox, IGeoboundingBoxStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GeoboundingBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeoboundingBox>();
}
unsafe impl windows_core::Interface for GeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
    const IID: windows_core::GUID = <IGeoboundingBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeoboundingBox";
}
unsafe impl Send for GeoboundingBox {}
unsafe impl Sync for GeoboundingBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geocircle(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geocircle, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Geocircle, IGeoshape);
impl Geocircle {
    pub fn Center(&self) -> windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Center)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Radius(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Radius)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(position: BasicGeoposition, radius: f64) -> windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), position, radius, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAltitudeReferenceSystem(position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystem)(windows_core::Interface::as_raw(this), position, radius, altitudereferencesystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(windows_core::Interface::as_raw(this), position, radius, altitudereferencesystem, spatialreferenceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GeoshapeType(&self) -> windows_core::Result<GeoshapeType> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeoshapeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> windows_core::Result<AltitudeReferenceSystem> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeReferenceSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeocircleFactory<R, F: FnOnce(&IGeocircleFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geocircle, IGeocircleFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Geocircle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeocircle>();
}
unsafe impl windows_core::Interface for Geocircle {
    type Vtable = IGeocircle_Vtbl;
    const IID: windows_core::GUID = <IGeocircle as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocircle";
}
unsafe impl Send for Geocircle {}
unsafe impl Sync for Geocircle {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geocoordinate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geocoordinate, windows_core::IUnknown, windows_core::IInspectable);
impl Geocoordinate {
    #[cfg(feature = "deprecated")]
    pub fn Latitude(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Latitude)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Longitude(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Longitude)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Altitude(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Altitude)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Accuracy(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Accuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeAccuracy(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeAccuracy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Heading(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Heading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Speed(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Speed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Point(&self) -> windows_core::Result<Geopoint> {
        let this = &windows_core::Interface::cast::<IGeocoordinateWithPoint>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Point)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PositionSource(&self) -> windows_core::Result<PositionSource> {
        let this = &windows_core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SatelliteData(&self) -> windows_core::Result<GeocoordinateSatelliteData> {
        let this = &windows_core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SatelliteData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PositionSourceTimestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IGeocoordinateWithPositionSourceTimestamp>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionSourceTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRemoteSource(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGeocoordinateWithRemoteSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRemoteSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for Geocoordinate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeocoordinate>();
}
unsafe impl windows_core::Interface for Geocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
    const IID: windows_core::GUID = <IGeocoordinate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocoordinate";
}
unsafe impl Send for Geocoordinate {}
unsafe impl Sync for Geocoordinate {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GeocoordinateSatelliteData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeocoordinateSatelliteData, windows_core::IUnknown, windows_core::IInspectable);
impl GeocoordinateSatelliteData {
    pub fn PositionDilutionOfPrecision(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionDilutionOfPrecision)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HorizontalDilutionOfPrecision(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalDilutionOfPrecision)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VerticalDilutionOfPrecision(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalDilutionOfPrecision)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeometricDilutionOfPrecision(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeometricDilutionOfPrecision)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimeDilutionOfPrecision(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeDilutionOfPrecision)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GeocoordinateSatelliteData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeocoordinateSatelliteData>();
}
unsafe impl windows_core::Interface for GeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
    const IID: windows_core::GUID = <IGeocoordinateSatelliteData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeocoordinateSatelliteData";
}
unsafe impl Send for GeocoordinateSatelliteData {}
unsafe impl Sync for GeocoordinateSatelliteData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geolocator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geolocator, windows_core::IUnknown, windows_core::IInspectable);
impl Geolocator {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geolocator, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DesiredAccuracy(&self) -> windows_core::Result<PositionAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredAccuracy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MovementThreshold(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MovementThreshold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMovementThreshold(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMovementThreshold)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LocationStatus(&self) -> windows_core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetGeopositionAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGeopositionAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetGeopositionAsyncWithAgeAndTimeout(&self, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGeopositionAsyncWithAgeAndTimeout)(windows_core::Interface::as_raw(this), maximumage, timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PositionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePositionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AllowFallbackToConsentlessPositions(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGeolocator2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AllowFallbackToConsentlessPositions)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGeopositionHistoryAsync(starttime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGeopositionHistoryAsync)(windows_core::Interface::as_raw(this), starttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGeopositionHistoryWithDurationAsync(starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGeopositionHistoryWithDurationAsync)(windows_core::Interface::as_raw(this), starttime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsDefaultGeopositionRecommended() -> windows_core::Result<bool> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDefaultGeopositionRecommended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetDefaultGeoposition<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<BasicGeoposition>>,
    {
        Self::IGeolocatorStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetDefaultGeoposition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn DefaultGeoposition() -> windows_core::Result<super::super::Foundation::IReference<BasicGeoposition>> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultGeoposition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DesiredAccuracyInMeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &windows_core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredAccuracyInMeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredAccuracyInMeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredAccuracyInMeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IGeolocatorStatics<R, F: FnOnce(&IGeolocatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geolocator, IGeolocatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeolocatorStatics2<R, F: FnOnce(&IGeolocatorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geolocator, IGeolocatorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Geolocator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeolocator>();
}
unsafe impl windows_core::Interface for Geolocator {
    type Vtable = IGeolocator_Vtbl;
    const IID: windows_core::GUID = <IGeolocator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geolocator";
}
unsafe impl Send for Geolocator {}
unsafe impl Sync for Geolocator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geopath(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geopath, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Geopath, IGeoshape);
impl Geopath {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Positions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Positions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(positions: P0) -> windows_core::Result<Geopath>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), positions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReference<P0>(positions: P0, altitudereferencesystem: AltitudeReferenceSystem) -> windows_core::Result<Geopath>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReference)(windows_core::Interface::as_raw(this), positions.param().abi(), altitudereferencesystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<P0>(positions: P0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> windows_core::Result<Geopath>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(windows_core::Interface::as_raw(this), positions.param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GeoshapeType(&self) -> windows_core::Result<GeoshapeType> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeoshapeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> windows_core::Result<AltitudeReferenceSystem> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeReferenceSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeopathFactory<R, F: FnOnce(&IGeopathFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geopath, IGeopathFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Geopath {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeopath>();
}
unsafe impl windows_core::Interface for Geopath {
    type Vtable = IGeopath_Vtbl;
    const IID: windows_core::GUID = <IGeopath as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopath";
}
unsafe impl Send for Geopath {}
unsafe impl Sync for Geopath {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geopoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geopoint, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Geopoint, IGeoshape);
impl Geopoint {
    pub fn Position(&self) -> windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(position: BasicGeoposition) -> windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAltitudeReferenceSystem(position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystem)(windows_core::Interface::as_raw(this), position, altitudereferencesystem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(windows_core::Interface::as_raw(this), position, altitudereferencesystem, spatialreferenceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GeoshapeType(&self) -> windows_core::Result<GeoshapeType> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeoshapeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> windows_core::Result<AltitudeReferenceSystem> {
        let this = &windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeReferenceSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeopointFactory<R, F: FnOnce(&IGeopointFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geopoint, IGeopointFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Geopoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeopoint>();
}
unsafe impl windows_core::Interface for Geopoint {
    type Vtable = IGeopoint_Vtbl;
    const IID: windows_core::GUID = <IGeopoint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopoint";
}
unsafe impl Send for Geopoint {}
unsafe impl Sync for Geopoint {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geoposition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geoposition, windows_core::IUnknown, windows_core::IInspectable);
impl Geoposition {
    pub fn Coordinate(&self) -> windows_core::Result<Geocoordinate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Coordinate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CivicAddress(&self) -> windows_core::Result<CivicAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CivicAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VenueData(&self) -> windows_core::Result<VenueData> {
        let this = &windows_core::Interface::cast::<IGeoposition2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VenueData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Geoposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeoposition>();
}
unsafe impl windows_core::Interface for Geoposition {
    type Vtable = IGeoposition_Vtbl;
    const IID: windows_core::GUID = <IGeoposition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geoposition";
}
unsafe impl Send for Geoposition {}
unsafe impl Sync for Geoposition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Geovisit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geovisit, windows_core::IUnknown, windows_core::IInspectable);
impl Geovisit {
    pub fn Position(&self) -> windows_core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StateChange(&self) -> windows_core::Result<VisitStateChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for Geovisit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeovisit>();
}
unsafe impl windows_core::Interface for Geovisit {
    type Vtable = IGeovisit_Vtbl;
    const IID: windows_core::GUID = <IGeovisit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geovisit";
}
unsafe impl Send for Geovisit {}
unsafe impl Sync for Geovisit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GeovisitMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeovisitMonitor, windows_core::IUnknown, windows_core::IInspectable);
impl GeovisitMonitor {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeovisitMonitor, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MonitoringScope(&self) -> windows_core::Result<VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonitoringScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self, value: VisitMonitoringScope) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn VisitStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisitStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVisitStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVisitStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetLastReportAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<Geovisit>> {
        Self::IGeovisitMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLastReportAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGeovisitMonitorStatics<R, F: FnOnce(&IGeovisitMonitorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeovisitMonitor, IGeovisitMonitorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GeovisitMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeovisitMonitor>();
}
unsafe impl windows_core::Interface for GeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
    const IID: windows_core::GUID = <IGeovisitMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitMonitor";
}
unsafe impl Send for GeovisitMonitor {}
unsafe impl Sync for GeovisitMonitor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GeovisitStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeovisitStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GeovisitStateChangedEventArgs {
    pub fn Visit(&self) -> windows_core::Result<Geovisit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visit)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GeovisitStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeovisitStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for GeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGeovisitStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
}
unsafe impl Send for GeovisitStateChangedEventArgs {}
unsafe impl Sync for GeovisitStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GeovisitTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeovisitTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl GeovisitTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadReports)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GeovisitTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeovisitTriggerDetails>();
}
unsafe impl windows_core::Interface for GeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IGeovisitTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitTriggerDetails";
}
unsafe impl Send for GeovisitTriggerDetails {}
unsafe impl Sync for GeovisitTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PositionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PositionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PositionChangedEventArgs {
    pub fn Position(&self) -> windows_core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PositionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPositionChangedEventArgs>();
}
unsafe impl windows_core::Interface for PositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPositionChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.PositionChangedEventArgs";
}
unsafe impl Send for PositionChangedEventArgs {}
unsafe impl Sync for PositionChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl StatusChangedEventArgs {
    pub fn Status(&self) -> windows_core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for StatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for StatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.StatusChangedEventArgs";
}
unsafe impl Send for StatusChangedEventArgs {}
unsafe impl Sync for StatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VenueData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VenueData, windows_core::IUnknown, windows_core::IInspectable);
impl VenueData {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Level(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Level)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VenueData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVenueData>();
}
unsafe impl windows_core::Interface for VenueData {
    type Vtable = IVenueData_Vtbl;
    const IID: windows_core::GUID = <IVenueData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.VenueData";
}
unsafe impl Send for VenueData {}
unsafe impl Sync for VenueData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: Self = Self(0i32);
    pub const Terrain: Self = Self(1i32);
    pub const Ellipsoid: Self = Self(2i32);
    pub const Geoid: Self = Self(3i32);
    pub const Surface: Self = Self(4i32);
}
impl windows_core::TypeKind for AltitudeReferenceSystem {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AltitudeReferenceSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AltitudeReferenceSystem").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AltitudeReferenceSystem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.AltitudeReferenceSystem;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl windows_core::TypeKind for GeolocationAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GeolocationAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GeolocationAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GeolocationAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeolocationAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
}
impl windows_core::TypeKind for GeoshapeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GeoshapeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GeoshapeType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GeoshapeType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeoshapeType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for PositionAccuracy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PositionAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PositionAccuracy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PositionAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionAccuracy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: Self = Self(0i32);
    pub const Satellite: Self = Self(1i32);
    pub const WiFi: Self = Self(2i32);
    pub const IPAddress: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const Default: Self = Self(5i32);
    pub const Obfuscated: Self = Self(6i32);
}
impl windows_core::TypeKind for PositionSource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PositionSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PositionSource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PositionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionSource;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl windows_core::TypeKind for PositionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PositionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PositionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PositionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
impl windows_core::TypeKind for VisitMonitoringScope {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VisitMonitoringScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VisitMonitoringScope").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VisitMonitoringScope {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitMonitoringScope;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}
impl windows_core::TypeKind for VisitStateChange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VisitStateChange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VisitStateChange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VisitStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitStateChange;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl windows_core::TypeKind for BasicGeoposition {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BasicGeoposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Geolocation.BasicGeoposition;f8;f8;f8)");
}
impl Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
windows_core::imp::define_interface!(IAccelerometer, IAccelerometer_Vtbl, 0xdf184548_2711_4da7_8098_4b82205d3c7d);
impl windows_core::RuntimeType for IAccelerometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Shaken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShaken: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometer2, IAccelerometer2_Vtbl, 0xe8f092ee_4964_401a_b602_220d7153c60a);
impl windows_core::RuntimeType for IAccelerometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(IAccelerometer3, IAccelerometer3_Vtbl, 0x87e0022a_ed80_49eb_bf8a_a4ea31e5cd84);
impl windows_core::RuntimeType for IAccelerometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometer4, IAccelerometer4_Vtbl, 0x1d373c4f_42d3_45b2_8144_ab7fb665eb59);
impl windows_core::RuntimeType for IAccelerometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AccelerometerReadingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometer5, IAccelerometer5_Vtbl, 0x7e7e7021_def4_53a6_af43_806fd538edf6);
impl windows_core::RuntimeType for IAccelerometer5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometer5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerDataThreshold, IAccelerometerDataThreshold_Vtbl, 0xf92c1b68_6320_5577_879e_9942621c3dd9);
impl windows_core::RuntimeType for IAccelerometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetXAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub YAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetYAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ZAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetZAxisInGForce: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerDeviceId, IAccelerometerDeviceId_Vtbl, 0x7eac64a9_97d5_446d_ab5a_917df9b96a2c);
impl windows_core::RuntimeType for IAccelerometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerReading, IAccelerometerReading_Vtbl, 0xb9fe7acb_d351_40af_8bb6_7aa9ae641fb7);
impl windows_core::RuntimeType for IAccelerometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AccelerationX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AccelerationY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AccelerationZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerReading2, IAccelerometerReading2_Vtbl, 0x0a864aa2_15ae_4a40_be55_db58d7de7389);
impl windows_core::RuntimeType for IAccelerometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IAccelerometerReadingChangedEventArgs, IAccelerometerReadingChangedEventArgs_Vtbl, 0x0095c65b_b6ac_475a_9f44_8b32d35a3f25);
impl windows_core::RuntimeType for IAccelerometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerShakenEventArgs, IAccelerometerShakenEventArgs_Vtbl, 0x95ff01d1_4a28_4f35_98e8_8178aae4084a);
impl windows_core::RuntimeType for IAccelerometerShakenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerShakenEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerStatics, IAccelerometerStatics_Vtbl, 0xa5e28b74_5a87_4a2d_becc_0f906ea061dd);
impl windows_core::RuntimeType for IAccelerometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerStatics2, IAccelerometerStatics2_Vtbl, 0xc4c4842f_d86b_4685_b2d7_3396f798d57b);
impl windows_core::RuntimeType for IAccelerometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultWithAccelerometerReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, AccelerometerReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccelerometerStatics3, IAccelerometerStatics3_Vtbl, 0x9de218cf_455d_4cf3_8200_70e1410340f8);
impl windows_core::RuntimeType for IAccelerometerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccelerometerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, AccelerometerReadingType, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensor, IActivitySensor_Vtbl, 0xcd7a630c_fb5f_48eb_b09b_a2708d1c61ef);
impl windows_core::RuntimeType for IActivitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReadingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SubscribedActivities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubscribedActivities: usize,
    pub PowerInMilliwatts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedActivities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorReading, IActivitySensorReading_Vtbl, 0x85125a96_1472_40a2_b2ae_e1ef29226c78);
impl windows_core::RuntimeType for IActivitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Activity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivityType) -> windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivitySensorReadingConfidence) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorReadingChangeReport, IActivitySensorReadingChangeReport_Vtbl, 0x4f3c2915_d93b_47bd_960a_f20fb2f322b9);
impl windows_core::RuntimeType for IActivitySensorReadingChangeReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorReadingChangeReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorReadingChangedEventArgs, IActivitySensorReadingChangedEventArgs_Vtbl, 0xde386717_aeb6_4ec7_946a_d9cc19b951ec);
impl windows_core::RuntimeType for IActivitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorStatics, IActivitySensorStatics_Vtbl, 0xa71e0e9d_ee8b_45d1_b25b_08cc0df92ab6);
impl windows_core::RuntimeType for IActivitySensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
windows_core::imp::define_interface!(IActivitySensorTriggerDetails, IActivitySensorTriggerDetails_Vtbl, 0x2c9e6612_b9ca_4677_b263_243297f79d3a);
impl windows_core::RuntimeType for IActivitySensorTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
windows_core::imp::define_interface!(IAdaptiveDimmingOptions, IAdaptiveDimmingOptions_Vtbl, 0xd3213cf7_89b5_5732_b2a0_aefe324f54e6);
impl windows_core::RuntimeType for IAdaptiveDimmingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveDimmingOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeter, IAltimeter_Vtbl, 0x72f057fd_8f04_49f1_b4a7_f4e363b701a2);
impl windows_core::RuntimeType for IAltimeter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAltimeter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeter2, IAltimeter2_Vtbl, 0xc9471bf9_2add_48f5_9f08_3d0c7660d938);
impl windows_core::RuntimeType for IAltimeter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAltimeter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterReading, IAltimeterReading_Vtbl, 0xfbe8ef73_7f5e_48c8_aa1a_f1f3befc1144);
impl windows_core::RuntimeType for IAltimeterReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAltimeterReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AltitudeChangeInMeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterReading2, IAltimeterReading2_Vtbl, 0x543a1bd9_6d0b_42b2_bd69_bc8fae0f782c);
impl windows_core::RuntimeType for IAltimeterReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAltimeterReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IAltimeterReadingChangedEventArgs, IAltimeterReadingChangedEventArgs_Vtbl, 0x7069d077_446d_47f7_998c_ebc23b45e4a2);
impl windows_core::RuntimeType for IAltimeterReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAltimeterReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAltimeterStatics, IAltimeterStatics_Vtbl, 0x9eb4d7c3_e5ac_47ce_8eef_d3718168c01f);
impl windows_core::RuntimeType for IAltimeterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAltimeterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometer, IBarometer_Vtbl, 0x934475a8_78bf_452f_b017_f0209ce6dab4);
impl windows_core::RuntimeType for IBarometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometer2, IBarometer2_Vtbl, 0x32bcc418_3eeb_4d04_9574_7633a8781f9f);
impl windows_core::RuntimeType for IBarometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometer3, IBarometer3_Vtbl, 0x0e35f0ea_02b5_5a04_b03d_822084863a54);
impl windows_core::RuntimeType for IBarometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerDataThreshold, IBarometerDataThreshold_Vtbl, 0x076b952c_cb62_5a90_a0d1_f85e4a936394);
impl windows_core::RuntimeType for IBarometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Hectopascals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHectopascals: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerReading, IBarometerReading_Vtbl, 0xf5b9d2e6_1df6_4a1a_a7ad_321d4f5db247);
impl windows_core::RuntimeType for IBarometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub StationPressureInHectopascals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerReading2, IBarometerReading2_Vtbl, 0x85a244eb_90c5_4875_891c_3865b4c357e7);
impl windows_core::RuntimeType for IBarometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IBarometerReadingChangedEventArgs, IBarometerReadingChangedEventArgs_Vtbl, 0x3d84945f_037b_404f_9bbb_6232d69543c3);
impl windows_core::RuntimeType for IBarometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerStatics, IBarometerStatics_Vtbl, 0x286b270a_02e3_4f86_84fc_fdd892b5940f);
impl windows_core::RuntimeType for IBarometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBarometerStatics2, IBarometerStatics2_Vtbl, 0x8fc6b1e7_95ff_44ac_878e_d65c8308c34c);
impl windows_core::RuntimeType for IBarometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBarometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompass, ICompass_Vtbl, 0x292ffa94_1b45_403c_ba06_b106dba69a64);
impl windows_core::RuntimeType for ICompass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompass2, ICompass2_Vtbl, 0x36f26d09_c7d7_434f_b461_979ddfc2322f);
impl windows_core::RuntimeType for ICompass2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompass2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(ICompass3, ICompass3_Vtbl, 0xa424801b_c5ea_4d45_a0ec_4b791f041a89);
impl windows_core::RuntimeType for ICompass3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompass3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompass4, ICompass4_Vtbl, 0x291e7f11_ec32_5dcc_bfcb_0bb39eba5774);
impl windows_core::RuntimeType for ICompass4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompass4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassDataThreshold, ICompassDataThreshold_Vtbl, 0xd15b52b3_d39d_5ec8_b2e4_f193e6ab34ed);
impl windows_core::RuntimeType for ICompassDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Degrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassDeviceId, ICompassDeviceId_Vtbl, 0xd181ca29_b085_4b1d_870a_4ff57ba74fd4);
impl windows_core::RuntimeType for ICompassDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReading, ICompassReading_Vtbl, 0x82911128_513d_4dc9_b781_5eedfbf02d0c);
impl windows_core::RuntimeType for ICompassReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub HeadingMagneticNorth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HeadingTrueNorth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReading2, ICompassReading2_Vtbl, 0xb13a661e_51bb_4a12_bedd_ad47ff87d2e8);
impl windows_core::RuntimeType for ICompassReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(ICompassReadingChangedEventArgs, ICompassReadingChangedEventArgs_Vtbl, 0x8f1549b0_e8bc_4c7e_b009_4e41df137072);
impl windows_core::RuntimeType for ICompassReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassReadingHeadingAccuracy, ICompassReadingHeadingAccuracy_Vtbl, 0xe761354e_8911_40f7_9e16_6ecc7daec5de);
impl windows_core::RuntimeType for ICompassReadingHeadingAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassReadingHeadingAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeadingAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassStatics, ICompassStatics_Vtbl, 0x9abc97df_56ec_4c25_b54d_40a68bb5b269);
impl windows_core::RuntimeType for ICompassStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompassStatics2, ICompassStatics2_Vtbl, 0x0ace0ead_3baa_4990_9ce4_be0913754ed2);
impl windows_core::RuntimeType for ICompassStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompassStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDetectedPerson, IDetectedPerson_Vtbl, 0x168cc0d9_3f05_5029_a0bf_cdcab4be3f9e);
impl windows_core::RuntimeType for IDetectedPerson {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDetectedPerson_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Engagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanEngagement) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HeadOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HeadPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PersonId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometer, IGyrometer_Vtbl, 0xfdb9a9c4_84b1_4ca2_9763_9b589506c70c);
impl windows_core::RuntimeType for IGyrometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometer2, IGyrometer2_Vtbl, 0x63df2443_8ce8_41c3_ac44_8698810b557f);
impl windows_core::RuntimeType for IGyrometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(IGyrometer3, IGyrometer3_Vtbl, 0x5d6f88d5_8fbc_4484_914b_528adfd947b1);
impl windows_core::RuntimeType for IGyrometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometer4, IGyrometer4_Vtbl, 0x0628a60c_4c4b_5096_94e6_c356df68bef7);
impl windows_core::RuntimeType for IGyrometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerDataThreshold, IGyrometerDataThreshold_Vtbl, 0x8648b31e_6e52_5259_bbad_242a69dc38c8);
impl windows_core::RuntimeType for IGyrometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetXAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub YAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetYAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ZAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetZAxisInDegreesPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerDeviceId, IGyrometerDeviceId_Vtbl, 0x1ee5e978_89a2_4275_9e95_7126f4708760);
impl windows_core::RuntimeType for IGyrometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerReading, IGyrometerReading_Vtbl, 0xb3d6de5c_1ee4_456f_9de7_e2493b5c8e03);
impl windows_core::RuntimeType for IGyrometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AngularVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AngularVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AngularVelocityZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerReading2, IGyrometerReading2_Vtbl, 0x16afe13c_2b89_44bb_822b_d1e1556ff09b);
impl windows_core::RuntimeType for IGyrometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IGyrometerReadingChangedEventArgs, IGyrometerReadingChangedEventArgs_Vtbl, 0x0fdf1895_6f9e_42ce_8d58_388c0ab8356d);
impl windows_core::RuntimeType for IGyrometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerStatics, IGyrometerStatics_Vtbl, 0x83b6e7c9_e49d_4b39_86e6_cd554be4c5c1);
impl windows_core::RuntimeType for IGyrometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGyrometerStatics2, IGyrometerStatics2_Vtbl, 0xef83f7a1_d700_4204_9613_79c6b161df4e);
impl windows_core::RuntimeType for IGyrometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGyrometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHeadOrientation, IHeadOrientation_Vtbl, 0x519f54a9_513e_55e8_9c35_3e8da21dee69);
impl windows_core::RuntimeType for IHeadOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHeadOrientation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RollInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PitchInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHeadPosition, IHeadPosition_Vtbl, 0x585aeb65_cf35_5e6d_a76a_37db131e17de);
impl windows_core::RuntimeType for IHeadPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHeadPosition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AzimuthInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AltitudeInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleReading, IHingeAngleReading_Vtbl, 0xa3cd45b9_1bf1_4f65_a704_e2da04f182c0);
impl windows_core::RuntimeType for IHingeAngleReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHingeAngleReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AngleInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IHingeAngleSensor, IHingeAngleSensor_Vtbl, 0xe9d3be02_bfdf_437f_8c29_88c77393d309);
impl windows_core::RuntimeType for IHingeAngleSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHingeAngleSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReadingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MinReportThresholdInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ReportThresholdInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetReportThresholdInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleSensorReadingChangedEventArgs, IHingeAngleSensorReadingChangedEventArgs_Vtbl, 0x24d9558b_fad0_42b8_a854_78923049a1ba);
impl windows_core::RuntimeType for IHingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHingeAngleSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHingeAngleSensorStatics, IHingeAngleSensorStatics_Vtbl, 0xb7b63910_fbb1_4123_89ce_4ea34eb0dfca);
impl windows_core::RuntimeType for IHingeAngleSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHingeAngleSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelatedToAdjacentPanelsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceFeatures, IHumanPresenceFeatures_Vtbl, 0xbdb09fda_3244_557a_bd29_8b004f59f2cc);
impl windows_core::RuntimeType for IHumanPresenceFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceFeatures_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SensorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedWakeOrLockDistancesInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedWakeOrLockDistancesInMillimeters: usize,
    pub IsWakeOnApproachSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLockOnLeaveSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsAttentionAwareDimmingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsAttentionAwareDimmingSupported: usize,
}
windows_core::imp::define_interface!(IHumanPresenceFeatures2, IHumanPresenceFeatures2_Vtbl, 0x08a9cdda_d929_5ec2_81e2_940bafa089cf);
impl windows_core::RuntimeType for IHumanPresenceFeatures2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceFeatures2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAdaptiveDimmingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensor, IHumanPresenceSensor_Vtbl, 0x2116788b_e389_5cc3_9a97_cb17be1008bd);
impl windows_core::RuntimeType for IHumanPresenceSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MaxDetectableDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinDetectableDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensor2, IHumanPresenceSensor2_Vtbl, 0xf8833779_65fe_541a_b9d6_1e474a485e7a);
impl windows_core::RuntimeType for IHumanPresenceSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPresenceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEngagementSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensor3, IHumanPresenceSensor3_Vtbl, 0x963f006d_090d_532c_9eaf_803a9f69285b);
impl windows_core::RuntimeType for IHumanPresenceSensor3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensor3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxDetectablePersons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinDetectableAzimuthInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDetectableAzimuthInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinDetectableAltitudeInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDetectableAltitudeInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorExtension, IHumanPresenceSensorExtension_Vtbl, 0x3e526a71_2d1d_5d43_8a8e_a434a8242ef0);
impl core::ops::Deref for IHumanPresenceSensorExtension {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHumanPresenceSensorExtension, windows_core::IUnknown, windows_core::IInspectable);
impl IHumanPresenceSensorExtension {
    pub fn Initialize(&self, deviceinterface: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Initialize)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceinterface)).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ProcessReading<P0>(&self, reading: P0) -> windows_core::Result<HumanPresenceSensorReadingUpdate>
    where
        P0: windows_core::Param<HumanPresenceSensorReading>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessReading)(windows_core::Interface::as_raw(this), reading.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessReadingTimeoutExpired<P0>(&self, reading: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HumanPresenceSensorReading>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessReadingTimeoutExpired)(windows_core::Interface::as_raw(this), reading.param().abi()).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Uninitialize(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Uninitialize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IHumanPresenceSensorExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorExtension_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessReadingTimeoutExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReading, IHumanPresenceSensorReading_Vtbl, 0x83533bf5_a85a_5d50_8be4_6072d745a3bb);
impl windows_core::RuntimeType for IHumanPresenceSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Presence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanPresence) -> windows_core::HRESULT,
    pub Engagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanEngagement) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReading2, IHumanPresenceSensorReading2_Vtbl, 0xc4f0e950_3bff_53d6_a0f8_514ea3705c66);
impl windows_core::RuntimeType for IHumanPresenceSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReading3, IHumanPresenceSensorReading3_Vtbl, 0xb876d918_f069_586f_90e3_7c6fa5c5d33a);
impl windows_core::RuntimeType for IHumanPresenceSensorReading3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorReading3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnlookerPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HumanPresence) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DetectedPersons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DetectedPersons: usize,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReadingChangedEventArgs, IHumanPresenceSensorReadingChangedEventArgs_Vtbl, 0xa9dc4583_fd69_5c5e_ab1f_942204eae2db);
impl windows_core::RuntimeType for IHumanPresenceSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorReadingUpdate, IHumanPresenceSensorReadingUpdate_Vtbl, 0x42419c77_6d2f_55a0_9e01_c9cbe7b2d6df);
impl windows_core::RuntimeType for IHumanPresenceSensorReadingUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorReadingUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Presence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Engagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEngagement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorStatics, IHumanPresenceSensorStatics_Vtbl, 0x2ae89842_dba9_56b2_9f27_eac69d621004);
impl windows_core::RuntimeType for IHumanPresenceSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSensorStatics2, IHumanPresenceSensorStatics2_Vtbl, 0x5de35843_d260_5a87_995e_ace91326e1c4);
impl windows_core::RuntimeType for IHumanPresenceSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSettings, IHumanPresenceSettings_Vtbl, 0xef4daf5b_07b7_5eb6_86bb_b7ff49ce44fb);
impl windows_core::RuntimeType for IHumanPresenceSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SensorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSensorId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsWakeOnApproachEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsWakeOnApproachEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WakeOnApproachDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWakeOnApproachDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLockOnLeaveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsLockOnLeaveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub LockOnLeaveDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLockOnLeaveDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockOnLeaveTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetLockOnLeaveTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsAttentionAwareDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsAttentionAwareDimmingEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsAttentionAwareDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsAttentionAwareDimmingEnabled: usize,
}
windows_core::imp::define_interface!(IHumanPresenceSettings2, IHumanPresenceSettings2_Vtbl, 0xa26f705e_8696_5eb4_b9e1_26a508de1cd4);
impl windows_core::RuntimeType for IHumanPresenceSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAdaptiveDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAdaptiveDimmingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WakeOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DimmingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHumanPresenceSettingsStatics, IHumanPresenceSettingsStatics_Vtbl, 0x7f343202_e010_52c4_af0c_04a8f1e033da);
impl windows_core::RuntimeType for IHumanPresenceSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHumanPresenceSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFeaturesForSensorIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedFeaturesForSensorId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedLockOnLeaveTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedLockOnLeaveTimeouts: usize,
    pub SettingsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSettingsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer, IInclinometer_Vtbl, 0x2648ca6f_2286_406f_9161_f0c4bd806ebf);
impl windows_core::RuntimeType for IInclinometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer2, IInclinometer2_Vtbl, 0x029f3393_28b2_45f8_bb16_61e86a7fae6e);
impl windows_core::RuntimeType for IInclinometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorReadingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer3, IInclinometer3_Vtbl, 0x3a095004_d765_4384_a3d7_0283f3abe6ae);
impl windows_core::RuntimeType for IInclinometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometer4, IInclinometer4_Vtbl, 0x43852618_8fca_548e_bbf5_5c50412b6aa4);
impl windows_core::RuntimeType for IInclinometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerDataThreshold, IInclinometerDataThreshold_Vtbl, 0xf80a4783_7bfe_545e_bb60_a0ebc47bd2fb);
impl windows_core::RuntimeType for IInclinometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PitchInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPitchInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub RollInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRollInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub YawInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetYawInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerDeviceId, IInclinometerDeviceId_Vtbl, 0x01e91982_41ff_4406_ae83_62210ff16fe3);
impl windows_core::RuntimeType for IInclinometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReading, IInclinometerReading_Vtbl, 0x9f44f055_b6f6_497f_b127_1a775e501458);
impl windows_core::RuntimeType for IInclinometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PitchDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RollDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub YawDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReading2, IInclinometerReading2_Vtbl, 0x4f164781_e90b_4658_8915_0103e08a805a);
impl windows_core::RuntimeType for IInclinometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IInclinometerReadingChangedEventArgs, IInclinometerReadingChangedEventArgs_Vtbl, 0x4ae91dc1_e7eb_4938_8511_ae0d6b440438);
impl windows_core::RuntimeType for IInclinometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerReadingYawAccuracy, IInclinometerReadingYawAccuracy_Vtbl, 0xb453e880_1fe3_4986_a257_e6ece2723949);
impl windows_core::RuntimeType for IInclinometerReadingYawAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerReadingYawAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub YawAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics, IInclinometerStatics_Vtbl, 0xf22ec551_9c30_453a_8b49_3c3eeb33cb61);
impl windows_core::RuntimeType for IInclinometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics2, IInclinometerStatics2_Vtbl, 0x043f9775_6a1e_499c_86e0_638c1a864b00);
impl windows_core::RuntimeType for IInclinometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics3, IInclinometerStatics3_Vtbl, 0xbd9a4280_b91a_4829_9392_abc0b6bdf2b4);
impl windows_core::RuntimeType for IInclinometerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInclinometerStatics4, IInclinometerStatics4_Vtbl, 0xe8ba96f9_6e85_4a83_aed0_d7cdcc9856c8);
impl windows_core::RuntimeType for IInclinometerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInclinometerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor, ILightSensor_Vtbl, 0xf84c0718_0c54_47ae_922e_789f57fb03a0);
impl windows_core::RuntimeType for ILightSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor2, ILightSensor2_Vtbl, 0x486b24e8_a94c_4090_8f48_09f782a9f7d5);
impl windows_core::RuntimeType for ILightSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensor3, ILightSensor3_Vtbl, 0x4876d0ff_9f4c_5f72_adbd_a3471b063c00);
impl windows_core::RuntimeType for ILightSensor3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensor3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorDataThreshold, ILightSensorDataThreshold_Vtbl, 0xb160afd1_878f_5492_9f2c_33dc3ae584a3);
impl windows_core::RuntimeType for ILightSensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LuxPercentage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetLuxPercentage: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub AbsoluteLux: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetAbsoluteLux: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorDeviceId, ILightSensorDeviceId_Vtbl, 0x7fee49f8_0afb_4f51_87f0_6c26375ce94f);
impl windows_core::RuntimeType for ILightSensorDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorReading, ILightSensorReading_Vtbl, 0xffdf6300_227c_4d2b_b302_fc0142485c68);
impl windows_core::RuntimeType for ILightSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IlluminanceInLux: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorReading2, ILightSensorReading2_Vtbl, 0xb7512185_44a3_44c9_8190_9ef6de0a8a74);
impl windows_core::RuntimeType for ILightSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(ILightSensorReadingChangedEventArgs, ILightSensorReadingChangedEventArgs_Vtbl, 0xa3a2f4cf_258b_420c_b8ab_8edd601ecf50);
impl windows_core::RuntimeType for ILightSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorStatics, ILightSensorStatics_Vtbl, 0x45db8c84_c3a8_471e_9a53_6457fad87c0e);
impl windows_core::RuntimeType for ILightSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILightSensorStatics2, ILightSensorStatics2_Vtbl, 0x0ec0a650_ddc6_40ab_ace3_ec3359d42c51);
impl windows_core::RuntimeType for ILightSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILightSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockOnLeaveOptions, ILockOnLeaveOptions_Vtbl, 0x3c6bf8bd_04c1_5829_8d4e_70521755b8be);
impl windows_core::RuntimeType for ILockOnLeaveOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILockOnLeaveOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometer, IMagnetometer_Vtbl, 0x484f626e_d3c9_4111_b3f6_2cf1faa418d5);
impl windows_core::RuntimeType for IMagnetometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometer2, IMagnetometer2_Vtbl, 0xb4656c85_26f6_444b_a9e2_a23f966cd368);
impl windows_core::RuntimeType for IMagnetometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(IMagnetometer3, IMagnetometer3_Vtbl, 0xbe93db7c_a625_48ef_acf7_fac104832671);
impl windows_core::RuntimeType for IMagnetometer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometer4, IMagnetometer4_Vtbl, 0xdfb17901_3e0f_508f_b24b_f2bb75015f40);
impl windows_core::RuntimeType for IMagnetometer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerDataThreshold, IMagnetometerDataThreshold_Vtbl, 0xd177cb01_9063_5fa5_b596_b445e9dc3401);
impl windows_core::RuntimeType for IMagnetometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetXAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub YAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetYAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ZAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetZAxisMicroteslas: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerDeviceId, IMagnetometerDeviceId_Vtbl, 0x58b498c2_7e4b_404c_9fc5_5de8b40ebae3);
impl windows_core::RuntimeType for IMagnetometerDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerReading, IMagnetometerReading_Vtbl, 0x0c2cc40d_ebfd_4e5c_bb11_afc29b3cae61);
impl windows_core::RuntimeType for IMagnetometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub MagneticFieldX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MagneticFieldY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MagneticFieldZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub DirectionalAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerReading2, IMagnetometerReading2_Vtbl, 0xd4c95c61_61d9_404b_a328_066f177a1409);
impl windows_core::RuntimeType for IMagnetometerReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IMagnetometerReadingChangedEventArgs, IMagnetometerReadingChangedEventArgs_Vtbl, 0x17eae872_2eb9_4ee7_8ad0_3127537d949b);
impl windows_core::RuntimeType for IMagnetometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerStatics, IMagnetometerStatics_Vtbl, 0x853c64cc_0698_4dda_a6df_9cb9cc4ab40a);
impl windows_core::RuntimeType for IMagnetometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMagnetometerStatics2, IMagnetometerStatics2_Vtbl, 0x2c0819f0_ffc6_4f89_a06f_18fa10792933);
impl windows_core::RuntimeType for IMagnetometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMagnetometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensor, IOrientationSensor_Vtbl, 0x5e354635_cf6b_4c63_abd8_10252b0bf6ec);
impl windows_core::RuntimeType for IOrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensor2, IOrientationSensor2_Vtbl, 0x0d924cf9_2f1f_49c9_8042_4a1813d67760);
impl windows_core::RuntimeType for IOrientationSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
    pub ReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorReadingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensor3, IOrientationSensor3_Vtbl, 0x2cce578d_646b_48c5_b7ee_44fdc4c6aafd);
impl windows_core::RuntimeType for IOrientationSensor3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensor3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorDeviceId, IOrientationSensorDeviceId_Vtbl, 0x5a69b648_4c29_49ec_b28f_ea1d117b66f0);
impl windows_core::RuntimeType for IOrientationSensorDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReading, IOrientationSensorReading_Vtbl, 0x4756c993_6595_4897_bcc6_d537ee757564);
impl windows_core::RuntimeType for IOrientationSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub RotationMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Quaternion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReading2, IOrientationSensorReading2_Vtbl, 0x00576e5f_49f8_4c05_9e07_24fac79408c3);
impl windows_core::RuntimeType for IOrientationSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IOrientationSensorReadingChangedEventArgs, IOrientationSensorReadingChangedEventArgs_Vtbl, 0x012c1186_c3ba_46bc_ae65_7a98996cbfb8);
impl windows_core::RuntimeType for IOrientationSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorReadingYawAccuracy, IOrientationSensorReadingYawAccuracy_Vtbl, 0xd1ac9824_3f5a_49a2_bc7b_1180bc38cd2b);
impl windows_core::RuntimeType for IOrientationSensorReadingYawAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorReadingYawAccuracy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub YawAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MagnetometerAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics, IOrientationSensorStatics_Vtbl, 0x10ef8712_fb4c_428a_898b_2765e409e669);
impl windows_core::RuntimeType for IOrientationSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics2, IOrientationSensorStatics2_Vtbl, 0x59da0d0b_d40a_4c71_9276_8a272a0a6619);
impl windows_core::RuntimeType for IOrientationSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultForRelativeReadings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics3, IOrientationSensorStatics3_Vtbl, 0xd82ce920_2777_40ff_9f59_d654b085f12f);
impl windows_core::RuntimeType for IOrientationSensorStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultWithSensorReadingType: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, SensorOptimizationGoal, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOrientationSensorStatics4, IOrientationSensorStatics4_Vtbl, 0xa67feb55_2c85_4b28_a0fe_58c4b20495f5);
impl windows_core::RuntimeType for IOrientationSensorStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOrientationSensorStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal: unsafe extern "system" fn(*mut core::ffi::c_void, SensorReadingType, SensorOptimizationGoal, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometer, IPedometer_Vtbl, 0x9a1e013d_3d98_45f8_8920_8e4ecaca5f97);
impl windows_core::RuntimeType for IPedometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PowerInMilliwatts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometer2, IPedometer2_Vtbl, 0xe5a406df_2b81_4add_b2ff_77ab6c98ba19);
impl windows_core::RuntimeType for IPedometer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentReadings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentReadings: usize,
}
windows_core::imp::define_interface!(IPedometerDataThresholdFactory, IPedometerDataThresholdFactory_Vtbl, 0xcbad8f50_7a54_466b_9010_77a162fca5d7);
impl windows_core::RuntimeType for IPedometerDataThresholdFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometerDataThresholdFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerReading, IPedometerReading_Vtbl, 0x2245dcf4_a8e1_432f_896a_be0dd9b02d24);
impl windows_core::RuntimeType for IPedometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometerReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StepKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PedometerStepKind) -> windows_core::HRESULT,
    pub CumulativeSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub CumulativeStepsDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerReadingChangedEventArgs, IPedometerReadingChangedEventArgs_Vtbl, 0xf855e47e_abbc_4456_86a8_25cf2b333742);
impl windows_core::RuntimeType for IPedometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometerReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPedometerStatics, IPedometerStatics_Vtbl, 0x82980a2f_4083_4dfb_b411_938ea0f4b946);
impl windows_core::RuntimeType for IPedometerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemHistoryWithDurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemHistoryWithDurationAsync: usize,
}
windows_core::imp::define_interface!(IPedometerStatics2, IPedometerStatics2_Vtbl, 0x79f5c6bb_ce0e_4133_b47e_8627ea72f677);
impl windows_core::RuntimeType for IPedometerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPedometerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
windows_core::imp::define_interface!(IProximitySensor, IProximitySensor_Vtbl, 0x54c076b8_ecfb_4944_b928_74fc504d47ee);
impl windows_core::RuntimeType for IProximitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximitySensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MaxDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinDistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateDisplayOnOffController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorDataThresholdFactory, IProximitySensorDataThresholdFactory_Vtbl, 0x905ac121_6d27_4ad3_9db5_6467f2a5ad9d);
impl windows_core::RuntimeType for IProximitySensorDataThresholdFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximitySensorDataThresholdFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorReading, IProximitySensorReading_Vtbl, 0x71228d59_132d_4d5f_8ff9_2f0db8751ced);
impl windows_core::RuntimeType for IProximitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximitySensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IsDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DistanceInMillimeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorReadingChangedEventArgs, IProximitySensorReadingChangedEventArgs_Vtbl, 0xcfc2f366_c3e8_40fd_8cc3_67e289004938);
impl windows_core::RuntimeType for IProximitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximitySensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorStatics, IProximitySensorStatics_Vtbl, 0x29186649_6269_4e57_a5ad_82be80813392);
impl windows_core::RuntimeType for IProximitySensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximitySensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximitySensorStatics2, IProximitySensorStatics2_Vtbl, 0xcbf473ae_e9ca_422f_ad67_4c3d25df350c);
impl windows_core::RuntimeType for IProximitySensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximitySensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetReadingsFromTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetReadingsFromTriggerDetails: usize,
}
windows_core::imp::define_interface!(ISensorDataThreshold, ISensorDataThreshold_Vtbl, 0x54daec61_fe4b_4e07_b260_3a4cdfbe396e);
impl core::ops::Deref for ISensorDataThreshold {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISensorDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl ISensorDataThreshold {}
impl windows_core::RuntimeType for ISensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISensorDataThreshold_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISensorDataThresholdTriggerDetails, ISensorDataThresholdTriggerDetails_Vtbl, 0x9106f1b7_e88d_48b1_bc90_619c7b349391);
impl windows_core::RuntimeType for ISensorDataThresholdTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISensorDataThresholdTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SensorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorQuaternion, ISensorQuaternion_Vtbl, 0xc9c5c827_c71c_46e7_9da3_36a193b232bc);
impl windows_core::RuntimeType for ISensorQuaternion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISensorQuaternion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub W: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub X: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Y: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Z: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISensorRotationMatrix, ISensorRotationMatrix_Vtbl, 0x0a3d5a67_22f4_4392_9538_65d0bd064aa6);
impl windows_core::RuntimeType for ISensorRotationMatrix {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISensorRotationMatrix_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub M11: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M13: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M21: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M22: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M23: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M31: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub M33: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensor, ISimpleOrientationSensor_Vtbl, 0x5ff53856_214a_4dee_a3f9_616f1ab06ffd);
impl windows_core::RuntimeType for ISimpleOrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISimpleOrientationSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SimpleOrientation) -> windows_core::HRESULT,
    pub OrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveOrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensor2, ISimpleOrientationSensor2_Vtbl, 0xa277a798_8870_453e_8bd6_b8f5d8d7941b);
impl windows_core::RuntimeType for ISimpleOrientationSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISimpleOrientationSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Display")]
    pub SetReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    SetReadingTransform: usize,
    #[cfg(feature = "Graphics_Display")]
    pub ReadingTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Display::DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Display"))]
    ReadingTransform: usize,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorDeviceId, ISimpleOrientationSensorDeviceId_Vtbl, 0xfbc00acb_3b76_41f6_8091_30efe646d3cf);
impl windows_core::RuntimeType for ISimpleOrientationSensorDeviceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISimpleOrientationSensorDeviceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorOrientationChangedEventArgs, ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl, 0xbcd5c660_23d4_4b4c_a22e_ba81ade0c601);
impl windows_core::RuntimeType for ISimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SimpleOrientation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorStatics, ISimpleOrientationSensorStatics_Vtbl, 0x72ed066f_70aa_40c6_9b1b_3433f7459b4e);
impl windows_core::RuntimeType for ISimpleOrientationSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISimpleOrientationSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleOrientationSensorStatics2, ISimpleOrientationSensorStatics2_Vtbl, 0x848f9c7f_b138_4e11_8910_a2a2a3b56d83);
impl windows_core::RuntimeType for ISimpleOrientationSensorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISimpleOrientationSensorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWakeOnApproachOptions, IWakeOnApproachOptions_Vtbl, 0xf0b87ae7_7e1f_5ea5_814d_6b7e07defc2b);
impl windows_core::RuntimeType for IWakeOnApproachOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWakeOnApproachOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenExternalDisplayConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DisableWhenBatterySaverOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDisableWhenBatterySaverOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Accelerometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Accelerometer, windows_core::IUnknown, windows_core::IInspectable);
impl Accelerometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Shaken<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shaken)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShaken(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShaken)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IAccelerometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAccelerometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingType(&self) -> windows_core::Result<AccelerometerReadingType> {
        let this = &windows_core::Interface::cast::<IAccelerometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<AccelerometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IAccelerometer5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccelerometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Accelerometer> {
        Self::IAccelerometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithAccelerometerReadingType(readingtype: AccelerometerReadingType) -> windows_core::Result<Accelerometer> {
        Self::IAccelerometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithAccelerometerReadingType)(windows_core::Interface::as_raw(this), readingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector(readingtype: AccelerometerReadingType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IAccelerometerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), readingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAccelerometerStatics<R, F: FnOnce(&IAccelerometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Accelerometer, IAccelerometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccelerometerStatics2<R, F: FnOnce(&IAccelerometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Accelerometer, IAccelerometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccelerometerStatics3<R, F: FnOnce(&IAccelerometerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Accelerometer, IAccelerometerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Accelerometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometer>();
}
unsafe impl windows_core::Interface for Accelerometer {
    type Vtable = IAccelerometer_Vtbl;
    const IID: windows_core::GUID = <IAccelerometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Accelerometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Accelerometer";
}
unsafe impl Send for Accelerometer {}
unsafe impl Sync for Accelerometer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccelerometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerDataThreshold {
    pub fn XAxisInGForce(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XAxisInGForce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetXAxisInGForce(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetXAxisInGForce)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInGForce(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YAxisInGForce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYAxisInGForce(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYAxisInGForce)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInGForce(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZAxisInGForce)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZAxisInGForce(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZAxisInGForce)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AccelerometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerDataThreshold>();
}
unsafe impl windows_core::Interface for AccelerometerDataThreshold {
    type Vtable = IAccelerometerDataThreshold_Vtbl;
    const IID: windows_core::GUID = <IAccelerometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerDataThreshold";
}
unsafe impl Send for AccelerometerDataThreshold {}
unsafe impl Sync for AccelerometerDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccelerometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccelerationX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccelerationX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccelerationY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccelerationY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccelerationZ(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccelerationZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IAccelerometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AccelerometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerReading>();
}
unsafe impl windows_core::Interface for AccelerometerReading {
    type Vtable = IAccelerometerReading_Vtbl;
    const IID: windows_core::GUID = <IAccelerometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReading";
}
unsafe impl Send for AccelerometerReading {}
unsafe impl Sync for AccelerometerReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccelerometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<AccelerometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AccelerometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for AccelerometerReadingChangedEventArgs {
    type Vtable = IAccelerometerReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAccelerometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerReadingChangedEventArgs";
}
unsafe impl Send for AccelerometerReadingChangedEventArgs {}
unsafe impl Sync for AccelerometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccelerometerShakenEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccelerometerShakenEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccelerometerShakenEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AccelerometerShakenEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccelerometerShakenEventArgs>();
}
unsafe impl windows_core::Interface for AccelerometerShakenEventArgs {
    type Vtable = IAccelerometerShakenEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAccelerometerShakenEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccelerometerShakenEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AccelerometerShakenEventArgs";
}
unsafe impl Send for AccelerometerShakenEventArgs {}
unsafe impl Sync for AccelerometerShakenEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivitySensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensor, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensor {
    pub fn GetCurrentReadingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReadingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubscribedActivities(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribedActivities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PowerInMilliwatts(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerInMilliwatts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedActivities(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedActivities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryAsync(fromtime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryAsync)(windows_core::Interface::as_raw(this), fromtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryWithDurationAsync(fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>> {
        Self::IActivitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(windows_core::Interface::as_raw(this), fromtime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IActivitySensorStatics<R, F: FnOnce(&IActivitySensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ActivitySensor, IActivitySensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ActivitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensor>();
}
unsafe impl windows_core::Interface for ActivitySensor {
    type Vtable = IActivitySensor_Vtbl;
    const IID: windows_core::GUID = <IActivitySensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensor";
}
unsafe impl Send for ActivitySensor {}
unsafe impl Sync for ActivitySensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivitySensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Activity(&self) -> windows_core::Result<ActivityType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Confidence(&self) -> windows_core::Result<ActivitySensorReadingConfidence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Confidence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorReading>();
}
unsafe impl windows_core::Interface for ActivitySensorReading {
    type Vtable = IActivitySensorReading_Vtbl;
    const IID: windows_core::GUID = <IActivitySensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReading";
}
unsafe impl Send for ActivitySensorReading {}
unsafe impl Sync for ActivitySensorReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivitySensorReadingChangeReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorReadingChangeReport, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorReadingChangeReport {
    pub fn Reading(&self) -> windows_core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorReadingChangeReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorReadingChangeReport>();
}
unsafe impl windows_core::Interface for ActivitySensorReadingChangeReport {
    type Vtable = IActivitySensorReadingChangeReport_Vtbl;
    const IID: windows_core::GUID = <IActivitySensorReadingChangeReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorReadingChangeReport {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangeReport";
}
unsafe impl Send for ActivitySensorReadingChangeReport {}
unsafe impl Sync for ActivitySensorReadingChangeReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivitySensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<ActivitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for ActivitySensorReadingChangedEventArgs {
    type Vtable = IActivitySensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IActivitySensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorReadingChangedEventArgs";
}
unsafe impl Send for ActivitySensorReadingChangedEventArgs {}
unsafe impl Sync for ActivitySensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivitySensorTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ActivitySensorTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadReports)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivitySensorTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorTriggerDetails>();
}
unsafe impl windows_core::Interface for ActivitySensorTriggerDetails {
    type Vtable = IActivitySensorTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IActivitySensorTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.ActivitySensorTriggerDetails";
}
unsafe impl Send for ActivitySensorTriggerDetails {}
unsafe impl Sync for ActivitySensorTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveDimmingOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveDimmingOptions, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveDimmingOptions {
    pub fn AllowWhenExternalDisplayConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenExternalDisplayConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveDimmingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveDimmingOptions>();
}
unsafe impl windows_core::Interface for AdaptiveDimmingOptions {
    type Vtable = IAdaptiveDimmingOptions_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveDimmingOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveDimmingOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.AdaptiveDimmingOptions";
}
unsafe impl Send for AdaptiveDimmingOptions {}
unsafe impl Sync for AdaptiveDimmingOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Altimeter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Altimeter, windows_core::IUnknown, windows_core::IInspectable);
impl Altimeter {
    pub fn GetCurrentReading(&self) -> windows_core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAltimeter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDefault() -> windows_core::Result<Altimeter> {
        Self::IAltimeterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAltimeterStatics<R, F: FnOnce(&IAltimeterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Altimeter, IAltimeterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Altimeter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAltimeter>();
}
unsafe impl windows_core::Interface for Altimeter {
    type Vtable = IAltimeter_Vtbl;
    const IID: windows_core::GUID = <IAltimeter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Altimeter {
    const NAME: &'static str = "Windows.Devices.Sensors.Altimeter";
}
unsafe impl Send for Altimeter {}
unsafe impl Sync for Altimeter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AltimeterReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AltimeterReading, windows_core::IUnknown, windows_core::IInspectable);
impl AltimeterReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AltitudeChangeInMeters(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeChangeInMeters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IAltimeterReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AltimeterReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAltimeterReading>();
}
unsafe impl windows_core::Interface for AltimeterReading {
    type Vtable = IAltimeterReading_Vtbl;
    const IID: windows_core::GUID = <IAltimeterReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AltimeterReading {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReading";
}
unsafe impl Send for AltimeterReading {}
unsafe impl Sync for AltimeterReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AltimeterReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AltimeterReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AltimeterReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<AltimeterReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AltimeterReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAltimeterReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for AltimeterReadingChangedEventArgs {
    type Vtable = IAltimeterReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAltimeterReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AltimeterReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.AltimeterReadingChangedEventArgs";
}
unsafe impl Send for AltimeterReadingChangedEventArgs {}
unsafe impl Sync for AltimeterReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Barometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Barometer, windows_core::IUnknown, windows_core::IInspectable);
impl Barometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBarometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<BarometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IBarometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Barometer> {
        Self::IBarometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Barometer>> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IBarometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBarometerStatics<R, F: FnOnce(&IBarometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Barometer, IBarometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBarometerStatics2<R, F: FnOnce(&IBarometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Barometer, IBarometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Barometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometer>();
}
unsafe impl windows_core::Interface for Barometer {
    type Vtable = IBarometer_Vtbl;
    const IID: windows_core::GUID = <IBarometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Barometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Barometer";
}
unsafe impl Send for Barometer {}
unsafe impl Sync for Barometer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl BarometerDataThreshold {
    pub fn Hectopascals(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hectopascals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHectopascals(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHectopascals)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for BarometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometerDataThreshold>();
}
unsafe impl windows_core::Interface for BarometerDataThreshold {
    type Vtable = IBarometerDataThreshold_Vtbl;
    const IID: windows_core::GUID = <IBarometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerDataThreshold";
}
unsafe impl Send for BarometerDataThreshold {}
unsafe impl Sync for BarometerDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl BarometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StationPressureInHectopascals(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StationPressureInHectopascals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IBarometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometerReading>();
}
unsafe impl windows_core::Interface for BarometerReading {
    type Vtable = IBarometerReading_Vtbl;
    const IID: windows_core::GUID = <IBarometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReading";
}
unsafe impl Send for BarometerReading {}
unsafe impl Sync for BarometerReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BarometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BarometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BarometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<BarometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BarometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBarometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for BarometerReadingChangedEventArgs {
    type Vtable = IBarometerReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBarometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BarometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.BarometerReadingChangedEventArgs";
}
unsafe impl Send for BarometerReadingChangedEventArgs {}
unsafe impl Sync for BarometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Compass(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Compass, windows_core::IUnknown, windows_core::IInspectable);
impl Compass {
    pub fn GetCurrentReading(&self) -> windows_core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompass2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<ICompass2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICompass3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<CompassDataThreshold> {
        let this = &windows_core::Interface::cast::<ICompass4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICompassDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Compass> {
        Self::ICompassStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Compass>> {
        Self::ICompassStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICompassStatics<R, F: FnOnce(&ICompassStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Compass, ICompassStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICompassStatics2<R, F: FnOnce(&ICompassStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Compass, ICompassStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Compass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompass>();
}
unsafe impl windows_core::Interface for Compass {
    type Vtable = ICompass_Vtbl;
    const IID: windows_core::GUID = <ICompass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Compass {
    const NAME: &'static str = "Windows.Devices.Sensors.Compass";
}
unsafe impl Send for Compass {}
unsafe impl Sync for Compass {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompassDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompassDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl CompassDataThreshold {
    pub fn Degrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Degrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CompassDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompassDataThreshold>();
}
unsafe impl windows_core::Interface for CompassDataThreshold {
    type Vtable = ICompassDataThreshold_Vtbl;
    const IID: windows_core::GUID = <ICompassDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompassDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassDataThreshold";
}
unsafe impl Send for CompassDataThreshold {}
unsafe impl Sync for CompassDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompassReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompassReading, windows_core::IUnknown, windows_core::IInspectable);
impl CompassReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HeadingMagneticNorth(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadingMagneticNorth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HeadingTrueNorth(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadingTrueNorth)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<ICompassReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadingAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = &windows_core::Interface::cast::<ICompassReadingHeadingAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadingAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CompassReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompassReading>();
}
unsafe impl windows_core::Interface for CompassReading {
    type Vtable = ICompassReading_Vtbl;
    const IID: windows_core::GUID = <ICompassReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompassReading {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReading";
}
unsafe impl Send for CompassReading {}
unsafe impl Sync for CompassReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompassReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompassReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CompassReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<CompassReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CompassReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompassReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for CompassReadingChangedEventArgs {
    type Vtable = ICompassReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICompassReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompassReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.CompassReadingChangedEventArgs";
}
unsafe impl Send for CompassReadingChangedEventArgs {}
unsafe impl Sync for CompassReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DetectedPerson(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DetectedPerson, windows_core::IUnknown, windows_core::IInspectable);
impl DetectedPerson {
    pub fn Engagement(&self) -> windows_core::Result<HumanEngagement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Engagement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadOrientation(&self) -> windows_core::Result<HeadOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadOrientation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeadPosition(&self) -> windows_core::Result<HeadPosition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadPosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PersonId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PersonId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DetectedPerson {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDetectedPerson>();
}
unsafe impl windows_core::Interface for DetectedPerson {
    type Vtable = IDetectedPerson_Vtbl;
    const IID: windows_core::GUID = <IDetectedPerson as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DetectedPerson {
    const NAME: &'static str = "Windows.Devices.Sensors.DetectedPerson";
}
unsafe impl Send for DetectedPerson {}
unsafe impl Sync for DetectedPerson {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Gyrometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Gyrometer, windows_core::IUnknown, windows_core::IInspectable);
impl Gyrometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGyrometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IGyrometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGyrometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<GyrometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IGyrometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IGyrometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Gyrometer> {
        Self::IGyrometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>> {
        Self::IGyrometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGyrometerStatics<R, F: FnOnce(&IGyrometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Gyrometer, IGyrometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGyrometerStatics2<R, F: FnOnce(&IGyrometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Gyrometer, IGyrometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Gyrometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometer>();
}
unsafe impl windows_core::Interface for Gyrometer {
    type Vtable = IGyrometer_Vtbl;
    const IID: windows_core::GUID = <IGyrometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Gyrometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Gyrometer";
}
unsafe impl Send for Gyrometer {}
unsafe impl Sync for Gyrometer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GyrometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GyrometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl GyrometerDataThreshold {
    pub fn XAxisInDegreesPerSecond(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetXAxisInDegreesPerSecond(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetXAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisInDegreesPerSecond(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYAxisInDegreesPerSecond(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisInDegreesPerSecond(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZAxisInDegreesPerSecond(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZAxisInDegreesPerSecond)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for GyrometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometerDataThreshold>();
}
unsafe impl windows_core::Interface for GyrometerDataThreshold {
    type Vtable = IGyrometerDataThreshold_Vtbl;
    const IID: windows_core::GUID = <IGyrometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GyrometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerDataThreshold";
}
unsafe impl Send for GyrometerDataThreshold {}
unsafe impl Sync for GyrometerDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GyrometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GyrometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl GyrometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngularVelocityX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocityX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngularVelocityY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocityY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngularVelocityZ(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocityZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IGyrometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GyrometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometerReading>();
}
unsafe impl windows_core::Interface for GyrometerReading {
    type Vtable = IGyrometerReading_Vtbl;
    const IID: windows_core::GUID = <IGyrometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GyrometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReading";
}
unsafe impl Send for GyrometerReading {}
unsafe impl Sync for GyrometerReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GyrometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GyrometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GyrometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<GyrometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GyrometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGyrometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for GyrometerReadingChangedEventArgs {
    type Vtable = IGyrometerReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGyrometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GyrometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.GyrometerReadingChangedEventArgs";
}
unsafe impl Send for GyrometerReadingChangedEventArgs {}
unsafe impl Sync for GyrometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HeadOrientation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HeadOrientation, windows_core::IUnknown, windows_core::IInspectable);
impl HeadOrientation {
    pub fn RollInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RollInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PitchInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn YawInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HeadOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHeadOrientation>();
}
unsafe impl windows_core::Interface for HeadOrientation {
    type Vtable = IHeadOrientation_Vtbl;
    const IID: windows_core::GUID = <IHeadOrientation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HeadOrientation {
    const NAME: &'static str = "Windows.Devices.Sensors.HeadOrientation";
}
unsafe impl Send for HeadOrientation {}
unsafe impl Sync for HeadOrientation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HeadPosition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HeadPosition, windows_core::IUnknown, windows_core::IInspectable);
impl HeadPosition {
    pub fn AzimuthInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AzimuthInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AltitudeInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AltitudeInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HeadPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHeadPosition>();
}
unsafe impl windows_core::Interface for HeadPosition {
    type Vtable = IHeadPosition_Vtbl;
    const IID: windows_core::GUID = <IHeadPosition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HeadPosition {
    const NAME: &'static str = "Windows.Devices.Sensors.HeadPosition";
}
unsafe impl Send for HeadPosition {}
unsafe impl Sync for HeadPosition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HingeAngleReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HingeAngleReading, windows_core::IUnknown, windows_core::IInspectable);
impl HingeAngleReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AngleInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngleInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HingeAngleReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHingeAngleReading>();
}
unsafe impl windows_core::Interface for HingeAngleReading {
    type Vtable = IHingeAngleReading_Vtbl;
    const IID: windows_core::GUID = <IHingeAngleReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HingeAngleReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleReading";
}
unsafe impl Send for HingeAngleReading {}
unsafe impl Sync for HingeAngleReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HingeAngleSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HingeAngleSensor, windows_core::IUnknown, windows_core::IInspectable);
impl HingeAngleSensor {
    pub fn GetCurrentReadingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReadingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinReportThresholdInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinReportThresholdInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThresholdInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThresholdInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportThresholdInDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportThresholdInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetRelatedToAdjacentPanelsAsync(firstpanelid: &windows_core::HSTRING, secondpanelid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRelatedToAdjacentPanelsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(firstpanelid), core::mem::transmute_copy(secondpanelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>> {
        Self::IHingeAngleSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHingeAngleSensorStatics<R, F: FnOnce(&IHingeAngleSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HingeAngleSensor, IHingeAngleSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HingeAngleSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHingeAngleSensor>();
}
unsafe impl windows_core::Interface for HingeAngleSensor {
    type Vtable = IHingeAngleSensor_Vtbl;
    const IID: windows_core::GUID = <IHingeAngleSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HingeAngleSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensor";
}
unsafe impl Send for HingeAngleSensor {}
unsafe impl Sync for HingeAngleSensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HingeAngleSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HingeAngleSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HingeAngleSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<HingeAngleReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HingeAngleSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHingeAngleSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for HingeAngleSensorReadingChangedEventArgs {
    type Vtable = IHingeAngleSensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHingeAngleSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HingeAngleSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HingeAngleSensorReadingChangedEventArgs";
}
unsafe impl Send for HingeAngleSensorReadingChangedEventArgs {}
unsafe impl Sync for HingeAngleSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HumanPresenceFeatures(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceFeatures, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceFeatures {
    pub fn SensorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedWakeOrLockDistancesInMillimeters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedWakeOrLockDistancesInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWakeOnApproachSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWakeOnApproachSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLockOnLeaveSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLockOnLeaveSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsAttentionAwareDimmingSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAttentionAwareDimmingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAdaptiveDimmingSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceFeatures2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAdaptiveDimmingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HumanPresenceFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceFeatures>();
}
unsafe impl windows_core::Interface for HumanPresenceFeatures {
    type Vtable = IHumanPresenceFeatures_Vtbl;
    const IID: windows_core::GUID = <IHumanPresenceFeatures as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceFeatures {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceFeatures";
}
unsafe impl Send for HumanPresenceFeatures {}
unsafe impl Sync for HumanPresenceFeatures {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HumanPresenceSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensor, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensor {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDetectableDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectableDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinDetectableDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDetectableDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<HumanPresenceSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HumanPresenceSensor, HumanPresenceSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsPresenceSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPresenceSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEngagementSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEngagementSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxDetectablePersons(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectablePersons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinDetectableAzimuthInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDetectableAzimuthInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDetectableAzimuthInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectableAzimuthInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinDetectableAltitudeInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDetectableAltitudeInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDetectableAltitudeInDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDetectableAltitudeInDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(sensorid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceSensor>> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceSensor>> {
        Self::IHumanPresenceSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromId(sensorid: &windows_core::HSTRING) -> windows_core::Result<HumanPresenceSensor> {
        Self::IHumanPresenceSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefault() -> windows_core::Result<HumanPresenceSensor> {
        Self::IHumanPresenceSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHumanPresenceSensorStatics<R, F: FnOnce(&IHumanPresenceSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSensor, IHumanPresenceSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHumanPresenceSensorStatics2<R, F: FnOnce(&IHumanPresenceSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSensor, IHumanPresenceSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HumanPresenceSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensor>();
}
unsafe impl windows_core::Interface for HumanPresenceSensor {
    type Vtable = IHumanPresenceSensor_Vtbl;
    const IID: windows_core::GUID = <IHumanPresenceSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensor";
}
unsafe impl Send for HumanPresenceSensor {}
unsafe impl Sync for HumanPresenceSensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HumanPresenceSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Presence(&self) -> windows_core::Result<HumanPresence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Presence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Engagement(&self) -> windows_core::Result<HumanEngagement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Engagement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnlookerPresence(&self) -> windows_core::Result<HumanPresence> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReading3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnlookerPresence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DetectedPersons(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DetectedPerson>> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSensorReading3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetectedPersons)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HumanPresenceSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensorReading>();
}
unsafe impl windows_core::Interface for HumanPresenceSensorReading {
    type Vtable = IHumanPresenceSensorReading_Vtbl;
    const IID: windows_core::GUID = <IHumanPresenceSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReading";
}
unsafe impl Send for HumanPresenceSensorReading {}
unsafe impl Sync for HumanPresenceSensorReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HumanPresenceSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<HumanPresenceSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HumanPresenceSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for HumanPresenceSensorReadingChangedEventArgs {
    type Vtable = IHumanPresenceSensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHumanPresenceSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReadingChangedEventArgs";
}
unsafe impl Send for HumanPresenceSensorReadingChangedEventArgs {}
unsafe impl Sync for HumanPresenceSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HumanPresenceSensorReadingUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSensorReadingUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSensorReadingUpdate {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSensorReadingUpdate, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTimestamp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTimestamp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Presence(&self) -> windows_core::Result<super::super::Foundation::IReference<HumanPresence>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Presence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPresence<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<HumanPresence>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPresence)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Engagement(&self) -> windows_core::Result<super::super::Foundation::IReference<HumanEngagement>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Engagement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEngagement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<HumanEngagement>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEngagement)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDistanceInMillimeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDistanceInMillimeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for HumanPresenceSensorReadingUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSensorReadingUpdate>();
}
unsafe impl windows_core::Interface for HumanPresenceSensorReadingUpdate {
    type Vtable = IHumanPresenceSensorReadingUpdate_Vtbl;
    const IID: windows_core::GUID = <IHumanPresenceSensorReadingUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSensorReadingUpdate {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSensorReadingUpdate";
}
unsafe impl Send for HumanPresenceSensorReadingUpdate {}
unsafe impl Sync for HumanPresenceSensorReadingUpdate {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HumanPresenceSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HumanPresenceSettings, windows_core::IUnknown, windows_core::IInspectable);
impl HumanPresenceSettings {
    pub fn SensorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSensorId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSensorId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsWakeOnApproachEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWakeOnApproachEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsWakeOnApproachEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsWakeOnApproachEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WakeOnApproachDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WakeOnApproachDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWakeOnApproachDistanceInMillimeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWakeOnApproachDistanceInMillimeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsLockOnLeaveEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLockOnLeaveEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLockOnLeaveEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsLockOnLeaveEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LockOnLeaveDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockOnLeaveDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLockOnLeaveDistanceInMillimeters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLockOnLeaveDistanceInMillimeters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LockOnLeaveTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockOnLeaveTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLockOnLeaveTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLockOnLeaveTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsAttentionAwareDimmingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAttentionAwareDimmingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsAttentionAwareDimmingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAttentionAwareDimmingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAdaptiveDimmingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAdaptiveDimmingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAdaptiveDimmingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAdaptiveDimmingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WakeOptions(&self) -> windows_core::Result<WakeOnApproachOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WakeOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DimmingOptions(&self) -> windows_core::Result<AdaptiveDimmingOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DimmingOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LockOptions(&self) -> windows_core::Result<LockOnLeaveOptions> {
        let this = &windows_core::Interface::cast::<IHumanPresenceSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentSettingsAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceSettings>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettingsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentSettings() -> windows_core::Result<HumanPresenceSettings> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UpdateSettingsAsync<P0>(settings: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<HumanPresenceSettings>,
    {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateSettingsAsync)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UpdateSettings<P0>(settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HumanPresenceSettings>,
    {
        Self::IHumanPresenceSettingsStatics(|this| unsafe { (windows_core::Interface::vtable(this).UpdateSettings)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() })
    }
    pub fn GetSupportedFeaturesForSensorIdAsync(sensorid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HumanPresenceFeatures>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedFeaturesForSensorIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSupportedFeaturesForSensorId(sensorid: &windows_core::HSTRING) -> windows_core::Result<HumanPresenceFeatures> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedFeaturesForSensorId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedLockOnLeaveTimeouts() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::TimeSpan>> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedLockOnLeaveTimeouts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SettingsChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IHumanPresenceSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveSettingsChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IHumanPresenceSettingsStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveSettingsChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IHumanPresenceSettingsStatics<R, F: FnOnce(&IHumanPresenceSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HumanPresenceSettings, IHumanPresenceSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HumanPresenceSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHumanPresenceSettings>();
}
unsafe impl windows_core::Interface for HumanPresenceSettings {
    type Vtable = IHumanPresenceSettings_Vtbl;
    const IID: windows_core::GUID = <IHumanPresenceSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HumanPresenceSettings {
    const NAME: &'static str = "Windows.Devices.Sensors.HumanPresenceSettings";
}
unsafe impl Send for HumanPresenceSettings {}
unsafe impl Sync for HumanPresenceSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Inclinometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Inclinometer, windows_core::IUnknown, windows_core::IInspectable);
impl Inclinometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingType(&self) -> windows_core::Result<SensorReadingType> {
        let this = &windows_core::Interface::cast::<IInclinometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInclinometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<InclinometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IInclinometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IInclinometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultForRelativeReadings() -> windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultForRelativeReadings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> windows_core::Result<Inclinometer> {
        Self::IInclinometerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithSensorReadingType)(windows_core::Interface::as_raw(this), sensorreadingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), readingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>> {
        Self::IInclinometerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics<R, F: FnOnce(&IInclinometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics2<R, F: FnOnce(&IInclinometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics3<R, F: FnOnce(&IInclinometerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInclinometerStatics4<R, F: FnOnce(&IInclinometerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Inclinometer, IInclinometerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Inclinometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometer>();
}
unsafe impl windows_core::Interface for Inclinometer {
    type Vtable = IInclinometer_Vtbl;
    const IID: windows_core::GUID = <IInclinometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Inclinometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Inclinometer";
}
unsafe impl Send for Inclinometer {}
unsafe impl Sync for Inclinometer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InclinometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InclinometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl InclinometerDataThreshold {
    pub fn PitchInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPitchInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPitchInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RollInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RollInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRollInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRollInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YawInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYawInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYawInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InclinometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometerDataThreshold>();
}
unsafe impl windows_core::Interface for InclinometerDataThreshold {
    type Vtable = IInclinometerDataThreshold_Vtbl;
    const IID: windows_core::GUID = <IInclinometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InclinometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerDataThreshold";
}
unsafe impl Send for InclinometerDataThreshold {}
unsafe impl Sync for InclinometerDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InclinometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InclinometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl InclinometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PitchDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RollDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RollDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn YawDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IInclinometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn YawAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = &windows_core::Interface::cast::<IInclinometerReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InclinometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometerReading>();
}
unsafe impl windows_core::Interface for InclinometerReading {
    type Vtable = IInclinometerReading_Vtbl;
    const IID: windows_core::GUID = <IInclinometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InclinometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReading";
}
unsafe impl Send for InclinometerReading {}
unsafe impl Sync for InclinometerReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InclinometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InclinometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InclinometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<InclinometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InclinometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInclinometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for InclinometerReadingChangedEventArgs {
    type Vtable = IInclinometerReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IInclinometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InclinometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.InclinometerReadingChangedEventArgs";
}
unsafe impl Send for InclinometerReadingChangedEventArgs {}
unsafe impl Sync for InclinometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LightSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensor, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensor {
    pub fn GetCurrentReading(&self) -> windows_core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ILightSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<LightSensorDataThreshold> {
        let this = &windows_core::Interface::cast::<ILightSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILightSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<LightSensor> {
        Self::ILightSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LightSensor>> {
        Self::ILightSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILightSensorStatics<R, F: FnOnce(&ILightSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LightSensor, ILightSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILightSensorStatics2<R, F: FnOnce(&ILightSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LightSensor, ILightSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LightSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensor>();
}
unsafe impl windows_core::Interface for LightSensor {
    type Vtable = ILightSensor_Vtbl;
    const IID: windows_core::GUID = <ILightSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensor";
}
unsafe impl Send for LightSensor {}
unsafe impl Sync for LightSensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LightSensorDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensorDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensorDataThreshold {
    pub fn LuxPercentage(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LuxPercentage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLuxPercentage(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLuxPercentage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AbsoluteLux(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteLux)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAbsoluteLux(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAbsoluteLux)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for LightSensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensorDataThreshold>();
}
unsafe impl windows_core::Interface for LightSensorDataThreshold {
    type Vtable = ILightSensorDataThreshold_Vtbl;
    const IID: windows_core::GUID = <ILightSensorDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorDataThreshold";
}
unsafe impl Send for LightSensorDataThreshold {}
unsafe impl Sync for LightSensorDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LightSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IlluminanceInLux(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IlluminanceInLux)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<ILightSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LightSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensorReading>();
}
unsafe impl windows_core::Interface for LightSensorReading {
    type Vtable = ILightSensorReading_Vtbl;
    const IID: windows_core::GUID = <ILightSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReading";
}
unsafe impl Send for LightSensorReading {}
unsafe impl Sync for LightSensorReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LightSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LightSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LightSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<LightSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LightSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILightSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for LightSensorReadingChangedEventArgs {
    type Vtable = ILightSensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILightSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LightSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.LightSensorReadingChangedEventArgs";
}
unsafe impl Send for LightSensorReadingChangedEventArgs {}
unsafe impl Sync for LightSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LockOnLeaveOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockOnLeaveOptions, windows_core::IUnknown, windows_core::IInspectable);
impl LockOnLeaveOptions {
    pub fn AllowWhenExternalDisplayConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenExternalDisplayConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for LockOnLeaveOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockOnLeaveOptions>();
}
unsafe impl windows_core::Interface for LockOnLeaveOptions {
    type Vtable = ILockOnLeaveOptions_Vtbl;
    const IID: windows_core::GUID = <ILockOnLeaveOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockOnLeaveOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.LockOnLeaveOptions";
}
unsafe impl Send for LockOnLeaveOptions {}
unsafe impl Sync for LockOnLeaveOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Magnetometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Magnetometer, windows_core::IUnknown, windows_core::IInspectable);
impl Magnetometer {
    pub fn GetCurrentReading(&self) -> windows_core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IMagnetometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMagnetometer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportThreshold(&self) -> windows_core::Result<MagnetometerDataThreshold> {
        let this = &windows_core::Interface::cast::<IMagnetometer4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportThreshold)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMagnetometerDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<Magnetometer> {
        Self::IMagnetometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>> {
        Self::IMagnetometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMagnetometerStatics<R, F: FnOnce(&IMagnetometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Magnetometer, IMagnetometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMagnetometerStatics2<R, F: FnOnce(&IMagnetometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Magnetometer, IMagnetometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Magnetometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometer>();
}
unsafe impl windows_core::Interface for Magnetometer {
    type Vtable = IMagnetometer_Vtbl;
    const IID: windows_core::GUID = <IMagnetometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Magnetometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Magnetometer";
}
unsafe impl Send for Magnetometer {}
unsafe impl Sync for Magnetometer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MagnetometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagnetometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
impl MagnetometerDataThreshold {
    pub fn XAxisMicroteslas(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XAxisMicroteslas)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetXAxisMicroteslas(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetXAxisMicroteslas)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn YAxisMicroteslas(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YAxisMicroteslas)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetYAxisMicroteslas(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetYAxisMicroteslas)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZAxisMicroteslas(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZAxisMicroteslas)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZAxisMicroteslas(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZAxisMicroteslas)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MagnetometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometerDataThreshold>();
}
unsafe impl windows_core::Interface for MagnetometerDataThreshold {
    type Vtable = IMagnetometerDataThreshold_Vtbl;
    const IID: windows_core::GUID = <IMagnetometerDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagnetometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerDataThreshold";
}
unsafe impl Send for MagnetometerDataThreshold {}
unsafe impl Sync for MagnetometerDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MagnetometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagnetometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl MagnetometerReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MagneticFieldX(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MagneticFieldX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MagneticFieldY(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MagneticFieldY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MagneticFieldZ(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MagneticFieldZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DirectionalAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DirectionalAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMagnetometerReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagnetometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometerReading>();
}
unsafe impl windows_core::Interface for MagnetometerReading {
    type Vtable = IMagnetometerReading_Vtbl;
    const IID: windows_core::GUID = <IMagnetometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagnetometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReading";
}
unsafe impl Send for MagnetometerReading {}
unsafe impl Sync for MagnetometerReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MagnetometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MagnetometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MagnetometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<MagnetometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MagnetometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMagnetometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for MagnetometerReadingChangedEventArgs {
    type Vtable = IMagnetometerReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMagnetometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MagnetometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.MagnetometerReadingChangedEventArgs";
}
unsafe impl Send for MagnetometerReadingChangedEventArgs {}
unsafe impl Sync for MagnetometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OrientationSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OrientationSensor, windows_core::IUnknown, windows_core::IInspectable);
impl OrientationSensor {
    pub fn GetCurrentReading(&self) -> windows_core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingType(&self) -> windows_core::Result<SensorReadingType> {
        let this = &windows_core::Interface::cast::<IOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IOrientationSensor3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultForRelativeReadings() -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultForRelativeReadings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithSensorReadingType(sensorreadingtype: SensorReadingType) -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithSensorReadingType)(windows_core::Interface::as_raw(this), sensorreadingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> windows_core::Result<OrientationSensor> {
        Self::IOrientationSensorStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal)(windows_core::Interface::as_raw(this), sensorreadingtype, optimizationgoal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector(readingtype: SensorReadingType) -> windows_core::Result<windows_core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), readingtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> windows_core::Result<windows_core::HSTRING> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal)(windows_core::Interface::as_raw(this), readingtype, optimizationgoal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>> {
        Self::IOrientationSensorStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics<R, F: FnOnce(&IOrientationSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics2<R, F: FnOnce(&IOrientationSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics3<R, F: FnOnce(&IOrientationSensorStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOrientationSensorStatics4<R, F: FnOnce(&IOrientationSensorStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OrientationSensor, IOrientationSensorStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOrientationSensor>();
}
unsafe impl windows_core::Interface for OrientationSensor {
    type Vtable = IOrientationSensor_Vtbl;
    const IID: windows_core::GUID = <IOrientationSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensor";
}
unsafe impl Send for OrientationSensor {}
unsafe impl Sync for OrientationSensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OrientationSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OrientationSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl OrientationSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RotationMatrix(&self) -> windows_core::Result<SensorRotationMatrix> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationMatrix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Quaternion(&self) -> windows_core::Result<SensorQuaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Quaternion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IOrientationSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn YawAccuracy(&self) -> windows_core::Result<MagnetometerAccuracy> {
        let this = &windows_core::Interface::cast::<IOrientationSensorReadingYawAccuracy>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YawAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for OrientationSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOrientationSensorReading>();
}
unsafe impl windows_core::Interface for OrientationSensorReading {
    type Vtable = IOrientationSensorReading_Vtbl;
    const IID: windows_core::GUID = <IOrientationSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OrientationSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReading";
}
unsafe impl Send for OrientationSensorReading {}
unsafe impl Sync for OrientationSensorReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OrientationSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OrientationSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OrientationSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<OrientationSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OrientationSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOrientationSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for OrientationSensorReadingChangedEventArgs {
    type Vtable = IOrientationSensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IOrientationSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OrientationSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.OrientationSensorReadingChangedEventArgs";
}
unsafe impl Send for OrientationSensorReadingChangedEventArgs {}
unsafe impl Sync for OrientationSensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Pedometer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Pedometer, windows_core::IUnknown, windows_core::IInspectable);
impl Pedometer {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PowerInMilliwatts(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerInMilliwatts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentReadings(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>> {
        let this = &windows_core::Interface::cast::<IPedometer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReadings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<Pedometer>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryAsync(fromtime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryAsync)(windows_core::Interface::as_raw(this), fromtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemHistoryWithDurationAsync(fromtime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>> {
        Self::IPedometerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSystemHistoryWithDurationAsync)(windows_core::Interface::as_raw(this), fromtime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails<P0>(triggerdetails: P0) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>>
    where
        P0: windows_core::Param<SensorDataThresholdTriggerDetails>,
    {
        Self::IPedometerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReadingsFromTriggerDetails)(windows_core::Interface::as_raw(this), triggerdetails.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPedometerStatics<R, F: FnOnce(&IPedometerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Pedometer, IPedometerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPedometerStatics2<R, F: FnOnce(&IPedometerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Pedometer, IPedometerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Pedometer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPedometer>();
}
unsafe impl windows_core::Interface for Pedometer {
    type Vtable = IPedometer_Vtbl;
    const IID: windows_core::GUID = <IPedometer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Pedometer {
    const NAME: &'static str = "Windows.Devices.Sensors.Pedometer";
}
unsafe impl Send for Pedometer {}
unsafe impl Sync for Pedometer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PedometerDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PedometerDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PedometerDataThreshold, ISensorDataThreshold);
impl PedometerDataThreshold {
    pub fn Create<P0>(sensor: P0, stepgoal: i32) -> windows_core::Result<PedometerDataThreshold>
    where
        P0: windows_core::Param<Pedometer>,
    {
        Self::IPedometerDataThresholdFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), sensor.param().abi(), stepgoal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPedometerDataThresholdFactory<R, F: FnOnce(&IPedometerDataThresholdFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PedometerDataThreshold, IPedometerDataThresholdFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PedometerDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThreshold>();
}
unsafe impl windows_core::Interface for PedometerDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
    const IID: windows_core::GUID = <ISensorDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PedometerDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerDataThreshold";
}
unsafe impl Send for PedometerDataThreshold {}
unsafe impl Sync for PedometerDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PedometerReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PedometerReading, windows_core::IUnknown, windows_core::IInspectable);
impl PedometerReading {
    pub fn StepKind(&self) -> windows_core::Result<PedometerStepKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StepKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CumulativeSteps(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CumulativeSteps)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CumulativeStepsDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CumulativeStepsDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PedometerReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPedometerReading>();
}
unsafe impl windows_core::Interface for PedometerReading {
    type Vtable = IPedometerReading_Vtbl;
    const IID: windows_core::GUID = <IPedometerReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PedometerReading {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReading";
}
unsafe impl Send for PedometerReading {}
unsafe impl Sync for PedometerReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PedometerReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PedometerReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PedometerReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<PedometerReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PedometerReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPedometerReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for PedometerReadingChangedEventArgs {
    type Vtable = IPedometerReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPedometerReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PedometerReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.PedometerReadingChangedEventArgs";
}
unsafe impl Send for PedometerReadingChangedEventArgs {}
unsafe impl Sync for PedometerReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProximitySensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensor, windows_core::IUnknown, windows_core::IInspectable);
impl ProximitySensor {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinDistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinDistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateDisplayOnOffController(&self) -> windows_core::Result<ProximitySensorDisplayOnOffController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDisplayOnOffController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromId(sensorid: &windows_core::HSTRING) -> windows_core::Result<ProximitySensor> {
        Self::IProximitySensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetReadingsFromTriggerDetails<P0>(triggerdetails: P0) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>
    where
        P0: windows_core::Param<SensorDataThresholdTriggerDetails>,
    {
        Self::IProximitySensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReadingsFromTriggerDetails)(windows_core::Interface::as_raw(this), triggerdetails.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProximitySensorStatics<R, F: FnOnce(&IProximitySensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximitySensor, IProximitySensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProximitySensorStatics2<R, F: FnOnce(&IProximitySensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximitySensor, IProximitySensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProximitySensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximitySensor>();
}
unsafe impl windows_core::Interface for ProximitySensor {
    type Vtable = IProximitySensor_Vtbl;
    const IID: windows_core::GUID = <IProximitySensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensor {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensor";
}
unsafe impl Send for ProximitySensor {}
unsafe impl Sync for ProximitySensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProximitySensorDataThreshold(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorDataThreshold, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ProximitySensorDataThreshold, ISensorDataThreshold);
impl ProximitySensorDataThreshold {
    pub fn Create<P0>(sensor: P0) -> windows_core::Result<ProximitySensorDataThreshold>
    where
        P0: windows_core::Param<ProximitySensor>,
    {
        Self::IProximitySensorDataThresholdFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), sensor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProximitySensorDataThresholdFactory<R, F: FnOnce(&IProximitySensorDataThresholdFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximitySensorDataThreshold, IProximitySensorDataThresholdFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProximitySensorDataThreshold {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThreshold>();
}
unsafe impl windows_core::Interface for ProximitySensorDataThreshold {
    type Vtable = ISensorDataThreshold_Vtbl;
    const IID: windows_core::GUID = <ISensorDataThreshold as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDataThreshold";
}
unsafe impl Send for ProximitySensorDataThreshold {}
unsafe impl Sync for ProximitySensorDataThreshold {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProximitySensorDisplayOnOffController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorDisplayOnOffController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ProximitySensorDisplayOnOffController, super::super::Foundation::IClosable);
impl ProximitySensorDisplayOnOffController {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ProximitySensorDisplayOnOffController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::IClosable>();
}
unsafe impl windows_core::Interface for ProximitySensorDisplayOnOffController {
    type Vtable = super::super::Foundation::IClosable_Vtbl;
    const IID: windows_core::GUID = <super::super::Foundation::IClosable as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorDisplayOnOffController {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorDisplayOnOffController";
}
unsafe impl Send for ProximitySensorDisplayOnOffController {}
unsafe impl Sync for ProximitySensorDisplayOnOffController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProximitySensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl ProximitySensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDetected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDetected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistanceInMillimeters(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceInMillimeters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProximitySensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximitySensorReading>();
}
unsafe impl windows_core::Interface for ProximitySensorReading {
    type Vtable = IProximitySensorReading_Vtbl;
    const IID: windows_core::GUID = <IProximitySensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReading";
}
unsafe impl Send for ProximitySensorReading {}
unsafe impl Sync for ProximitySensorReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProximitySensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximitySensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ProximitySensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<ProximitySensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProximitySensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximitySensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for ProximitySensorReadingChangedEventArgs {
    type Vtable = IProximitySensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IProximitySensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximitySensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.ProximitySensorReadingChangedEventArgs";
}
unsafe impl Send for ProximitySensorReadingChangedEventArgs {}
unsafe impl Sync for ProximitySensorReadingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SensorDataThresholdTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorDataThresholdTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SensorDataThresholdTriggerDetails {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SensorType(&self) -> windows_core::Result<SensorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SensorDataThresholdTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThresholdTriggerDetails>();
}
unsafe impl windows_core::Interface for SensorDataThresholdTriggerDetails {
    type Vtable = ISensorDataThresholdTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ISensorDataThresholdTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorDataThresholdTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorDataThresholdTriggerDetails";
}
unsafe impl Send for SensorDataThresholdTriggerDetails {}
unsafe impl Sync for SensorDataThresholdTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SensorQuaternion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorQuaternion, windows_core::IUnknown, windows_core::IInspectable);
impl SensorQuaternion {
    pub fn W(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).W)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn X(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).X)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Y(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Y)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Z(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Z)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SensorQuaternion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorQuaternion>();
}
unsafe impl windows_core::Interface for SensorQuaternion {
    type Vtable = ISensorQuaternion_Vtbl;
    const IID: windows_core::GUID = <ISensorQuaternion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorQuaternion {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorQuaternion";
}
unsafe impl Send for SensorQuaternion {}
unsafe impl Sync for SensorQuaternion {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SensorRotationMatrix(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorRotationMatrix, windows_core::IUnknown, windows_core::IInspectable);
impl SensorRotationMatrix {
    pub fn M11(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M11)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M12(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M12)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M13(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M13)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M21(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M21)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M22(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M22)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M23(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M23)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M31(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M31)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M32(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn M33(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).M33)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SensorRotationMatrix {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorRotationMatrix>();
}
unsafe impl windows_core::Interface for SensorRotationMatrix {
    type Vtable = ISensorRotationMatrix_Vtbl;
    const IID: windows_core::GUID = <ISensorRotationMatrix as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorRotationMatrix {
    const NAME: &'static str = "Windows.Devices.Sensors.SensorRotationMatrix";
}
unsafe impl Send for SensorRotationMatrix {}
unsafe impl Sync for SensorRotationMatrix {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SimpleOrientationSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SimpleOrientationSensor, windows_core::IUnknown, windows_core::IInspectable);
impl SimpleOrientationSensor {
    pub fn GetCurrentOrientation(&self) -> windows_core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientationChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOrientationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOrientationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReadingTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Display")]
    pub fn ReadingTransform(&self) -> windows_core::Result<super::super::Graphics::Display::DisplayOrientations> {
        let this = &windows_core::Interface::cast::<ISimpleOrientationSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISimpleOrientationSensorDeviceId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<SimpleOrientationSensor> {
        Self::ISimpleOrientationSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>> {
        Self::ISimpleOrientationSensorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISimpleOrientationSensorStatics<R, F: FnOnce(&ISimpleOrientationSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISimpleOrientationSensorStatics2<R, F: FnOnce(&ISimpleOrientationSensorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SimpleOrientationSensor, ISimpleOrientationSensorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SimpleOrientationSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISimpleOrientationSensor>();
}
unsafe impl windows_core::Interface for SimpleOrientationSensor {
    type Vtable = ISimpleOrientationSensor_Vtbl;
    const IID: windows_core::GUID = <ISimpleOrientationSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SimpleOrientationSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensor";
}
unsafe impl Send for SimpleOrientationSensor {}
unsafe impl Sync for SimpleOrientationSensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SimpleOrientationSensorOrientationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SimpleOrientationSensorOrientationChangedEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Orientation(&self) -> windows_core::Result<SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SimpleOrientationSensorOrientationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISimpleOrientationSensorOrientationChangedEventArgs>();
}
unsafe impl windows_core::Interface for SimpleOrientationSensorOrientationChangedEventArgs {
    type Vtable = ISimpleOrientationSensorOrientationChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISimpleOrientationSensorOrientationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SimpleOrientationSensorOrientationChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.SimpleOrientationSensorOrientationChangedEventArgs";
}
unsafe impl Send for SimpleOrientationSensorOrientationChangedEventArgs {}
unsafe impl Sync for SimpleOrientationSensorOrientationChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WakeOnApproachOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WakeOnApproachOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WakeOnApproachOptions {
    pub fn AllowWhenExternalDisplayConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenExternalDisplayConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenExternalDisplayConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableWhenBatterySaverOn(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableWhenBatterySaverOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisableWhenBatterySaverOn(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisableWhenBatterySaverOn)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for WakeOnApproachOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWakeOnApproachOptions>();
}
unsafe impl windows_core::Interface for WakeOnApproachOptions {
    type Vtable = IWakeOnApproachOptions_Vtbl;
    const IID: windows_core::GUID = <IWakeOnApproachOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WakeOnApproachOptions {
    const NAME: &'static str = "Windows.Devices.Sensors.WakeOnApproachOptions";
}
unsafe impl Send for WakeOnApproachOptions {}
unsafe impl Sync for WakeOnApproachOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Gravity: Self = Self(2i32);
}
impl windows_core::TypeKind for AccelerometerReadingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AccelerometerReadingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AccelerometerReadingType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AccelerometerReadingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.AccelerometerReadingType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl windows_core::TypeKind for ActivitySensorReadingConfidence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivitySensorReadingConfidence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivitySensorReadingConfidence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivitySensorReadingConfidence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivitySensorReadingConfidence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Stationary: Self = Self(2i32);
    pub const Fidgeting: Self = Self(3i32);
    pub const Walking: Self = Self(4i32);
    pub const Running: Self = Self(5i32);
    pub const InVehicle: Self = Self(6i32);
    pub const Biking: Self = Self(7i32);
}
impl windows_core::TypeKind for ActivityType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivityType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivityType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivityType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.ActivityType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HumanEngagement(pub i32);
impl HumanEngagement {
    pub const Unknown: Self = Self(0i32);
    pub const Engaged: Self = Self(1i32);
    pub const Unengaged: Self = Self(2i32);
}
impl windows_core::TypeKind for HumanEngagement {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HumanEngagement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HumanEngagement").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HumanEngagement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.HumanEngagement;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HumanPresence(pub i32);
impl HumanPresence {
    pub const Unknown: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
    pub const NotPresent: Self = Self(2i32);
}
impl windows_core::TypeKind for HumanPresence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HumanPresence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HumanPresence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HumanPresence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.HumanPresence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
    pub const Approximate: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl windows_core::TypeKind for MagnetometerAccuracy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MagnetometerAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MagnetometerAccuracy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MagnetometerAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.MagnetometerAccuracy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: Self = Self(0i32);
    pub const Walking: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
}
impl windows_core::TypeKind for PedometerStepKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PedometerStepKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PedometerStepKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PedometerStepKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.PedometerStepKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: Self = Self(0i32);
    pub const PowerEfficiency: Self = Self(1i32);
}
impl windows_core::TypeKind for SensorOptimizationGoal {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SensorOptimizationGoal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SensorOptimizationGoal").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SensorOptimizationGoal {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorOptimizationGoal;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl windows_core::TypeKind for SensorReadingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SensorReadingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SensorReadingType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SensorReadingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorReadingType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: Self = Self(0i32);
    pub const ActivitySensor: Self = Self(1i32);
    pub const Barometer: Self = Self(2i32);
    pub const Compass: Self = Self(3i32);
    pub const CustomSensor: Self = Self(4i32);
    pub const Gyroscope: Self = Self(5i32);
    pub const ProximitySensor: Self = Self(6i32);
    pub const Inclinometer: Self = Self(7i32);
    pub const LightSensor: Self = Self(8i32);
    pub const OrientationSensor: Self = Self(9i32);
    pub const Pedometer: Self = Self(10i32);
    pub const RelativeInclinometer: Self = Self(11i32);
    pub const RelativeOrientationSensor: Self = Self(12i32);
    pub const SimpleOrientationSensor: Self = Self(13i32);
}
impl windows_core::TypeKind for SensorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SensorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SensorType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SensorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SensorType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: Self = Self(0i32);
    pub const Rotated90DegreesCounterclockwise: Self = Self(1i32);
    pub const Rotated180DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotated270DegreesCounterclockwise: Self = Self(3i32);
    pub const Faceup: Self = Self(4i32);
    pub const Facedown: Self = Self(5i32);
}
impl windows_core::TypeKind for SimpleOrientation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SimpleOrientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SimpleOrientation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SimpleOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sensors.SimpleOrientation;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

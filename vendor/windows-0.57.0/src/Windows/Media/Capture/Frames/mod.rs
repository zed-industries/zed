windows_core::imp::define_interface!(IAudioMediaFrame, IAudioMediaFrame_Vtbl, 0xa3a9feff_8021_441b_9a46_e7f0137b7981);
impl windows_core::RuntimeType for IAudioMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioMediaFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
    pub GetAudioFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBufferMediaFrame, IBufferMediaFrame_Vtbl, 0xb5b153c7_9b84_4062_b79c_a365b2596854);
impl windows_core::RuntimeType for IBufferMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBufferMediaFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
}
windows_core::imp::define_interface!(IDepthMediaFrame, IDepthMediaFrame_Vtbl, 0x47135e4f_8549_45c0_925b_80d35efdb10a);
impl windows_core::RuntimeType for IDepthMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDepthMediaFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    pub TryCreateCoordinateMapper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "Perception_Spatial")))]
    TryCreateCoordinateMapper: usize,
}
windows_core::imp::define_interface!(IDepthMediaFrame2, IDepthMediaFrame2_Vtbl, 0x6cca473d_c4a4_4176_b0cd_33eae3b35aa3);
impl windows_core::RuntimeType for IDepthMediaFrame2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDepthMediaFrame2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxReliableDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MinReliableDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDepthMediaFrameFormat, IDepthMediaFrameFormat_Vtbl, 0xc312cf40_d729_453e_8780_2e04f140d28e);
impl windows_core::RuntimeType for IDepthMediaFrameFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDepthMediaFrameFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DepthScaleInMeters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInfraredMediaFrame, IInfraredMediaFrame_Vtbl, 0x3fd13503_004b_4f0e_91ac_465299b41658);
impl windows_core::RuntimeType for IInfraredMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInfraredMediaFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsIlluminated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameArrivedEventArgs, IMediaFrameArrivedEventArgs_Vtbl, 0x0b430add_a490_4435_ada1_9affd55239f7);
impl windows_core::RuntimeType for IMediaFrameArrivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameArrivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaFrameFormat, IMediaFrameFormat_Vtbl, 0x71902b4e_b279_4a97_a9db_bd5a2fb78f39);
impl windows_core::RuntimeType for IMediaFrameFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MajorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subtype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameRate: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub VideoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameFormat2, IMediaFrameFormat2_Vtbl, 0x63856340_5e87_4c10_86d1_6df097a6c6a8);
impl windows_core::RuntimeType for IMediaFrameFormat2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameFormat2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub AudioEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    AudioEncodingProperties: usize,
}
windows_core::imp::define_interface!(IMediaFrameReader, IMediaFrameReader_Vtbl, 0xe4c94395_2028_48ed_90b0_d1c1b162e24c);
impl windows_core::RuntimeType for IMediaFrameReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TryAcquireLatestFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameReader2, IMediaFrameReader2_Vtbl, 0x871127b3_8531_4050_87cc_a13733cf3e9b);
impl windows_core::RuntimeType for IMediaFrameReader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameReader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAcquisitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, MediaFrameReaderAcquisitionMode) -> windows_core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaFrameReaderAcquisitionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameReference, IMediaFrameReference_Vtbl, 0xf6b88641_f0dc_4044_8dc9_961cedd05bad);
impl windows_core::RuntimeType for IMediaFrameReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaFrameSourceKind) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemRelativeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub BufferMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
}
windows_core::imp::define_interface!(IMediaFrameReference2, IMediaFrameReference2_Vtbl, 0xddbc3ecc_d5b2_49ef_836a_947d989b80c1);
impl windows_core::RuntimeType for IMediaFrameReference2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameReference2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameSource, IMediaFrameSource_Vtbl, 0xd6782953_90db_46a8_8add_2aa884a8d253);
impl windows_core::RuntimeType for IMediaFrameSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Controller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFormats: usize,
    pub CurrentFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFormatChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices_Core")]
    pub TryGetCameraIntrinsics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    TryGetCameraIntrinsics: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceController, IMediaFrameSourceController_Vtbl, 0x6d076635_316d_4b8f_b7b6_eeb04a8c6525);
impl windows_core::RuntimeType for IMediaFrameSourceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPropertyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPropertyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceController2, IMediaFrameSourceController2_Vtbl, 0xefc49fd4_fcf2_4a03_b4e4_ac9628739bee);
impl windows_core::RuntimeType for IMediaFrameSourceController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPropertyByExtendedIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPropertyByExtendedIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameSourceController3, IMediaFrameSourceController3_Vtbl, 0x1f0cf815_2464_4651_b1e8_4a82dbdb54de);
impl windows_core::RuntimeType for IMediaFrameSourceController3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceController3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceGetPropertyResult, IMediaFrameSourceGetPropertyResult_Vtbl, 0x088616c2_3a64_4bd5_bd2b_e7c898d2f37a);
impl windows_core::RuntimeType for IMediaFrameSourceGetPropertyResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceGetPropertyResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaFrameSourceGetPropertyStatus) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameSourceGroup, IMediaFrameSourceGroup_Vtbl, 0x7f605b87_4832_4b5f_ae3d_412faab37d34);
impl windows_core::RuntimeType for IMediaFrameSourceGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceInfos: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceGroupStatics, IMediaFrameSourceGroupStatics_Vtbl, 0x1c48bfc5_436f_4508_94cf_d5d8b7326445);
impl windows_core::RuntimeType for IMediaFrameSourceGroupStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceGroupStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaFrameSourceInfo, IMediaFrameSourceInfo_Vtbl, 0x87bdc9cd_4601_408f_91cf_038318cd0af3);
impl windows_core::RuntimeType for IMediaFrameSourceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MediaStreamType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaStreamType) -> windows_core::HRESULT,
    pub SourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaFrameSourceKind) -> windows_core::HRESULT,
    pub SourceGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceInfo2, IMediaFrameSourceInfo2_Vtbl, 0x195a7855_6457_42c6_a769_19b65bd32e6e);
impl windows_core::RuntimeType for IMediaFrameSourceInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProfileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoProfileMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoProfileMediaDescription: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceInfo3, IMediaFrameSourceInfo3_Vtbl, 0xca824ab6_66ea_5885_a2b6_26c0eeec3c7b);
impl windows_core::RuntimeType for IMediaFrameSourceInfo3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceInfo3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub GetRelativePanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Devices::Enumeration::Panel) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement")))]
    GetRelativePanel: usize,
}
windows_core::imp::define_interface!(IMediaFrameSourceInfo4, IMediaFrameSourceInfo4_Vtbl, 0x4817d721_85eb_470c_8f37_43ca5498e41d);
impl windows_core::RuntimeType for IMediaFrameSourceInfo4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaFrameSourceInfo4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsShareable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiSourceMediaFrameArrivedEventArgs, IMultiSourceMediaFrameArrivedEventArgs_Vtbl, 0x63115e01_cf51_48fd_aab0_6d693eb48127);
impl windows_core::RuntimeType for IMultiSourceMediaFrameArrivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMultiSourceMediaFrameArrivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMultiSourceMediaFrameReader, IMultiSourceMediaFrameReader_Vtbl, 0x8d144402_f763_488d_98f2_b437bcf075e7);
impl windows_core::RuntimeType for IMultiSourceMediaFrameReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TryAcquireLatestFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiSourceMediaFrameReader2, IMultiSourceMediaFrameReader2_Vtbl, 0xef5c8abd_fc5c_4c6b_9d81_3cb9cc637c26);
impl windows_core::RuntimeType for IMultiSourceMediaFrameReader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAcquisitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, MediaFrameReaderAcquisitionMode) -> windows_core::HRESULT,
    pub AcquisitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaFrameReaderAcquisitionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiSourceMediaFrameReference, IMultiSourceMediaFrameReference_Vtbl, 0x21964b1a_7fe2_44d6_92e5_298e6d2810e9);
impl windows_core::RuntimeType for IMultiSourceMediaFrameReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMultiSourceMediaFrameReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetFrameReferenceBySourceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoMediaFrame, IVideoMediaFrame_Vtbl, 0x00dd4ccb_32bd_4fe1_a013_7cc13cf5dbcf);
impl windows_core::RuntimeType for IVideoMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoMediaFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
    #[cfg(feature = "Media_Devices_Core")]
    pub CameraIntrinsics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    CameraIntrinsics: usize,
    pub InfraredMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DepthMediaFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoMediaFrameFormat, IVideoMediaFrameFormat_Vtbl, 0x46027fc0_d71b_45c7_8f14_6d9a0ae604e4);
impl windows_core::RuntimeType for IVideoMediaFrameFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoMediaFrameFormat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MediaFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DepthFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioMediaFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioMediaFrame, windows_core::IUnknown, windows_core::IInspectable);
impl AudioMediaFrame {
    pub fn FrameReference(&self) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn AudioEncodingProperties(&self) -> windows_core::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAudioFrame(&self) -> windows_core::Result<super::super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioMediaFrame>();
}
unsafe impl windows_core::Interface for AudioMediaFrame {
    type Vtable = IAudioMediaFrame_Vtbl;
    const IID: windows_core::GUID = <IAudioMediaFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.AudioMediaFrame";
}
unsafe impl Send for AudioMediaFrame {}
unsafe impl Sync for AudioMediaFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BufferMediaFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BufferMediaFrame, windows_core::IUnknown, windows_core::IInspectable);
impl BufferMediaFrame {
    pub fn FrameReference(&self) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Buffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BufferMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBufferMediaFrame>();
}
unsafe impl windows_core::Interface for BufferMediaFrame {
    type Vtable = IBufferMediaFrame_Vtbl;
    const IID: windows_core::GUID = <IBufferMediaFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BufferMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.BufferMediaFrame";
}
unsafe impl Send for BufferMediaFrame {}
unsafe impl Sync for BufferMediaFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DepthMediaFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DepthMediaFrame, windows_core::IUnknown, windows_core::IInspectable);
impl DepthMediaFrame {
    pub fn FrameReference(&self) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoMediaFrame(&self) -> windows_core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DepthFormat(&self) -> windows_core::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepthFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial"))]
    pub fn TryCreateCoordinateMapper<P0, P1>(&self, cameraintrinsics: P0, coordinatesystem: P1) -> windows_core::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>
    where
        P0: windows_core::Param<super::super::Devices::Core::CameraIntrinsics>,
        P1: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateCoordinateMapper)(windows_core::Interface::as_raw(this), cameraintrinsics.param().abi(), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxReliableDepth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxReliableDepth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinReliableDepth(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IDepthMediaFrame2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinReliableDepth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DepthMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDepthMediaFrame>();
}
unsafe impl windows_core::Interface for DepthMediaFrame {
    type Vtable = IDepthMediaFrame_Vtbl;
    const IID: windows_core::GUID = <IDepthMediaFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DepthMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrame";
}
unsafe impl Send for DepthMediaFrame {}
unsafe impl Sync for DepthMediaFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DepthMediaFrameFormat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DepthMediaFrameFormat, windows_core::IUnknown, windows_core::IInspectable);
impl DepthMediaFrameFormat {
    pub fn VideoFormat(&self) -> windows_core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DepthScaleInMeters(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepthScaleInMeters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DepthMediaFrameFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDepthMediaFrameFormat>();
}
unsafe impl windows_core::Interface for DepthMediaFrameFormat {
    type Vtable = IDepthMediaFrameFormat_Vtbl;
    const IID: windows_core::GUID = <IDepthMediaFrameFormat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DepthMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.DepthMediaFrameFormat";
}
unsafe impl Send for DepthMediaFrameFormat {}
unsafe impl Sync for DepthMediaFrameFormat {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InfraredMediaFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InfraredMediaFrame, windows_core::IUnknown, windows_core::IInspectable);
impl InfraredMediaFrame {
    pub fn FrameReference(&self) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoMediaFrame(&self) -> windows_core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsIlluminated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIlluminated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InfraredMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInfraredMediaFrame>();
}
unsafe impl windows_core::Interface for InfraredMediaFrame {
    type Vtable = IInfraredMediaFrame_Vtbl;
    const IID: windows_core::GUID = <IInfraredMediaFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InfraredMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.InfraredMediaFrame";
}
unsafe impl Send for InfraredMediaFrame {}
unsafe impl Sync for InfraredMediaFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameArrivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameArrivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameArrivedEventArgs {}
impl windows_core::RuntimeType for MediaFrameArrivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameArrivedEventArgs>();
}
unsafe impl windows_core::Interface for MediaFrameArrivedEventArgs {
    type Vtable = IMediaFrameArrivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameArrivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs";
}
unsafe impl Send for MediaFrameArrivedEventArgs {}
unsafe impl Sync for MediaFrameArrivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameFormat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameFormat, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameFormat {
    pub fn MajorType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MajorType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subtype(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtype)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameRate(&self) -> windows_core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn VideoFormat(&self) -> windows_core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn AudioEncodingProperties(&self) -> windows_core::Result<super::super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IMediaFrameFormat2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaFrameFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameFormat>();
}
unsafe impl windows_core::Interface for MediaFrameFormat {
    type Vtable = IMediaFrameFormat_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameFormat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameFormat";
}
unsafe impl Send for MediaFrameFormat {}
unsafe impl Sync for MediaFrameFormat {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameReader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaFrameReader, super::super::super::Foundation::IClosable);
impl MediaFrameReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FrameArrived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameArrived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameArrived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryAcquireLatestFrame(&self) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireLatestFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAcquisitionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionMode(&self) -> windows_core::Result<MediaFrameReaderAcquisitionMode> {
        let this = &windows_core::Interface::cast::<IMediaFrameReader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcquisitionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaFrameReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameReader>();
}
unsafe impl windows_core::Interface for MediaFrameReader {
    type Vtable = IMediaFrameReader_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReader";
}
unsafe impl Send for MediaFrameReader {}
unsafe impl Sync for MediaFrameReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameReference, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaFrameReference, super::super::super::Foundation::IClosable);
impl MediaFrameReference {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SourceKind(&self) -> windows_core::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemRelativeTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
    pub fn BufferMediaFrame(&self) -> windows_core::Result<BufferMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoMediaFrame(&self) -> windows_core::Result<VideoMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> windows_core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AudioMediaFrame(&self) -> windows_core::Result<AudioMediaFrame> {
        let this = &windows_core::Interface::cast::<IMediaFrameReference2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaFrameReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameReference>();
}
unsafe impl windows_core::Interface for MediaFrameReference {
    type Vtable = IMediaFrameReference_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameReference";
}
unsafe impl Send for MediaFrameReference {}
unsafe impl Sync for MediaFrameReference {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameSource, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameSource {
    pub fn Info(&self) -> windows_core::Result<MediaFrameSourceInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Info)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Controller(&self) -> windows_core::Result<MediaFrameSourceController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Controller)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFormats(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentFormat(&self) -> windows_core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFormatAsync<P0>(&self, format: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<MediaFrameFormat>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetFormatAsync)(windows_core::Interface::as_raw(this), format.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<MediaFrameSource, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFormatChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFormatChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_Devices_Core")]
    pub fn TryGetCameraIntrinsics<P0>(&self, format: P0) -> windows_core::Result<super::super::Devices::Core::CameraIntrinsics>
    where
        P0: windows_core::Param<MediaFrameFormat>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetCameraIntrinsics)(windows_core::Interface::as_raw(this), format.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaFrameSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameSource>();
}
unsafe impl windows_core::Interface for MediaFrameSource {
    type Vtable = IMediaFrameSource_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameSource {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSource";
}
unsafe impl Send for MediaFrameSource {}
unsafe impl Sync for MediaFrameSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameSourceController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameSourceController, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameSourceController {
    pub fn GetPropertyAsync(&self, propertyid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPropertyAsync<P0>(&self, propertyid: &windows_core::HSTRING, propertyvalue: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPropertyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyid), propertyvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> windows_core::Result<super::super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertyByExtendedIdAsync<P0>(&self, extendedpropertyid: &[u8], maxpropertyvaluesize: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyByExtendedIdAsync)(windows_core::Interface::as_raw(this), extendedpropertyid.len().try_into().unwrap(), extendedpropertyid.as_ptr(), maxpropertyvaluesize.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[u8], propertyvalue: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>> {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPropertyByExtendedIdAsync)(windows_core::Interface::as_raw(this), extendedpropertyid.len().try_into().unwrap(), extendedpropertyid.as_ptr(), propertyvalue.len().try_into().unwrap(), propertyvalue.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> windows_core::Result<super::super::Devices::AudioDeviceController> {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceController3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaFrameSourceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameSourceController>();
}
unsafe impl windows_core::Interface for MediaFrameSourceController {
    type Vtable = IMediaFrameSourceController_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameSourceController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameSourceController {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceController";
}
unsafe impl Send for MediaFrameSourceController {}
unsafe impl Sync for MediaFrameSourceController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameSourceGetPropertyResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameSourceGetPropertyResult, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameSourceGetPropertyResult {
    pub fn Status(&self) -> windows_core::Result<MediaFrameSourceGetPropertyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaFrameSourceGetPropertyResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameSourceGetPropertyResult>();
}
unsafe impl windows_core::Interface for MediaFrameSourceGetPropertyResult {
    type Vtable = IMediaFrameSourceGetPropertyResult_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameSourceGetPropertyResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameSourceGetPropertyResult {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult";
}
unsafe impl Send for MediaFrameSourceGetPropertyResult {}
unsafe impl Sync for MediaFrameSourceGetPropertyResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameSourceGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameSourceGroup, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameSourceGroup {
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceInfos(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(id: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaFrameSourceGroupStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaFrameSourceGroupStatics<R, F: FnOnce(&IMediaFrameSourceGroupStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaFrameSourceGroup, IMediaFrameSourceGroupStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MediaFrameSourceGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameSourceGroup>();
}
unsafe impl windows_core::Interface for MediaFrameSourceGroup {
    type Vtable = IMediaFrameSourceGroup_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameSourceGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameSourceGroup {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceGroup";
}
unsafe impl Send for MediaFrameSourceGroup {}
unsafe impl Sync for MediaFrameSourceGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaFrameSourceInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaFrameSourceInfo, windows_core::IUnknown, windows_core::IInspectable);
impl MediaFrameSourceInfo {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaStreamType(&self) -> windows_core::Result<super::MediaStreamType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaStreamType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceKind(&self) -> windows_core::Result<MediaFrameSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceGroup(&self) -> windows_core::Result<MediaFrameSourceGroup> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> windows_core::Result<super::super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> windows_core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProfileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoProfileMediaDescription(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>> {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProfileMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement"))]
    pub fn GetRelativePanel<P0>(&self, displayregion: P0) -> windows_core::Result<super::super::super::Devices::Enumeration::Panel>
    where
        P0: windows_core::Param<super::super::super::UI::WindowManagement::DisplayRegion>,
    {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceInfo3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRelativePanel)(windows_core::Interface::as_raw(this), displayregion.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsShareable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaFrameSourceInfo4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsShareable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaFrameSourceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaFrameSourceInfo>();
}
unsafe impl windows_core::Interface for MediaFrameSourceInfo {
    type Vtable = IMediaFrameSourceInfo_Vtbl;
    const IID: windows_core::GUID = <IMediaFrameSourceInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaFrameSourceInfo {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MediaFrameSourceInfo";
}
unsafe impl Send for MediaFrameSourceInfo {}
unsafe impl Sync for MediaFrameSourceInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MultiSourceMediaFrameArrivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MultiSourceMediaFrameArrivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MultiSourceMediaFrameArrivedEventArgs {}
impl windows_core::RuntimeType for MultiSourceMediaFrameArrivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMultiSourceMediaFrameArrivedEventArgs>();
}
unsafe impl windows_core::Interface for MultiSourceMediaFrameArrivedEventArgs {
    type Vtable = IMultiSourceMediaFrameArrivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMultiSourceMediaFrameArrivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MultiSourceMediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs";
}
unsafe impl Send for MultiSourceMediaFrameArrivedEventArgs {}
unsafe impl Sync for MultiSourceMediaFrameArrivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MultiSourceMediaFrameReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MultiSourceMediaFrameReader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MultiSourceMediaFrameReader, super::super::super::Foundation::IClosable);
impl MultiSourceMediaFrameReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FrameArrived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameArrived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameArrived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryAcquireLatestFrame(&self) -> windows_core::Result<MultiSourceMediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireLatestFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAcquisitionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionMode(&self) -> windows_core::Result<MediaFrameReaderAcquisitionMode> {
        let this = &windows_core::Interface::cast::<IMultiSourceMediaFrameReader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcquisitionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MultiSourceMediaFrameReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMultiSourceMediaFrameReader>();
}
unsafe impl windows_core::Interface for MultiSourceMediaFrameReader {
    type Vtable = IMultiSourceMediaFrameReader_Vtbl;
    const IID: windows_core::GUID = <IMultiSourceMediaFrameReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MultiSourceMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReader";
}
unsafe impl Send for MultiSourceMediaFrameReader {}
unsafe impl Sync for MultiSourceMediaFrameReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MultiSourceMediaFrameReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MultiSourceMediaFrameReference, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MultiSourceMediaFrameReference, super::super::super::Foundation::IClosable);
impl MultiSourceMediaFrameReference {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryGetFrameReferenceBySourceId(&self, sourceid: &windows_core::HSTRING) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetFrameReferenceBySourceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sourceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MultiSourceMediaFrameReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMultiSourceMediaFrameReference>();
}
unsafe impl windows_core::Interface for MultiSourceMediaFrameReference {
    type Vtable = IMultiSourceMediaFrameReference_Vtbl;
    const IID: windows_core::GUID = <IMultiSourceMediaFrameReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MultiSourceMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.MultiSourceMediaFrameReference";
}
unsafe impl Send for MultiSourceMediaFrameReference {}
unsafe impl Sync for MultiSourceMediaFrameReference {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct VideoMediaFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoMediaFrame, windows_core::IUnknown, windows_core::IInspectable);
impl VideoMediaFrame {
    pub fn FrameReference(&self) -> windows_core::Result<MediaFrameReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoFormat(&self) -> windows_core::Result<VideoMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> windows_core::Result<super::super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoftwareBitmap)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3DSurface)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices_Core")]
    pub fn CameraIntrinsics(&self) -> windows_core::Result<super::super::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraIntrinsics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InfraredMediaFrame(&self) -> windows_core::Result<InfraredMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InfraredMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DepthMediaFrame(&self) -> windows_core::Result<DepthMediaFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepthMediaFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetVideoFrame(&self) -> windows_core::Result<super::super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVideoFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VideoMediaFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoMediaFrame>();
}
unsafe impl windows_core::Interface for VideoMediaFrame {
    type Vtable = IVideoMediaFrame_Vtbl;
    const IID: windows_core::GUID = <IVideoMediaFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrame";
}
unsafe impl Send for VideoMediaFrame {}
unsafe impl Sync for VideoMediaFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct VideoMediaFrameFormat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoMediaFrameFormat, windows_core::IUnknown, windows_core::IInspectable);
impl VideoMediaFrameFormat {
    pub fn MediaFrameFormat(&self) -> windows_core::Result<MediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaFrameFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DepthFormat(&self) -> windows_core::Result<DepthMediaFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepthFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
}
impl windows_core::RuntimeType for VideoMediaFrameFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoMediaFrameFormat>();
}
unsafe impl windows_core::Interface for VideoMediaFrameFormat {
    type Vtable = IVideoMediaFrameFormat_Vtbl;
    const IID: windows_core::GUID = <IVideoMediaFrameFormat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.VideoMediaFrameFormat";
}
unsafe impl Send for VideoMediaFrameFormat {}
unsafe impl Sync for VideoMediaFrameFormat {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: Self = Self(0i32);
    pub const Buffered: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaFrameReaderAcquisitionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaFrameReaderAcquisitionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaFrameReaderAcquisitionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaFrameReaderAcquisitionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderAcquisitionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const OutputFormatNotSupported: Self = Self(3i32);
    pub const ExclusiveControlNotAvailable: Self = Self(4i32);
}
impl windows_core::TypeKind for MediaFrameReaderStartStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaFrameReaderStartStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaFrameReaderStartStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaFrameReaderStartStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameReaderStartStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(4i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(5i32);
}
impl windows_core::TypeKind for MediaFrameSourceGetPropertyStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaFrameSourceGetPropertyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaFrameSourceGetPropertyStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaFrameSourceGetPropertyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const Infrared: Self = Self(2i32);
    pub const Depth: Self = Self(3i32);
    pub const Audio: Self = Self(4i32);
    pub const Image: Self = Self(5i32);
    pub const Metadata: Self = Self(6i32);
}
impl windows_core::TypeKind for MediaFrameSourceKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaFrameSourceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaFrameSourceKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaFrameSourceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl windows_core::TypeKind for MediaFrameSourceSetPropertyStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaFrameSourceSetPropertyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaFrameSourceSetPropertyStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaFrameSourceSetPropertyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const NotSupported: Self = Self(1i32);
    pub const InsufficientResources: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl windows_core::TypeKind for MultiSourceMediaFrameReaderStartStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MultiSourceMediaFrameReaderStartStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MultiSourceMediaFrameReaderStartStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MultiSourceMediaFrameReaderStartStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus;i4)");
}

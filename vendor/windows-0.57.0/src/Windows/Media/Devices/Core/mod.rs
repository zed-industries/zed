windows_core::imp::define_interface!(ICameraIntrinsics, ICameraIntrinsics_Vtbl, 0x0aa6ed32_6589_49da_afde_594270ca0aac);
impl windows_core::RuntimeType for ICameraIntrinsics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraIntrinsics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FocalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FocalLength: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PrincipalPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PrincipalPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RadialDistortion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RadialDistortion: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TangentialDistortion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TangentialDistortion: usize,
    pub ImageWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ImageHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectOntoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectAtUnitDepth: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut super::super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectAtUnitDepth: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectManyOntoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Foundation::Numerics::Vector3, u32, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectManyOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectPixelsAtUnitDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Foundation::Point, u32, *mut super::super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectPixelsAtUnitDepth: usize,
}
windows_core::imp::define_interface!(ICameraIntrinsics2, ICameraIntrinsics2_Vtbl, 0x0cdaa447_0798_4b4d_839f_c5ec414db27a);
impl windows_core::RuntimeType for ICameraIntrinsics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraIntrinsics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub UndistortedProjectionTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UndistortedProjectionTransform: usize,
    pub DistortPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub DistortPoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Foundation::Point, u32, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub UndistortPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub UndistortPoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Foundation::Point, u32, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraIntrinsicsFactory, ICameraIntrinsicsFactory_Vtbl, 0xc0ddc486_2132_4a34_a659_9bfe2a055712);
impl windows_core::RuntimeType for ICameraIntrinsicsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraIntrinsicsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector2, super::super::super::Foundation::Numerics::Vector2, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Vector2, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
windows_core::imp::define_interface!(IDepthCorrelatedCoordinateMapper, IDepthCorrelatedCoordinateMapper_Vtbl, 0xf95d89fb_8af0_4cb0_926d_696866e5046a);
impl windows_core::RuntimeType for IDepthCorrelatedCoordinateMapper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDepthCorrelatedCoordinateMapper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Foundation::Point, *mut core::ffi::c_void, u32, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoints: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub MapPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    MapPoint: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub MapPoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Foundation::Point, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    MapPoints: usize,
}
windows_core::imp::define_interface!(IFrameControlCapabilities, IFrameControlCapabilities_Vtbl, 0xa8ffae60_4e9e_4377_a789_e24c4ae7e544);
impl windows_core::RuntimeType for IFrameControlCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameControlCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Exposure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Focus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhotoConfirmationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameControlCapabilities2, IFrameControlCapabilities2_Vtbl, 0xce9b0464_4730_440f_bd3e_efe8a8f230a8);
impl windows_core::RuntimeType for IFrameControlCapabilities2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameControlCapabilities2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Flash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameController, IFrameController_Vtbl, 0xc16459d9_baef_4052_9177_48aff2af7522);
impl windows_core::RuntimeType for IFrameController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExposureControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhotoConfirmationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPhotoConfirmationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameController2, IFrameController2_Vtbl, 0x00d3bc75_d87c_485b_8a09_5c358568b427);
impl windows_core::RuntimeType for IFrameController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FlashControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameExposureCapabilities, IFrameExposureCapabilities_Vtbl, 0xbdbe9ce3_3985_4e72_97c2_0590d61307a1);
impl windows_core::RuntimeType for IFrameExposureCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameExposureCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameExposureCompensationCapabilities, IFrameExposureCompensationCapabilities_Vtbl, 0xb988a823_8065_41ee_b04f_722265954500);
impl windows_core::RuntimeType for IFrameExposureCompensationCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameExposureCompensationCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameExposureCompensationControl, IFrameExposureCompensationControl_Vtbl, 0xe95896c9_f7f9_48ca_8591_a26531cb1578);
impl windows_core::RuntimeType for IFrameExposureCompensationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameExposureCompensationControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameExposureControl, IFrameExposureControl_Vtbl, 0xb1605a61_ffaf_4752_b621_f5b6f117f432);
impl windows_core::RuntimeType for IFrameExposureControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameExposureControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Auto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameFlashCapabilities, IFrameFlashCapabilities_Vtbl, 0xbb9341a2_5ebe_4f62_8223_0e2b05bfbbd0);
impl windows_core::RuntimeType for IFrameFlashCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameFlashCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameFlashControl, IFrameFlashControl_Vtbl, 0x75d5f6c7_bd45_4fab_9375_45ac04b332c2);
impl windows_core::RuntimeType for IFrameFlashControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameFlashControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FrameFlashMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, FrameFlashMode) -> windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameFocusCapabilities, IFrameFocusCapabilities_Vtbl, 0x7b25cd58_01c0_4065_9c40_c1a721425c1a);
impl windows_core::RuntimeType for IFrameFocusCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameFocusCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameFocusControl, IFrameFocusControl_Vtbl, 0x272df1d0_d912_4214_a67b_e38a8d48d8c6);
impl windows_core::RuntimeType for IFrameFocusControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameFocusControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameIsoSpeedCapabilities, IFrameIsoSpeedCapabilities_Vtbl, 0x16bdff61_6df6_4ac9_b92a_9f6ecd1ad2fa);
impl windows_core::RuntimeType for IFrameIsoSpeedCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameIsoSpeedCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameIsoSpeedControl, IFrameIsoSpeedControl_Vtbl, 0x1a03efed_786a_4c75_a557_7ab9a85f588c);
impl windows_core::RuntimeType for IFrameIsoSpeedControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameIsoSpeedControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Auto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVariablePhotoSequenceController, IVariablePhotoSequenceController_Vtbl, 0x7fbff880_ed8c_43fd_a7c3_b35809e4229a);
impl windows_core::RuntimeType for IVariablePhotoSequenceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVariablePhotoSequenceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub FrameCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFrameControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFrameControllers: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CameraIntrinsics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraIntrinsics, windows_core::IUnknown, windows_core::IInspectable);
impl CameraIntrinsics {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FocalLength(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocalLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PrincipalPoint(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrincipalPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RadialDistortion(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RadialDistortion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TangentialDistortion(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TangentialDistortion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ImageWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ImageHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectOntoFrame(&self, coordinate: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectOntoFrame)(windows_core::Interface::as_raw(this), coordinate, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UnprojectAtUnitDepth(&self, pixelcoordinate: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnprojectAtUnitDepth)(windows_core::Interface::as_raw(this), pixelcoordinate, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectManyOntoFrame(&self, coordinates: &[super::super::super::Foundation::Numerics::Vector3], results: &mut [super::super::super::Foundation::Point]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProjectManyOntoFrame)(windows_core::Interface::as_raw(this), coordinates.len().try_into().unwrap(), coordinates.as_ptr(), results.len().try_into().unwrap(), results.as_mut_ptr()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UnprojectPixelsAtUnitDepth(&self, pixelcoordinates: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Numerics::Vector2]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnprojectPixelsAtUnitDepth)(windows_core::Interface::as_raw(this), pixelcoordinates.len().try_into().unwrap(), pixelcoordinates.as_ptr(), results.len().try_into().unwrap(), results.as_mut_ptr()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UndistortedProjectionTransform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndistortedProjectionTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DistortPoint(&self, input: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistortPoint)(windows_core::Interface::as_raw(this), input, &mut result__).map(|| result__)
        }
    }
    pub fn DistortPoints(&self, inputs: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Point]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DistortPoints)(windows_core::Interface::as_raw(this), inputs.len().try_into().unwrap(), inputs.as_ptr(), results.len().try_into().unwrap(), results.as_mut_ptr()).ok() }
    }
    pub fn UndistortPoint(&self, input: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UndistortPoint)(windows_core::Interface::as_raw(this), input, &mut result__).map(|| result__)
        }
    }
    pub fn UndistortPoints(&self, inputs: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Point]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UndistortPoints)(windows_core::Interface::as_raw(this), inputs.len().try_into().unwrap(), inputs.as_ptr(), results.len().try_into().unwrap(), results.as_mut_ptr()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create(focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32) -> windows_core::Result<CameraIntrinsics> {
        Self::ICameraIntrinsicsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), focallength, principalpoint, radialdistortion, tangentialdistortion, imagewidth, imageheight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICameraIntrinsicsFactory<R, F: FnOnce(&ICameraIntrinsicsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CameraIntrinsics, ICameraIntrinsicsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CameraIntrinsics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraIntrinsics>();
}
unsafe impl windows_core::Interface for CameraIntrinsics {
    type Vtable = ICameraIntrinsics_Vtbl;
    const IID: windows_core::GUID = <ICameraIntrinsics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraIntrinsics {
    const NAME: &'static str = "Windows.Media.Devices.Core.CameraIntrinsics";
}
unsafe impl Send for CameraIntrinsics {}
unsafe impl Sync for CameraIntrinsics {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DepthCorrelatedCoordinateMapper(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DepthCorrelatedCoordinateMapper, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DepthCorrelatedCoordinateMapper, super::super::super::Foundation::IClosable);
impl DepthCorrelatedCoordinateMapper {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoint<P0>(&self, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: P0) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnprojectPoint)(windows_core::Interface::as_raw(this), sourcepoint, targetcoordinatesystem.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoints<P0>(&self, sourcepoints: &[super::super::super::Foundation::Point], targetcoordinatesystem: P0, results: &mut [super::super::super::Foundation::Numerics::Vector3]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnprojectPoints)(windows_core::Interface::as_raw(this), sourcepoints.len().try_into().unwrap(), sourcepoints.as_ptr(), targetcoordinatesystem.param().abi(), results.len().try_into().unwrap(), results.as_mut_ptr()).ok() }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn MapPoint<P0, P1>(&self, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: P0, targetcameraintrinsics: P1) -> windows_core::Result<super::super::super::Foundation::Point>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
        P1: windows_core::Param<CameraIntrinsics>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapPoint)(windows_core::Interface::as_raw(this), sourcepoint, targetcoordinatesystem.param().abi(), targetcameraintrinsics.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn MapPoints<P0, P1>(&self, sourcepoints: &[super::super::super::Foundation::Point], targetcoordinatesystem: P0, targetcameraintrinsics: P1, results: &mut [super::super::super::Foundation::Point]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
        P1: windows_core::Param<CameraIntrinsics>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).MapPoints)(windows_core::Interface::as_raw(this), sourcepoints.len().try_into().unwrap(), sourcepoints.as_ptr(), targetcoordinatesystem.param().abi(), targetcameraintrinsics.param().abi(), results.len().try_into().unwrap(), results.as_mut_ptr()).ok() }
    }
}
impl windows_core::RuntimeType for DepthCorrelatedCoordinateMapper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDepthCorrelatedCoordinateMapper>();
}
unsafe impl windows_core::Interface for DepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: windows_core::GUID = <IDepthCorrelatedCoordinateMapper as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper";
}
unsafe impl Send for DepthCorrelatedCoordinateMapper {}
unsafe impl Sync for DepthCorrelatedCoordinateMapper {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameControlCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameControlCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl FrameControlCapabilities {
    pub fn Exposure(&self) -> windows_core::Result<FrameExposureCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exposure)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExposureCompensation(&self) -> windows_core::Result<FrameExposureCompensationCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposureCompensation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsoSpeed(&self) -> windows_core::Result<FrameIsoSpeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsoSpeed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Focus(&self) -> windows_core::Result<FrameFocusCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhotoConfirmationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoConfirmationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Flash(&self) -> windows_core::Result<FrameFlashCapabilities> {
        let this = &windows_core::Interface::cast::<IFrameControlCapabilities2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Flash)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FrameControlCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameControlCapabilities>();
}
unsafe impl windows_core::Interface for FrameControlCapabilities {
    type Vtable = IFrameControlCapabilities_Vtbl;
    const IID: windows_core::GUID = <IFrameControlCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameControlCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameController, windows_core::IUnknown, windows_core::IInspectable);
impl FrameController {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameController, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ExposureControl(&self) -> windows_core::Result<FrameExposureControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposureControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExposureCompensationControl(&self) -> windows_core::Result<FrameExposureCompensationControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposureCompensationControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsoSpeedControl(&self) -> windows_core::Result<FrameIsoSpeedControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsoSpeedControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusControl(&self) -> windows_core::Result<FrameFocusControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhotoConfirmationEnabled(&self) -> windows_core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoConfirmationEnabled)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPhotoConfirmationEnabled<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPhotoConfirmationEnabled)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FlashControl(&self) -> windows_core::Result<FrameFlashControl> {
        let this = &windows_core::Interface::cast::<IFrameController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlashControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FrameController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameController>();
}
unsafe impl windows_core::Interface for FrameController {
    type Vtable = IFrameController_Vtbl;
    const IID: windows_core::GUID = <IFrameController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameController {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameController";
}
unsafe impl Send for FrameController {}
unsafe impl Sync for FrameController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameExposureCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameExposureCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl FrameExposureCapabilities {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FrameExposureCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameExposureCapabilities>();
}
unsafe impl windows_core::Interface for FrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilities_Vtbl;
    const IID: windows_core::GUID = <IFrameExposureCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameExposureCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameExposureCompensationCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameExposureCompensationCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl FrameExposureCompensationCapabilities {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FrameExposureCompensationCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameExposureCompensationCapabilities>();
}
unsafe impl windows_core::Interface for FrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilities_Vtbl;
    const IID: windows_core::GUID = <IFrameExposureCompensationCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameExposureCompensationCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameExposureCompensationControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameExposureCompensationControl, windows_core::IUnknown, windows_core::IInspectable);
impl FrameExposureCompensationControl {
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<f32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for FrameExposureCompensationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameExposureCompensationControl>();
}
unsafe impl windows_core::Interface for FrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControl_Vtbl;
    const IID: windows_core::GUID = <IFrameExposureCompensationControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameExposureControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameExposureControl, windows_core::IUnknown, windows_core::IInspectable);
impl FrameExposureControl {
    pub fn Auto(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Auto)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuto)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for FrameExposureControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameExposureControl>();
}
unsafe impl windows_core::Interface for FrameExposureControl {
    type Vtable = IFrameExposureControl_Vtbl;
    const IID: windows_core::GUID = <IFrameExposureControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameFlashCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameFlashCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl FrameFlashCapabilities {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedEyeReductionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PowerSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FrameFlashCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameFlashCapabilities>();
}
unsafe impl windows_core::Interface for FrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilities_Vtbl;
    const IID: windows_core::GUID = <IFrameFlashCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameFlashCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameFlashControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameFlashControl, windows_core::IUnknown, windows_core::IInspectable);
impl FrameFlashControl {
    pub fn Mode(&self) -> windows_core::Result<FrameFlashMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: FrameFlashMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Auto(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Auto)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuto)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedEyeReduction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRedEyeReduction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerPercent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPowerPercent)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for FrameFlashControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameFlashControl>();
}
unsafe impl windows_core::Interface for FrameFlashControl {
    type Vtable = IFrameFlashControl_Vtbl;
    const IID: windows_core::GUID = <IFrameFlashControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameFocusCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameFocusCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl FrameFocusCapabilities {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FrameFocusCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameFocusCapabilities>();
}
unsafe impl windows_core::Interface for FrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilities_Vtbl;
    const IID: windows_core::GUID = <IFrameFocusCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameFocusCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameFocusControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameFocusControl, windows_core::IUnknown, windows_core::IInspectable);
impl FrameFocusControl {
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for FrameFocusControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameFocusControl>();
}
unsafe impl windows_core::Interface for FrameFocusControl {
    type Vtable = IFrameFocusControl_Vtbl;
    const IID: windows_core::GUID = <IFrameFocusControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameIsoSpeedCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameIsoSpeedCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl FrameIsoSpeedCapabilities {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FrameIsoSpeedCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameIsoSpeedCapabilities>();
}
unsafe impl windows_core::Interface for FrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilities_Vtbl;
    const IID: windows_core::GUID = <IFrameIsoSpeedCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameIsoSpeedCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameIsoSpeedControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameIsoSpeedControl, windows_core::IUnknown, windows_core::IInspectable);
impl FrameIsoSpeedControl {
    pub fn Auto(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Auto)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuto)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for FrameIsoSpeedControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameIsoSpeedControl>();
}
unsafe impl windows_core::Interface for FrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControl_Vtbl;
    const IID: windows_core::GUID = <IFrameIsoSpeedControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct VariablePhotoSequenceController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VariablePhotoSequenceController, windows_core::IUnknown, windows_core::IInspectable);
impl VariablePhotoSequenceController {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPhotosPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhotosPerSecondLimit(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotosPerSecondLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPhotosPerSecondLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<P0>(&self, captureproperties: P0) -> windows_core::Result<super::super::MediaProperties::MediaRatio>
    where
        P0: windows_core::Param<super::super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHighestConcurrentFrameRate)(windows_core::Interface::as_raw(this), captureproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> windows_core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentFrameRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameCapabilities(&self) -> windows_core::Result<FrameControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFrameControllers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<FrameController>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredFrameControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VariablePhotoSequenceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVariablePhotoSequenceController>();
}
unsafe impl windows_core::Interface for VariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceController_Vtbl;
    const IID: windows_core::GUID = <IVariablePhotoSequenceController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VariablePhotoSequenceController {
    const NAME: &'static str = "Windows.Media.Devices.Core.VariablePhotoSequenceController";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: Self = Self(0i32);
    pub const Enable: Self = Self(1i32);
    pub const Global: Self = Self(2i32);
}
impl windows_core::TypeKind for FrameFlashMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FrameFlashMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FrameFlashMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FrameFlashMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.Core.FrameFlashMode;i4)");
}

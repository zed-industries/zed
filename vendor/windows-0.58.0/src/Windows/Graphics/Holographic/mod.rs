windows_core::imp::define_interface!(IHolographicCamera, IHolographicCamera_Vtbl, 0xe4e98445_9bed_4980_9ba0_e87680d1cb74);
impl windows_core::RuntimeType for IHolographicCamera {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCamera_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RenderTargetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ViewportScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetViewportScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNearPlaneDistance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetFarPlaneDistance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCamera2, IHolographicCamera2_Vtbl, 0xb55b9f1a_ba8c_4f84_ad79_2e7e1e2450f3);
impl windows_core::RuntimeType for IHolographicCamera2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCamera2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LeftViewportParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RightViewportParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCamera3, IHolographicCamera3_Vtbl, 0x45aa4fb3_7b59_524e_4a3f_4a6ad6650477);
impl windows_core::RuntimeType for IHolographicCamera3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCamera3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPrimaryLayerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPrimaryLayerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MaxQuadLayerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub QuadLayers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QuadLayers: usize,
}
windows_core::imp::define_interface!(IHolographicCamera4, IHolographicCamera4_Vtbl, 0x9a2531d6_4723_4f39_a9a5_9d05181d9b44);
impl windows_core::RuntimeType for IHolographicCamera4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCamera4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanOverrideViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCamera5, IHolographicCamera5_Vtbl, 0x229706f2_628d_4ef5_9c08_a63fdd7787c6);
impl windows_core::RuntimeType for IHolographicCamera5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCamera5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsHardwareContentProtectionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHardwareContentProtectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHardwareContentProtectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCamera6, IHolographicCamera6_Vtbl, 0x0209194f_632d_5154_ab52_0b5d15b12505);
impl windows_core::RuntimeType for IHolographicCamera6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCamera6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCameraPose, IHolographicCameraPose_Vtbl, 0x0d7d7e30_12de_45bd_912b_c7f6561599d1);
impl windows_core::RuntimeType for IHolographicCameraPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraPose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HolographicCamera: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Viewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetViewTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectionTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicStereoTransform) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectionTransform: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetCullingFrustum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetCullingFrustum: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetVisibleFrustum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetVisibleFrustum: usize,
    pub NearPlaneDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub FarPlaneDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCameraPose2, IHolographicCameraPose2_Vtbl, 0x232be073_5d2d_4560_814e_2697c4fce16b);
impl windows_core::RuntimeType for IHolographicCameraPose2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraPose2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub OverrideViewTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, HolographicStereoTransform) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    OverrideViewTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub OverrideProjectionTransform: unsafe extern "system" fn(*mut core::ffi::c_void, HolographicStereoTransform) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OverrideProjectionTransform: usize,
    pub OverrideViewport: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCameraRenderingParameters, IHolographicCameraRenderingParameters_Vtbl, 0x8eac2ed1_5bf4_4e16_8236_ae0800c11d0d);
impl windows_core::RuntimeType for IHolographicCameraRenderingParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormal: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetFocusPointWithNormalLinearVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetFocusPointWithNormalLinearVelocity: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11BackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11BackBuffer: usize,
}
windows_core::imp::define_interface!(IHolographicCameraRenderingParameters2, IHolographicCameraRenderingParameters2_Vtbl, 0x261270e3_b696_4634_94d6_be0681643599);
impl windows_core::RuntimeType for IHolographicCameraRenderingParameters2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReprojectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicReprojectionMode) -> windows_core::HRESULT,
    pub SetReprojectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, HolographicReprojectionMode) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CommitDirect3D11DepthBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CommitDirect3D11DepthBuffer: usize,
}
windows_core::imp::define_interface!(IHolographicCameraRenderingParameters3, IHolographicCameraRenderingParameters3_Vtbl, 0xb1aa513f_136d_4b06_b9d4_e4b914cd0683);
impl windows_core::RuntimeType for IHolographicCameraRenderingParameters3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsContentProtectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsContentProtectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCameraRenderingParameters4, IHolographicCameraRenderingParameters4_Vtbl, 0x0878fa4c_e163_57dc_82b7_c406ab3e0537);
impl windows_core::RuntimeType for IHolographicCameraRenderingParameters4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraRenderingParameters4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DepthReprojectionMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicDepthReprojectionMethod) -> windows_core::HRESULT,
    pub SetDepthReprojectionMethod: unsafe extern "system" fn(*mut core::ffi::c_void, HolographicDepthReprojectionMethod) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicCameraViewportParameters, IHolographicCameraViewportParameters_Vtbl, 0x80cdf3f7_842a_41e1_93ed_5692ab1fbb10);
impl windows_core::RuntimeType for IHolographicCameraViewportParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicCameraViewportParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub HiddenAreaMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HiddenAreaMesh: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub VisibleAreaMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VisibleAreaMesh: usize,
}
windows_core::imp::define_interface!(IHolographicDisplay, IHolographicDisplay_Vtbl, 0x9acea414_1d9f_4090_a388_90c06f6eae9c);
impl windows_core::RuntimeType for IHolographicDisplay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicDisplay_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MaxViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub IsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsOpaque: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicAdapterId) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub SpatialLocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    SpatialLocator: usize,
}
windows_core::imp::define_interface!(IHolographicDisplay2, IHolographicDisplay2_Vtbl, 0x75ac3f82_e755_436c_8d96_4d32d131473e);
impl windows_core::RuntimeType for IHolographicDisplay2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicDisplay2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RefreshRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicDisplay3, IHolographicDisplay3_Vtbl, 0xfc4c6ac6_6480_5008_b29e_157d77c843f7);
impl windows_core::RuntimeType for IHolographicDisplay3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicDisplay3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetViewConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, HolographicViewConfigurationKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicDisplayStatics, IHolographicDisplayStatics_Vtbl, 0xcb374983_e7b0_4841_8355_3ae5b536e9a4);
impl windows_core::RuntimeType for IHolographicDisplayStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicDisplayStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicFrame, IHolographicFrame_Vtbl, 0xc6988eb6_a8b9_3054_a6eb_d624b6536375);
impl windows_core::RuntimeType for IHolographicFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedCameras: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedCameras: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemovedCameras: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemovedCameras: usize,
    pub GetRenderingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub CurrentPrediction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateCurrentPrediction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PresentUsingCurrentPrediction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicFramePresentResult) -> windows_core::HRESULT,
    pub PresentUsingCurrentPredictionWithBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, HolographicFramePresentWaitBehavior, *mut HolographicFramePresentResult) -> windows_core::HRESULT,
    pub WaitForFrameToFinish: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicFrame2, IHolographicFrame2_Vtbl, 0x283f37bf_3bf2_5e91_6633_870574e6f217);
impl windows_core::RuntimeType for IHolographicFrame2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFrame2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetQuadLayerUpdateParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicFrame3, IHolographicFrame3_Vtbl, 0xe5e964c9_8a27_55d3_9f98_94530d369052);
impl windows_core::RuntimeType for IHolographicFrame3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFrame3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicFrameId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicFramePrediction, IHolographicFramePrediction_Vtbl, 0x520f4de1_5c0a_4e79_a81e_6abe02bb2739);
impl windows_core::RuntimeType for IHolographicFramePrediction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFramePrediction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CameraPoses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CameraPoses: usize,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IHolographicFramePresentationMonitor, IHolographicFramePresentationMonitor_Vtbl, 0xca87256c_6fae_428e_bb83_25dfee51136b);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IHolographicFramePresentationMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicFramePresentationMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ReadReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ReadReports: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IHolographicFramePresentationReport, IHolographicFramePresentationReport_Vtbl, 0x80baf614_f2f4_4c8a_8de3_065c78f6d5de);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IHolographicFramePresentationReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicFramePresentationReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CompositorGpuDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CompositorGpuDuration: usize,
    #[cfg(feature = "deprecated")]
    pub AppGpuDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppGpuDuration: usize,
    #[cfg(feature = "deprecated")]
    pub AppGpuOverrun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppGpuOverrun: usize,
    #[cfg(feature = "deprecated")]
    pub MissedPresentationOpportunityCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MissedPresentationOpportunityCount: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationCount: usize,
}
windows_core::imp::define_interface!(IHolographicFrameRenderingReport, IHolographicFrameRenderingReport_Vtbl, 0x05f32de4_e384_51b3_b934_f0d3a0f78606);
impl windows_core::RuntimeType for IHolographicFrameRenderingReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFrameRenderingReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicFrameId) -> windows_core::HRESULT,
    pub MissedLatchCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SystemRelativeFrameReadyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SystemRelativeActualGpuFinishTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SystemRelativeTargetLatchTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicFrameScanoutMonitor, IHolographicFrameScanoutMonitor_Vtbl, 0x7e83efa9_843c_5401_8095_9bc1b8b08638);
impl windows_core::RuntimeType for IHolographicFrameScanoutMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFrameScanoutMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
windows_core::imp::define_interface!(IHolographicFrameScanoutReport, IHolographicFrameScanoutReport_Vtbl, 0x0ebbe606_03a0_5ca0_b46e_bba068d7233f);
impl windows_core::RuntimeType for IHolographicFrameScanoutReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicFrameScanoutReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RenderingReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MissedScanoutCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SystemRelativeLatchTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SystemRelativeScanoutStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SystemRelativePhotonTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicQuadLayer, IHolographicQuadLayer_Vtbl, 0x903460c9_c9d9_5d5c_41ac_a2d5ab0fd331);
impl windows_core::RuntimeType for IHolographicQuadLayer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicQuadLayer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicQuadLayerFactory, IHolographicQuadLayerFactory_Vtbl, 0xa67538f3_5a14_5a10_489a_455065b37b76);
impl windows_core::RuntimeType for IHolographicQuadLayerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicQuadLayerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateWithPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size, super::DirectX::DirectXPixelFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateWithPixelFormat: usize,
}
windows_core::imp::define_interface!(IHolographicQuadLayerUpdateParameters, IHolographicQuadLayerUpdateParameters_Vtbl, 0x2b0ea3b0_798d_5bca_55c2_2c0c762ebb08);
impl windows_core::RuntimeType for IHolographicQuadLayerUpdateParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContent: usize,
    pub UpdateViewport: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub UpdateContentProtectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateExtents: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateExtents: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UpdateLocationWithStationaryMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UpdateLocationWithStationaryMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpdateLocationWithDisplayRelativeMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpdateLocationWithDisplayRelativeMode: usize,
}
windows_core::imp::define_interface!(IHolographicQuadLayerUpdateParameters2, IHolographicQuadLayerUpdateParameters2_Vtbl, 0x4f33d32d_82c1_46c1_8980_3cb70d98182b);
impl windows_core::RuntimeType for IHolographicQuadLayerUpdateParameters2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParameters2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanAcquireWithHardwareProtection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub AcquireBufferToUpdateContentWithHardwareProtection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    AcquireBufferToUpdateContentWithHardwareProtection: usize,
}
windows_core::imp::define_interface!(IHolographicSpace, IHolographicSpace_Vtbl, 0x4380dba6_5e78_434f_807c_3433d1efe8b7);
impl windows_core::RuntimeType for IHolographicSpace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpace_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryAdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicAdapterId) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub SetDirect3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    SetDirect3D11Device: usize,
    pub CameraAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCameraAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CameraRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCameraRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateNextFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicSpace2, IHolographicSpace2_Vtbl, 0x4f81a9a8_b7ff_4883_9827_7d677287ea70);
impl windows_core::RuntimeType for IHolographicSpace2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpace2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserPresence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicSpaceUserPresence) -> windows_core::HRESULT,
    pub UserPresenceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUserPresenceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WaitForNextFrameReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForNextFrameReadyWithHeadStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub CreateFramePresentationMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateFramePresentationMonitor: usize,
}
windows_core::imp::define_interface!(IHolographicSpace3, IHolographicSpace3_Vtbl, 0xdf1733d1_f224_587e_8d71_1e8fc8f07b1f);
impl windows_core::RuntimeType for IHolographicSpace3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpace3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFrameScanoutMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicSpaceCameraAddedEventArgs, IHolographicSpaceCameraAddedEventArgs_Vtbl, 0x58f1da35_bbb3_3c8f_993d_6c80e7feb99f);
impl windows_core::RuntimeType for IHolographicSpaceCameraAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpaceCameraAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Camera: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicSpaceCameraRemovedEventArgs, IHolographicSpaceCameraRemovedEventArgs_Vtbl, 0x805444a8_f2ae_322e_8da9_836a0a95a4c1);
impl windows_core::RuntimeType for IHolographicSpaceCameraRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpaceCameraRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Camera: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicSpaceStatics, IHolographicSpaceStatics_Vtbl, 0x364e6064_c8f2_3ba1_8391_66b8489e67fd);
impl windows_core::RuntimeType for IHolographicSpaceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpaceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub CreateForCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateForCoreWindow: usize,
}
windows_core::imp::define_interface!(IHolographicSpaceStatics2, IHolographicSpaceStatics2_Vtbl, 0x0e777088_75fc_48af_8758_0652f6f07c59);
impl windows_core::RuntimeType for IHolographicSpaceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpaceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAvailableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsAvailableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicSpaceStatics3, IHolographicSpaceStatics3_Vtbl, 0x3b00de3d_b1a3_4dfe_8e79_fec5909e6df8);
impl windows_core::RuntimeType for IHolographicSpaceStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicSpaceStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsConfigured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicViewConfiguration, IHolographicViewConfiguration_Vtbl, 0x5c1de6e6_67e9_5004_b02c_67a3a122b576);
impl windows_core::RuntimeType for IHolographicViewConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicViewConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NativeRenderTargetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub RenderTargetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub RequestRenderTargetSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedPixelFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedPixelFormats: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub PixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HolographicViewConfigurationKind) -> windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicViewConfiguration2, IHolographicViewConfiguration2_Vtbl, 0xe241756e_e0d0_5019_9af5_1b165bc2f54e);
impl windows_core::RuntimeType for IHolographicViewConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicViewConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDepthReprojectionMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDepthReprojectionMethods: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicCamera(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicCamera, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicCamera {
    pub fn RenderTargetSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTargetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ViewportScaleFactor(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewportScaleFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetViewportScaleFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewportScaleFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStereo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNearPlaneDistance(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNearPlaneDistance)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetFarPlaneDistance(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFarPlaneDistance)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftViewportParameters(&self) -> windows_core::Result<HolographicCameraViewportParameters> {
        let this = &windows_core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeftViewportParameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RightViewportParameters(&self) -> windows_core::Result<HolographicCameraViewportParameters> {
        let this = &windows_core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightViewportParameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Display(&self) -> windows_core::Result<HolographicDisplay> {
        let this = &windows_core::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Display)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPrimaryLayerEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrimaryLayerEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPrimaryLayerEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsPrimaryLayerEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxQuadLayerCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxQuadLayerCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn QuadLayers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>> {
        let this = &windows_core::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuadLayers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanOverrideViewport(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHolographicCamera4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanOverrideViewport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHardwareContentProtectionSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHardwareContentProtectionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHardwareContentProtectionEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHardwareContentProtectionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHardwareContentProtectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHardwareContentProtectionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ViewConfiguration(&self) -> windows_core::Result<HolographicViewConfiguration> {
        let this = &windows_core::Interface::cast::<IHolographicCamera6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HolographicCamera {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicCamera>();
}
unsafe impl windows_core::Interface for HolographicCamera {
    type Vtable = IHolographicCamera_Vtbl;
    const IID: windows_core::GUID = <IHolographicCamera as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicCamera {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCamera";
}
unsafe impl Send for HolographicCamera {}
unsafe impl Sync for HolographicCamera {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicCameraPose(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicCameraPose, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicCameraPose {
    pub fn HolographicCamera(&self) -> windows_core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HolographicCamera)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Viewport(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Viewport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetViewTransform<P0>(&self, coordinatesystem: P0) -> windows_core::Result<super::super::Foundation::IReference<HolographicStereoTransform>>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetViewTransform)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectionTransform(&self) -> windows_core::Result<HolographicStereoTransform> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectionTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetCullingFrustum<P0>(&self, coordinatesystem: P0) -> windows_core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetCullingFrustum)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetVisibleFrustum<P0>(&self, coordinatesystem: P0) -> windows_core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetVisibleFrustum)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NearPlaneDistance(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NearPlaneDistance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FarPlaneDistance(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FarPlaneDistance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn OverrideViewTransform<P0>(&self, coordinatesystem: P0, coordinatesystemtoviewtransform: HolographicStereoTransform) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = &windows_core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OverrideViewTransform)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), coordinatesystemtoviewtransform).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OverrideProjectionTransform(&self, projectiontransform: HolographicStereoTransform) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OverrideProjectionTransform)(windows_core::Interface::as_raw(this), projectiontransform).ok() }
    }
    pub fn OverrideViewport(&self, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OverrideViewport)(windows_core::Interface::as_raw(this), leftviewport, rightviewport).ok() }
    }
}
impl windows_core::RuntimeType for HolographicCameraPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicCameraPose>();
}
unsafe impl windows_core::Interface for HolographicCameraPose {
    type Vtable = IHolographicCameraPose_Vtbl;
    const IID: windows_core::GUID = <IHolographicCameraPose as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicCameraPose {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraPose";
}
unsafe impl Send for HolographicCameraPose {}
unsafe impl Sync for HolographicCameraPose {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicCameraRenderingParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicCameraRenderingParameters, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicCameraRenderingParameters {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPoint<P0>(&self, coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFocusPoint)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), position).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPointWithNormal<P0>(&self, coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFocusPointWithNormal)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), position, normal).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetFocusPointWithNormalLinearVelocity<P0>(&self, coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFocusPointWithNormalLinearVelocity)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), position, normal, linearvelocity).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> windows_core::Result<super::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3D11Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11BackBuffer(&self) -> windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3D11BackBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReprojectionMode(&self) -> windows_core::Result<HolographicReprojectionMode> {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReprojectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReprojectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CommitDirect3D11DepthBuffer<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CommitDirect3D11DepthBuffer)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsContentProtectionEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContentProtectionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsContentProtectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsContentProtectionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DepthReprojectionMethod(&self) -> windows_core::Result<HolographicDepthReprojectionMethod> {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepthReprojectionMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDepthReprojectionMethod(&self, value: HolographicDepthReprojectionMethod) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDepthReprojectionMethod)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for HolographicCameraRenderingParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicCameraRenderingParameters>();
}
unsafe impl windows_core::Interface for HolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_Vtbl;
    const IID: windows_core::GUID = <IHolographicCameraRenderingParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicCameraRenderingParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraRenderingParameters";
}
unsafe impl Send for HolographicCameraRenderingParameters {}
unsafe impl Sync for HolographicCameraRenderingParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicCameraViewportParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicCameraViewportParameters, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicCameraViewportParameters {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HiddenAreaMesh(&self) -> windows_core::Result<windows_core::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).HiddenAreaMesh)(windows_core::Interface::as_raw(this), windows_core::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VisibleAreaMesh(&self) -> windows_core::Result<windows_core::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).VisibleAreaMesh)(windows_core::Interface::as_raw(this), windows_core::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for HolographicCameraViewportParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicCameraViewportParameters>();
}
unsafe impl windows_core::Interface for HolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_Vtbl;
    const IID: windows_core::GUID = <IHolographicCameraViewportParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicCameraViewportParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraViewportParameters";
}
unsafe impl Send for HolographicCameraViewportParameters {}
unsafe impl Sync for HolographicCameraViewportParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicDisplay(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicDisplay, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicDisplay {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxViewportSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxViewportSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStereo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStereo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOpaque(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOpaque)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdapterId(&self) -> windows_core::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn SpatialLocator(&self) -> windows_core::Result<super::super::Perception::Spatial::SpatialLocator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialLocator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RefreshRate(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IHolographicDisplay2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RefreshRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryGetViewConfiguration(&self, kind: HolographicViewConfigurationKind) -> windows_core::Result<HolographicViewConfiguration> {
        let this = &windows_core::Interface::cast::<IHolographicDisplay3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetViewConfiguration)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<HolographicDisplay> {
        Self::IHolographicDisplayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHolographicDisplayStatics<R, F: FnOnce(&IHolographicDisplayStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicDisplay, IHolographicDisplayStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HolographicDisplay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicDisplay>();
}
unsafe impl windows_core::Interface for HolographicDisplay {
    type Vtable = IHolographicDisplay_Vtbl;
    const IID: windows_core::GUID = <IHolographicDisplay as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicDisplay {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicDisplay";
}
unsafe impl Send for HolographicDisplay {}
unsafe impl Sync for HolographicDisplay {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicFrame, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicFrame {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddedCameras(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddedCameras)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemovedCameras(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovedCameras)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRenderingParameters<P0>(&self, camerapose: P0) -> windows_core::Result<HolographicCameraRenderingParameters>
    where
        P0: windows_core::Param<HolographicCameraPose>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRenderingParameters)(windows_core::Interface::as_raw(this), camerapose.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentPrediction(&self) -> windows_core::Result<HolographicFramePrediction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPrediction)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateCurrentPrediction(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateCurrentPrediction)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PresentUsingCurrentPrediction(&self) -> windows_core::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentUsingCurrentPrediction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PresentUsingCurrentPredictionWithBehavior(&self, waitbehavior: HolographicFramePresentWaitBehavior) -> windows_core::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentUsingCurrentPredictionWithBehavior)(windows_core::Interface::as_raw(this), waitbehavior, &mut result__).map(|| result__)
        }
    }
    pub fn WaitForFrameToFinish(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WaitForFrameToFinish)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetQuadLayerUpdateParameters<P0>(&self, layer: P0) -> windows_core::Result<HolographicQuadLayerUpdateParameters>
    where
        P0: windows_core::Param<HolographicQuadLayer>,
    {
        let this = &windows_core::Interface::cast::<IHolographicFrame2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetQuadLayerUpdateParameters)(windows_core::Interface::as_raw(this), layer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<HolographicFrameId> {
        let this = &windows_core::Interface::cast::<IHolographicFrame3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HolographicFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFrame>();
}
unsafe impl windows_core::Interface for HolographicFrame {
    type Vtable = IHolographicFrame_Vtbl;
    const IID: windows_core::GUID = <IHolographicFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicFrame {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrame";
}
unsafe impl Send for HolographicFrame {}
unsafe impl Sync for HolographicFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFramePrediction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicFramePrediction, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicFramePrediction {
    #[cfg(feature = "Foundation_Collections")]
    pub fn CameraPoses(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraPoses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HolographicFramePrediction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFramePrediction>();
}
unsafe impl windows_core::Interface for HolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_Vtbl;
    const IID: windows_core::GUID = <IHolographicFramePrediction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicFramePrediction {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePrediction";
}
unsafe impl Send for HolographicFramePrediction {}
unsafe impl Sync for HolographicFramePrediction {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFramePresentationMonitor(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(HolographicFramePresentationMonitor, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(HolographicFramePresentationMonitor, super::super::Foundation::IClosable);
#[cfg(feature = "deprecated")]
impl HolographicFramePresentationMonitor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ReadReports(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadReports)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for HolographicFramePresentationMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFramePresentationMonitor>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for HolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_Vtbl;
    const IID: windows_core::GUID = <IHolographicFramePresentationMonitor as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for HolographicFramePresentationMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationMonitor";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for HolographicFramePresentationMonitor {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFramePresentationReport(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(HolographicFramePresentationReport, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl HolographicFramePresentationReport {
    #[cfg(feature = "deprecated")]
    pub fn CompositorGpuDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositorGpuDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn AppGpuDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppGpuDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn AppGpuOverrun(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppGpuOverrun)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn MissedPresentationOpportunityCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MissedPresentationOpportunityCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PresentationCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for HolographicFramePresentationReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFramePresentationReport>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for HolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_Vtbl;
    const IID: windows_core::GUID = <IHolographicFramePresentationReport as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for HolographicFramePresentationReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationReport";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for HolographicFramePresentationReport {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for HolographicFramePresentationReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFrameRenderingReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicFrameRenderingReport, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicFrameRenderingReport {
    pub fn FrameId(&self) -> windows_core::Result<HolographicFrameId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MissedLatchCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MissedLatchCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativeFrameReadyTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeFrameReadyTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativeActualGpuFinishTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeActualGpuFinishTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativeTargetLatchTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTargetLatchTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HolographicFrameRenderingReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFrameRenderingReport>();
}
unsafe impl windows_core::Interface for HolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_Vtbl;
    const IID: windows_core::GUID = <IHolographicFrameRenderingReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicFrameRenderingReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameRenderingReport";
}
unsafe impl Send for HolographicFrameRenderingReport {}
unsafe impl Sync for HolographicFrameRenderingReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFrameScanoutMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicFrameScanoutMonitor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HolographicFrameScanoutMonitor, super::super::Foundation::IClosable);
impl HolographicFrameScanoutMonitor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadReports)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HolographicFrameScanoutMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFrameScanoutMonitor>();
}
unsafe impl windows_core::Interface for HolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_Vtbl;
    const IID: windows_core::GUID = <IHolographicFrameScanoutMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicFrameScanoutMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutMonitor";
}
unsafe impl Send for HolographicFrameScanoutMonitor {}
unsafe impl Sync for HolographicFrameScanoutMonitor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicFrameScanoutReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicFrameScanoutReport, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicFrameScanoutReport {
    pub fn RenderingReport(&self) -> windows_core::Result<HolographicFrameRenderingReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderingReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MissedScanoutCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MissedScanoutCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativeLatchTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeLatchTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativeScanoutStartTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeScanoutStartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativePhotonTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativePhotonTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HolographicFrameScanoutReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicFrameScanoutReport>();
}
unsafe impl windows_core::Interface for HolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_Vtbl;
    const IID: windows_core::GUID = <IHolographicFrameScanoutReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicFrameScanoutReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutReport";
}
unsafe impl Send for HolographicFrameScanoutReport {}
unsafe impl Sync for HolographicFrameScanoutReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicQuadLayer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicQuadLayer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HolographicQuadLayer, super::super::Foundation::IClosable);
impl HolographicQuadLayer {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PixelFormat(&self) -> windows_core::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Size(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(size: super::super::Foundation::Size) -> windows_core::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateWithPixelFormat(size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat) -> windows_core::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPixelFormat)(windows_core::Interface::as_raw(this), size, pixelformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHolographicQuadLayerFactory<R, F: FnOnce(&IHolographicQuadLayerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicQuadLayer, IHolographicQuadLayerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HolographicQuadLayer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicQuadLayer>();
}
unsafe impl windows_core::Interface for HolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_Vtbl;
    const IID: windows_core::GUID = <IHolographicQuadLayer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicQuadLayer {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayer";
}
unsafe impl Send for HolographicQuadLayer {}
unsafe impl Sync for HolographicQuadLayer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicQuadLayerUpdateParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicQuadLayerUpdateParameters, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicQuadLayerUpdateParameters {
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn AcquireBufferToUpdateContent(&self) -> windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcquireBufferToUpdateContent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateViewport(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateViewport)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateContentProtectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateContentProtectionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpdateExtents(&self, value: super::super::Foundation::Numerics::Vector2) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateExtents)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UpdateLocationWithStationaryMode<P0>(&self, coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateLocationWithStationaryMode)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), position, orientation).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpdateLocationWithDisplayRelativeMode(&self, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateLocationWithDisplayRelativeMode)(windows_core::Interface::as_raw(this), position, orientation).ok() }
    }
    pub fn CanAcquireWithHardwareProtection(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanAcquireWithHardwareProtection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &windows_core::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcquireBufferToUpdateContentWithHardwareProtection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HolographicQuadLayerUpdateParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicQuadLayerUpdateParameters>();
}
unsafe impl windows_core::Interface for HolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_Vtbl;
    const IID: windows_core::GUID = <IHolographicQuadLayerUpdateParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicQuadLayerUpdateParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters";
}
unsafe impl Send for HolographicQuadLayerUpdateParameters {}
unsafe impl Sync for HolographicQuadLayerUpdateParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicSpace(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicSpace, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicSpace {
    pub fn PrimaryAdapterId(&self) -> windows_core::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryAdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn SetDirect3D11Device<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDirect3D11Device)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CameraAdded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCameraAdded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCameraAdded)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CameraRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCameraRemoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCameraRemoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CreateNextFrame(&self) -> windows_core::Result<HolographicFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNextFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserPresence(&self) -> windows_core::Result<HolographicSpaceUserPresence> {
        let this = &windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserPresence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserPresenceChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HolographicSpace, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserPresenceChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserPresenceChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserPresenceChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WaitForNextFrameReady(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).WaitForNextFrameReady)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn WaitForNextFrameReadyWithHeadStart(&self, requestedheadstartduration: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).WaitForNextFrameReadyWithHeadStart)(windows_core::Interface::as_raw(this), requestedheadstartduration).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> windows_core::Result<HolographicFramePresentationMonitor> {
        let this = &windows_core::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFramePresentationMonitor)(windows_core::Interface::as_raw(this), maxqueuedreports, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFrameScanoutMonitor(&self, maxqueuedreports: u32) -> windows_core::Result<HolographicFrameScanoutMonitor> {
        let this = &windows_core::Interface::cast::<IHolographicSpace3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameScanoutMonitor)(windows_core::Interface::as_raw(this), maxqueuedreports, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn CreateForCoreWindow<P0>(window: P0) -> windows_core::Result<HolographicSpace>
    where
        P0: windows_core::Param<super::super::UI::Core::CoreWindow>,
    {
        Self::IHolographicSpaceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForCoreWindow)(windows_core::Interface::as_raw(this), window.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsAvailable() -> windows_core::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsAvailableChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailableChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveIsAvailableChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IHolographicSpaceStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveIsAvailableChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn IsConfigured() -> windows_core::Result<bool> {
        Self::IHolographicSpaceStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConfigured)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicSpaceStatics<R, F: FnOnce(&IHolographicSpaceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicSpace, IHolographicSpaceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHolographicSpaceStatics2<R, F: FnOnce(&IHolographicSpaceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicSpace, IHolographicSpaceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHolographicSpaceStatics3<R, F: FnOnce(&IHolographicSpaceStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicSpace, IHolographicSpaceStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HolographicSpace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicSpace>();
}
unsafe impl windows_core::Interface for HolographicSpace {
    type Vtable = IHolographicSpace_Vtbl;
    const IID: windows_core::GUID = <IHolographicSpace as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicSpace {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpace";
}
unsafe impl Send for HolographicSpace {}
unsafe impl Sync for HolographicSpace {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicSpaceCameraAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicSpaceCameraAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicSpaceCameraAddedEventArgs {
    pub fn Camera(&self) -> windows_core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Camera)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for HolographicSpaceCameraAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicSpaceCameraAddedEventArgs>();
}
unsafe impl windows_core::Interface for HolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHolographicSpaceCameraAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicSpaceCameraAddedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs";
}
unsafe impl Send for HolographicSpaceCameraAddedEventArgs {}
unsafe impl Sync for HolographicSpaceCameraAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicSpaceCameraRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicSpaceCameraRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicSpaceCameraRemovedEventArgs {
    pub fn Camera(&self) -> windows_core::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Camera)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HolographicSpaceCameraRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicSpaceCameraRemovedEventArgs>();
}
unsafe impl windows_core::Interface for HolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHolographicSpaceCameraRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicSpaceCameraRemovedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs";
}
unsafe impl Send for HolographicSpaceCameraRemovedEventArgs {}
unsafe impl Sync for HolographicSpaceCameraRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HolographicViewConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicViewConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicViewConfiguration {
    pub fn NativeRenderTargetSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NativeRenderTargetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RenderTargetSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTargetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestRenderTargetSize(&self, size: super::super::Foundation::Size) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRenderTargetSize)(windows_core::Interface::as_raw(this), size, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedPixelFormats(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPixelFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PixelFormat(&self) -> windows_core::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetPixelFormat(&self, value: super::DirectX::DirectXPixelFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPixelFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStereo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStereo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RefreshRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RefreshRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<HolographicViewConfigurationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Display(&self) -> windows_core::Result<HolographicDisplay> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Display)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDepthReprojectionMethods(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>> {
        let this = &windows_core::Interface::cast::<IHolographicViewConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedDepthReprojectionMethods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HolographicViewConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicViewConfiguration>();
}
unsafe impl windows_core::Interface for HolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_Vtbl;
    const IID: windows_core::GUID = <IHolographicViewConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicViewConfiguration {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicViewConfiguration";
}
unsafe impl Send for HolographicViewConfiguration {}
unsafe impl Sync for HolographicViewConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: Self = Self(0i32);
    pub const AutoPlanar: Self = Self(1i32);
}
impl windows_core::TypeKind for HolographicDepthReprojectionMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HolographicDepthReprojectionMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HolographicDepthReprojectionMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HolographicDepthReprojectionMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicDepthReprojectionMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: Self = Self(0i32);
    pub const DeviceRemoved: Self = Self(1i32);
}
impl windows_core::TypeKind for HolographicFramePresentResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HolographicFramePresentResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HolographicFramePresentResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HolographicFramePresentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: Self = Self(0i32);
    pub const DoNotWaitForFrameToFinish: Self = Self(1i32);
}
impl windows_core::TypeKind for HolographicFramePresentWaitBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HolographicFramePresentWaitBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HolographicFramePresentWaitBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HolographicFramePresentWaitBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentWaitBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl windows_core::TypeKind for HolographicReprojectionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HolographicReprojectionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HolographicReprojectionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HolographicReprojectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicReprojectionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: Self = Self(0i32);
    pub const PresentPassive: Self = Self(1i32);
    pub const PresentActive: Self = Self(2i32);
}
impl windows_core::TypeKind for HolographicSpaceUserPresence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HolographicSpaceUserPresence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HolographicSpaceUserPresence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HolographicSpaceUserPresence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicSpaceUserPresence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: Self = Self(0i32);
    pub const PhotoVideoCamera: Self = Self(1i32);
}
impl windows_core::TypeKind for HolographicViewConfigurationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HolographicViewConfigurationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HolographicViewConfigurationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HolographicViewConfigurationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicViewConfigurationKind;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HolographicAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl windows_core::TypeKind for HolographicAdapterId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HolographicAdapterId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicAdapterId;u4;i4)");
}
impl Default for HolographicAdapterId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HolographicFrameId {
    pub Value: u64,
}
impl windows_core::TypeKind for HolographicFrameId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HolographicFrameId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicFrameId;u8)");
}
impl Default for HolographicFrameId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HolographicStereoTransform {
    pub Left: super::super::Foundation::Numerics::Matrix4x4,
    pub Right: super::super::Foundation::Numerics::Matrix4x4,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for HolographicStereoTransform {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for HolographicStereoTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicStereoTransform;struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for HolographicStereoTransform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

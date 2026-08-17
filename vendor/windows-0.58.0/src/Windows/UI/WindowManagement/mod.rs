#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(IAppWindow, IAppWindow_Vtbl, 0x663014a6_b75e_5dbd_995c_f0117fa3fb61);
impl windows_core::RuntimeType for IAppWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PersistedStateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Presenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WindowingEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    pub RequestMoveToDisplayRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestMoveAdjacentToCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestMoveAdjacentToWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestMoveRelativeToWindowContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
    pub RequestMoveRelativeToCurrentViewContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
    pub RequestMoveRelativeToDisplayRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
    pub RequestSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    pub TryShowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CloseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCloseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowChangedEventArgs, IAppWindowChangedEventArgs_Vtbl, 0x1de1f3be_a655_55ad_b2b6_eb240f880356);
impl windows_core::RuntimeType for IAppWindowChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DidAvailableWindowPresentationsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidDisplayRegionsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidFrameChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidTitleBarChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidWindowingEnvironmentChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DidWindowPresentationChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowCloseRequestedEventArgs, IAppWindowCloseRequestedEventArgs_Vtbl, 0xe9ff01da_e7a2_57a8_8b5e_39c4003afdbb);
impl windows_core::RuntimeType for IAppWindowCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowCloseRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowClosedEventArgs, IAppWindowClosedEventArgs_Vtbl, 0xcc7df816_9520_5a06_821e_456ad8b358aa);
impl windows_core::RuntimeType for IAppWindowClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppWindowClosedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowFrame, IAppWindowFrame_Vtbl, 0x9ee22601_7e5d_52af_846b_01dc6c296567);
impl windows_core::RuntimeType for IAppWindowFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub DragRegionVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Composition")))]
    DragRegionVisuals: usize,
}
windows_core::imp::define_interface!(IAppWindowFrameStyle, IAppWindowFrameStyle_Vtbl, 0xac412946_e1ac_5230_944a_c60873dcf4a9);
impl windows_core::RuntimeType for IAppWindowFrameStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowFrameStyle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFrameStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppWindowFrameStyle) -> windows_core::HRESULT,
    pub SetFrameStyle: unsafe extern "system" fn(*mut core::ffi::c_void, AppWindowFrameStyle) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowPlacement, IAppWindowPlacement_Vtbl, 0x03dc815e_e7a9_5857_9c03_7d670594410e);
impl windows_core::RuntimeType for IAppWindowPlacement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowPlacement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowPresentationConfiguration, IAppWindowPresentationConfiguration_Vtbl, 0xb5a43ee3_df33_5e67_bd31_1072457300df);
impl windows_core::RuntimeType for IAppWindowPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowPresentationConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppWindowPresentationKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowPresentationConfigurationFactory, IAppWindowPresentationConfigurationFactory_Vtbl, 0xfd3606a6_7875_5de8_84ff_6351ee13dd0d);
impl windows_core::RuntimeType for IAppWindowPresentationConfigurationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowPresentationConfigurationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IAppWindowPresenter, IAppWindowPresenter_Vtbl, 0x5ae9ed73_e1fd_5317_ad78_5a3ed271bbde);
impl windows_core::RuntimeType for IAppWindowPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPresentationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, AppWindowPresentationKind, *mut bool) -> windows_core::HRESULT,
    pub RequestPresentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RequestPresentationByKind: unsafe extern "system" fn(*mut core::ffi::c_void, AppWindowPresentationKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowStatics, IAppWindowStatics_Vtbl, 0xff1f3ea3_b769_50ef_9873_108cd0e89746);
impl windows_core::RuntimeType for IAppWindowStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearAllPersistedState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowTitleBar, IAppWindowTitleBar_Vtbl, 0x6e932c84_f644_541d_a2d7_0c262437842d);
impl windows_core::RuntimeType for IAppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTitleBarOcclusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTitleBarOcclusions: usize,
}
windows_core::imp::define_interface!(IAppWindowTitleBarOcclusion, IAppWindowTitleBarOcclusion_Vtbl, 0xfea3cffd_2ccf_5fc3_aeae_f843876bf37e);
impl windows_core::RuntimeType for IAppWindowTitleBarOcclusion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowTitleBarOcclusion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OccludingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppWindowTitleBarVisibility, IAppWindowTitleBarVisibility_Vtbl, 0xa215a4e3_6e7e_5651_8c3b_624819528154);
impl windows_core::RuntimeType for IAppWindowTitleBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppWindowTitleBarVisibility_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPreferredVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppWindowTitleBarVisibility) -> windows_core::HRESULT,
    pub SetPreferredVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, AppWindowTitleBarVisibility) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompactOverlayPresentationConfiguration, ICompactOverlayPresentationConfiguration_Vtbl, 0xa7e5750f_5730_56c6_8e1f_d63ff4d7980d);
impl windows_core::RuntimeType for ICompactOverlayPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompactOverlayPresentationConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDefaultPresentationConfiguration, IDefaultPresentationConfiguration_Vtbl, 0xd8c2b53b_2168_5703_a853_d525589fe2b9);
impl windows_core::RuntimeType for IDefaultPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDefaultPresentationConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDisplayRegion, IDisplayRegion_Vtbl, 0xdb50c3a2_4094_5f47_8cb1_ea01ddafaa94);
impl windows_core::RuntimeType for IDisplayRegion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayRegion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayMonitorDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub WorkAreaOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub WorkAreaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub WindowingEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFullScreenPresentationConfiguration, IFullScreenPresentationConfiguration_Vtbl, 0x43d3dcd8_d2a8_503d_a626_15533d6d5f62);
impl windows_core::RuntimeType for IFullScreenPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFullScreenPresentationConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowServicesStatics, IWindowServicesStatics_Vtbl, 0xcff4d519_50a6_5c64_97f6_c2d96add7f42);
impl windows_core::RuntimeType for IWindowServicesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowServicesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllTopLevelWindowIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllTopLevelWindowIds: usize,
}
windows_core::imp::define_interface!(IWindowingEnvironment, IWindowingEnvironment_Vtbl, 0x264363c0_2a49_5417_b3ae_48a71c63a3bd);
impl windows_core::RuntimeType for IWindowingEnvironment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowingEnvironment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowingEnvironmentKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowingEnvironmentAddedEventArgs, IWindowingEnvironmentAddedEventArgs_Vtbl, 0xff2a5b7f_f183_5c66_99b2_429082069299);
impl windows_core::RuntimeType for IWindowingEnvironmentAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowingEnvironmentAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowingEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowingEnvironmentChangedEventArgs, IWindowingEnvironmentChangedEventArgs_Vtbl, 0x4160cfc6_023d_5e9a_b431_350e67dc978a);
impl windows_core::RuntimeType for IWindowingEnvironmentChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowingEnvironmentChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IWindowingEnvironmentRemovedEventArgs, IWindowingEnvironmentRemovedEventArgs_Vtbl, 0x2e5b5473_beff_5e53_9316_7e775fe568b3);
impl windows_core::RuntimeType for IWindowingEnvironmentRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowingEnvironmentRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowingEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowingEnvironmentStatics, IWindowingEnvironmentStatics_Vtbl, 0x874e9fb7_c642_55ab_8aa2_162f734a9a72);
impl windows_core::RuntimeType for IWindowingEnvironmentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowingEnvironmentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithKind: unsafe extern "system" fn(*mut core::ffi::c_void, WindowingEnvironmentKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithKind: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindow, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindow {
    pub fn Content(&self) -> windows_core::Result<super::UIContentRoot> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Frame(&self) -> windows_core::Result<AppWindowFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PersistedStateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PersistedStateId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPersistedStateId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPersistedStateId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Presenter(&self) -> windows_core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Presenter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TitleBar(&self) -> windows_core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleBar)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UIContext(&self) -> windows_core::Result<super::UIContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WindowingEnvironment(&self) -> windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowingEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloseAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPlacement(&self) -> windows_core::Result<AppWindowPlacement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPlacement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDisplayRegions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestMoveToDisplayRegion<P0>(&self, displayregion: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayRegion>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestMoveToDisplayRegion)(windows_core::Interface::as_raw(this), displayregion.param().abi()).ok() }
    }
    pub fn RequestMoveAdjacentToCurrentView(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestMoveAdjacentToCurrentView)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestMoveAdjacentToWindow<P0>(&self, anchorwindow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppWindow>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestMoveAdjacentToWindow)(windows_core::Interface::as_raw(this), anchorwindow.param().abi()).ok() }
    }
    pub fn RequestMoveRelativeToWindowContent<P0>(&self, anchorwindow: P0, contentoffset: super::super::Foundation::Point) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppWindow>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestMoveRelativeToWindowContent)(windows_core::Interface::as_raw(this), anchorwindow.param().abi(), contentoffset).ok() }
    }
    pub fn RequestMoveRelativeToCurrentViewContent(&self, contentoffset: super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestMoveRelativeToCurrentViewContent)(windows_core::Interface::as_raw(this), contentoffset).ok() }
    }
    pub fn RequestMoveRelativeToDisplayRegion<P0>(&self, displayregion: P0, displayregionoffset: super::super::Foundation::Point) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DisplayRegion>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestMoveRelativeToDisplayRegion)(windows_core::Interface::as_raw(this), displayregion.param().abi(), displayregionoffset).ok() }
    }
    pub fn RequestSize(&self, framesize: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestSize)(windows_core::Interface::as_raw(this), framesize).ok() }
    }
    pub fn TryShowAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CloseRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCloseRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCloseRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryCreateAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppWindow>> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ClearAllPersistedState() -> windows_core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (windows_core::Interface::vtable(this).ClearAllPersistedState)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ClearPersistedState(key: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (windows_core::Interface::vtable(this).ClearPersistedState)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() })
    }
    #[doc(hidden)]
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindow>();
}
unsafe impl windows_core::Interface for AppWindow {
    type Vtable = IAppWindow_Vtbl;
    const IID: windows_core::GUID = <IAppWindow as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindow {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindow";
}
unsafe impl Send for AppWindow {}
unsafe impl Sync for AppWindow {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowChangedEventArgs {
    pub fn DidAvailableWindowPresentationsChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidAvailableWindowPresentationsChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidDisplayRegionsChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidDisplayRegionsChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidFrameChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidFrameChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidSizeChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidSizeChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidTitleBarChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidTitleBarChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidVisibilityChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidVisibilityChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidWindowingEnvironmentChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidWindowingEnvironmentChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DidWindowPresentationChange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DidWindowPresentationChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppWindowChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowChangedEventArgs";
}
unsafe impl Send for AppWindowChangedEventArgs {}
unsafe impl Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowCloseRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowCloseRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowCloseRequestedEventArgs {
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCancel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppWindowCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowCloseRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppWindowCloseRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs";
}
unsafe impl Send for AppWindowCloseRequestedEventArgs {}
unsafe impl Sync for AppWindowCloseRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowClosedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<AppWindowClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppWindowClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowClosedEventArgs>();
}
unsafe impl windows_core::Interface for AppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppWindowClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowClosedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowClosedEventArgs";
}
unsafe impl Send for AppWindowClosedEventArgs {}
unsafe impl Sync for AppWindowClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowFrame, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowFrame {
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub fn DragRegionVisuals(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragRegionVisuals)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFrameStyle(&self) -> windows_core::Result<AppWindowFrameStyle> {
        let this = &windows_core::Interface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFrameStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFrameStyle(&self, framestyle: AppWindowFrameStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFrameStyle)(windows_core::Interface::as_raw(this), framestyle).ok() }
    }
}
impl windows_core::RuntimeType for AppWindowFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowFrame>();
}
unsafe impl windows_core::Interface for AppWindowFrame {
    type Vtable = IAppWindowFrame_Vtbl;
    const IID: windows_core::GUID = <IAppWindowFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowFrame {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowFrame";
}
unsafe impl Send for AppWindowFrame {}
unsafe impl Sync for AppWindowFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowPlacement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowPlacement, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowPlacement {
    pub fn DisplayRegion(&self) -> windows_core::Result<DisplayRegion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Offset(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Offset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Size(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppWindowPlacement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowPlacement>();
}
unsafe impl windows_core::Interface for AppWindowPlacement {
    type Vtable = IAppWindowPlacement_Vtbl;
    const IID: windows_core::GUID = <IAppWindowPlacement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowPlacement {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPlacement";
}
unsafe impl Send for AppWindowPlacement {}
unsafe impl Sync for AppWindowPlacement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowPresentationConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowPresentationConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowPresentationConfiguration {
    pub fn Kind(&self) -> windows_core::Result<AppWindowPresentationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppWindowPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowPresentationConfiguration>();
}
unsafe impl windows_core::Interface for AppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfiguration_Vtbl;
    const IID: windows_core::GUID = <IAppWindowPresentationConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresentationConfiguration";
}
unsafe impl Send for AppWindowPresentationConfiguration {}
unsafe impl Sync for AppWindowPresentationConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowPresenter, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowPresenter {
    pub fn GetConfiguration(&self) -> windows_core::Result<AppWindowPresentationConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPresentationSupported(&self, presentationkind: AppWindowPresentationKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPresentationSupported)(windows_core::Interface::as_raw(this), presentationkind, &mut result__).map(|| result__)
        }
    }
    pub fn RequestPresentation<P0>(&self, configuration: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<AppWindowPresentationConfiguration>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPresentation)(windows_core::Interface::as_raw(this), configuration.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RequestPresentationByKind(&self, presentationkind: AppWindowPresentationKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPresentationByKind)(windows_core::Interface::as_raw(this), presentationkind, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowPresenter>();
}
unsafe impl windows_core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
    const IID: windows_core::GUID = <IAppWindowPresenter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresenter";
}
unsafe impl Send for AppWindowPresenter {}
unsafe impl Sync for AppWindowPresenter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowTitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowTitleBar, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowTitleBar {
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonHoverBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonHoverBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonHoverBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonHoverBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonHoverForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonHoverForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonHoverForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonHoverForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonInactiveBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonInactiveBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonInactiveBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonInactiveForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonInactiveForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonInactiveForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonInactiveForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonPressedBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonPressedBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonPressedBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonPressedBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonPressedForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonPressedForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonPressedForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonPressedForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendsContentIntoTitleBar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExtendsContentIntoTitleBar)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn InactiveBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InactiveBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInactiveBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInactiveBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn InactiveForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InactiveForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInactiveForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInactiveForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTitleBarOcclusions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTitleBarOcclusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPreferredVisibility(&self) -> windows_core::Result<AppWindowTitleBarVisibility> {
        let this = &windows_core::Interface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPreferredVisibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreferredVisibility(&self, visibilitymode: AppWindowTitleBarVisibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredVisibility)(windows_core::Interface::as_raw(this), visibilitymode).ok() }
    }
}
impl windows_core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowTitleBar>();
}
unsafe impl windows_core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
    const IID: windows_core::GUID = <IAppWindowTitleBar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBar";
}
unsafe impl Send for AppWindowTitleBar {}
unsafe impl Sync for AppWindowTitleBar {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppWindowTitleBarOcclusion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppWindowTitleBarOcclusion, windows_core::IUnknown, windows_core::IInspectable);
impl AppWindowTitleBarOcclusion {
    pub fn OccludingRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OccludingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppWindowTitleBarOcclusion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindowTitleBarOcclusion>();
}
unsafe impl windows_core::Interface for AppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusion_Vtbl;
    const IID: windows_core::GUID = <IAppWindowTitleBarOcclusion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppWindowTitleBarOcclusion {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBarOcclusion";
}
unsafe impl Send for AppWindowTitleBarOcclusion {}
unsafe impl Sync for AppWindowTitleBarOcclusion {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompactOverlayPresentationConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompactOverlayPresentationConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CompactOverlayPresentationConfiguration, AppWindowPresentationConfiguration);
impl CompactOverlayPresentationConfiguration {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CompactOverlayPresentationConfiguration, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> windows_core::Result<AppWindowPresentationKind> {
        let this = &windows_core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CompactOverlayPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompactOverlayPresentationConfiguration>();
}
unsafe impl windows_core::Interface for CompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfiguration_Vtbl;
    const IID: windows_core::GUID = <ICompactOverlayPresentationConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompactOverlayPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration";
}
unsafe impl Send for CompactOverlayPresentationConfiguration {}
unsafe impl Sync for CompactOverlayPresentationConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DefaultPresentationConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DefaultPresentationConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DefaultPresentationConfiguration, AppWindowPresentationConfiguration);
impl DefaultPresentationConfiguration {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DefaultPresentationConfiguration, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> windows_core::Result<AppWindowPresentationKind> {
        let this = &windows_core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DefaultPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDefaultPresentationConfiguration>();
}
unsafe impl windows_core::Interface for DefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfiguration_Vtbl;
    const IID: windows_core::GUID = <IDefaultPresentationConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DefaultPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.DefaultPresentationConfiguration";
}
unsafe impl Send for DefaultPresentationConfiguration {}
unsafe impl Sync for DefaultPresentationConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayRegion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayRegion, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayRegion {
    pub fn DisplayMonitorDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayMonitorDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WorkAreaOffset(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WorkAreaOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WorkAreaSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WorkAreaSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WindowingEnvironment(&self) -> windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowingEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayRegion, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for DisplayRegion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayRegion>();
}
unsafe impl windows_core::Interface for DisplayRegion {
    type Vtable = IDisplayRegion_Vtbl;
    const IID: windows_core::GUID = <IDisplayRegion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayRegion {
    const NAME: &'static str = "Windows.UI.WindowManagement.DisplayRegion";
}
unsafe impl Send for DisplayRegion {}
unsafe impl Sync for DisplayRegion {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FullScreenPresentationConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FullScreenPresentationConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(FullScreenPresentationConfiguration, AppWindowPresentationConfiguration);
impl FullScreenPresentationConfiguration {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FullScreenPresentationConfiguration, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> windows_core::Result<AppWindowPresentationKind> {
        let this = &windows_core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsExclusive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExclusive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsExclusive(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsExclusive)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for FullScreenPresentationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFullScreenPresentationConfiguration>();
}
unsafe impl windows_core::Interface for FullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfiguration_Vtbl;
    const IID: windows_core::GUID = <IFullScreenPresentationConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FullScreenPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.FullScreenPresentationConfiguration";
}
unsafe impl Send for FullScreenPresentationConfiguration {}
unsafe impl Sync for FullScreenPresentationConfiguration {}
pub struct WindowServices;
impl WindowServices {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllTopLevelWindowIds() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::WindowId>> {
        Self::IWindowServicesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllTopLevelWindowIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWindowServicesStatics<R, F: FnOnce(&IWindowServicesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowServices, IWindowServicesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WindowServices {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowServices";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowingEnvironment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowingEnvironment, windows_core::IUnknown, windows_core::IInspectable);
impl WindowingEnvironment {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<WindowingEnvironmentKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDisplayRegions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAll)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllWithKind(kind: WindowingEnvironmentKind) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllWithKind)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWindowingEnvironmentStatics<R, F: FnOnce(&IWindowingEnvironmentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowingEnvironment, IWindowingEnvironmentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowingEnvironment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowingEnvironment>();
}
unsafe impl windows_core::Interface for WindowingEnvironment {
    type Vtable = IWindowingEnvironment_Vtbl;
    const IID: windows_core::GUID = <IWindowingEnvironment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowingEnvironment {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironment";
}
unsafe impl Send for WindowingEnvironment {}
unsafe impl Sync for WindowingEnvironment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowingEnvironmentAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowingEnvironmentAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowingEnvironmentAddedEventArgs {
    pub fn WindowingEnvironment(&self) -> windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowingEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowingEnvironmentAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowingEnvironmentAddedEventArgs>();
}
unsafe impl windows_core::Interface for WindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowingEnvironmentAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowingEnvironmentAddedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs";
}
unsafe impl Send for WindowingEnvironmentAddedEventArgs {}
unsafe impl Sync for WindowingEnvironmentAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowingEnvironmentChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowingEnvironmentChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowingEnvironmentChangedEventArgs {}
impl windows_core::RuntimeType for WindowingEnvironmentChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowingEnvironmentChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowingEnvironmentChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowingEnvironmentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs";
}
unsafe impl Send for WindowingEnvironmentChangedEventArgs {}
unsafe impl Sync for WindowingEnvironmentChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowingEnvironmentRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowingEnvironmentRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowingEnvironmentRemovedEventArgs {
    pub fn WindowingEnvironment(&self) -> windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowingEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowingEnvironmentRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowingEnvironmentRemovedEventArgs>();
}
unsafe impl windows_core::Interface for WindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowingEnvironmentRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowingEnvironmentRemovedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs";
}
unsafe impl Send for WindowingEnvironmentRemovedEventArgs {}
unsafe impl Sync for WindowingEnvironmentRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppWindowClosedReason(pub i32);
impl AppWindowClosedReason {
    pub const Other: Self = Self(0i32);
    pub const AppInitiated: Self = Self(1i32);
    pub const UserInitiated: Self = Self(2i32);
}
impl windows_core::TypeKind for AppWindowClosedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppWindowClosedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppWindowClosedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppWindowClosedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowClosedReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppWindowFrameStyle(pub i32);
impl AppWindowFrameStyle {
    pub const Default: Self = Self(0i32);
    pub const NoFrame: Self = Self(1i32);
}
impl windows_core::TypeKind for AppWindowFrameStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppWindowFrameStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppWindowFrameStyle").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppWindowFrameStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowFrameStyle;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppWindowPresentationKind(pub i32);
impl AppWindowPresentationKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
}
impl windows_core::TypeKind for AppWindowPresentationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppWindowPresentationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppWindowPresentationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppWindowPresentationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowPresentationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppWindowTitleBarVisibility(pub i32);
impl AppWindowTitleBarVisibility {
    pub const Default: Self = Self(0i32);
    pub const AlwaysHidden: Self = Self(1i32);
}
impl windows_core::TypeKind for AppWindowTitleBarVisibility {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppWindowTitleBarVisibility {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarVisibility").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppWindowTitleBarVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowTitleBarVisibility;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WindowingEnvironmentKind(pub i32);
impl WindowingEnvironmentKind {
    pub const Unknown: Self = Self(0i32);
    pub const Overlapped: Self = Self(1i32);
    pub const Tiled: Self = Self(2i32);
}
impl windows_core::TypeKind for WindowingEnvironmentKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WindowingEnvironmentKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WindowingEnvironmentKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.WindowingEnvironmentKind;i4)");
}

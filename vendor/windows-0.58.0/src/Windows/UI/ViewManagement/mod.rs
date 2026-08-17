#[cfg(feature = "UI_ViewManagement_Core")]
pub mod Core;
windows_core::imp::define_interface!(IAccessibilitySettings, IAccessibilitySettings_Vtbl, 0xfe0e8147_c4c0_4562_b962_1327b52ad5b9);
impl windows_core::RuntimeType for IAccessibilitySettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccessibilitySettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HighContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HighContrastScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HighContrastChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHighContrastChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivationViewSwitcher, IActivationViewSwitcher_Vtbl, 0xdca71bb6_7350_492b_aac7_c8a13d7224ad);
impl windows_core::RuntimeType for IActivationViewSwitcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivationViewSwitcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowAsStandaloneAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAsStandaloneWithSizePreferenceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, ViewSizePreference, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsViewPresentedOnActivationVirtualDesktop: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationView, IApplicationView_Vtbl, 0xd222d519_4361_451e_96c4_60f4f9742db0);
impl windows_core::RuntimeType for IApplicationView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationViewOrientation) -> windows_core::HRESULT,
    pub AdjacentToLeftDisplayEdge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AdjacentToRightDisplayEdge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsFullScreen: usize,
    pub IsOnLockScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsScreenCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsScreenCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Consolidated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConsolidated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationView2, IApplicationView2_Vtbl, 0xe876b196_a545_40dc_b594_450cba68cc00);
impl windows_core::RuntimeType for IApplicationView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SuppressSystemOverlays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SuppressSystemOverlays: usize,
    #[cfg(feature = "deprecated")]
    pub SetSuppressSystemOverlays: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSuppressSystemOverlays: usize,
    pub VisibleBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub VisibleBoundsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVisibleBoundsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetDesiredBoundsMode: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationViewBoundsMode, *mut bool) -> windows_core::HRESULT,
    pub DesiredBoundsMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationViewBoundsMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationView3, IApplicationView3_Vtbl, 0x903c9ce5_793a_4fdf_a2b2_af1ac21e3108);
impl windows_core::RuntimeType for IApplicationView3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationView3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FullScreenSystemOverlayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FullScreenSystemOverlayMode) -> windows_core::HRESULT,
    pub SetFullScreenSystemOverlayMode: unsafe extern "system" fn(*mut core::ffi::c_void, FullScreenSystemOverlayMode) -> windows_core::HRESULT,
    pub IsFullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryEnterFullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExitFullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowStandardSystemOverlays: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryResizeView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size, *mut bool) -> windows_core::HRESULT,
    pub SetPreferredMinSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationView4, IApplicationView4_Vtbl, 0x15e5cbec_9e0f_46b5_bc3f_9bf653e74b5e);
impl windows_core::RuntimeType for IApplicationView4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationView4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationViewMode) -> windows_core::HRESULT,
    pub IsViewModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationViewMode, *mut bool) -> windows_core::HRESULT,
    pub TryEnterViewModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationViewMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryEnterViewModeWithPreferencesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationViewMode, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryConsolidateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationView7, IApplicationView7_Vtbl, 0xa0369647_5faf_5aa6_9c38_befbb12a071e);
impl windows_core::RuntimeType for IApplicationView7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationView7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PersistedStateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationView9, IApplicationView9_Vtbl, 0x9c6516f9_021a_5f01_93e5_9bdad2647574);
impl windows_core::RuntimeType for IApplicationView9 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationView9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub WindowingEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    WindowingEnvironment: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement"))]
    pub GetDisplayRegions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_WindowManagement")))]
    GetDisplayRegions: usize,
}
windows_core::imp::define_interface!(IApplicationViewConsolidatedEventArgs, IApplicationViewConsolidatedEventArgs_Vtbl, 0x514449ec_7ea2_4de7_a6a6_7dfbaaebb6fb);
impl windows_core::RuntimeType for IApplicationViewConsolidatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewConsolidatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsUserInitiated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewConsolidatedEventArgs2, IApplicationViewConsolidatedEventArgs2_Vtbl, 0x1c199ecc_6dc1_40f4_afee_07d9ea296430);
impl windows_core::RuntimeType for IApplicationViewConsolidatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewConsolidatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAppInitiated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IApplicationViewFullscreenStatics, IApplicationViewFullscreenStatics_Vtbl, 0xbc792ebd_64fe_4b65_a0c0_901ce2b68636);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IApplicationViewFullscreenStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IApplicationViewFullscreenStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub TryUnsnapToFullscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryUnsnapToFullscreen: usize,
}
windows_core::imp::define_interface!(IApplicationViewInteropStatics, IApplicationViewInteropStatics_Vtbl, 0xc446fb5d_4793_4896_a8e2_be57a8bb0f50);
impl windows_core::RuntimeType for IApplicationViewInteropStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewInteropStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub GetApplicationViewIdForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    GetApplicationViewIdForWindow: usize,
}
windows_core::imp::define_interface!(IApplicationViewScaling, IApplicationViewScaling_Vtbl, 0x1d0ddc23_23f3_4b2d_84fe_74bf37b48b66);
impl windows_core::RuntimeType for IApplicationViewScaling {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewScaling_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IApplicationViewScalingStatics, IApplicationViewScalingStatics_Vtbl, 0xb08fecf0_b946_45c8_a5e3_71f5aa78f861);
impl windows_core::RuntimeType for IApplicationViewScalingStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewScalingStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisableLayoutScaling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetDisableLayoutScaling: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut bool) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IApplicationViewStatics, IApplicationViewStatics_Vtbl, 0x010a6306_c433_44e5_a9f2_bd84d4030a95);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IApplicationViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IApplicationViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationViewState) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(feature = "deprecated")]
    pub TryUnsnap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryUnsnap: usize,
}
windows_core::imp::define_interface!(IApplicationViewStatics2, IApplicationViewStatics2_Vtbl, 0xaf338ae5_cf64_423c_85e5_f3e72448fb23);
impl windows_core::RuntimeType for IApplicationViewStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminateAppOnFinalViewClose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetTerminateAppOnFinalViewClose: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewStatics3, IApplicationViewStatics3_Vtbl, 0xa28d7594_8c41_4e13_9719_5164796fe4c7);
impl windows_core::RuntimeType for IApplicationViewStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredLaunchWindowingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationViewWindowingMode) -> windows_core::HRESULT,
    pub SetPreferredLaunchWindowingMode: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationViewWindowingMode) -> windows_core::HRESULT,
    pub PreferredLaunchViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetPreferredLaunchViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewStatics4, IApplicationViewStatics4_Vtbl, 0x08fd8d33_2611_5336_a315_d98e6366c9db);
impl windows_core::RuntimeType for IApplicationViewStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClearAllPersistedState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewSwitcherStatics, IApplicationViewSwitcherStatics_Vtbl, 0x975f2f1e_e656_4c5e_a0a1_717c6ffa7d64);
impl windows_core::RuntimeType for IApplicationViewSwitcherStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewSwitcherStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisableShowingMainViewOnActivation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryShowAsStandaloneAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryShowAsStandaloneWithSizePreferenceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, ViewSizePreference, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, ViewSizePreference, i32, ViewSizePreference, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SwitchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SwitchFromViewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SwitchFromViewWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, ApplicationViewSwitchingOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareForCustomAnimatedSwitchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, ApplicationViewSwitchingOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewSwitcherStatics2, IApplicationViewSwitcherStatics2_Vtbl, 0x60e995cd_4fc2_48c4_b8e3_395f2b9f0fc1);
impl windows_core::RuntimeType for IApplicationViewSwitcherStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewSwitcherStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisableSystemViewActivationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewSwitcherStatics3, IApplicationViewSwitcherStatics3_Vtbl, 0x92059420_80a7_486d_b21f_c7a4a242a383);
impl windows_core::RuntimeType for IApplicationViewSwitcherStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewSwitcherStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryShowAsViewModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, ApplicationViewMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryShowAsViewModeWithPreferencesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, ApplicationViewMode, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewTitleBar, IApplicationViewTitleBar_Vtbl, 0x00924ac0_932b_4a6b_9c4b_dc38c82478ce);
impl windows_core::RuntimeType for IApplicationViewTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewTransferContext, IApplicationViewTransferContext_Vtbl, 0x8574bc63_3c17_408e_9408_8a1a9ea81bfa);
impl windows_core::RuntimeType for IApplicationViewTransferContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewTransferContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetViewId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewTransferContextStatics, IApplicationViewTransferContextStatics_Vtbl, 0x15a89d92_dd79_4b0b_bc47_d5f195f14661);
impl windows_core::RuntimeType for IApplicationViewTransferContextStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewTransferContextStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DataPackageFormatId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationViewWithContext, IApplicationViewWithContext_Vtbl, 0xbd55d512_9dc1_44fc_8501_666625df60dc);
impl windows_core::RuntimeType for IApplicationViewWithContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationViewWithContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPane, IInputPane_Vtbl, 0x640ada70_06f3_4c87_a678_9829c9127c28);
impl windows_core::RuntimeType for IInputPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPane_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Showing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShowing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Hiding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHiding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub OccludedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPane2, IInputPane2_Vtbl, 0x8a6b3f26_7090_4793_944c_c3f2cde26276);
impl windows_core::RuntimeType for IInputPane2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPane2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryHide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPaneControl, IInputPaneControl_Vtbl, 0x088bb24f_962f_489d_aa6e_c6be1a0a6e52);
impl windows_core::RuntimeType for IInputPaneControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPaneControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPaneStatics, IInputPaneStatics_Vtbl, 0x95f4af3a_ef47_424a_9741_fd2815eba2bd);
impl windows_core::RuntimeType for IInputPaneStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPaneStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPaneStatics2, IInputPaneStatics2_Vtbl, 0x1b63529b_d9ec_4531_8445_71bab9fb828e);
impl windows_core::RuntimeType for IInputPaneStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPaneStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPaneVisibilityEventArgs, IInputPaneVisibilityEventArgs_Vtbl, 0xd243e016_d907_4fcc_bb8d_f77baa5028f1);
impl windows_core::RuntimeType for IInputPaneVisibilityEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputPaneVisibilityEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OccludedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetEnsuredFocusedElementInView: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub EnsuredFocusedElementInView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProjectionManagerStatics, IProjectionManagerStatics_Vtbl, 0xb65f913d_e2f0_4ffd_ba95_34241647e45c);
impl windows_core::RuntimeType for IProjectionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProjectionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartProjectingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SwapDisplaysForViewsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopProjectingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProjectionDisplayAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ProjectionDisplayAvailableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProjectionDisplayAvailableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProjectionManagerStatics2, IProjectionManagerStatics2_Vtbl, 0xf33d2f43_2749_4cde_b977_c0c41e7415d1);
impl windows_core::RuntimeType for IProjectionManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProjectionManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub StartProjectingWithDeviceInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    StartProjectingWithDeviceInfoAsync: usize,
    pub RequestStartProjectingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub RequestStartProjectingWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::Rect, super::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    RequestStartProjectingWithPlacementAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStatusBar, IStatusBar_Vtbl, 0x0ffcc5bf_98d0_4864_b1e8_b3f4020be8b4);
impl windows_core::RuntimeType for IStatusBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStatusBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HideAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackgroundOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetBackgroundOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgressIndicator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OccludedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Showing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShowing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Hiding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHiding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStatusBarProgressIndicator, IStatusBarProgressIndicator_Vtbl, 0x76cb2670_a3d7_49cf_8200_4f3eedca27bb);
impl windows_core::RuntimeType for IStatusBarProgressIndicator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStatusBarProgressIndicator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HideAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProgressValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProgressValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStatusBarStatics, IStatusBarStatics_Vtbl, 0x8b463fdf_422f_4561_8806_fb1289cadfb7);
impl windows_core::RuntimeType for IStatusBarStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStatusBarStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings, IUISettings_Vtbl, 0x85361600_1c63_4627_bcb1_3a89e0bc9c55);
impl windows_core::RuntimeType for IUISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HandPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HandPreference) -> windows_core::HRESULT,
    pub CursorSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ScrollBarSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ScrollBarArrowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ScrollBarThumbBoxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub MessageDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AnimationsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CaretBrowsingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CaretBlinkRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CaretWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DoubleClickTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MouseHoverTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UIElementColor: unsafe extern "system" fn(*mut core::ffi::c_void, UIElementType, *mut super::Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings2, IUISettings2_Vtbl, 0xbad82401_2721_44f9_bb91_2bb228be442f);
impl windows_core::RuntimeType for IUISettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TextScaleFactorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTextScaleFactorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings3, IUISettings3_Vtbl, 0x03021be4_5254_4781_8194_5168f7d06d7b);
impl windows_core::RuntimeType for IUISettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetColorValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIColorType, *mut super::Color) -> windows_core::HRESULT,
    pub ColorValuesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveColorValuesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings4, IUISettings4_Vtbl, 0x52bb3002_919b_4d6b_9b78_8dd66ff4b93b);
impl windows_core::RuntimeType for IUISettings4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AdvancedEffectsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AdvancedEffectsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdvancedEffectsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings5, IUISettings5_Vtbl, 0x5349d588_0cb5_5f05_bd34_706b3231f0bd);
impl windows_core::RuntimeType for IUISettings5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoHideScrollBars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutoHideScrollBarsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAutoHideScrollBarsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings6, IUISettings6_Vtbl, 0xaef19bd7_fe31_5a04_ada4_469aaec6dfa9);
impl windows_core::RuntimeType for IUISettings6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettings6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AnimationsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAnimationsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MessageDurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageDurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettingsAnimationsEnabledChangedEventArgs, IUISettingsAnimationsEnabledChangedEventArgs_Vtbl, 0x0c7b4b3d_2ea1_533e_894d_415bc5243c29);
impl windows_core::RuntimeType for IUISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUISettingsAutoHideScrollBarsChangedEventArgs, IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl, 0x87afd4b2_9146_5f02_8f6b_06d454174c0f);
impl windows_core::RuntimeType for IUISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUISettingsMessageDurationChangedEventArgs, IUISettingsMessageDurationChangedEventArgs_Vtbl, 0x338aad52_4a5d_5b59_8002_d930f608fd6e);
impl windows_core::RuntimeType for IUISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsMessageDurationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUIViewSettings, IUIViewSettings_Vtbl, 0xc63657f6_8850_470d_88f8_455e16ea2c26);
impl windows_core::RuntimeType for IUIViewSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIViewSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserInteractionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserInteractionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIViewSettingsStatics, IUIViewSettingsStatics_Vtbl, 0x595c97a5_f8f6_41cf_b0fb_aacdb81fd5f6);
impl windows_core::RuntimeType for IUIViewSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIViewSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewModePreferences, IViewModePreferences_Vtbl, 0x878fcd3a_0b99_42c9_84d0_d3f1d403554b);
impl windows_core::RuntimeType for IViewModePreferences {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IViewModePreferences_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewSizePreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ViewSizePreference) -> windows_core::HRESULT,
    pub SetViewSizePreference: unsafe extern "system" fn(*mut core::ffi::c_void, ViewSizePreference) -> windows_core::HRESULT,
    pub CustomSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetCustomSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewModePreferencesStatics, IViewModePreferencesStatics_Vtbl, 0x69b60a65_5de5_40d8_8306_3833df7a2274);
impl windows_core::RuntimeType for IViewModePreferencesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IViewModePreferencesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDefault: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationViewMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccessibilitySettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccessibilitySettings, windows_core::IUnknown, windows_core::IInspectable);
impl AccessibilitySettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessibilitySettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn HighContrast(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HighContrastScheme(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastScheme)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HighContrastChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AccessibilitySettings, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHighContrastChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHighContrastChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for AccessibilitySettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccessibilitySettings>();
}
unsafe impl windows_core::Interface for AccessibilitySettings {
    type Vtable = IAccessibilitySettings_Vtbl;
    const IID: windows_core::GUID = <IAccessibilitySettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccessibilitySettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.AccessibilitySettings";
}
unsafe impl Send for AccessibilitySettings {}
unsafe impl Sync for AccessibilitySettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivationViewSwitcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivationViewSwitcher, windows_core::IUnknown, windows_core::IInspectable);
impl ActivationViewSwitcher {
    pub fn ShowAsStandaloneAsync(&self, viewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsStandaloneAsync)(windows_core::Interface::as_raw(this), viewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAsStandaloneWithSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsStandaloneWithSizePreferenceAsync)(windows_core::Interface::as_raw(this), viewid, sizepreference, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsViewPresentedOnActivationVirtualDesktop(&self, viewid: i32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsViewPresentedOnActivationVirtualDesktop)(windows_core::Interface::as_raw(this), viewid, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ActivationViewSwitcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivationViewSwitcher>();
}
unsafe impl windows_core::Interface for ActivationViewSwitcher {
    type Vtable = IActivationViewSwitcher_Vtbl;
    const IID: windows_core::GUID = <IActivationViewSwitcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivationViewSwitcher {
    const NAME: &'static str = "Windows.UI.ViewManagement.ActivationViewSwitcher";
}
unsafe impl Send for ActivationViewSwitcher {}
unsafe impl Sync for ActivationViewSwitcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ApplicationView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationView, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationView {
    pub fn Orientation(&self) -> windows_core::Result<ApplicationViewOrientation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdjacentToLeftDisplayEdge(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdjacentToLeftDisplayEdge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdjacentToRightDisplayEdge(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdjacentToRightDisplayEdge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsFullScreen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFullScreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOnLockScreen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOnLockScreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsScreenCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScreenCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsScreenCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsScreenCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Consolidated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Consolidated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConsolidated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConsolidated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SuppressSystemOverlays(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressSystemOverlays)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetSuppressSystemOverlays(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuppressSystemOverlays)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VisibleBounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisibleBounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VisibleBoundsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ApplicationView, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisibleBoundsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVisibleBoundsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveVisibleBoundsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetDesiredBoundsMode(&self, boundsmode: ApplicationViewBoundsMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDesiredBoundsMode)(windows_core::Interface::as_raw(this), boundsmode, &mut result__).map(|| result__)
        }
    }
    pub fn DesiredBoundsMode(&self) -> windows_core::Result<ApplicationViewBoundsMode> {
        let this = &windows_core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredBoundsMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TitleBar(&self) -> windows_core::Result<ApplicationViewTitleBar> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleBar)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FullScreenSystemOverlayMode(&self) -> windows_core::Result<FullScreenSystemOverlayMode> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullScreenSystemOverlayMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFullScreenSystemOverlayMode(&self, value: FullScreenSystemOverlayMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFullScreenSystemOverlayMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFullScreenMode(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFullScreenMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryEnterFullScreenMode(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryEnterFullScreenMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExitFullScreenMode(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ExitFullScreenMode)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ShowStandardSystemOverlays(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ShowStandardSystemOverlays)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryResizeView(&self, value: super::super::Foundation::Size) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryResizeView)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn SetPreferredMinSize(&self, minsize: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredMinSize)(windows_core::Interface::as_raw(this), minsize).ok() }
    }
    pub fn ViewMode(&self) -> windows_core::Result<ApplicationViewMode> {
        let this = &windows_core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsViewModeSupported(&self, viewmode: ApplicationViewMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsViewModeSupported)(windows_core::Interface::as_raw(this), viewmode, &mut result__).map(|| result__)
        }
    }
    pub fn TryEnterViewModeAsync(&self, viewmode: ApplicationViewMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryEnterViewModeAsync)(windows_core::Interface::as_raw(this), viewmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryEnterViewModeWithPreferencesAsync<P0>(&self, viewmode: ApplicationViewMode, viewmodepreferences: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<ViewModePreferences>,
    {
        let this = &windows_core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryEnterViewModeWithPreferencesAsync)(windows_core::Interface::as_raw(this), viewmode, viewmodepreferences.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryConsolidateAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryConsolidateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PersistedStateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IApplicationView7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PersistedStateId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPersistedStateId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplicationView7>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPersistedStateId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_WindowManagement")]
    pub fn WindowingEnvironment(&self) -> windows_core::Result<super::WindowManagement::WindowingEnvironment> {
        let this = &windows_core::Interface::cast::<IApplicationView9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowingEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement"))]
    pub fn GetDisplayRegions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::WindowManagement::DisplayRegion>> {
        let this = &windows_core::Interface::cast::<IApplicationView9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDisplayRegions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TryUnsnapToFullscreen() -> windows_core::Result<bool> {
        Self::IApplicationViewFullscreenStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUnsnapToFullscreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "UI_Core")]
    pub fn GetApplicationViewIdForWindow<P0>(window: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::Core::ICoreWindow>,
    {
        Self::IApplicationViewInteropStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicationViewIdForWindow)(windows_core::Interface::as_raw(this), window.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Value() -> windows_core::Result<ApplicationViewState> {
        Self::IApplicationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn TryUnsnap() -> windows_core::Result<bool> {
        Self::IApplicationViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUnsnap)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetForCurrentView() -> windows_core::Result<ApplicationView> {
        Self::IApplicationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TerminateAppOnFinalViewClose() -> windows_core::Result<bool> {
        Self::IApplicationViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TerminateAppOnFinalViewClose)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetTerminateAppOnFinalViewClose(value: bool) -> windows_core::Result<()> {
        Self::IApplicationViewStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetTerminateAppOnFinalViewClose)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PreferredLaunchWindowingMode() -> windows_core::Result<ApplicationViewWindowingMode> {
        Self::IApplicationViewStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredLaunchWindowingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetPreferredLaunchWindowingMode(value: ApplicationViewWindowingMode) -> windows_core::Result<()> {
        Self::IApplicationViewStatics3(|this| unsafe { (windows_core::Interface::vtable(this).SetPreferredLaunchWindowingMode)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PreferredLaunchViewSize() -> windows_core::Result<super::super::Foundation::Size> {
        Self::IApplicationViewStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredLaunchViewSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetPreferredLaunchViewSize(value: super::super::Foundation::Size) -> windows_core::Result<()> {
        Self::IApplicationViewStatics3(|this| unsafe { (windows_core::Interface::vtable(this).SetPreferredLaunchViewSize)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn ClearAllPersistedState() -> windows_core::Result<()> {
        Self::IApplicationViewStatics4(|this| unsafe { (windows_core::Interface::vtable(this).ClearAllPersistedState)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ClearPersistedState(key: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IApplicationViewStatics4(|this| unsafe { (windows_core::Interface::vtable(this).ClearPersistedState)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() })
    }
    pub fn UIContext(&self) -> windows_core::Result<super::UIContext> {
        let this = &windows_core::Interface::cast::<IApplicationViewWithContext>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IApplicationViewFullscreenStatics<R, F: FnOnce(&IApplicationViewFullscreenStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationView, IApplicationViewFullscreenStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewInteropStatics<R, F: FnOnce(&IApplicationViewInteropStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationView, IApplicationViewInteropStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IApplicationViewStatics<R, F: FnOnce(&IApplicationViewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationView, IApplicationViewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewStatics2<R, F: FnOnce(&IApplicationViewStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationView, IApplicationViewStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewStatics3<R, F: FnOnce(&IApplicationViewStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationView, IApplicationViewStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewStatics4<R, F: FnOnce(&IApplicationViewStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationView, IApplicationViewStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ApplicationView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationView>();
}
unsafe impl windows_core::Interface for ApplicationView {
    type Vtable = IApplicationView_Vtbl;
    const IID: windows_core::GUID = <IApplicationView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationView {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationView";
}
unsafe impl Send for ApplicationView {}
unsafe impl Sync for ApplicationView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ApplicationViewConsolidatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationViewConsolidatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationViewConsolidatedEventArgs {
    pub fn IsUserInitiated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUserInitiated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAppInitiated(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IApplicationViewConsolidatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppInitiated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ApplicationViewConsolidatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationViewConsolidatedEventArgs>();
}
unsafe impl windows_core::Interface for ApplicationViewConsolidatedEventArgs {
    type Vtable = IApplicationViewConsolidatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IApplicationViewConsolidatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationViewConsolidatedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs";
}
unsafe impl Send for ApplicationViewConsolidatedEventArgs {}
unsafe impl Sync for ApplicationViewConsolidatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ApplicationViewScaling(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationViewScaling, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationViewScaling {
    pub fn DisableLayoutScaling() -> windows_core::Result<bool> {
        Self::IApplicationViewScalingStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableLayoutScaling)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn TrySetDisableLayoutScaling(disablelayoutscaling: bool) -> windows_core::Result<bool> {
        Self::IApplicationViewScalingStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetDisableLayoutScaling)(windows_core::Interface::as_raw(this), disablelayoutscaling, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationViewScalingStatics<R, F: FnOnce(&IApplicationViewScalingStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationViewScaling, IApplicationViewScalingStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ApplicationViewScaling {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationViewScaling>();
}
unsafe impl windows_core::Interface for ApplicationViewScaling {
    type Vtable = IApplicationViewScaling_Vtbl;
    const IID: windows_core::GUID = <IApplicationViewScaling as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationViewScaling {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewScaling";
}
pub struct ApplicationViewSwitcher;
impl ApplicationViewSwitcher {
    pub fn DisableShowingMainViewOnActivation() -> windows_core::Result<()> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe { (windows_core::Interface::vtable(this).DisableShowingMainViewOnActivation)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn TryShowAsStandaloneAsync(viewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowAsStandaloneAsync)(windows_core::Interface::as_raw(this), viewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryShowAsStandaloneWithSizePreferenceAsync(viewid: i32, sizepreference: ViewSizePreference) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowAsStandaloneWithSizePreferenceAsync)(windows_core::Interface::as_raw(this), viewid, sizepreference, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync)(windows_core::Interface::as_raw(this), viewid, sizepreference, anchorviewid, anchorsizepreference, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SwitchAsync(viewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwitchAsync)(windows_core::Interface::as_raw(this), viewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SwitchFromViewAsync(toviewid: i32, fromviewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwitchFromViewAsync)(windows_core::Interface::as_raw(this), toviewid, fromviewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SwitchFromViewWithOptionsAsync(toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwitchFromViewWithOptionsAsync)(windows_core::Interface::as_raw(this), toviewid, fromviewid, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PrepareForCustomAnimatedSwitchAsync(toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareForCustomAnimatedSwitchAsync)(windows_core::Interface::as_raw(this), toviewid, fromviewid, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DisableSystemViewActivationPolicy() -> windows_core::Result<()> {
        Self::IApplicationViewSwitcherStatics2(|this| unsafe { (windows_core::Interface::vtable(this).DisableSystemViewActivationPolicy)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn TryShowAsViewModeAsync(viewid: i32, viewmode: ApplicationViewMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowAsViewModeAsync)(windows_core::Interface::as_raw(this), viewid, viewmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryShowAsViewModeWithPreferencesAsync<P0>(viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<ViewModePreferences>,
    {
        Self::IApplicationViewSwitcherStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowAsViewModeWithPreferencesAsync)(windows_core::Interface::as_raw(this), viewid, viewmode, viewmodepreferences.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IApplicationViewSwitcherStatics<R, F: FnOnce(&IApplicationViewSwitcherStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationViewSwitcher, IApplicationViewSwitcherStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewSwitcherStatics2<R, F: FnOnce(&IApplicationViewSwitcherStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationViewSwitcher, IApplicationViewSwitcherStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewSwitcherStatics3<R, F: FnOnce(&IApplicationViewSwitcherStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationViewSwitcher, IApplicationViewSwitcherStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ApplicationViewSwitcher {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewSwitcher";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ApplicationViewTitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationViewTitleBar, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationViewTitleBar {
    pub fn SetForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonHoverForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonHoverForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonHoverForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonHoverForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonHoverBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonHoverBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonHoverBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonHoverBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonPressedForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonPressedForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonPressedForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonPressedForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonPressedBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonPressedBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonPressedBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonPressedBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInactiveForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInactiveForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn InactiveForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InactiveForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInactiveBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInactiveBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn InactiveBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InactiveBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonInactiveForegroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonInactiveForegroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonInactiveForegroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonInactiveForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetButtonInactiveBackgroundColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ButtonInactiveBackgroundColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonInactiveBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ApplicationViewTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationViewTitleBar>();
}
unsafe impl windows_core::Interface for ApplicationViewTitleBar {
    type Vtable = IApplicationViewTitleBar_Vtbl;
    const IID: windows_core::GUID = <IApplicationViewTitleBar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationViewTitleBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewTitleBar";
}
unsafe impl Send for ApplicationViewTitleBar {}
unsafe impl Sync for ApplicationViewTitleBar {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ApplicationViewTransferContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationViewTransferContext, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationViewTransferContext {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationViewTransferContext, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetViewId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataPackageFormatId() -> windows_core::Result<windows_core::HSTRING> {
        Self::IApplicationViewTransferContextStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataPackageFormatId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IApplicationViewTransferContextStatics<R, F: FnOnce(&IApplicationViewTransferContextStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationViewTransferContext, IApplicationViewTransferContextStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ApplicationViewTransferContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationViewTransferContext>();
}
unsafe impl windows_core::Interface for ApplicationViewTransferContext {
    type Vtable = IApplicationViewTransferContext_Vtbl;
    const IID: windows_core::GUID = <IApplicationViewTransferContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationViewTransferContext {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewTransferContext";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InputPane(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputPane, windows_core::IUnknown, windows_core::IInspectable);
impl InputPane {
    pub fn Showing<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Showing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShowing(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShowing)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Hiding<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hiding)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHiding(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHiding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn OccludedRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OccludedRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryShow(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInputPane2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryHide(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInputPane2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryHide)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInputPaneControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInputPaneControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<InputPane> {
        Self::IInputPaneStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<InputPane>
    where
        P0: windows_core::Param<super::UIContext>,
    {
        Self::IInputPaneStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInputPaneStatics<R, F: FnOnce(&IInputPaneStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputPane, IInputPaneStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInputPaneStatics2<R, F: FnOnce(&IInputPaneStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputPane, IInputPaneStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputPane>();
}
unsafe impl windows_core::Interface for InputPane {
    type Vtable = IInputPane_Vtbl;
    const IID: windows_core::GUID = <IInputPane as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputPane {
    const NAME: &'static str = "Windows.UI.ViewManagement.InputPane";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InputPaneVisibilityEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputPaneVisibilityEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InputPaneVisibilityEventArgs {
    pub fn OccludedRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OccludedRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnsuredFocusedElementInView(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnsuredFocusedElementInView)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnsuredFocusedElementInView(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnsuredFocusedElementInView)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InputPaneVisibilityEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputPaneVisibilityEventArgs>();
}
unsafe impl windows_core::Interface for InputPaneVisibilityEventArgs {
    type Vtable = IInputPaneVisibilityEventArgs_Vtbl;
    const IID: windows_core::GUID = <IInputPaneVisibilityEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputPaneVisibilityEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.InputPaneVisibilityEventArgs";
}
pub struct ProjectionManager;
impl ProjectionManager {
    pub fn StartProjectingAsync(projectionviewid: i32, anchorviewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProjectingAsync)(windows_core::Interface::as_raw(this), projectionviewid, anchorviewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SwapDisplaysForViewsAsync(projectionviewid: i32, anchorviewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwapDisplaysForViewsAsync)(windows_core::Interface::as_raw(this), projectionviewid, anchorviewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn StopProjectingAsync(projectionviewid: i32, anchorviewid: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopProjectingAsync)(windows_core::Interface::as_raw(this), projectionviewid, anchorviewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ProjectionDisplayAvailable() -> windows_core::Result<bool> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectionDisplayAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ProjectionDisplayAvailableChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectionDisplayAvailableChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveProjectionDisplayAvailableChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IProjectionManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveProjectionDisplayAvailableChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn StartProjectingWithDeviceInfoAsync<P0>(projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Devices::Enumeration::DeviceInformation>,
    {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProjectingWithDeviceInfoAsync)(windows_core::Interface::as_raw(this), projectionviewid, anchorviewid, displaydeviceinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestStartProjectingAsync(projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStartProjectingAsync)(windows_core::Interface::as_raw(this), projectionviewid, anchorviewid, selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Popups")]
    pub fn RequestStartProjectingWithPlacementAsync(projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStartProjectingWithPlacementAsync)(windows_core::Interface::as_raw(this), projectionviewid, anchorviewid, selection, prefferedplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProjectionManagerStatics<R, F: FnOnce(&IProjectionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProjectionManager, IProjectionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProjectionManagerStatics2<R, F: FnOnce(&IProjectionManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProjectionManager, IProjectionManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ProjectionManager {
    const NAME: &'static str = "Windows.UI.ViewManagement.ProjectionManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StatusBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StatusBar, windows_core::IUnknown, windows_core::IInspectable);
impl StatusBar {
    pub fn ShowAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HideAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HideAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BackgroundOpacity(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundOpacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundOpacity)(windows_core::Interface::as_raw(this), value).ok() }
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
    pub fn ProgressIndicator(&self) -> windows_core::Result<StatusBarProgressIndicator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgressIndicator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OccludedRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OccludedRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Showing<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StatusBar, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Showing)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShowing(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShowing)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Hiding<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StatusBar, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hiding)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHiding(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHiding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<StatusBar> {
        Self::IStatusBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStatusBarStatics<R, F: FnOnce(&IStatusBarStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StatusBar, IStatusBarStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StatusBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStatusBar>();
}
unsafe impl windows_core::Interface for StatusBar {
    type Vtable = IStatusBar_Vtbl;
    const IID: windows_core::GUID = <IStatusBar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StatusBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.StatusBar";
}
unsafe impl Send for StatusBar {}
unsafe impl Sync for StatusBar {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StatusBarProgressIndicator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StatusBarProgressIndicator, windows_core::IUnknown, windows_core::IInspectable);
impl StatusBarProgressIndicator {
    pub fn ShowAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HideAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HideAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProgressValue(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgressValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProgressValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<f64>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProgressValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StatusBarProgressIndicator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStatusBarProgressIndicator>();
}
unsafe impl windows_core::Interface for StatusBarProgressIndicator {
    type Vtable = IStatusBarProgressIndicator_Vtbl;
    const IID: windows_core::GUID = <IStatusBarProgressIndicator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StatusBarProgressIndicator {
    const NAME: &'static str = "Windows.UI.ViewManagement.StatusBarProgressIndicator";
}
unsafe impl Send for StatusBarProgressIndicator {}
unsafe impl Sync for StatusBarProgressIndicator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UISettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettings, windows_core::IUnknown, windows_core::IInspectable);
impl UISettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UISettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn HandPreference(&self) -> windows_core::Result<HandPreference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HandPreference)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CursorSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CursorSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScrollBarSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScrollBarSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScrollBarArrowSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScrollBarArrowSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScrollBarThumbBoxSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScrollBarThumbBoxSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageDuration(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AnimationsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnimationsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CaretBrowsingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaretBrowsingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CaretBlinkRate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaretBlinkRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CaretWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaretWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DoubleClickTime(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoubleClickTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MouseHoverTime(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseHoverTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UIElementColor(&self, desiredelement: UIElementType) -> windows_core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIElementColor)(windows_core::Interface::as_raw(this), desiredelement, &mut result__).map(|| result__)
        }
    }
    pub fn TextScaleFactor(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextScaleFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TextScaleFactorChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UISettings, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextScaleFactorChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextScaleFactorChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextScaleFactorChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetColorValue(&self, desiredcolor: UIColorType) -> windows_core::Result<super::Color> {
        let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetColorValue)(windows_core::Interface::as_raw(this), desiredcolor, &mut result__).map(|| result__)
        }
    }
    pub fn ColorValuesChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UISettings, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorValuesChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveColorValuesChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveColorValuesChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AdvancedEffectsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvancedEffectsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdvancedEffectsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UISettings, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvancedEffectsEnabledChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdvancedEffectsEnabledChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdvancedEffectsEnabledChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AutoHideScrollBars(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoHideScrollBars)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoHideScrollBarsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoHideScrollBarsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAutoHideScrollBarsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAutoHideScrollBarsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AnimationsEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnimationsEnabledChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAnimationsEnabledChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAnimationsEnabledChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MessageDurationChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageDurationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageDurationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageDurationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for UISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettings>();
}
unsafe impl windows_core::Interface for UISettings {
    type Vtable = IUISettings_Vtbl;
    const IID: windows_core::GUID = <IUISettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
}
unsafe impl Send for UISettings {}
unsafe impl Sync for UISettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UISettingsAnimationsEnabledChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsAnimationsEnabledChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UISettingsAnimationsEnabledChangedEventArgs {}
impl windows_core::RuntimeType for UISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsAnimationsEnabledChangedEventArgs>();
}
unsafe impl windows_core::Interface for UISettingsAnimationsEnabledChangedEventArgs {
    type Vtable = IUISettingsAnimationsEnabledChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
}
unsafe impl Send for UISettingsAnimationsEnabledChangedEventArgs {}
unsafe impl Sync for UISettingsAnimationsEnabledChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UISettingsAutoHideScrollBarsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsAutoHideScrollBarsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UISettingsAutoHideScrollBarsChangedEventArgs {}
impl windows_core::RuntimeType for UISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsAutoHideScrollBarsChangedEventArgs>();
}
unsafe impl windows_core::Interface for UISettingsAutoHideScrollBarsChangedEventArgs {
    type Vtable = IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
}
unsafe impl Send for UISettingsAutoHideScrollBarsChangedEventArgs {}
unsafe impl Sync for UISettingsAutoHideScrollBarsChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UISettingsMessageDurationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsMessageDurationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UISettingsMessageDurationChangedEventArgs {}
impl windows_core::RuntimeType for UISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsMessageDurationChangedEventArgs>();
}
unsafe impl windows_core::Interface for UISettingsMessageDurationChangedEventArgs {
    type Vtable = IUISettingsMessageDurationChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
}
unsafe impl Send for UISettingsMessageDurationChangedEventArgs {}
unsafe impl Sync for UISettingsMessageDurationChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UIViewSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UIViewSettings, windows_core::IUnknown, windows_core::IInspectable);
impl UIViewSettings {
    pub fn UserInteractionMode(&self) -> windows_core::Result<UserInteractionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInteractionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<UIViewSettings> {
        Self::IUIViewSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUIViewSettingsStatics<R, F: FnOnce(&IUIViewSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIViewSettings, IUIViewSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UIViewSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUIViewSettings>();
}
unsafe impl windows_core::Interface for UIViewSettings {
    type Vtable = IUIViewSettings_Vtbl;
    const IID: windows_core::GUID = <IUIViewSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UIViewSettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UIViewSettings";
}
unsafe impl Send for UIViewSettings {}
unsafe impl Sync for UIViewSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ViewModePreferences(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ViewModePreferences, windows_core::IUnknown, windows_core::IInspectable);
impl ViewModePreferences {
    pub fn ViewSizePreference(&self) -> windows_core::Result<ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewSizePreference)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetViewSizePreference(&self, value: ViewSizePreference) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewSizePreference)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomSize(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateDefault(mode: ApplicationViewMode) -> windows_core::Result<ViewModePreferences> {
        Self::IViewModePreferencesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDefault)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IViewModePreferencesStatics<R, F: FnOnce(&IViewModePreferencesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ViewModePreferences, IViewModePreferencesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ViewModePreferences {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IViewModePreferences>();
}
unsafe impl windows_core::Interface for ViewModePreferences {
    type Vtable = IViewModePreferences_Vtbl;
    const IID: windows_core::GUID = <IViewModePreferences as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ViewModePreferences {
    const NAME: &'static str = "Windows.UI.ViewManagement.ViewModePreferences";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationViewBoundsMode(pub i32);
impl ApplicationViewBoundsMode {
    pub const UseVisible: Self = Self(0i32);
    pub const UseCoreWindow: Self = Self(1i32);
}
impl windows_core::TypeKind for ApplicationViewBoundsMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationViewBoundsMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationViewBoundsMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ApplicationViewBoundsMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewBoundsMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationViewMode(pub i32);
impl ApplicationViewMode {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
}
impl windows_core::TypeKind for ApplicationViewMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationViewMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationViewMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ApplicationViewMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationViewOrientation(pub i32);
impl ApplicationViewOrientation {
    pub const Landscape: Self = Self(0i32);
    pub const Portrait: Self = Self(1i32);
}
impl windows_core::TypeKind for ApplicationViewOrientation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationViewOrientation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationViewOrientation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ApplicationViewOrientation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewOrientation;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationViewState(pub i32);
#[cfg(feature = "deprecated")]
impl ApplicationViewState {
    pub const FullScreenLandscape: Self = Self(0i32);
    pub const Filled: Self = Self(1i32);
    pub const Snapped: Self = Self(2i32);
    pub const FullScreenPortrait: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for ApplicationViewState {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for ApplicationViewState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationViewState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for ApplicationViewState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationViewSwitchingOptions(pub u32);
impl ApplicationViewSwitchingOptions {
    pub const Default: Self = Self(0u32);
    pub const SkipAnimation: Self = Self(1u32);
    pub const ConsolidateViews: Self = Self(2u32);
}
impl windows_core::TypeKind for ApplicationViewSwitchingOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationViewSwitchingOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationViewSwitchingOptions").field(&self.0).finish()
    }
}
impl ApplicationViewSwitchingOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ApplicationViewSwitchingOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ApplicationViewSwitchingOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for ApplicationViewSwitchingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewSwitchingOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationViewWindowingMode(pub i32);
impl ApplicationViewWindowingMode {
    pub const Auto: Self = Self(0i32);
    pub const PreferredLaunchViewSize: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const CompactOverlay: Self = Self(3i32);
    pub const Maximized: Self = Self(4i32);
}
impl windows_core::TypeKind for ApplicationViewWindowingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationViewWindowingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationViewWindowingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ApplicationViewWindowingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewWindowingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FullScreenSystemOverlayMode(pub i32);
impl FullScreenSystemOverlayMode {
    pub const Standard: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
}
impl windows_core::TypeKind for FullScreenSystemOverlayMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FullScreenSystemOverlayMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FullScreenSystemOverlayMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FullScreenSystemOverlayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.FullScreenSystemOverlayMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HandPreference(pub i32);
impl HandPreference {
    pub const LeftHanded: Self = Self(0i32);
    pub const RightHanded: Self = Self(1i32);
}
impl windows_core::TypeKind for HandPreference {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HandPreference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HandPreference").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HandPreference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.HandPreference;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ScreenCaptureDisabledBehavior(pub i32);
impl ScreenCaptureDisabledBehavior {
    pub const DrawAsBlack: Self = Self(0i32);
    pub const ExcludeFromCapture: Self = Self(1i32);
}
impl windows_core::TypeKind for ScreenCaptureDisabledBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ScreenCaptureDisabledBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ScreenCaptureDisabledBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ScreenCaptureDisabledBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ScreenCaptureDisabledBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
    pub const AccentDark3: Self = Self(2i32);
    pub const AccentDark2: Self = Self(3i32);
    pub const AccentDark1: Self = Self(4i32);
    pub const Accent: Self = Self(5i32);
    pub const AccentLight1: Self = Self(6i32);
    pub const AccentLight2: Self = Self(7i32);
    pub const AccentLight3: Self = Self(8i32);
    pub const Complement: Self = Self(9i32);
}
impl windows_core::TypeKind for UIColorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIColorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIColorType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UIColorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UIColorType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIElementType(pub i32);
impl UIElementType {
    pub const ActiveCaption: Self = Self(0i32);
    pub const Background: Self = Self(1i32);
    pub const ButtonFace: Self = Self(2i32);
    pub const ButtonText: Self = Self(3i32);
    pub const CaptionText: Self = Self(4i32);
    pub const GrayText: Self = Self(5i32);
    pub const Highlight: Self = Self(6i32);
    pub const HighlightText: Self = Self(7i32);
    pub const Hotlight: Self = Self(8i32);
    pub const InactiveCaption: Self = Self(9i32);
    pub const InactiveCaptionText: Self = Self(10i32);
    pub const Window: Self = Self(11i32);
    pub const WindowText: Self = Self(12i32);
    pub const AccentColor: Self = Self(1000i32);
    pub const TextHigh: Self = Self(1001i32);
    pub const TextMedium: Self = Self(1002i32);
    pub const TextLow: Self = Self(1003i32);
    pub const TextContrastWithHigh: Self = Self(1004i32);
    pub const NonTextHigh: Self = Self(1005i32);
    pub const NonTextMediumHigh: Self = Self(1006i32);
    pub const NonTextMedium: Self = Self(1007i32);
    pub const NonTextMediumLow: Self = Self(1008i32);
    pub const NonTextLow: Self = Self(1009i32);
    pub const PageBackground: Self = Self(1010i32);
    pub const PopupBackground: Self = Self(1011i32);
    pub const OverlayOutsidePopup: Self = Self(1012i32);
}
impl windows_core::TypeKind for UIElementType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIElementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIElementType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UIElementType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UIElementType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserInteractionMode(pub i32);
impl UserInteractionMode {
    pub const Mouse: Self = Self(0i32);
    pub const Touch: Self = Self(1i32);
}
impl windows_core::TypeKind for UserInteractionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserInteractionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserInteractionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserInteractionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UserInteractionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ViewSizePreference(pub i32);
impl ViewSizePreference {
    pub const Default: Self = Self(0i32);
    pub const UseLess: Self = Self(1i32);
    pub const UseHalf: Self = Self(2i32);
    pub const UseMore: Self = Self(3i32);
    pub const UseMinimum: Self = Self(4i32);
    pub const UseNone: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl windows_core::TypeKind for ViewSizePreference {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ViewSizePreference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ViewSizePreference").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ViewSizePreference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ViewSizePreference;i4)");
}

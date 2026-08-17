windows_core::imp::define_interface!(IAppListEntry, IAppListEntry_Vtbl, 0xef00f07f_2108_490a_877a_8a9f17c25fad);
impl windows_core::RuntimeType for IAppListEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppListEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppListEntry2, IAppListEntry2_Vtbl, 0xd0a618ad_bf35_42ac_ac06_86eeeb41d04b);
impl windows_core::RuntimeType for IAppListEntry2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppListEntry2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppListEntry3, IAppListEntry3_Vtbl, 0x6099f28d_fc32_470a_bc69_4b061a76ef2e);
impl windows_core::RuntimeType for IAppListEntry3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppListEntry3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub LaunchForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    LaunchForUserAsync: usize,
}
windows_core::imp::define_interface!(IAppListEntry4, IAppListEntry4_Vtbl, 0x2a131ed2_56f5_487c_8697_5166f3b33da0);
impl windows_core::RuntimeType for IAppListEntry4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppListEntry4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplication, ICoreApplication_Vtbl, 0x0aacf7a4_5e1d_49df_8034_fb6a68bc5ed1);
impl windows_core::RuntimeType for ICoreApplication {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Suspending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSuspending: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Resuming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveResuming: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub GetCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunWithActivationFactories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplication2, ICoreApplication2_Vtbl, 0x998681fb_1ab6_4b7f_be4a_9a0645224c04);
impl windows_core::RuntimeType for ICoreApplication2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplication2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub BackgroundActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    BackgroundActivated: usize,
    pub RemoveBackgroundActivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub LeavingBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLeavingBackground: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnteredBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnteredBackground: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnablePrelaunch: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplication3, ICoreApplication3_Vtbl, 0xfeec0d39_598b_4507_8a67_772632580a57);
impl windows_core::RuntimeType for ICoreApplication3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplication3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestRestartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    RequestRestartForUserAsync: usize,
}
windows_core::imp::define_interface!(ICoreApplicationExit, ICoreApplicationExit_Vtbl, 0xcf86461d_261e_4b72_9acd_44ed2ace6a29);
impl windows_core::RuntimeType for ICoreApplicationExit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationExit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Exit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Exiting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveExiting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplicationUnhandledError, ICoreApplicationUnhandledError_Vtbl, 0xf0e24ab0_dd09_42e1_b0bc_e0e131f78d7e);
impl core::ops::Deref for ICoreApplicationUnhandledError {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreApplicationUnhandledError, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreApplicationUnhandledError {
    pub fn UnhandledErrorDetected<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnhandledErrorDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnhandledErrorDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnhandledErrorDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ICoreApplicationUnhandledError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationUnhandledError_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnhandledErrorDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUnhandledErrorDetected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplicationUseCount, ICoreApplicationUseCount_Vtbl, 0x518dc408_c077_475b_809e_0bc0c57e4b74);
impl windows_core::RuntimeType for ICoreApplicationUseCount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationUseCount_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IncrementApplicationUseCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DecrementApplicationUseCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplicationView, ICoreApplicationView_Vtbl, 0x638bb2db_451d_4661_b099_414f34ffb9f1);
impl windows_core::RuntimeType for ICoreApplicationView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub CoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CoreWindow: usize,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Activated: usize,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsMain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHosted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplicationView2, ICoreApplicationView2_Vtbl, 0x68eb7adf_917f_48eb_9aeb_7de53e086ab1);
impl windows_core::RuntimeType for ICoreApplicationView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
windows_core::imp::define_interface!(ICoreApplicationView3, ICoreApplicationView3_Vtbl, 0x07ebe1b3_a4cf_4550_ab70_b07e85330bc8);
impl windows_core::RuntimeType for ICoreApplicationView3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationView3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostedViewClosing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHostedViewClosing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreApplicationView5, ICoreApplicationView5_Vtbl, 0x2bc095a8_8ef0_446d_9e60_3a3e0428c671);
impl windows_core::RuntimeType for ICoreApplicationView5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationView5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(ICoreApplicationView6, ICoreApplicationView6_Vtbl, 0xc119d49a_0679_49ba_803f_b79c5cf34cca);
impl windows_core::RuntimeType for ICoreApplicationView6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationView6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
windows_core::imp::define_interface!(ICoreApplicationViewTitleBar, ICoreApplicationViewTitleBar_Vtbl, 0x006d35e3_e1f1_431b_9508_29b96926ac53);
impl windows_core::RuntimeType for ICoreApplicationViewTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreApplicationViewTitleBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetExtendViewIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ExtendViewIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SystemOverlayLeftInset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SystemOverlayRightInset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LayoutMetricsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLayoutMetricsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVisibleChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsVisibleChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreImmersiveApplication, ICoreImmersiveApplication_Vtbl, 0x1ada0e3e_e4a2_4123_b451_dc96bf800419);
impl windows_core::RuntimeType for ICoreImmersiveApplication {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreImmersiveApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    pub CreateNewView: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MainView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreImmersiveApplication2, ICoreImmersiveApplication2_Vtbl, 0x828e1e36_e9e3_4cfc_9b66_48b78ea9bb2c);
impl windows_core::RuntimeType for ICoreImmersiveApplication2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreImmersiveApplication2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateNewViewFromMainView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreImmersiveApplication3, ICoreImmersiveApplication3_Vtbl, 0x34a05b2f_ee0d_41e5_8314_cf10c91bf0af);
impl windows_core::RuntimeType for ICoreImmersiveApplication3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreImmersiveApplication3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateNewViewWithViewSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameworkView, IFrameworkView_Vtbl, 0xfaab5cd0_8924_45ac_ad0f_a08fae5d0324);
impl core::ops::Deref for IFrameworkView {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFrameworkView, windows_core::IUnknown, windows_core::IInspectable);
impl IFrameworkView {
    pub fn Initialize<P0>(&self, applicationview: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreApplicationView>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Initialize)(windows_core::Interface::as_raw(this), applicationview.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn SetWindow<P0>(&self, window: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Core::CoreWindow>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWindow)(windows_core::Interface::as_raw(this), window.param().abi()).ok() }
    }
    pub fn Load(&self, entrypoint: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Load)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(entrypoint)).ok() }
    }
    pub fn Run(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Run)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Uninitialize(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Uninitialize)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IFrameworkView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameworkView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub SetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetWindow: usize,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameworkViewSource, IFrameworkViewSource_Vtbl, 0xcd770614_65c4_426c_9494_34fc43554862);
impl core::ops::Deref for IFrameworkViewSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFrameworkViewSource, windows_core::IUnknown, windows_core::IInspectable);
impl IFrameworkViewSource {
    pub fn CreateView(&self) -> windows_core::Result<IFrameworkView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IFrameworkViewSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameworkViewSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostedViewClosingEventArgs, IHostedViewClosingEventArgs_Vtbl, 0xd238943c_b24e_4790_acb5_3e4243c4ff87);
impl windows_core::RuntimeType for IHostedViewClosingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHostedViewClosingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnhandledError, IUnhandledError_Vtbl, 0x9459b726_53b5_4686_9eaf_fa8162dc3980);
impl windows_core::RuntimeType for IUnhandledError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnhandledError_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Propagate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnhandledErrorDetectedEventArgs, IUnhandledErrorDetectedEventArgs_Vtbl, 0x679ab78b_b336_4822_ac40_0d750f0b7a2b);
impl windows_core::RuntimeType for IUnhandledErrorDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnhandledErrorDetectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnhandledError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppListEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppListEntry, windows_core::IUnknown, windows_core::IInspectable);
impl AppListEntry {
    pub fn DisplayInfo(&self) -> windows_core::Result<super::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LaunchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppUserModelId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn LaunchForUserAsync<P0>(&self, user: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppListEntry3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppInfo(&self) -> windows_core::Result<super::AppInfo> {
        let this = &windows_core::Interface::cast::<IAppListEntry4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppListEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppListEntry>();
}
unsafe impl windows_core::Interface for AppListEntry {
    type Vtable = IAppListEntry_Vtbl;
    const IID: windows_core::GUID = <IAppListEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.AppListEntry";
}
unsafe impl Send for AppListEntry {}
unsafe impl Sync for AppListEntry {}
pub struct CoreApplication;
impl CoreApplication {
    pub fn Id() -> windows_core::Result<windows_core::HSTRING> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Suspending<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<super::SuspendingEventArgs>>,
    {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suspending)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveSuspending(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (windows_core::Interface::vtable(this).RemoveSuspending)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn Resuming<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resuming)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveResuming(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (windows_core::Interface::vtable(this).RemoveResuming)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties() -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentView() -> windows_core::Result<CoreApplicationView> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Run<P0>(viewsource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFrameworkViewSource>,
    {
        Self::ICoreApplication(|this| unsafe { (windows_core::Interface::vtable(this).Run)(windows_core::Interface::as_raw(this), viewsource.param().abi()).ok() })
    }
    pub fn RunWithActivationFactories<P0>(activationfactorycallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IGetActivationFactory>,
    {
        Self::ICoreApplication(|this| unsafe { (windows_core::Interface::vtable(this).RunWithActivationFactories)(windows_core::Interface::as_raw(this), activationfactorycallback.param().abi()).ok() })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn BackgroundActivated<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>,
    {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundActivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveBackgroundActivated(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveBackgroundActivated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn LeavingBackground<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>,
    {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeavingBackground)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveLeavingBackground(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveLeavingBackground)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn EnteredBackground<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>,
    {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnteredBackground)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveEnteredBackground(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveEnteredBackground)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (windows_core::Interface::vtable(this).EnablePrelaunch)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn RequestRestartAsync(launcharguments: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestartAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(launcharguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn RequestRestartForUserAsync<P0>(user: P0, launcharguments: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestartForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(launcharguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Exit() -> windows_core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (windows_core::Interface::vtable(this).Exit)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn Exiting<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::ICoreApplicationExit(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exiting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveExiting(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (windows_core::Interface::vtable(this).RemoveExiting)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn UnhandledErrorDetected<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>,
    {
        Self::ICoreApplicationUnhandledError(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnhandledErrorDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveUnhandledErrorDetected(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::ICoreApplicationUnhandledError(|this| unsafe { (windows_core::Interface::vtable(this).RemoveUnhandledErrorDetected)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn IncrementApplicationUseCount() -> windows_core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (windows_core::Interface::vtable(this).IncrementApplicationUseCount)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn DecrementApplicationUseCount() -> windows_core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (windows_core::Interface::vtable(this).DecrementApplicationUseCount)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Views)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNewView(runtimetype: &windows_core::HSTRING, entrypoint: &windows_core::HSTRING) -> windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNewView)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(runtimetype), core::mem::transmute_copy(entrypoint), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MainView() -> windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MainView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNewViewFromMainView() -> windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNewViewFromMainView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNewViewWithViewSource<P0>(viewsource: P0) -> windows_core::Result<CoreApplicationView>
    where
        P0: windows_core::Param<IFrameworkViewSource>,
    {
        Self::ICoreImmersiveApplication3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNewViewWithViewSource)(windows_core::Interface::as_raw(this), viewsource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreApplication<R, F: FnOnce(&ICoreApplication) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreApplication> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplication2<R, F: FnOnce(&ICoreApplication2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreApplication2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplication3<R, F: FnOnce(&ICoreApplication3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreApplication3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplicationExit<R, F: FnOnce(&ICoreApplicationExit) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreApplicationExit> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplicationUnhandledError<R, F: FnOnce(&ICoreApplicationUnhandledError) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreApplicationUnhandledError> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreApplicationUseCount<R, F: FnOnce(&ICoreApplicationUseCount) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreApplicationUseCount> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication<R, F: FnOnce(&ICoreImmersiveApplication) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreImmersiveApplication> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication2<R, F: FnOnce(&ICoreImmersiveApplication2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreImmersiveApplication2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreImmersiveApplication3<R, F: FnOnce(&ICoreImmersiveApplication3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreApplication, ICoreImmersiveApplication3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplication";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreApplicationView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreApplicationView, windows_core::IUnknown, windows_core::IInspectable);
impl CoreApplicationView {
    #[cfg(feature = "UI_Core")]
    pub fn CoreWindow(&self) -> windows_core::Result<super::super::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoreWindow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Activated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsMain(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHosted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHosted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<ICoreApplicationView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsComponent(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComponent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TitleBar(&self) -> windows_core::Result<CoreApplicationViewTitleBar> {
        let this = &windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleBar)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HostedViewClosing<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostedViewClosing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHostedViewClosing(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHostedViewClosing)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<ICoreApplicationView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<ICoreApplicationView6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreApplicationView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreApplicationView>();
}
unsafe impl windows_core::Interface for CoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
    const IID: windows_core::GUID = <ICoreApplicationView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationView";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreApplicationViewTitleBar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreApplicationViewTitleBar, windows_core::IUnknown, windows_core::IInspectable);
impl CoreApplicationViewTitleBar {
    pub fn SetExtendViewIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExtendViewIntoTitleBar)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExtendViewIntoTitleBar(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendViewIntoTitleBar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemOverlayLeftInset(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemOverlayLeftInset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemOverlayRightInset(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemOverlayRightInset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LayoutMetricsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutMetricsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLayoutMetricsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLayoutMetricsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVisibleChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisibleChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsVisibleChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsVisibleChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for CoreApplicationViewTitleBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreApplicationViewTitleBar>();
}
unsafe impl windows_core::Interface for CoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
    const IID: windows_core::GUID = <ICoreApplicationViewTitleBar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HostedViewClosingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HostedViewClosingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HostedViewClosingEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HostedViewClosingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHostedViewClosingEventArgs>();
}
unsafe impl windows_core::Interface for HostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHostedViewClosingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
}
unsafe impl Send for HostedViewClosingEventArgs {}
unsafe impl Sync for HostedViewClosingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UnhandledError(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UnhandledError, windows_core::IUnknown, windows_core::IInspectable);
impl UnhandledError {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Propagate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Propagate)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for UnhandledError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnhandledError>();
}
unsafe impl windows_core::Interface for UnhandledError {
    type Vtable = IUnhandledError_Vtbl;
    const IID: windows_core::GUID = <IUnhandledError as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledError";
}
unsafe impl Send for UnhandledError {}
unsafe impl Sync for UnhandledError {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UnhandledErrorDetectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UnhandledErrorDetectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UnhandledErrorDetectedEventArgs {
    pub fn UnhandledError(&self) -> windows_core::Result<UnhandledError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnhandledError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UnhandledErrorDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnhandledErrorDetectedEventArgs>();
}
unsafe impl windows_core::Interface for UnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUnhandledErrorDetectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
}
unsafe impl Send for UnhandledErrorDetectedEventArgs {}
unsafe impl Sync for UnhandledErrorDetectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: Self = Self(0i32);
    pub const NotInForeground: Self = Self(1i32);
    pub const InvalidUser: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl windows_core::TypeKind for AppRestartFailureReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppRestartFailureReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppRestartFailureReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppRestartFailureReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Core.AppRestartFailureReason;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");

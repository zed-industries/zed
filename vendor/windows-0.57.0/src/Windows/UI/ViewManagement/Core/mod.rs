windows_core::imp::define_interface!(ICoreFrameworkInputView, ICoreFrameworkInputView_Vtbl, 0xd77c94ae_46b8_5d4a_9489_8ddec3d639a6);
impl windows_core::RuntimeType for ICoreFrameworkInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreFrameworkInputView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub OcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreFrameworkInputViewAnimationStartingEventArgs, ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl, 0xc0ec901c_bba4_501b_ae8b_65c9e756a719);
impl windows_core::RuntimeType for ICoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub FrameworkAnimationRecommended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AnimationDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreFrameworkInputViewOcclusionsChangedEventArgs, ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl, 0xf36f4949_c82c_53d1_a75d_2b2baf0d9b0d);
impl windows_core::RuntimeType for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreFrameworkInputViewStatics, ICoreFrameworkInputViewStatics_Vtbl, 0x6eebd9b6_eac2_5f8b_975f_772ee3e42eeb);
impl windows_core::RuntimeType for ICoreFrameworkInputViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreFrameworkInputViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView, ICoreInputView_Vtbl, 0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
impl windows_core::RuntimeType for ICoreInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCoreInputViewOcclusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCoreInputViewOcclusions: usize,
    pub TryShowPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryHidePrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView2, ICoreInputView2_Vtbl, 0x0ed726c1_e09a_4ae8_aedf_dfa4857d1a01);
impl windows_core::RuntimeType for ICoreInputView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XYFocusTransferringFromPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveXYFocusTransferringFromPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub XYFocusTransferredToPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveXYFocusTransferredToPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TryTransferXYFocusToPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Rect, CoreInputViewXYFocusTransferDirection, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView3, ICoreInputView3_Vtbl, 0xbc941653_3ab9_4849_8f58_46e7f0353cfc);
impl windows_core::RuntimeType for ICoreInputView3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputView3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryShowWithKind: unsafe extern "system" fn(*mut core::ffi::c_void, CoreInputViewKind, *mut bool) -> windows_core::HRESULT,
    pub TryHide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView4, ICoreInputView4_Vtbl, 0x002863d6_d9ef_57eb_8cef_77f6ce1b7ee7);
impl windows_core::RuntimeType for ICoreInputView4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputView4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryViewShowing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrimaryViewShowing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PrimaryViewHiding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrimaryViewHiding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView5, ICoreInputView5_Vtbl, 0x136316e0_c6d5_5c57_811e_1ad8a99ba6ab);
impl windows_core::RuntimeType for ICoreInputView5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputView5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsKindSupported: unsafe extern "system" fn(*mut core::ffi::c_void, CoreInputViewKind, *mut bool) -> windows_core::HRESULT,
    pub SupportedKindsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSupportedKindsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewAnimationStartingEventArgs, ICoreInputViewAnimationStartingEventArgs_Vtbl, 0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
impl windows_core::RuntimeType for ICoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewAnimationStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AnimationDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewHidingEventArgs, ICoreInputViewHidingEventArgs_Vtbl, 0xeada47bd_bac5_5336_848d_41083584daad);
impl windows_core::RuntimeType for ICoreInputViewHidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewHidingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewOcclusion, ICoreInputViewOcclusion_Vtbl, 0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
impl windows_core::RuntimeType for ICoreInputViewOcclusion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewOcclusion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OccludingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub OcclusionKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreInputViewOcclusionKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewOcclusionsChangedEventArgs, ICoreInputViewOcclusionsChangedEventArgs_Vtbl, 0xbe1027e8_b3ee_4df7_9554_89cdc66082c2);
impl windows_core::RuntimeType for ICoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Occlusions: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewShowingEventArgs, ICoreInputViewShowingEventArgs_Vtbl, 0xca52261b_fb9e_5daf_a98c_262b8b76af50);
impl windows_core::RuntimeType for ICoreInputViewShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewShowingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewStatics, ICoreInputViewStatics_Vtbl, 0x7d9b97cd_edbe_49cf_a54f_337de052907f);
impl windows_core::RuntimeType for ICoreInputViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewStatics2, ICoreInputViewStatics2_Vtbl, 0x7ebc0862_d049_4e52_87b0_1e90e98c49ed);
impl windows_core::RuntimeType for ICoreInputViewStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewTransferringXYFocusEventArgs, ICoreInputViewTransferringXYFocusEventArgs_Vtbl, 0x04de169f_ba02_4850_8b55_d82d03ba6d7f);
impl windows_core::RuntimeType for ICoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputViewTransferringXYFocusEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreInputViewXYFocusTransferDirection) -> windows_core::HRESULT,
    pub SetTransferHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TransferHandled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeepPrimaryViewVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeepPrimaryViewVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettingsController, IUISettingsController_Vtbl, 0x78a51ac4_15c0_5a1b_a75b_acbf9cb8bb9e);
impl windows_core::RuntimeType for IUISettingsController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAdvancedEffectsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetAnimationsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetAutoHideScrollBars: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetMessageDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetTextScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettingsControllerStatics, IUISettingsControllerStatics_Vtbl, 0xeb3c68cc_c220_578c_8119_7db324ed26a6);
impl windows_core::RuntimeType for IUISettingsControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUISettingsControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreFrameworkInputView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreFrameworkInputView, windows_core::IUnknown, windows_core::IInspectable);
impl CoreFrameworkInputView {
    pub fn PrimaryViewAnimationStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewAnimationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn OcclusionsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOcclusionsChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOcclusionsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<CoreFrameworkInputView>
    where
        P0: windows_core::Param<super::super::UIContext>,
    {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreFrameworkInputViewStatics<R, F: FnOnce(&ICoreFrameworkInputViewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreFrameworkInputView, ICoreFrameworkInputViewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreFrameworkInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreFrameworkInputView>();
}
unsafe impl windows_core::Interface for CoreFrameworkInputView {
    type Vtable = ICoreFrameworkInputView_Vtbl;
    const IID: windows_core::GUID = <ICoreFrameworkInputView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
}
unsafe impl Send for CoreFrameworkInputView {}
unsafe impl Sync for CoreFrameworkInputView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreFrameworkInputViewAnimationStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreFrameworkInputViewAnimationStartingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameworkAnimationRecommended(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameworkAnimationRecommended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AnimationDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnimationDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreFrameworkInputViewAnimationStartingEventArgs>();
}
unsafe impl windows_core::Interface for CoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreFrameworkInputViewAnimationStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
}
unsafe impl Send for CoreFrameworkInputViewAnimationStartingEventArgs {}
unsafe impl Sync for CoreFrameworkInputViewAnimationStartingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreFrameworkInputViewOcclusionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreFrameworkInputViewOcclusionsChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreFrameworkInputViewOcclusionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreFrameworkInputViewOcclusionsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
}
unsafe impl Send for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
unsafe impl Sync for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputView, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputView {
    pub fn OcclusionsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOcclusionsChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOcclusionsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCoreInputViewOcclusions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCoreInputViewOcclusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryShowPrimaryView(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowPrimaryView)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryHidePrimaryView(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryHidePrimaryView)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XYFocusTransferringFromPrimaryView<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusTransferringFromPrimaryView)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveXYFocusTransferringFromPrimaryView(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveXYFocusTransferringFromPrimaryView)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn XYFocusTransferredToPrimaryView<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusTransferredToPrimaryView)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveXYFocusTransferredToPrimaryView(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveXYFocusTransferredToPrimaryView)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryTransferXYFocusToPrimaryView(&self, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryTransferXYFocusToPrimaryView)(windows_core::Interface::as_raw(this), origin, direction, &mut result__).map(|| result__)
        }
    }
    pub fn TryShow(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowWithKind)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    pub fn TryHide(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryHide)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PrimaryViewShowing<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewShowing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewShowing(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewShowing)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PrimaryViewHiding<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewHiding)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewHiding(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewHiding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsKindSupported(&self, r#type: CoreInputViewKind) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsKindSupported)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    pub fn SupportedKindsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedKindsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSupportedKindsChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSupportedKindsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PrimaryViewAnimationStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewAnimationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreInputView> {
        Self::ICoreInputViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<CoreInputView>
    where
        P0: windows_core::Param<super::super::UIContext>,
    {
        Self::ICoreInputViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreInputViewStatics<R, F: FnOnce(&ICoreInputViewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreInputView, ICoreInputViewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICoreInputViewStatics2<R, F: FnOnce(&ICoreInputViewStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreInputView, ICoreInputViewStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputView>();
}
unsafe impl windows_core::Interface for CoreInputView {
    type Vtable = ICoreInputView_Vtbl;
    const IID: windows_core::GUID = <ICoreInputView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputView";
}
unsafe impl Send for CoreInputView {}
unsafe impl Sync for CoreInputView {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputViewAnimationStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewAnimationStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewAnimationStartingEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub fn AnimationDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnimationDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewAnimationStartingEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewAnimationStartingEventArgs {
    type Vtable = ICoreInputViewAnimationStartingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreInputViewAnimationStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
}
unsafe impl Send for CoreInputViewAnimationStartingEventArgs {}
unsafe impl Sync for CoreInputViewAnimationStartingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputViewHidingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewHidingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewHidingEventArgs {
    pub fn TryCancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewHidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewHidingEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewHidingEventArgs {
    type Vtable = ICoreInputViewHidingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreInputViewHidingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewHidingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
}
unsafe impl Send for CoreInputViewHidingEventArgs {}
unsafe impl Sync for CoreInputViewHidingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputViewOcclusion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewOcclusion, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewOcclusion {
    pub fn OccludingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OccludingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OcclusionKind(&self) -> windows_core::Result<CoreInputViewOcclusionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewOcclusion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewOcclusion>();
}
unsafe impl windows_core::Interface for CoreInputViewOcclusion {
    type Vtable = ICoreInputViewOcclusion_Vtbl;
    const IID: windows_core::GUID = <ICoreInputViewOcclusion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
}
unsafe impl Send for CoreInputViewOcclusion {}
unsafe impl Sync for CoreInputViewOcclusion {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputViewOcclusionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewOcclusionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewOcclusionsChangedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Occlusions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
}
impl windows_core::RuntimeType for CoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewOcclusionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewOcclusionsChangedEventArgs {
    type Vtable = ICoreInputViewOcclusionsChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreInputViewOcclusionsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
}
unsafe impl Send for CoreInputViewOcclusionsChangedEventArgs {}
unsafe impl Sync for CoreInputViewOcclusionsChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputViewShowingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewShowingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewShowingEventArgs {
    pub fn TryCancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewShowingEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewShowingEventArgs {
    type Vtable = ICoreInputViewShowingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreInputViewShowingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewShowingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
}
unsafe impl Send for CoreInputViewShowingEventArgs {}
unsafe impl Sync for CoreInputViewShowingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInputViewTransferringXYFocusEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewTransferringXYFocusEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewTransferringXYFocusEventArgs {
    pub fn Origin(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Origin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Direction(&self) -> windows_core::Result<CoreInputViewXYFocusTransferDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransferHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransferHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransferHandled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferHandled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeepPrimaryViewVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeepPrimaryViewVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepPrimaryViewVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepPrimaryViewVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewTransferringXYFocusEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewTransferringXYFocusEventArgs {
    type Vtable = ICoreInputViewTransferringXYFocusEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreInputViewTransferringXYFocusEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
}
unsafe impl Send for CoreInputViewTransferringXYFocusEventArgs {}
unsafe impl Sync for CoreInputViewTransferringXYFocusEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UISettingsController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsController, windows_core::IUnknown, windows_core::IInspectable);
impl UISettingsController {
    pub fn SetAdvancedEffectsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdvancedEffectsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAnimationsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAnimationsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAutoHideScrollBars(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoHideScrollBars)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMessageDuration(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessageDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetTextScaleFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextScaleFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestDefaultAsync() -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<UISettingsController>> {
        Self::IUISettingsControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUISettingsControllerStatics<R, F: FnOnce(&IUISettingsControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UISettingsController, IUISettingsControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UISettingsController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsController>();
}
unsafe impl windows_core::Interface for UISettingsController {
    type Vtable = IUISettingsController_Vtbl;
    const IID: windows_core::GUID = <IUISettingsController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsController {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.UISettingsController";
}
unsafe impl Send for UISettingsController {}
unsafe impl Sync for UISettingsController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Handwriting: Self = Self(2i32);
    pub const Emoji: Self = Self(3i32);
    pub const Symbols: Self = Self(4i32);
    pub const Clipboard: Self = Self(5i32);
    pub const Dictation: Self = Self(6i32);
}
impl windows_core::TypeKind for CoreInputViewKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreInputViewKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreInputViewKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreInputViewKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: Self = Self(0i32);
    pub const Floating: Self = Self(1i32);
    pub const Overlay: Self = Self(2i32);
}
impl windows_core::TypeKind for CoreInputViewOcclusionKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreInputViewOcclusionKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreInputViewOcclusionKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreInputViewOcclusionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Down: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
}
impl windows_core::TypeKind for CoreInputViewXYFocusTransferDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreInputViewXYFocusTransferDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreInputViewXYFocusTransferDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreInputViewXYFocusTransferDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection;i4)");
}

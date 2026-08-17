windows_core::imp::define_interface!(IWebViewControlAcceleratorKeyPressedEventArgs, IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl, 0x77a2a53e_7c74_437d_a290_3ac0d8cd5655);
impl windows_core::RuntimeType for IWebViewControlAcceleratorKeyPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::UI::Core::CoreAcceleratorKeyEventType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    EventType: usize,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::UI::Core::CorePhysicalKeyStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub RoutingStage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebViewControlAcceleratorKeyRoutingStage) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlMoveFocusRequestedEventArgs, IWebViewControlMoveFocusRequestedEventArgs_Vtbl, 0x6b2a340d_4bd0_405e_b7c1_1e72a492f446);
impl windows_core::RuntimeType for IWebViewControlMoveFocusRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlMoveFocusRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebViewControlMoveFocusReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlProcess, IWebViewControlProcess_Vtbl, 0x02c723ec_98d6_424a_b63e_c6136c36a0f2);
impl windows_core::RuntimeType for IWebViewControlProcess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlProcess_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsPrivateNetworkClientServerCapabilityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CreateWebViewControlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i64, super::super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWebViewControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWebViewControls: usize,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProcessExited: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlProcessFactory, IWebViewControlProcessFactory_Vtbl, 0x47b65cf9_a2d2_453c_b097_f6779d4b8e02);
impl windows_core::RuntimeType for IWebViewControlProcessFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlProcessFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlProcessOptions, IWebViewControlProcessOptions_Vtbl, 0x1cca72a7_3bd6_4826_8261_6c8189505d89);
impl windows_core::RuntimeType for IWebViewControlProcessOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlProcessOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetEnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPrivateNetworkClientServerCapability: unsafe extern "system" fn(*mut core::ffi::c_void, WebViewControlProcessCapabilityState) -> windows_core::HRESULT,
    pub PrivateNetworkClientServerCapability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebViewControlProcessCapabilityState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlSite, IWebViewControlSite_Vtbl, 0x133f47c6_12dc_4898_bd47_04967de648ba);
impl windows_core::RuntimeType for IWebViewControlSite {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlSite_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScale: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveFocus: unsafe extern "system" fn(*mut core::ffi::c_void, WebViewControlMoveFocusReason) -> windows_core::HRESULT,
    pub MoveFocusRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMoveFocusRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AcceleratorKeyPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAcceleratorKeyPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlSite2, IWebViewControlSite2_Vtbl, 0xd13b2e3f_48ee_4730_8243_d2ed0c05606a);
impl windows_core::RuntimeType for IWebViewControlSite2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebViewControlSite2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebViewControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControl, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebViewControl, super::IWebViewControl, super::IWebViewControl2);
impl WebViewControl {
    pub fn Source(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanGoBack(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanGoBack)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanGoForward(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanGoForward)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetDefaultBackgroundColor(&self, value: super::super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsFullScreenElement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Settings(&self) -> windows_core::Result<super::WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Settings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeferredPermissionRequests(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferredPermissionRequests)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GoForward(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GoForward)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoBack(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GoBack)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Refresh(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Refresh)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Navigate<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Navigate)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn NavigateToString(&self, text: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NavigateToString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text)).ok() }
    }
    pub fn NavigateToLocalStreamUri<P0, P1>(&self, source: P0, streamresolver: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::IUriToStreamResolver>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NavigateToLocalStreamUri)(windows_core::Interface::as_raw(this), source.param().abi(), streamresolver.param().abi()).ok() }
    }
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage<P0>(&self, requestmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Http::HttpRequestMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NavigateWithHttpRequestMessage)(windows_core::Interface::as_raw(this), requestmessage.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InvokeScriptAsync<P0>(&self, scriptname: &windows_core::HSTRING, arguments: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeScriptAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scriptname), arguments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CapturePreviewToStreamAsync<P0>(&self, stream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePreviewToStreamAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BuildLocalStreamUri(&self, contentidentifier: &windows_core::HSTRING, relativepath: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BuildLocalStreamUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentidentifier), core::mem::transmute_copy(relativepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut Option<super::WebViewControlDeferredPermissionRequest>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetDeferredPermissionRequestById)(windows_core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    pub fn NavigationStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ContentLoading<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLoading)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentLoading(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentLoading)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DOMContentLoaded<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DOMContentLoaded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDOMContentLoaded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameNavigationStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameNavigationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameNavigationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameContentLoading<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameContentLoading)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameContentLoading(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameContentLoading)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameDOMContentLoaded<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameDOMContentLoaded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameDOMContentLoaded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameNavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameNavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ScriptNotify<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlScriptNotifyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScriptNotify)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScriptNotify(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScriptNotify)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LongRunningScriptDetected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlLongRunningScriptDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LongRunningScriptDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLongRunningScriptDetected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLongRunningScriptDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnsafeContentWarningDisplaying<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsafeContentWarningDisplaying)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnviewableContentIdentified<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnviewableContentIdentifiedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnviewableContentIdentified)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnviewableContentIdentified(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnviewableContentIdentified)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PermissionRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlPermissionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PermissionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePermissionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePermissionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnsupportedUriSchemeIdentified<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NewWindowRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNewWindowRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewWindowRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNewWindowRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNewWindowRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ContainsFullScreenElementChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContainsFullScreenElementChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WebResourceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlWebResourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebResourceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWebResourceRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWebResourceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddInitializeScript(&self, script: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IWebViewControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddInitializeScript)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(script)).ok() }
    }
    pub fn Process(&self) -> windows_core::Result<WebViewControlProcess> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Process)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBounds(&self, value: super::super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBounds)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bounds(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).MoveFocus)(windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn MoveFocusRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveFocusRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMoveFocusRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMoveFocusRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AcceleratorKeyPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceleratorKeyPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAcceleratorKeyPressed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAcceleratorKeyPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WebViewControl, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveGotFocus)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WebViewControl, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveLostFocus)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for WebViewControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::IWebViewControl>();
}
unsafe impl windows_core::Interface for WebViewControl {
    type Vtable = super::IWebViewControl_Vtbl;
    const IID: windows_core::GUID = <super::IWebViewControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControl {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebViewControlAcceleratorKeyPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlAcceleratorKeyPressedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlAcceleratorKeyPressedEventArgs {
    #[cfg(feature = "UI_Core")]
    pub fn EventType(&self) -> windows_core::Result<super::super::super::UI::Core::CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> windows_core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> windows_core::Result<super::super::super::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RoutingStage(&self) -> windows_core::Result<WebViewControlAcceleratorKeyRoutingStage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoutingStage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for WebViewControlAcceleratorKeyPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlAcceleratorKeyPressedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWebViewControlAcceleratorKeyPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlAcceleratorKeyPressedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlAcceleratorKeyPressedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebViewControlMoveFocusRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlMoveFocusRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlMoveFocusRequestedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<WebViewControlMoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebViewControlMoveFocusRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlMoveFocusRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWebViewControlMoveFocusRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlMoveFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlMoveFocusRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebViewControlProcess(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlProcess, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlProcess {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebViewControlProcess, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProcessId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPrivateNetworkClientServerCapabilityEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrivateNetworkClientServerCapabilityEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateWebViewControlAsync(&self, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWebViewControlAsync)(windows_core::Interface::as_raw(this), hostwindowhandle, bounds, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWebViewControls(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetWebViewControls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Terminate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Terminate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ProcessExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProcessExited(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProcessExited)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateWithOptions<P0>(processoptions: P0) -> windows_core::Result<WebViewControlProcess>
    where
        P0: windows_core::Param<WebViewControlProcessOptions>,
    {
        Self::IWebViewControlProcessFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithOptions)(windows_core::Interface::as_raw(this), processoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebViewControlProcessFactory<R, F: FnOnce(&IWebViewControlProcessFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebViewControlProcess, IWebViewControlProcessFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebViewControlProcess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlProcess>();
}
unsafe impl windows_core::Interface for WebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
    const IID: windows_core::GUID = <IWebViewControlProcess as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlProcess {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcess";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebViewControlProcessOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlProcessOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlProcessOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebViewControlProcessOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetEnterpriseId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnterpriseId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrivateNetworkClientServerCapability)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrivateNetworkClientServerCapability(&self) -> windows_core::Result<WebViewControlProcessCapabilityState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrivateNetworkClientServerCapability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebViewControlProcessOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlProcessOptions>();
}
unsafe impl windows_core::Interface for WebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
    const IID: windows_core::GUID = <IWebViewControlProcessOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlProcessOptions {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcessOptions";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebViewControlAcceleratorKeyRoutingStage(pub i32);
impl WebViewControlAcceleratorKeyRoutingStage {
    pub const Tunneling: Self = Self(0i32);
    pub const Bubbling: Self = Self(1i32);
}
impl windows_core::TypeKind for WebViewControlAcceleratorKeyRoutingStage {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebViewControlAcceleratorKeyRoutingStage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebViewControlAcceleratorKeyRoutingStage").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebViewControlAcceleratorKeyRoutingStage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlAcceleratorKeyRoutingStage;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebViewControlMoveFocusReason(pub i32);
impl WebViewControlMoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl windows_core::TypeKind for WebViewControlMoveFocusReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebViewControlMoveFocusReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebViewControlMoveFocusReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebViewControlMoveFocusReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlMoveFocusReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebViewControlProcessCapabilityState(pub i32);
impl WebViewControlProcessCapabilityState {
    pub const Default: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl windows_core::TypeKind for WebViewControlProcessCapabilityState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebViewControlProcessCapabilityState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebViewControlProcessCapabilityState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebViewControlProcessCapabilityState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlProcessCapabilityState;i4)");
}

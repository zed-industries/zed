#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
windows_core::imp::define_interface!(IWebViewControl, IWebViewControl_Vtbl, 0x3f921316_bc70_4bda_9136_c94370899fab);
impl windows_core::RuntimeType for IWebViewControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWebViewControl, windows_core::IUnknown, windows_core::IInspectable);
impl IWebViewControl {
    pub fn Source(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
    pub fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentTitle)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
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
    pub fn SetDefaultBackgroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
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
    pub fn Settings(&self) -> windows_core::Result<WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Settings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeferredPermissionRequests(&self) -> windows_core::Result<windows_collections::IVectorView<WebViewControlDeferredPermissionRequest>> {
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
        P0: windows_core::Param<super::super::Foundation::Uri>,
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
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::IUriToStreamResolver>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NavigateToLocalStreamUri)(windows_core::Interface::as_raw(this), source.param().abi(), streamresolver.param().abi()).ok() }
    }
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage<P0>(&self, requestmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Http::HttpRequestMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NavigateWithHttpRequestMessage)(windows_core::Interface::as_raw(this), requestmessage.param().abi()).ok() }
    }
    pub fn InvokeScriptAsync<P1>(&self, scriptname: &windows_core::HSTRING, arguments: P1) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeScriptAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scriptname), arguments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CapturePreviewToStreamAsync<P0>(&self, stream: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePreviewToStreamAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BuildLocalStreamUri(&self, contentidentifier: &windows_core::HSTRING, relativepath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BuildLocalStreamUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentidentifier), core::mem::transmute_copy(relativepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut Option<WebViewControlDeferredPermissionRequest>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetDeferredPermissionRequestById)(windows_core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    pub fn NavigationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ContentLoading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLoading)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentLoading)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DOMContentLoaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DOMContentLoaded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDOMContentLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameNavigationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameNavigationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameNavigationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameContentLoading<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameContentLoading)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameContentLoading(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameContentLoading)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameDOMContentLoaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameDOMContentLoaded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameDOMContentLoaded(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FrameNavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameNavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameNavigationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ScriptNotify<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScriptNotify)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScriptNotify(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScriptNotify)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LongRunningScriptDetected<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LongRunningScriptDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLongRunningScriptDetected(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLongRunningScriptDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnsafeContentWarningDisplaying<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsafeContentWarningDisplaying)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnviewableContentIdentified<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnviewableContentIdentified)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnviewableContentIdentified(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnviewableContentIdentified)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PermissionRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PermissionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePermissionRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePermissionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnsupportedUriSchemeIdentified<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NewWindowRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewWindowRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNewWindowRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNewWindowRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ContainsFullScreenElementChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContainsFullScreenElementChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WebResourceRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebResourceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWebResourceRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWebResourceRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl windows_core::RuntimeName for IWebViewControl {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
pub trait IWebViewControl_Impl: windows_core::IUnknownImpl {
    fn Source(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, source: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CanGoBack(&self) -> windows_core::Result<bool>;
    fn CanGoForward(&self) -> windows_core::Result<bool>;
    fn SetDefaultBackgroundColor(&self, value: &super::super::UI::Color) -> windows_core::Result<()>;
    fn DefaultBackgroundColor(&self) -> windows_core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(&self) -> windows_core::Result<bool>;
    fn Settings(&self) -> windows_core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(&self) -> windows_core::Result<windows_collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(&self) -> windows_core::Result<()>;
    fn GoBack(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Navigate(&self, source: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn NavigateToString(&self, text: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn NavigateToLocalStreamUri(&self, source: windows_core::Ref<super::super::Foundation::Uri>, streamResolver: windows_core::Ref<super::IUriToStreamResolver>) -> windows_core::Result<()>;
    fn NavigateWithHttpRequestMessage(&self, requestMessage: windows_core::Ref<super::Http::HttpRequestMessage>) -> windows_core::Result<()>;
    fn InvokeScriptAsync(&self, scriptName: &windows_core::HSTRING, arguments: windows_core::Ref<windows_collections::IIterable<windows_core::HSTRING>>) -> windows_core::Result<windows_future::IAsyncOperation<windows_core::HSTRING>>;
    fn CapturePreviewToStreamAsync(&self, stream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<windows_future::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(&self, contentIdentifier: &windows_core::HSTRING, relativePath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(&self, id: u32, result: windows_core::OutRef<WebViewControlDeferredPermissionRequest>) -> windows_core::Result<()>;
    fn NavigationStarting(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveNavigationStarting(&self, token: i64) -> windows_core::Result<()>;
    fn ContentLoading(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveContentLoading(&self, token: i64) -> windows_core::Result<()>;
    fn DOMContentLoaded(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveDOMContentLoaded(&self, token: i64) -> windows_core::Result<()>;
    fn NavigationCompleted(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveNavigationCompleted(&self, token: i64) -> windows_core::Result<()>;
    fn FrameNavigationStarting(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveFrameNavigationStarting(&self, token: i64) -> windows_core::Result<()>;
    fn FrameContentLoading(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveFrameContentLoading(&self, token: i64) -> windows_core::Result<()>;
    fn FrameDOMContentLoaded(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveFrameDOMContentLoaded(&self, token: i64) -> windows_core::Result<()>;
    fn FrameNavigationCompleted(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveFrameNavigationCompleted(&self, token: i64) -> windows_core::Result<()>;
    fn ScriptNotify(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveScriptNotify(&self, token: i64) -> windows_core::Result<()>;
    fn LongRunningScriptDetected(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveLongRunningScriptDetected(&self, token: i64) -> windows_core::Result<()>;
    fn UnsafeContentWarningDisplaying(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveUnsafeContentWarningDisplaying(&self, token: i64) -> windows_core::Result<()>;
    fn UnviewableContentIdentified(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveUnviewableContentIdentified(&self, token: i64) -> windows_core::Result<()>;
    fn PermissionRequested(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePermissionRequested(&self, token: i64) -> windows_core::Result<()>;
    fn UnsupportedUriSchemeIdentified(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveUnsupportedUriSchemeIdentified(&self, token: i64) -> windows_core::Result<()>;
    fn NewWindowRequested(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveNewWindowRequested(&self, token: i64) -> windows_core::Result<()>;
    fn ContainsFullScreenElementChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveContainsFullScreenElementChanged(&self, token: i64) -> windows_core::Result<()>;
    fn WebResourceRequested(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveWebResourceRequested(&self, token: i64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl IWebViewControl_Vtbl {
    pub const fn new<Identity: IWebViewControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Source<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::Source(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSource<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::SetSource(this, core::mem::transmute_copy(&source)).into()
            }
        }
        unsafe extern "system" fn DocumentTitle<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::DocumentTitle(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanGoBack<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::CanGoBack(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanGoForward<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::CanGoForward(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultBackgroundColor<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::SetDefaultBackgroundColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn DefaultBackgroundColor<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::DefaultBackgroundColor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContainsFullScreenElement<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::ContainsFullScreenElement(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Settings<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::Settings(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeferredPermissionRequests<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::DeferredPermissionRequests(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GoForward<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::GoForward(this).into()
            }
        }
        unsafe extern "system" fn GoBack<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::GoBack(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Navigate<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::Navigate(this, core::mem::transmute_copy(&source)).into()
            }
        }
        unsafe extern "system" fn NavigateToString<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::NavigateToString(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn NavigateToLocalStreamUri<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void, streamresolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::NavigateToLocalStreamUri(this, core::mem::transmute_copy(&source), core::mem::transmute_copy(&streamresolver)).into()
            }
        }
        unsafe extern "system" fn NavigateWithHttpRequestMessage<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::NavigateWithHttpRequestMessage(this, core::mem::transmute_copy(&requestmessage)).into()
            }
        }
        unsafe extern "system" fn InvokeScriptAsync<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptname: *mut core::ffi::c_void, arguments: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::InvokeScriptAsync(this, core::mem::transmute(&scriptname), core::mem::transmute_copy(&arguments)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CapturePreviewToStreamAsync<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::CapturePreviewToStreamAsync(this, core::mem::transmute_copy(&stream)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CaptureSelectedContentToDataPackageAsync<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::CaptureSelectedContentToDataPackageAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BuildLocalStreamUri<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentidentifier: *mut core::ffi::c_void, relativepath: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::BuildLocalStreamUri(this, core::mem::transmute(&contentidentifier), core::mem::transmute(&relativepath)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeferredPermissionRequestById<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::GetDeferredPermissionRequestById(this, id, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn NavigationStarting<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::NavigationStarting(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveNavigationStarting<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveNavigationStarting(this, token).into()
            }
        }
        unsafe extern "system" fn ContentLoading<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::ContentLoading(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveContentLoading<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveContentLoading(this, token).into()
            }
        }
        unsafe extern "system" fn DOMContentLoaded<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::DOMContentLoaded(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveDOMContentLoaded<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveDOMContentLoaded(this, token).into()
            }
        }
        unsafe extern "system" fn NavigationCompleted<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::NavigationCompleted(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveNavigationCompleted(this, token).into()
            }
        }
        unsafe extern "system" fn FrameNavigationStarting<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::FrameNavigationStarting(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationStarting<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveFrameNavigationStarting(this, token).into()
            }
        }
        unsafe extern "system" fn FrameContentLoading<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::FrameContentLoading(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveFrameContentLoading<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveFrameContentLoading(this, token).into()
            }
        }
        unsafe extern "system" fn FrameDOMContentLoaded<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::FrameDOMContentLoaded(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveFrameDOMContentLoaded<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveFrameDOMContentLoaded(this, token).into()
            }
        }
        unsafe extern "system" fn FrameNavigationCompleted<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::FrameNavigationCompleted(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationCompleted<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveFrameNavigationCompleted(this, token).into()
            }
        }
        unsafe extern "system" fn ScriptNotify<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::ScriptNotify(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveScriptNotify<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveScriptNotify(this, token).into()
            }
        }
        unsafe extern "system" fn LongRunningScriptDetected<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::LongRunningScriptDetected(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveLongRunningScriptDetected<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveLongRunningScriptDetected(this, token).into()
            }
        }
        unsafe extern "system" fn UnsafeContentWarningDisplaying<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::UnsafeContentWarningDisplaying(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveUnsafeContentWarningDisplaying<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveUnsafeContentWarningDisplaying(this, token).into()
            }
        }
        unsafe extern "system" fn UnviewableContentIdentified<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::UnviewableContentIdentified(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveUnviewableContentIdentified<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveUnviewableContentIdentified(this, token).into()
            }
        }
        unsafe extern "system" fn PermissionRequested<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::PermissionRequested(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePermissionRequested<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemovePermissionRequested(this, token).into()
            }
        }
        unsafe extern "system" fn UnsupportedUriSchemeIdentified<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::UnsupportedUriSchemeIdentified(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveUnsupportedUriSchemeIdentified<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveUnsupportedUriSchemeIdentified(this, token).into()
            }
        }
        unsafe extern "system" fn NewWindowRequested<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::NewWindowRequested(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveNewWindowRequested<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveNewWindowRequested(this, token).into()
            }
        }
        unsafe extern "system" fn ContainsFullScreenElementChanged<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::ContainsFullScreenElementChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveContainsFullScreenElementChanged<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveContainsFullScreenElementChanged(this, token).into()
            }
        }
        unsafe extern "system" fn WebResourceRequested<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebViewControl_Impl::WebResourceRequested(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveWebResourceRequested<Identity: IWebViewControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl_Impl::RemoveWebResourceRequested(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebViewControl, OFFSET>(),
            Source: Source::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            DocumentTitle: DocumentTitle::<Identity, OFFSET>,
            CanGoBack: CanGoBack::<Identity, OFFSET>,
            CanGoForward: CanGoForward::<Identity, OFFSET>,
            SetDefaultBackgroundColor: SetDefaultBackgroundColor::<Identity, OFFSET>,
            DefaultBackgroundColor: DefaultBackgroundColor::<Identity, OFFSET>,
            ContainsFullScreenElement: ContainsFullScreenElement::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            DeferredPermissionRequests: DeferredPermissionRequests::<Identity, OFFSET>,
            GoForward: GoForward::<Identity, OFFSET>,
            GoBack: GoBack::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Navigate: Navigate::<Identity, OFFSET>,
            NavigateToString: NavigateToString::<Identity, OFFSET>,
            NavigateToLocalStreamUri: NavigateToLocalStreamUri::<Identity, OFFSET>,
            NavigateWithHttpRequestMessage: NavigateWithHttpRequestMessage::<Identity, OFFSET>,
            InvokeScriptAsync: InvokeScriptAsync::<Identity, OFFSET>,
            CapturePreviewToStreamAsync: CapturePreviewToStreamAsync::<Identity, OFFSET>,
            CaptureSelectedContentToDataPackageAsync: CaptureSelectedContentToDataPackageAsync::<Identity, OFFSET>,
            BuildLocalStreamUri: BuildLocalStreamUri::<Identity, OFFSET>,
            GetDeferredPermissionRequestById: GetDeferredPermissionRequestById::<Identity, OFFSET>,
            NavigationStarting: NavigationStarting::<Identity, OFFSET>,
            RemoveNavigationStarting: RemoveNavigationStarting::<Identity, OFFSET>,
            ContentLoading: ContentLoading::<Identity, OFFSET>,
            RemoveContentLoading: RemoveContentLoading::<Identity, OFFSET>,
            DOMContentLoaded: DOMContentLoaded::<Identity, OFFSET>,
            RemoveDOMContentLoaded: RemoveDOMContentLoaded::<Identity, OFFSET>,
            NavigationCompleted: NavigationCompleted::<Identity, OFFSET>,
            RemoveNavigationCompleted: RemoveNavigationCompleted::<Identity, OFFSET>,
            FrameNavigationStarting: FrameNavigationStarting::<Identity, OFFSET>,
            RemoveFrameNavigationStarting: RemoveFrameNavigationStarting::<Identity, OFFSET>,
            FrameContentLoading: FrameContentLoading::<Identity, OFFSET>,
            RemoveFrameContentLoading: RemoveFrameContentLoading::<Identity, OFFSET>,
            FrameDOMContentLoaded: FrameDOMContentLoaded::<Identity, OFFSET>,
            RemoveFrameDOMContentLoaded: RemoveFrameDOMContentLoaded::<Identity, OFFSET>,
            FrameNavigationCompleted: FrameNavigationCompleted::<Identity, OFFSET>,
            RemoveFrameNavigationCompleted: RemoveFrameNavigationCompleted::<Identity, OFFSET>,
            ScriptNotify: ScriptNotify::<Identity, OFFSET>,
            RemoveScriptNotify: RemoveScriptNotify::<Identity, OFFSET>,
            LongRunningScriptDetected: LongRunningScriptDetected::<Identity, OFFSET>,
            RemoveLongRunningScriptDetected: RemoveLongRunningScriptDetected::<Identity, OFFSET>,
            UnsafeContentWarningDisplaying: UnsafeContentWarningDisplaying::<Identity, OFFSET>,
            RemoveUnsafeContentWarningDisplaying: RemoveUnsafeContentWarningDisplaying::<Identity, OFFSET>,
            UnviewableContentIdentified: UnviewableContentIdentified::<Identity, OFFSET>,
            RemoveUnviewableContentIdentified: RemoveUnviewableContentIdentified::<Identity, OFFSET>,
            PermissionRequested: PermissionRequested::<Identity, OFFSET>,
            RemovePermissionRequested: RemovePermissionRequested::<Identity, OFFSET>,
            UnsupportedUriSchemeIdentified: UnsupportedUriSchemeIdentified::<Identity, OFFSET>,
            RemoveUnsupportedUriSchemeIdentified: RemoveUnsupportedUriSchemeIdentified::<Identity, OFFSET>,
            NewWindowRequested: NewWindowRequested::<Identity, OFFSET>,
            RemoveNewWindowRequested: RemoveNewWindowRequested::<Identity, OFFSET>,
            ContainsFullScreenElementChanged: ContainsFullScreenElementChanged::<Identity, OFFSET>,
            RemoveContainsFullScreenElementChanged: RemoveContainsFullScreenElementChanged::<Identity, OFFSET>,
            WebResourceRequested: WebResourceRequested::<Identity, OFFSET>,
            RemoveWebResourceRequested: RemoveWebResourceRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebViewControl as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDefaultBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub DefaultBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DefaultBackgroundColor: usize,
    pub ContainsFullScreenElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeferredPermissionRequests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoForward: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoBack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NavigateToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NavigateToLocalStreamUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub NavigateWithHttpRequestMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    NavigateWithHttpRequestMessage: usize,
    pub InvokeScriptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CapturePreviewToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CapturePreviewToStreamAsync: usize,
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub CaptureSelectedContentToDataPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))]
    CaptureSelectedContentToDataPackageAsync: usize,
    pub BuildLocalStreamUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferredPermissionRequestById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NavigationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNavigationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ContentLoading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveContentLoading: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub DOMContentLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub NavigationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNavigationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FrameNavigationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FrameContentLoading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFrameContentLoading: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FrameDOMContentLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFrameDOMContentLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FrameNavigationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ScriptNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveScriptNotify: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LongRunningScriptDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLongRunningScriptDetected: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub UnsafeContentWarningDisplaying: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUnsafeContentWarningDisplaying: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub UnviewableContentIdentified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUnviewableContentIdentified: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PermissionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePermissionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub UnsupportedUriSchemeIdentified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUnsupportedUriSchemeIdentified: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub NewWindowRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNewWindowRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveContainsFullScreenElementChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WebResourceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveWebResourceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControl2, IWebViewControl2_Vtbl, 0x4d3c06f9_c8df_41cc_8bd5_2a947b204503);
impl windows_core::RuntimeType for IWebViewControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWebViewControl2, windows_core::IUnknown, windows_core::IInspectable);
impl IWebViewControl2 {
    pub fn AddInitializeScript(&self, script: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInitializeScript)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(script)).ok() }
    }
}
impl windows_core::RuntimeName for IWebViewControl2 {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl2";
}
pub trait IWebViewControl2_Impl: windows_core::IUnknownImpl {
    fn AddInitializeScript(&self, script: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IWebViewControl2_Vtbl {
    pub const fn new<Identity: IWebViewControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddInitializeScript<Identity: IWebViewControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, script: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebViewControl2_Impl::AddInitializeScript(this, core::mem::transmute(&script)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebViewControl2, OFFSET>(),
            AddInitializeScript: AddInitializeScript::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebViewControl2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddInitializeScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlContentLoadingEventArgs, IWebViewControlContentLoadingEventArgs_Vtbl, 0x9a3fccb2_b9bb_404b_a22b_66dccd1250c6);
impl windows_core::RuntimeType for IWebViewControlContentLoadingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlContentLoadingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlDOMContentLoadedEventArgs, IWebViewControlDOMContentLoadedEventArgs_Vtbl, 0xbe8bc008_9541_4545_9ff2_2df585b29f7d);
impl windows_core::RuntimeType for IWebViewControlDOMContentLoadedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDOMContentLoadedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlDeferredPermissionRequest, IWebViewControlDeferredPermissionRequest_Vtbl, 0x2ce349e0_d759_445c_9926_8995298f152b);
impl windows_core::RuntimeType for IWebViewControlDeferredPermissionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDeferredPermissionRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PermissionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebViewControlPermissionType) -> windows_core::HRESULT,
    pub Allow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deny: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlLongRunningScriptDetectedEventArgs, IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl, 0x2a6e5bba_98b4_45bc_bbeb_0f69ce49c599);
impl windows_core::RuntimeType for IWebViewControlLongRunningScriptDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExecutionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StopPageScriptExecution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStopPageScriptExecution: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlNavigationCompletedEventArgs, IWebViewControlNavigationCompletedEventArgs_Vtbl, 0x20409918_4a15_4c46_a55d_f79edb0bde8b);
impl windows_core::RuntimeType for IWebViewControlNavigationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSuccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WebErrorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlNavigationStartingEventArgs, IWebViewControlNavigationStartingEventArgs_Vtbl, 0x0c9057c5_0a08_41c7_863b_71e3a9549137);
impl windows_core::RuntimeType for IWebViewControlNavigationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlNewWindowRequestedEventArgs, IWebViewControlNewWindowRequestedEventArgs_Vtbl, 0x3df44bbb_a124_46d5_a083_d02cacdff5ad);
impl windows_core::RuntimeType for IWebViewControlNewWindowRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Referrer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlNewWindowRequestedEventArgs2, IWebViewControlNewWindowRequestedEventArgs2_Vtbl, 0xb53c5ca6_2aae_4bfc_92b9_c30e92b48098);
impl windows_core::RuntimeType for IWebViewControlNewWindowRequestedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlPermissionRequest, IWebViewControlPermissionRequest_Vtbl, 0xe5bc836c_f22f_40e2_95b2_7729f840eb7f);
impl windows_core::RuntimeType for IWebViewControlPermissionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PermissionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebViewControlPermissionType) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebViewControlPermissionState) -> windows_core::HRESULT,
    pub Defer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Allow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deny: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlPermissionRequestedEventArgs, IWebViewControlPermissionRequestedEventArgs_Vtbl, 0x27204d51_2488_4cc5_968e_0a771e59c147);
impl windows_core::RuntimeType for IWebViewControlPermissionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PermissionRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlScriptNotifyEventArgs, IWebViewControlScriptNotifyEventArgs_Vtbl, 0x491de57b_6f49_41bb_b591_51b85b817037);
impl windows_core::RuntimeType for IWebViewControlScriptNotifyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlScriptNotifyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlSettings, IWebViewControlSettings_Vtbl, 0xc9967fbf_5e98_4cfd_8cce_27b0911e3de8);
impl windows_core::RuntimeType for IWebViewControlSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsJavaScriptEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsJavaScriptEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsIndexedDBEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsIndexedDBEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsScriptNotifyAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsScriptNotifyAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs, IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl, 0xe3b81944_e4fc_43dc_94ca_f980f30bc51d);
impl windows_core::RuntimeType for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlUnviewableContentIdentifiedEventArgs, IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl, 0x4a9680db_88f2_4e20_b693_b4e2df4aa581);
impl windows_core::RuntimeType for IWebViewControlUnviewableContentIdentifiedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Referrer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebViewControlWebResourceRequestedEventArgs, IWebViewControlWebResourceRequestedEventArgs_Vtbl, 0x44d6524d_55a4_4d8b_891c_931d8e25d42e);
impl windows_core::RuntimeType for IWebViewControlWebResourceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlWebResourceRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Request: usize,
    #[cfg(feature = "Web_Http")]
    pub SetResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    SetResponse: usize,
    #[cfg(feature = "Web_Http")]
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Response: usize,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlContentLoadingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlContentLoadingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlContentLoadingEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlContentLoadingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlContentLoadingEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlContentLoadingEventArgs {
    type Vtable = <IWebViewControlContentLoadingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlContentLoadingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlContentLoadingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlContentLoadingEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlDOMContentLoadedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlDOMContentLoadedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlDOMContentLoadedEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlDOMContentLoadedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlDOMContentLoadedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlDOMContentLoadedEventArgs {
    type Vtable = <IWebViewControlDOMContentLoadedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlDOMContentLoadedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlDOMContentLoadedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlDeferredPermissionRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlDeferredPermissionRequest, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlDeferredPermissionRequest {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PermissionType(&self) -> windows_core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PermissionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Allow(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Allow)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Deny(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Deny)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WebViewControlDeferredPermissionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlDeferredPermissionRequest>();
}
unsafe impl windows_core::Interface for WebViewControlDeferredPermissionRequest {
    type Vtable = <IWebViewControlDeferredPermissionRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlDeferredPermissionRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlDeferredPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDeferredPermissionRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlLongRunningScriptDetectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlLongRunningScriptDetectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlLongRunningScriptDetectedEventArgs {
    pub fn ExecutionTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutionTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StopPageScriptExecution(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopPageScriptExecution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStopPageScriptExecution(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStopPageScriptExecution)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for WebViewControlLongRunningScriptDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlLongRunningScriptDetectedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = <IWebViewControlLongRunningScriptDetectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlLongRunningScriptDetectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlLongRunningScriptDetectedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlNavigationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlNavigationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlNavigationCompletedEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSuccess(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WebErrorStatus(&self) -> windows_core::Result<super::WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebErrorStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebViewControlNavigationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlNavigationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlNavigationCompletedEventArgs {
    type Vtable = <IWebViewControlNavigationCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlNavigationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationCompletedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlNavigationStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlNavigationStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlNavigationStartingEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
}
impl windows_core::RuntimeType for WebViewControlNavigationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlNavigationStartingEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlNavigationStartingEventArgs {
    type Vtable = <IWebViewControlNavigationStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlNavigationStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlNavigationStartingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationStartingEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlNewWindowRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlNewWindowRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlNewWindowRequestedEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Referrer(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Referrer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn NewWindow(&self) -> windows_core::Result<IWebViewControl> {
        let this = &windows_core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewWindow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNewWindow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWebViewControl>,
    {
        let this = &windows_core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNewWindow)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = &windows_core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlNewWindowRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlNewWindowRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlNewWindowRequestedEventArgs {
    type Vtable = <IWebViewControlNewWindowRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlNewWindowRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlNewWindowRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlPermissionRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlPermissionRequest, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlPermissionRequest {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PermissionType(&self) -> windows_core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PermissionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<WebViewControlPermissionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Defer(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Defer)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Allow(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Allow)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Deny(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Deny)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WebViewControlPermissionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlPermissionRequest>();
}
unsafe impl windows_core::Interface for WebViewControlPermissionRequest {
    type Vtable = <IWebViewControlPermissionRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlPermissionRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlPermissionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlPermissionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlPermissionRequestedEventArgs {
    pub fn PermissionRequest(&self) -> windows_core::Result<WebViewControlPermissionRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PermissionRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlPermissionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlPermissionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlPermissionRequestedEventArgs {
    type Vtable = <IWebViewControlPermissionRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlPermissionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlPermissionRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebViewControlPermissionState(pub i32);
impl WebViewControlPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl windows_core::TypeKind for WebViewControlPermissionState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WebViewControlPermissionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.WebViewControlPermissionState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebViewControlPermissionType(pub i32);
impl WebViewControlPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl windows_core::TypeKind for WebViewControlPermissionType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WebViewControlPermissionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.WebViewControlPermissionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlScriptNotifyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlScriptNotifyEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlScriptNotifyEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlScriptNotifyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlScriptNotifyEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlScriptNotifyEventArgs {
    type Vtable = <IWebViewControlScriptNotifyEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlScriptNotifyEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlScriptNotifyEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlScriptNotifyEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlSettings, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlSettings {
    pub fn SetIsJavaScriptEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsJavaScriptEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsJavaScriptEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsJavaScriptEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsIndexedDBEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsIndexedDBEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsIndexedDBEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIndexedDBEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsScriptNotifyAllowed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsScriptNotifyAllowed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsScriptNotifyAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScriptNotifyAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebViewControlSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlSettings>();
}
unsafe impl windows_core::Interface for WebViewControlSettings {
    type Vtable = <IWebViewControlSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlSettings {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlSettings";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlUnsupportedUriSchemeIdentifiedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = <IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlUnviewableContentIdentifiedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlUnviewableContentIdentifiedEventArgs {
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Referrer(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Referrer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlUnviewableContentIdentifiedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlUnviewableContentIdentifiedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = <IWebViewControlUnviewableContentIdentifiedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlUnviewableContentIdentifiedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlUnviewableContentIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebViewControlWebResourceRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebViewControlWebResourceRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebViewControlWebResourceRequestedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn Request(&self) -> windows_core::Result<super::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn SetResponse<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Http::HttpResponseMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResponse)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Web_Http")]
    pub fn Response(&self) -> windows_core::Result<super::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Response)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebViewControlWebResourceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebViewControlWebResourceRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WebViewControlWebResourceRequestedEventArgs {
    type Vtable = <IWebViewControlWebResourceRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebViewControlWebResourceRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebViewControlWebResourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs";
}

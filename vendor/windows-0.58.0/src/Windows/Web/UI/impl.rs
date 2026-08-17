#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
pub trait IWebViewControl_Impl: Sized {
    fn Source(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, source: Option<&super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn DocumentTitle(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CanGoBack(&self) -> windows_core::Result<bool>;
    fn CanGoForward(&self) -> windows_core::Result<bool>;
    fn SetDefaultBackgroundColor(&self, value: &super::super::UI::Color) -> windows_core::Result<()>;
    fn DefaultBackgroundColor(&self) -> windows_core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(&self) -> windows_core::Result<bool>;
    fn Settings(&self) -> windows_core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(&self) -> windows_core::Result<()>;
    fn GoBack(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Navigate(&self, source: Option<&super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn NavigateToString(&self, text: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn NavigateToLocalStreamUri(&self, source: Option<&super::super::Foundation::Uri>, streamresolver: Option<&super::IUriToStreamResolver>) -> windows_core::Result<()>;
    fn NavigateWithHttpRequestMessage(&self, requestmessage: Option<&super::Http::HttpRequestMessage>) -> windows_core::Result<()>;
    fn InvokeScriptAsync(&self, scriptname: &windows_core::HSTRING, arguments: Option<&super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>;
    fn CapturePreviewToStreamAsync(&self, stream: Option<&super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(&self, contentidentifier: &windows_core::HSTRING, relativepath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut Option<WebViewControlDeferredPermissionRequest>) -> windows_core::Result<()>;
    fn NavigationStarting(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn ContentLoading(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn DOMContentLoaded(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn NavigationCompleted(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn FrameNavigationStarting(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn FrameContentLoading(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn FrameDOMContentLoaded(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn FrameNavigationCompleted(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn ScriptNotify(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn LongRunningScriptDetected(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn UnsafeContentWarningDisplaying(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn UnviewableContentIdentified(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PermissionRequested(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn UnsupportedUriSchemeIdentified(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn NewWindowRequested(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn ContainsFullScreenElementChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn WebResourceRequested(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl windows_core::RuntimeName for IWebViewControl {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl IWebViewControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebViewControl_Vtbl
    where
        Identity: IWebViewControl_Impl,
    {
        unsafe extern "system" fn Source<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
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
        unsafe extern "system" fn SetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::SetSource(this, windows_core::from_raw_borrowed(&source)).into()
        }
        unsafe extern "system" fn DocumentTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
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
        unsafe extern "system" fn CanGoBack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::CanGoBack(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoForward<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::CanGoForward(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultBackgroundColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::UI::Color) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::SetDefaultBackgroundColor(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DefaultBackgroundColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::UI::Color) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::DefaultBackgroundColor(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsFullScreenElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::ContainsFullScreenElement(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
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
        unsafe extern "system" fn DeferredPermissionRequests<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
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
        unsafe extern "system" fn GoForward<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::GoForward(this).into()
        }
        unsafe extern "system" fn GoBack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::GoBack(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Navigate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::Navigate(this, windows_core::from_raw_borrowed(&source)).into()
        }
        unsafe extern "system" fn NavigateToString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::NavigateToString(this, core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn NavigateToLocalStreamUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void, streamresolver: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::NavigateToLocalStreamUri(this, windows_core::from_raw_borrowed(&source), windows_core::from_raw_borrowed(&streamresolver)).into()
        }
        unsafe extern "system" fn NavigateWithHttpRequestMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestmessage: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::NavigateWithHttpRequestMessage(this, windows_core::from_raw_borrowed(&requestmessage)).into()
        }
        unsafe extern "system" fn InvokeScriptAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptname: core::mem::MaybeUninit<windows_core::HSTRING>, arguments: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::InvokeScriptAsync(this, core::mem::transmute(&scriptname), windows_core::from_raw_borrowed(&arguments)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePreviewToStreamAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::CapturePreviewToStreamAsync(this, windows_core::from_raw_borrowed(&stream)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureSelectedContentToDataPackageAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
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
        unsafe extern "system" fn BuildLocalStreamUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentidentifier: core::mem::MaybeUninit<windows_core::HSTRING>, relativepath: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
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
        unsafe extern "system" fn GetDeferredPermissionRequestById<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::GetDeferredPermissionRequestById(this, id, core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn NavigationStarting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::NavigationStarting(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationStarting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveNavigationStarting(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContentLoading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::ContentLoading(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentLoading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveContentLoading(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn DOMContentLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::DOMContentLoaded(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDOMContentLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveDOMContentLoaded(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn NavigationCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::NavigationCompleted(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveNavigationCompleted(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameNavigationStarting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::FrameNavigationStarting(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationStarting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveFrameNavigationStarting(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameContentLoading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::FrameContentLoading(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameContentLoading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveFrameContentLoading(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameDOMContentLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::FrameDOMContentLoaded(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameDOMContentLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveFrameDOMContentLoaded(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameNavigationCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::FrameNavigationCompleted(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveFrameNavigationCompleted(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ScriptNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::ScriptNotify(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScriptNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveScriptNotify(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn LongRunningScriptDetected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::LongRunningScriptDetected(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLongRunningScriptDetected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveLongRunningScriptDetected(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnsafeContentWarningDisplaying<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::UnsafeContentWarningDisplaying(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsafeContentWarningDisplaying<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveUnsafeContentWarningDisplaying(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnviewableContentIdentified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::UnviewableContentIdentified(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnviewableContentIdentified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveUnviewableContentIdentified(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn PermissionRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::PermissionRequested(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePermissionRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemovePermissionRequested(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnsupportedUriSchemeIdentified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::UnsupportedUriSchemeIdentified(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsupportedUriSchemeIdentified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveUnsupportedUriSchemeIdentified(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn NewWindowRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::NewWindowRequested(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNewWindowRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveNewWindowRequested(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContainsFullScreenElementChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::ContainsFullScreenElementChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContainsFullScreenElementChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveContainsFullScreenElementChanged(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn WebResourceRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebViewControl_Impl::WebResourceRequested(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWebResourceRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebViewControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl_Impl::RemoveWebResourceRequested(this, core::mem::transmute(&token)).into()
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
pub trait IWebViewControl2_Impl: Sized {
    fn AddInitializeScript(&self, script: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWebViewControl2 {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl2";
}
impl IWebViewControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebViewControl2_Vtbl
    where
        Identity: IWebViewControl2_Impl,
    {
        unsafe extern "system" fn AddInitializeScript<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, script: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IWebViewControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebViewControl2_Impl::AddInitializeScript(this, core::mem::transmute(&script)).into()
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

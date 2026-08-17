windows_core::imp::define_interface!(IHttpDiagnosticProvider, IHttpDiagnosticProvider_Vtbl, 0xbd811501_a056_4d39_b174_833b7b03b02c);
impl windows_core::RuntimeType for IHttpDiagnosticProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRequestSent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ResponseReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveResponseReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestResponseCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRequestResponseCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpDiagnosticProviderRequestResponseCompletedEventArgs, IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl, 0x735f98ee_94f6_4532_b26e_61e1b1e4efd4);
impl windows_core::RuntimeType for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Timestamps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initiator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HttpDiagnosticRequestInitiator) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
windows_core::imp::define_interface!(IHttpDiagnosticProviderRequestResponseTimestamps, IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl, 0xe0afde10_55cf_4c01_91d4_a20557d849f0);
impl windows_core::RuntimeType for IHttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CacheCheckedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionInitiatedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NameResolvedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SslNegotiatedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionCompletedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestSentTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCompletedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseReceivedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseCompletedTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpDiagnosticProviderRequestSentEventArgs, IHttpDiagnosticProviderRequestSentEventArgs_Vtbl, 0x3f5196d0_4c1f_4ebe_a57a_06930771c50d);
impl windows_core::RuntimeType for IHttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initiator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HttpDiagnosticRequestInitiator) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceLocations: usize,
}
windows_core::imp::define_interface!(IHttpDiagnosticProviderResponseReceivedEventArgs, IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl, 0xa0a2566c_ab5f_4d66_bb2d_084cf41635d0);
impl windows_core::RuntimeType for IHttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpDiagnosticProviderStatics, IHttpDiagnosticProviderStatics_Vtbl, 0x5b824ec1_6a6c_47cc_afec_1e86bc26053b);
impl windows_core::RuntimeType for IHttpDiagnosticProviderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticProviderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System_Diagnostics")]
    pub CreateFromProcessDiagnosticInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System_Diagnostics"))]
    CreateFromProcessDiagnosticInfo: usize,
}
windows_core::imp::define_interface!(IHttpDiagnosticSourceLocation, IHttpDiagnosticSourceLocation_Vtbl, 0x54a9d260_8860_423f_b6fa_d77716f647a7);
impl windows_core::RuntimeType for IHttpDiagnosticSourceLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpDiagnosticSourceLocation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ColumnNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpDiagnosticProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDiagnosticProvider, windows_core::IUnknown, windows_core::IInspectable);
impl HttpDiagnosticProvider {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestSent<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSent)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRequestSent(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestSent)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ResponseReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResponseReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveResponseReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestResponseCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestResponseCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRequestResponseCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestResponseCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System_Diagnostics")]
    pub fn CreateFromProcessDiagnosticInfo<P0>(processdiagnosticinfo: P0) -> windows_core::Result<HttpDiagnosticProvider>
    where
        P0: windows_core::Param<super::super::super::System::Diagnostics::ProcessDiagnosticInfo>,
    {
        Self::IHttpDiagnosticProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromProcessDiagnosticInfo)(windows_core::Interface::as_raw(this), processdiagnosticinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHttpDiagnosticProviderStatics<R, F: FnOnce(&IHttpDiagnosticProviderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpDiagnosticProvider, IHttpDiagnosticProviderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpDiagnosticProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDiagnosticProvider>();
}
unsafe impl windows_core::Interface for HttpDiagnosticProvider {
    type Vtable = IHttpDiagnosticProvider_Vtbl;
    const IID: windows_core::GUID = <IHttpDiagnosticProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDiagnosticProvider {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProvider";
}
unsafe impl Send for HttpDiagnosticProvider {}
unsafe impl Sync for HttpDiagnosticProvider {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDiagnosticProviderRequestResponseCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamps(&self) -> windows_core::Result<HttpDiagnosticProviderRequestResponseTimestamps> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestedUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ThreadId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThreadId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Initiator(&self) -> windows_core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Initiator)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceLocations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDiagnosticProviderRequestResponseCompletedEventArgs>();
}
unsafe impl windows_core::Interface for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestResponseCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHttpDiagnosticProviderRequestResponseCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseCompletedEventArgs";
}
unsafe impl Send for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
unsafe impl Sync for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDiagnosticProviderRequestResponseTimestamps, windows_core::IUnknown, windows_core::IInspectable);
impl HttpDiagnosticProviderRequestResponseTimestamps {
    pub fn CacheCheckedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheCheckedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionInitiatedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionInitiatedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NameResolvedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NameResolvedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SslNegotiatedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SslNegotiatedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionCompletedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionCompletedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestSentTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSentTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestCompletedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCompletedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResponseReceivedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseReceivedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResponseCompletedTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCompletedTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpDiagnosticProviderRequestResponseTimestamps {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDiagnosticProviderRequestResponseTimestamps>();
}
unsafe impl windows_core::Interface for HttpDiagnosticProviderRequestResponseTimestamps {
    type Vtable = IHttpDiagnosticProviderRequestResponseTimestamps_Vtbl;
    const IID: windows_core::GUID = <IHttpDiagnosticProviderRequestResponseTimestamps as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDiagnosticProviderRequestResponseTimestamps {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestResponseTimestamps";
}
unsafe impl Send for HttpDiagnosticProviderRequestResponseTimestamps {}
unsafe impl Sync for HttpDiagnosticProviderRequestResponseTimestamps {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDiagnosticProviderRequestSentEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HttpDiagnosticProviderRequestSentEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ThreadId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThreadId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Initiator(&self) -> windows_core::Result<HttpDiagnosticRequestInitiator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Initiator)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceLocations(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<HttpDiagnosticSourceLocation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceLocations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpDiagnosticProviderRequestSentEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDiagnosticProviderRequestSentEventArgs>();
}
unsafe impl windows_core::Interface for HttpDiagnosticProviderRequestSentEventArgs {
    type Vtable = IHttpDiagnosticProviderRequestSentEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHttpDiagnosticProviderRequestSentEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDiagnosticProviderRequestSentEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderRequestSentEventArgs";
}
unsafe impl Send for HttpDiagnosticProviderRequestSentEventArgs {}
unsafe impl Sync for HttpDiagnosticProviderRequestSentEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDiagnosticProviderResponseReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HttpDiagnosticProviderResponseReceivedEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<super::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpDiagnosticProviderResponseReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDiagnosticProviderResponseReceivedEventArgs>();
}
unsafe impl windows_core::Interface for HttpDiagnosticProviderResponseReceivedEventArgs {
    type Vtable = IHttpDiagnosticProviderResponseReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHttpDiagnosticProviderResponseReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDiagnosticProviderResponseReceivedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticProviderResponseReceivedEventArgs";
}
unsafe impl Send for HttpDiagnosticProviderResponseReceivedEventArgs {}
unsafe impl Sync for HttpDiagnosticProviderResponseReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpDiagnosticSourceLocation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDiagnosticSourceLocation, windows_core::IUnknown, windows_core::IInspectable);
impl HttpDiagnosticSourceLocation {
    pub fn SourceUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LineNumber(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColumnNumber(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColumnNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HttpDiagnosticSourceLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDiagnosticSourceLocation>();
}
unsafe impl windows_core::Interface for HttpDiagnosticSourceLocation {
    type Vtable = IHttpDiagnosticSourceLocation_Vtbl;
    const IID: windows_core::GUID = <IHttpDiagnosticSourceLocation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDiagnosticSourceLocation {
    const NAME: &'static str = "Windows.Web.Http.Diagnostics.HttpDiagnosticSourceLocation";
}
unsafe impl Send for HttpDiagnosticSourceLocation {}
unsafe impl Sync for HttpDiagnosticSourceLocation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: Self = Self(0i32);
    pub const Script: Self = Self(1i32);
    pub const Image: Self = Self(2i32);
    pub const Link: Self = Self(3i32);
    pub const Style: Self = Self(4i32);
    pub const XmlHttpRequest: Self = Self(5i32);
    pub const Media: Self = Self(6i32);
    pub const HtmlDownload: Self = Self(7i32);
    pub const Prefetch: Self = Self(8i32);
    pub const Other: Self = Self(9i32);
    pub const CrossOriginPreFlight: Self = Self(10i32);
    pub const Fetch: Self = Self(11i32);
    pub const Beacon: Self = Self(12i32);
}
impl windows_core::TypeKind for HttpDiagnosticRequestInitiator {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HttpDiagnosticRequestInitiator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HttpDiagnosticRequestInitiator").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HttpDiagnosticRequestInitiator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Diagnostics.HttpDiagnosticRequestInitiator;i4)");
}

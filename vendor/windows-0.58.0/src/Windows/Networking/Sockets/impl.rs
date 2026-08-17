pub trait IControlChannelTriggerEventDetails_Impl: Sized {
    fn ControlChannelTrigger(&self) -> windows_core::Result<ControlChannelTrigger>;
}
impl windows_core::RuntimeName for IControlChannelTriggerEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerEventDetails";
}
impl IControlChannelTriggerEventDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IControlChannelTriggerEventDetails_Vtbl
    where
        Identity: IControlChannelTriggerEventDetails_Impl,
    {
        unsafe extern "system" fn ControlChannelTrigger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IControlChannelTriggerEventDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IControlChannelTriggerEventDetails_Impl::ControlChannelTrigger(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlChannelTriggerEventDetails, OFFSET>(),
            ControlChannelTrigger: ControlChannelTrigger::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlChannelTriggerEventDetails as windows_core::Interface>::IID
    }
}
pub trait IControlChannelTriggerResetEventDetails_Impl: Sized {
    fn ResetReason(&self) -> windows_core::Result<ControlChannelTriggerResetReason>;
    fn HardwareSlotReset(&self) -> windows_core::Result<bool>;
    fn SoftwareSlotReset(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IControlChannelTriggerResetEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails";
}
impl IControlChannelTriggerResetEventDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IControlChannelTriggerResetEventDetails_Vtbl
    where
        Identity: IControlChannelTriggerResetEventDetails_Impl,
    {
        unsafe extern "system" fn ResetReason<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> windows_core::HRESULT
        where
            Identity: IControlChannelTriggerResetEventDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IControlChannelTriggerResetEventDetails_Impl::ResetReason(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareSlotReset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IControlChannelTriggerResetEventDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IControlChannelTriggerResetEventDetails_Impl::HardwareSlotReset(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareSlotReset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IControlChannelTriggerResetEventDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IControlChannelTriggerResetEventDetails_Impl::SoftwareSlotReset(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IControlChannelTriggerResetEventDetails, OFFSET>(),
            ResetReason: ResetReason::<Identity, OFFSET>,
            HardwareSlotReset: HardwareSlotReset::<Identity, OFFSET>,
            SoftwareSlotReset: SoftwareSlotReset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlChannelTriggerResetEventDetails as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait IWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&self, uri: Option<&super::super::Foundation::Uri>) -> windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Closed(&self, eventhandler: Option<&super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &windows_core::HSTRING) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocket";
}
#[cfg(feature = "Storage_Streams")]
impl IWebSocket_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebSocket_Vtbl
    where
        Identity: IWebSocket_Impl,
    {
        unsafe extern "system" fn OutputStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocket_Impl::OutputStream(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocket_Impl::ConnectAsync(this, windows_core::from_raw_borrowed(&uri)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, headername: core::mem::MaybeUninit<windows_core::HSTRING>, headervalue: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IWebSocket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebSocket_Impl::SetRequestHeader(this, core::mem::transmute(&headername), core::mem::transmute(&headervalue)).into()
        }
        unsafe extern "system" fn Closed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebSocket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocket_Impl::Closed(this, windows_core::from_raw_borrowed(&eventhandler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IWebSocket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebSocket_Impl::RemoveClosed(this, core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, code: u16, reason: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IWebSocket_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebSocket_Impl::CloseWithStatus(this, code, core::mem::transmute(&reason)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebSocket, OFFSET>(),
            OutputStream: OutputStream::<Identity, OFFSET>,
            ConnectAsync: ConnectAsync::<Identity, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, OFFSET>,
            Closed: Closed::<Identity, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, OFFSET>,
            CloseWithStatus: CloseWithStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebSocket as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
pub trait IWebSocketControl_Impl: Sized {
    fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()>;
    fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: Option<&super::super::Security::Credentials::PasswordCredential>) -> windows_core::Result<()>;
    fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: Option<&super::super::Security::Credentials::PasswordCredential>) -> windows_core::Result<()>;
    fn SupportedProtocols(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl windows_core::RuntimeName for IWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl IWebSocketControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebSocketControl_Vtbl
    where
        Identity: IWebSocketControl_Impl,
    {
        unsafe extern "system" fn OutboundBufferSizeInBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketControl_Impl::OutboundBufferSizeInBytes(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebSocketControl_Impl::SetOutboundBufferSizeInBytes(this, value).into()
        }
        unsafe extern "system" fn ServerCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketControl_Impl::ServerCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebSocketControl_Impl::SetServerCredential(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn ProxyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketControl_Impl::ProxyCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebSocketControl_Impl::SetProxyCredential(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SupportedProtocols<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketControl_Impl::SupportedProtocols(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebSocketControl, OFFSET>(),
            OutboundBufferSizeInBytes: OutboundBufferSizeInBytes::<Identity, OFFSET>,
            SetOutboundBufferSizeInBytes: SetOutboundBufferSizeInBytes::<Identity, OFFSET>,
            ServerCredential: ServerCredential::<Identity, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, OFFSET>,
            SupportedProtocols: SupportedProtocols::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebSocketControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketControl2_Impl: Sized + IWebSocketControl_Impl {
    fn IgnorableServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl windows_core::RuntimeName for IWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebSocketControl2_Vtbl
    where
        Identity: IWebSocketControl2_Impl,
    {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketControl2_Impl::IgnorableServerCertificateErrors(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebSocketControl2, OFFSET>(),
            IgnorableServerCertificateErrors: IgnorableServerCertificateErrors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebSocketControl2 as windows_core::Interface>::IID
    }
}
pub trait IWebSocketInformation_Impl: Sized {
    fn LocalAddress(&self) -> windows_core::Result<super::HostName>;
    fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation";
}
impl IWebSocketInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebSocketInformation_Vtbl
    where
        Identity: IWebSocketInformation_Impl,
    {
        unsafe extern "system" fn LocalAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation_Impl::LocalAddress(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BandwidthStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BandwidthStatistics) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation_Impl::BandwidthStatistics(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation_Impl::Protocol(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebSocketInformation, OFFSET>(),
            LocalAddress: LocalAddress::<Identity, OFFSET>,
            BandwidthStatistics: BandwidthStatistics::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebSocketInformation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketInformation2_Impl: Sized + IWebSocketInformation_Impl {
    fn ServerCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl windows_core::RuntimeName for IWebSocketInformation2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketInformation2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebSocketInformation2_Vtbl
    where
        Identity: IWebSocketInformation2_Impl,
    {
        unsafe extern "system" fn ServerCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation2_Impl::ServerCertificate(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation2_Impl::ServerCertificateErrorSeverity(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation2_Impl::ServerCertificateErrors(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebSocketInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebSocketInformation2_Impl::ServerIntermediateCertificates(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebSocketInformation2, OFFSET>(),
            ServerCertificate: ServerCertificate::<Identity, OFFSET>,
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Identity, OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Identity, OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebSocketInformation2 as windows_core::Interface>::IID
    }
}

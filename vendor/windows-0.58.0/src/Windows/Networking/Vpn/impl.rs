pub trait IVpnChannelStatics_Impl: Sized {
    fn ProcessEventAsync(&self, thirdpartyplugin: Option<&windows_core::IInspectable>, event: Option<&windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVpnChannelStatics {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelStatics";
}
impl IVpnChannelStatics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnChannelStatics_Vtbl
    where
        Identity: IVpnChannelStatics_Impl,
    {
        unsafe extern "system" fn ProcessEventAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, thirdpartyplugin: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnChannelStatics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnChannelStatics_Impl::ProcessEventAsync(this, windows_core::from_raw_borrowed(&thirdpartyplugin), windows_core::from_raw_borrowed(&event)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnChannelStatics, OFFSET>(), ProcessEventAsync: ProcessEventAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnChannelStatics as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IVpnCredential_Impl: Sized {
    fn PasskeyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn OldPasswordCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl windows_core::RuntimeName for IVpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCredential";
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IVpnCredential_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnCredential_Vtbl
    where
        Identity: IVpnCredential_Impl,
    {
        unsafe extern "system" fn PasskeyCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnCredential_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCredential_Impl::PasskeyCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnCredential_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCredential_Impl::CertificateCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnCredential_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCredential_Impl::AdditionalPin(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnCredential_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCredential_Impl::OldPasswordCredential(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnCredential, OFFSET>(),
            PasskeyCredential: PasskeyCredential::<Identity, OFFSET>,
            CertificateCredential: CertificateCredential::<Identity, OFFSET>,
            AdditionalPin: AdditionalPin::<Identity, OFFSET>,
            OldPasswordCredential: OldPasswordCredential::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnCredential as windows_core::Interface>::IID
    }
}
pub trait IVpnCustomPrompt_Impl: Sized {
    fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> windows_core::Result<()>;
    fn Compulsory(&self) -> windows_core::Result<bool>;
    fn SetBordered(&self, value: bool) -> windows_core::Result<()>;
    fn Bordered(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IVpnCustomPrompt {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPrompt";
}
impl IVpnCustomPrompt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnCustomPrompt_Vtbl
    where
        Identity: IVpnCustomPrompt_Impl,
    {
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPrompt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnCustomPrompt_Impl::SetLabel(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPrompt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCustomPrompt_Impl::Label(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPrompt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnCustomPrompt_Impl::SetCompulsory(this, value).into()
        }
        unsafe extern "system" fn Compulsory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPrompt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCustomPrompt_Impl::Compulsory(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBordered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPrompt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnCustomPrompt_Impl::SetBordered(this, value).into()
        }
        unsafe extern "system" fn Bordered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPrompt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCustomPrompt_Impl::Bordered(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnCustomPrompt, OFFSET>(),
            SetLabel: SetLabel::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetCompulsory: SetCompulsory::<Identity, OFFSET>,
            Compulsory: Compulsory::<Identity, OFFSET>,
            SetBordered: SetBordered::<Identity, OFFSET>,
            Bordered: Bordered::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnCustomPrompt as windows_core::Interface>::IID
    }
}
pub trait IVpnCustomPromptElement_Impl: Sized {
    fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> windows_core::Result<()>;
    fn Compulsory(&self) -> windows_core::Result<bool>;
    fn SetEmphasized(&self, value: bool) -> windows_core::Result<()>;
    fn Emphasized(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IVpnCustomPromptElement {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptElement";
}
impl IVpnCustomPromptElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnCustomPromptElement_Vtbl
    where
        Identity: IVpnCustomPromptElement_Impl,
    {
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPromptElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnCustomPromptElement_Impl::SetDisplayName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPromptElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCustomPromptElement_Impl::DisplayName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPromptElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnCustomPromptElement_Impl::SetCompulsory(this, value).into()
        }
        unsafe extern "system" fn Compulsory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPromptElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCustomPromptElement_Impl::Compulsory(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmphasized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPromptElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnCustomPromptElement_Impl::SetEmphasized(this, value).into()
        }
        unsafe extern "system" fn Emphasized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVpnCustomPromptElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnCustomPromptElement_Impl::Emphasized(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnCustomPromptElement, OFFSET>(),
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetCompulsory: SetCompulsory::<Identity, OFFSET>,
            Compulsory: Compulsory::<Identity, OFFSET>,
            SetEmphasized: SetEmphasized::<Identity, OFFSET>,
            Emphasized: Emphasized::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnCustomPromptElement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnDomainNameInfoFactory_Impl: Sized {
    fn CreateVpnDomainNameInfo(&self, name: &windows_core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: Option<&super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: Option<&super::super::Foundation::Collections::IIterable<super::HostName>>) -> windows_core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IVpnDomainNameInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnDomainNameInfoFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnDomainNameInfoFactory_Vtbl
    where
        Identity: IVpnDomainNameInfoFactory_Impl,
    {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: *mut core::ffi::c_void, proxyserverlist: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnDomainNameInfoFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnDomainNameInfoFactory_Impl::CreateVpnDomainNameInfo(this, core::mem::transmute(&name), nametype, windows_core::from_raw_borrowed(&dnsserverlist), windows_core::from_raw_borrowed(&proxyserverlist)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnDomainNameInfoFactory, OFFSET>(),
            CreateVpnDomainNameInfo: CreateVpnDomainNameInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnDomainNameInfoFactory as windows_core::Interface>::IID
    }
}
pub trait IVpnInterfaceIdFactory_Impl: Sized {
    fn CreateVpnInterfaceId(&self, address: &[u8]) -> windows_core::Result<VpnInterfaceId>;
}
impl windows_core::RuntimeName for IVpnInterfaceIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceIdFactory";
}
impl IVpnInterfaceIdFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnInterfaceIdFactory_Vtbl
    where
        Identity: IVpnInterfaceIdFactory_Impl,
    {
        unsafe extern "system" fn CreateVpnInterfaceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnInterfaceIdFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnInterfaceIdFactory_Impl::CreateVpnInterfaceId(this, core::slice::from_raw_parts(core::mem::transmute_copy(&address), address_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnInterfaceIdFactory, OFFSET>(),
            CreateVpnInterfaceId: CreateVpnInterfaceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnInterfaceIdFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnNamespaceInfoFactory_Impl: Sized {
    fn CreateVpnNamespaceInfo(&self, name: &windows_core::HSTRING, dnsserverlist: Option<&super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: Option<&super::super::Foundation::Collections::IVector<super::HostName>>) -> windows_core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IVpnNamespaceInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnNamespaceInfoFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnNamespaceInfoFactory_Vtbl
    where
        Identity: IVpnNamespaceInfoFactory_Impl,
    {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, dnsserverlist: *mut core::ffi::c_void, proxyserverlist: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnNamespaceInfoFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnNamespaceInfoFactory_Impl::CreateVpnNamespaceInfo(this, core::mem::transmute(&name), windows_core::from_raw_borrowed(&dnsserverlist), windows_core::from_raw_borrowed(&proxyserverlist)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnNamespaceInfoFactory, OFFSET>(),
            CreateVpnNamespaceInfo: CreateVpnNamespaceInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnNamespaceInfoFactory as windows_core::Interface>::IID
    }
}
pub trait IVpnPacketBufferFactory_Impl: Sized {
    fn CreateVpnPacketBuffer(&self, parentbuffer: Option<&VpnPacketBuffer>, offset: u32, length: u32) -> windows_core::Result<VpnPacketBuffer>;
}
impl windows_core::RuntimeName for IVpnPacketBufferFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferFactory";
}
impl IVpnPacketBufferFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnPacketBufferFactory_Vtbl
    where
        Identity: IVpnPacketBufferFactory_Impl,
    {
        unsafe extern "system" fn CreateVpnPacketBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentbuffer: *mut core::ffi::c_void, offset: u32, length: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPacketBufferFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnPacketBufferFactory_Impl::CreateVpnPacketBuffer(this, windows_core::from_raw_borrowed(&parentbuffer), offset, length) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnPacketBufferFactory, OFFSET>(),
            CreateVpnPacketBuffer: CreateVpnPacketBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnPacketBufferFactory as windows_core::Interface>::IID
    }
}
pub trait IVpnPlugIn_Impl: Sized {
    fn Connect(&self, channel: Option<&VpnChannel>) -> windows_core::Result<()>;
    fn Disconnect(&self, channel: Option<&VpnChannel>) -> windows_core::Result<()>;
    fn GetKeepAlivePayload(&self, channel: Option<&VpnChannel>, keepalivepacket: &mut Option<VpnPacketBuffer>) -> windows_core::Result<()>;
    fn Encapsulate(&self, channel: Option<&VpnChannel>, packets: Option<&VpnPacketBufferList>, encapulatedpackets: Option<&VpnPacketBufferList>) -> windows_core::Result<()>;
    fn Decapsulate(&self, channel: Option<&VpnChannel>, encapbuffer: Option<&VpnPacketBuffer>, decapsulatedpackets: Option<&VpnPacketBufferList>, controlpacketstosend: Option<&VpnPacketBufferList>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVpnPlugIn {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugIn";
}
impl IVpnPlugIn_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnPlugIn_Vtbl
    where
        Identity: IVpnPlugIn_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPlugIn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnPlugIn_Impl::Connect(this, windows_core::from_raw_borrowed(&channel)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPlugIn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnPlugIn_Impl::Disconnect(this, windows_core::from_raw_borrowed(&channel)).into()
        }
        unsafe extern "system" fn GetKeepAlivePayload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: *mut core::ffi::c_void, keepalivepacket: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPlugIn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnPlugIn_Impl::GetKeepAlivePayload(this, windows_core::from_raw_borrowed(&channel), core::mem::transmute_copy(&keepalivepacket)).into()
        }
        unsafe extern "system" fn Encapsulate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: *mut core::ffi::c_void, packets: *mut core::ffi::c_void, encapulatedpackets: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPlugIn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnPlugIn_Impl::Encapsulate(this, windows_core::from_raw_borrowed(&channel), windows_core::from_raw_borrowed(&packets), windows_core::from_raw_borrowed(&encapulatedpackets)).into()
        }
        unsafe extern "system" fn Decapsulate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: *mut core::ffi::c_void, encapbuffer: *mut core::ffi::c_void, decapsulatedpackets: *mut core::ffi::c_void, controlpacketstosend: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPlugIn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnPlugIn_Impl::Decapsulate(this, windows_core::from_raw_borrowed(&channel), windows_core::from_raw_borrowed(&encapbuffer), windows_core::from_raw_borrowed(&decapsulatedpackets), windows_core::from_raw_borrowed(&controlpacketstosend)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnPlugIn, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            GetKeepAlivePayload: GetKeepAlivePayload::<Identity, OFFSET>,
            Encapsulate: Encapsulate::<Identity, OFFSET>,
            Decapsulate: Decapsulate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnPlugIn as windows_core::Interface>::IID
    }
}
pub trait IVpnPlugInReconnectTransport_Impl: Sized {
    fn ReconnectTransport(&self, channel: Option<&VpnChannel>, context: Option<&windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVpnPlugInReconnectTransport {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugInReconnectTransport";
}
impl IVpnPlugInReconnectTransport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnPlugInReconnectTransport_Vtbl
    where
        Identity: IVpnPlugInReconnectTransport_Impl,
    {
        unsafe extern "system" fn ReconnectTransport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channel: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnPlugInReconnectTransport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnPlugInReconnectTransport_Impl::ReconnectTransport(this, windows_core::from_raw_borrowed(&channel), windows_core::from_raw_borrowed(&context)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnPlugInReconnectTransport, OFFSET>(),
            ReconnectTransport: ReconnectTransport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnPlugInReconnectTransport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnProfile_Impl: Sized {
    fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetProfileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn AppTriggers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(&self) -> windows_core::Result<bool>;
    fn SetRememberCredentials(&self, value: bool) -> windows_core::Result<()>;
    fn AlwaysOn(&self) -> windows_core::Result<bool>;
    fn SetAlwaysOn(&self, value: bool) -> windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IVpnProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnProfile";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnProfile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnProfile_Vtbl
    where
        Identity: IVpnProfile_Impl,
    {
        unsafe extern "system" fn ProfileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::ProfileName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnProfile_Impl::SetProfileName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AppTriggers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::AppTriggers(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::Routes(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainNameInfoList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::DomainNameInfoList(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFilters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::TrafficFilters(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RememberCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::RememberCredentials(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRememberCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnProfile_Impl::SetRememberCredentials(this, value).into()
        }
        unsafe extern "system" fn AlwaysOn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnProfile_Impl::AlwaysOn(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysOn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IVpnProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVpnProfile_Impl::SetAlwaysOn(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnProfile, OFFSET>(),
            ProfileName: ProfileName::<Identity, OFFSET>,
            SetProfileName: SetProfileName::<Identity, OFFSET>,
            AppTriggers: AppTriggers::<Identity, OFFSET>,
            Routes: Routes::<Identity, OFFSET>,
            DomainNameInfoList: DomainNameInfoList::<Identity, OFFSET>,
            TrafficFilters: TrafficFilters::<Identity, OFFSET>,
            RememberCredentials: RememberCredentials::<Identity, OFFSET>,
            SetRememberCredentials: SetRememberCredentials::<Identity, OFFSET>,
            AlwaysOn: AlwaysOn::<Identity, OFFSET>,
            SetAlwaysOn: SetAlwaysOn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnProfile as windows_core::Interface>::IID
    }
}
pub trait IVpnRouteFactory_Impl: Sized {
    fn CreateVpnRoute(&self, address: Option<&super::HostName>, prefixsize: u8) -> windows_core::Result<VpnRoute>;
}
impl windows_core::RuntimeName for IVpnRouteFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteFactory";
}
impl IVpnRouteFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVpnRouteFactory_Vtbl
    where
        Identity: IVpnRouteFactory_Impl,
    {
        unsafe extern "system" fn CreateVpnRoute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: *mut core::ffi::c_void, prefixsize: u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVpnRouteFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVpnRouteFactory_Impl::CreateVpnRoute(this, windows_core::from_raw_borrowed(&address), prefixsize) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVpnRouteFactory, OFFSET>(), CreateVpnRoute: CreateVpnRoute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVpnRouteFactory as windows_core::Interface>::IID
    }
}
